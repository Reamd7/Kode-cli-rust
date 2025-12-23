# 设计文档 - 工具系统 / Design Document - Tool System

## Context

工具系统是 Kode-Rust 的核心功能之一，允许 AI Agent 执行各种操作（文件、命令、搜索等）来完成任务。

The tool system is one of the core features of Kode-Rust, allowing AI agents to perform various operations (files, commands, search, etc.) to accomplish tasks.

### 当前状态 / Current State

工具系统计划在 `crates/kode-tools/` 中实现，尚未开始实现。

The tool system is planned to be implemented in `crates/kode-tools/`, implementation has not started yet.

## Goals / Non-Goals

### Goals

- [ ] 定义 Tool trait / Define Tool trait
- [ ] 实现 ToolRegistry / Implement ToolRegistry
- [ ] 实现核心工具（FileRead, FileWrite, FileEdit, Bash, Grep, Glob, Task）/ Implement core tools
- [ ] 支持工具权限检查 / Support tool permission checking
- [ ] 支持工具参数验证 / Support tool parameter validation

### Non-Goals

- [ ] 工具执行的并发优化 / Concurrent optimization of tool execution
- [ ] 工具结果缓存 / Tool result caching
- [ ] 工具执行历史记录 / Tool execution history

## Decisions

### 决策 1: Tool trait 设计 / Decision 1: Tool Trait Design

**选择**: 使用 async-trait 定义异步 Tool trait

**Choice**: Use async-trait to define async Tool trait

**理由 / Rationale**:
- 工具执行可能是 I/O 密集型 / Tool execution can be I/O intensive
- 与项目异步架构一致 / Consistent with project async architecture
- 生态标准 / Ecosystem standard

**实现 / Implementation**:
```rust
/// 来自 ARCHITECTURE.md L206-221
use async_trait::async_trait;

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn schema(&self) -> ToolSchema;

    async fn execute(
        &self,
        params: Value,
        context: &ToolContext,
    ) -> Result<ToolResult>;

    fn requires_permission(&self) -> bool {
        false
    }
}

/// 来自 ARCHITECTURE.md L223-227
pub struct ToolSchema {
    pub name: String,
    pub description: String,
    pub parameters: JsonSchema,
}

/// 来自 ARCHITECTURE.md L229-232
pub struct ToolContext {
    pub working_dir: PathBuf,
    pub permissions: Arc<PermissionManager>,
}

/// 来自 ARCHITECTURE.md L234-237
pub struct ToolResult {
    pub output: String,
    pub artifacts: Vec<Artifact>,
}
```

### 决策 2: 工具注册表设计 / Decision 2: Tool Registry Design

**选择**: 使用 HashMap 存储工具，使用 Arc<dyn Tool> 实现动态分发

**Choice**: Use HashMap to store tools, use Arc<dyn Tool> for dynamic dispatch

**理由 / Rationale**:
- O(1) 查找性能 / O(1) lookup performance
- 支持运行时添加工具 / Support runtime tool addition
- Arc 允许工具共享 / Arc allows tool sharing

**实现 / Implementation**:
```rust
/// 来自 ARCHITECTURE.md L242-251
pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn Tool>>,
}

impl ToolRegistry {
    pub fn register(&mut self, tool: Arc<dyn Tool>);
    pub fn get(&self, name: &str) -> Option<Arc<dyn Tool>>;
    pub fn list(&self) -> Vec<&str>;
    pub fn filter(&self, names: &[String]) -> Vec<Arc<dyn Tool>>;
}
```

#### 核心工具实现
来自 ARCHITECTURE.md L254-261:
- `BashTool`: 执行 shell 命令
- `FileReadTool`: 读取文件
- `FileWriteTool`: 写入文件
- `FileEditTool`: 编辑文件（精确替换）
- `GrepTool`: 内容搜索（基于 ripgrep）
- `GlobTool`: 文件模式匹配
- `TaskTool`: 委托任务给子 Agent

### 决策 3: 工具参数验证 / Decision 3: Tool Parameter Validation

**选择**: 使用 JSON Schema 进行参数验证

**Choice**: Use JSON Schema for parameter validation

**理由 / Rationale**:
- 与 AI API 工具定义格式一致 / Consistent with AI API tool definition format
- 自动生成文档 / Automatic documentation generation
- 类型安全 / Type-safe

**设计 / Design**:
```rust
pub type JsonSchema = serde_json::Value;

/// 工具 schema（用于传递给 AI）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolSchema {
    pub name: String,
    pub description: String,
    pub parameters: JsonSchema,
}

/// JSON Schema 构建器
pub struct SchemaBuilder;

impl SchemaBuilder {
    pub fn object(properties: HashMap<&str, JsonSchema>) -> JsonSchema {
        json!({
            "type": "object",
            "properties": properties,
        })
    }

    pub fn string(description: &str) -> JsonSchema {
        json!({
            "type": "string",
            "description": description
        })
    }

    pub fn required(schema: JsonSchema, required: &[&str]) -> JsonSchema {
        let mut schema = schema;
        if let Some(obj) = schema.as_object_mut() {
            obj.insert("required".to_string(), json!(required));
        }
        schema
    }
}
```

### 决策 4: FileReadTool 实现 / Decision 4: FileReadTool Implementation

**设计 / Design**:
```rust
pub struct FileReadTool;

#[async_trait]
impl Tool for FileReadTool {
    fn name(&self) -> &str {
        "read_file"
    }

    fn description(&self) -> &str {
        "Read the contents of a file"
    }

    fn schema(&self) -> JsonSchema {
        SchemaBuilder::required(
            SchemaBuilder::object(maplit::hashmap!{
                "path" => SchemaBuilder::string("The file path to read"),
            }),
            &["path"],
        )
    }

    async fn execute(&self, params: Value, context: &ToolContext) -> Result<ToolResult> {
        let path = params["path"].as_str().ok_or_else(|| {
            Error::InvalidParams("path is required".to_string())
        })?;

        let full_path = context.working_dir.join(path);
        let content = tokio::fs::read_to_string(&full_path).await.map_err(|e| {
            Error::FileReadError {
                path: full_path.clone(),
                message: e.to_string(),
            }
        })?;

        Ok(ToolResult {
            output: content,
            artifacts: vec![],
        })
    }
}
```

### 决策 5: BashTool 实现 / Decision 5: BashTool Implementation

**设计 / Design**:
```rust
pub struct BashTool {
    timeout: Duration,
}

#[async_trait]
impl Tool for BashTool {
    fn name(&self) -> &str {
        "bash"
    }

    fn description(&self) -> &str {
        "Execute a shell command"
    }

    fn schema(&self) -> JsonSchema {
        SchemaBuilder::required(
            SchemaBuilder::object(maplit::hashmap!{
                "command" => SchemaBuilder::string("The shell command to execute"),
            }),
            &["command"],
        )
    }

    fn requires_permission(&self) -> bool {
        true  // 命令执行需要用户确认
    }

    async fn execute(&self, params: Value, context: &ToolContext) -> Result<ToolResult> {
        let command = params["command"].as_str().ok_or_else(|| {
            Error::InvalidParams("command is required".to_string())
        })?;

        // 权限检查
        context.permissions.request_command_execution(command).await?;

        // 执行命令
        let output = tokio::time::timeout(
            self.timeout,
            tokio::process::Command::new("sh")
                .arg("-c")
                .arg(command)
                .current_dir(&context.working_dir)
                .output()
        ).await.map_err(|_| Error::CommandTimeout)?;

        let output = output.map_err(|e| Error::CommandExecutionError {
            command: command.to_string(),
            message: e.to_string(),
        })?;

        Ok(ToolResult {
            output: String::from_utf8_lossy(&output.stdout).to_string(),
            artifacts: vec![],
        })
    }
}
```

### 决策 6: TaskTool 实现 / Decision 6: TaskTool Implementation

**设计 / Design**:
```rust
pub struct TaskTool {
    agent_loader: Arc<AgentLoader>,
    model_manager: Arc<ModelManager>,
    tool_registry: Arc<ToolRegistry>,
}

#[async_trait]
impl Tool for TaskTool {
    fn name(&self) -> &str {
        "task"
    }

    fn description(&self) -> &str {
        "Delegate a task to a sub-agent"
    }

    fn schema(&self) -> JsonSchema {
        SchemaBuilder::required(
            SchemaBuilder::object(maplit::hashmap!{
                "agent" => SchemaBuilder::string("The name of the sub-agent"),
                "prompt" => SchemaBuilder::string("The task description"),
            }),
            &["agent", "prompt"],
        )
    }

    async fn execute(&self, params: Value, context: &ToolContext) -> Result<ToolResult> {
        let agent_name = params["agent"].as_str().ok_or_else(|| {
            Error::InvalidParams("agent is required".to_string())
        })?;
        let prompt = params["prompt"].as_str().ok_or_else(|| {
            Error::InvalidParams("prompt is required".to_string())
        })?;

        // 加载子 Agent
        let agent = self.agent_loader.load_agent(agent_name).await?;

        // 过滤工具集
        let filtered_tools = match &agent.tools {
            ToolFilter::All => self.tool_registry.get_all(),
            ToolFilter::Specific(names) => self.tool_registry.filter(names),
        };

        // 创建子上下文
        let mut messages = vec![
            Message {
                role: MessageRole::System,
                content: vec![ContentBlock::Text(TextBlock {
                    text: agent.system_prompt.clone(),
                })],
            },
            Message {
                role: MessageRole::User,
                content: vec![ContentBlock::Text(TextBlock {
                    text: prompt.to_string(),
                })],
            },
        ];

        // 调用模型
        let response = self.model_manager.send_message_with_tools(
            messages,
            filtered_tools,
        ).await?;

        Ok(ToolResult {
            output: response.to_string(),
            artifacts: vec![],
        })
    }
}
```

## 数据流 / Data Flow

```
AI 调用工具 / AI calls tool
    ↓
ToolRegistry::get(tool_name)
    ↓
Tool::execute(params, context)
    ↓ (如果需要权限 / if permission required)
PermissionManager::request_permission()
    ↓ (用户批准 / user approves)
Tool 实际执行 / Tool actual execution
    ↓
返回 ToolResult / Return ToolResult
    ↓
转换为 ToolResult 消息 / Convert to ToolResult message
    ↓
发送回 AI / Send back to AI
```

## 工具列表 / Tool List

| 工具名称 / Tool Name | 描述 / Description | 需要权限 / Requires Permission |
|---------------------|-------------------|------------------------------|
| `read_file` | 读取文件内容 | No |
| `write_file` | 写入文件 | Yes |
| `edit_file` | 编辑文件 | Yes |
| `bash` | 执行 shell 命令 | Yes |
| `grep` | 搜索文件内容 | No |
| `glob` | 匹配文件模式 | No |
| `task` | 委托子任务 | No |

## 错误处理 / Error Handling

| 错误类型 / Error Type | 描述 / Description | 处理方式 / Handling |
|---------------------|-------------------|-------------------|
| `ToolNotFound` | 请求的工具不存在 / Requested tool does not exist | 返回可用工具列表 / Return list of available tools |
| `InvalidParams` | 工具参数无效 / Invalid tool parameters | 返回参数验证错误 / Return parameter validation error |
| `PermissionDenied` | 用户拒绝执行权限 / User denied execution permission | 返回取消状态 / Return cancellation status |
| `FileReadError` | 文件读取失败 / File read failure | 返回详细错误信息 / Return detailed error information |
| `CommandExecutionError` | 命令执行失败 / Command execution failure | 返回 stderr 内容 / Return stderr content |

## 测试策略 / Testing Strategy

### 单元测试 / Unit Tests

```rust
#[tokio::test]
async fn test_file_read_tool() {
    let tool = FileReadTool;
    let context = ToolContext::test_context();

    let result = tool.execute(
        json!({"path": "test.txt"}),
        &context,
    ).await.unwrap();

    assert!(result.output.contains("test content"));
}

#[tokio::test]
async fn test_tool_registry() {
    let mut registry = ToolRegistry::new();
    registry.register(Arc::new(FileReadTool)).unwrap();

    assert_eq!(registry.list().len(), 1);
    assert!(registry.get("read_file").is_some());
}
```

## Risks / Trade-offs

### 风险 1: 命令注入 / Risk 1: Command Injection

**风险 / Risk**: 恶意输入可能导致命令注入攻击

**缓解措施 / Mitigation**:
- 权限系统要求用户确认危险操作 / Permission system requires user confirmation for dangerous operations
- 显示将要执行的完整命令 / Display the complete command to be executed
- 考虑添加命令白名单 / Consider adding command whitelist

### 风险 2: 文件路径遍历 / Risk 2: File Path Traversal

**风险 / Risk**: 恶意路径可能访问系统敏感文件

**缓解措施 / Mitigation**:
- 限制工作目录范围 / Restrict working directory scope
- 验证路径不包含 .. / Validate paths don't contain ..
- 权限检查 / Permission checking

## Open Questions

1. **是否需要工具执行超时？**
   - 建议：是，默认 30 秒超时
   - Rationale: Yes, 30 second default timeout

2. **是否需要工具结果缓存？**
   - 建议：暂不实现，后续考虑性能优化
   - Rationale: Don't implement for now, consider for performance optimization later

3. **是否需要支持工具组合（pipeline）？**
   - 建议：暂不实现，通过 AI 调用序列实现
   - Rationale: Don't implement for now, achieve through AI call sequence

## 参考资源 / References

- [async-trait 文档](https://docs.rs/async-trait/)
- [serde_json 文档](https://docs.rs/serde_json/)
- [tokio::process 文档](https://tokio.rs/tokio/process/)
- [regex crate 文档](https://docs.rs/regex/)
- [glob crate 文档](https://docs.rs/glob/)
