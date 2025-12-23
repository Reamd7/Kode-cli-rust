# 设计文档 - Agent 系统 / Design Document - Agent System

## Context

Agent 系统是 Kode-Rust 的核心功能之一，允许用户创建专门化的 AI 助手来处理特定任务。

The Agent system is one of the core features of Kode-Rust, allowing users to create specialized AI assistants for specific tasks.

### 当前状态 / Current State

Agent 系统计划在 `crates/kode-core/src/agent/` 中实现，尚未开始实现。

The Agent system is planned to be implemented in `crates/kode-core/src/agent/`, implementation has not started yet.

## Goals / Non-Goals

### Goals

- [ ] 定义 Agent 结构体 / Define Agent struct
- [ ] 实现 AgentLoader / Implement AgentLoader
- [ ] 实现 YAML frontmatter 解析 / Implement YAML frontmatter parsing
- [ ] 实现五层目录加载 / Implement five-layer directory loading
- [ ] 实现 LRU 缓存 / Implement LRU cache

### Non-Goals

- [ ] Agent 热重载 / Agent hot reloading
- [ ] Agent 依赖管理 / Agent dependency management
- [ ] Agent 市场 / Agent marketplace

## Decisions

### 决策 1: Agent 结构设计 / Decision 1: Agent Structure Design

**选择**: 使用结构体表示 Agent，支持可选字段

**Choice**: Use struct to represent Agent with optional fields

**理由 / Rationale**:
- 类型安全 / Type-safe
- 清晰的字段语义 / Clear field semantics
- 易于序列化 / Easy to serialize

**实现 / Implementation**:
```rust
/// Agent 定义
/// 来自 ARCHITECTURE.md L113-119
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub name: String,
    pub description: String,
    pub tools: ToolFilter,
    pub model: Option<String>,
    pub system_prompt: String,
}

/// 工具过滤器
/// 来自 ARCHITECTURE.md L121-124
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolFilter {
    All,
    Specific(Vec<String>),
}

/// Agent 加载器
/// 来自 ARCHITECTURE.md L126-140
pub struct AgentLoader {
    cache: LruCache<PathBuf, Agent>,
    directories: Vec<PathBuf>,
}

impl AgentLoader {
    // 5层优先级加载
    // 1. Built-in
    // 2. ~/.claude/agents/
    // 3. ~/.kode/agents/
    // 4. ./.claude/agents/
    // 5. ./.kode/agents/
    pub async fn load_agents(&mut self) -> Result<Vec<Agent>>;
    pub async fn get_agent(&mut self, name: &str) -> Result<Agent>;
}
```

### 决策 2: Agent 文件格式 / Decision 2: Agent File Format

**选择**: Markdown + YAML frontmatter

**Choice**: Markdown + YAML frontmatter

**理由 / Rationale**:
- 与 TypeScript 版本兼容 / Compatible with TypeScript version
- 易读易写 / Easy to read and write
- 支持详细的文档 / Support detailed documentation

**示例 / Example**:
```markdown
---
name: code-reviewer
description: "Reviews code for best practices and potential issues"
tools:
  - FileRead
  - Grep
  - Bash
model: claude-sonnet
---

You are an expert code reviewer. When reviewing code:
1. Check for security vulnerabilities
2. Look for performance issues
3. Ensure code follows best practices
...
```

### 决策 3: 五层目录加载 / Decision 3: Five-Layer Directory Loading

**选择**: 按优先级顺序搜索，第一个找到的生效

**Choice**: Search in priority order, first found takes effect

**理由 / Rationale**:
- 允许内置 Agent 被覆盖 / Allow built-in agents to be overridden
- 支持全局和项目级 Agent / Support global and project-level agents
- 与 Claude Code 兼容 / Compatible with Claude Code

**设计 / Design**:
```rust
use dirs::home_dir;
use std::path::PathBuf;

pub struct AgentLoader {
    cache: LruCache<String, Agent>,
    search_paths: Vec<PathBuf>,
}

impl AgentLoader {
    pub fn new() -> Result<Self> {
        let mut search_paths = Vec::new();

        // 1. Built-in agents（编译到二进制中）
        search_paths.push(PathBuf::from("builtin:/agents"));

        // 2. ~/.claude/agents/
        if let Some(home) = home_dir() {
            search_paths.push(home.join(".claude/agents"));
        }

        // 3. ~/.kode/agents/
        if let Some(home) = home_dir() {
            search_paths.push(home.join(".kode/agents"));
        }

        // 4. ./.claude/agents/
        if let Ok(current) = std::env::current_dir() {
            search_paths.push(current.join(".claude/agents"));
        }

        // 5. ./.kode/agents/
        if let Ok(current) = std::env::current_dir() {
            search_paths.push(current.join(".kode/agents"));
        }

        Ok(Self {
            cache: LruCache::new(100),
            search_paths,
        })
    }

    /// 加载指定名称的 Agent
    pub async fn load_agent(&mut self, name: &str) -> Result<Agent> {
        // 首先检查缓存
        if let Some(agent) = self.cache.get(name) {
            return Ok(agent.clone());
        }

        // 在所有搜索路径中查找
        for path in &self.search_paths {
            if let Some(agent) = self.try_load_from_path(path, name).await? {
                self.cache.put(name.to_string(), agent.clone());
                return Ok(agent);
            }
        }

        Err(Error::AgentNotFound(name.to_string()))
    }

    /// 从指定路径加载 Agent
    async fn try_load_from_path(&self, base: &Path, name: &str) -> Result<Option<Agent>> {
        // 处理内置 Agent
        if base.to_str() == Some("builtin:/agents") {
            return Ok(self.load_builtin_agent(name));
        }

        // 从文件系统加载
        let agent_file = base.join(format!("{}.md", name));
        if agent_file.exists() {
            let content = tokio::fs::read_to_string(&agent_file).await?;
            let agent = self.parse_agent(&content)?;
            return Ok(Some(agent));
        }

        Ok(None)
    }

    /// 加载内置 Agent
    fn load_builtin_agent(&self, name: &str) -> Option<Agent> {
        match name {
            "general-purpose" => Some(Agent {
                name: "general-purpose".to_string(),
                description: Some("General purpose AI assistant".to_string()),
                tools: ToolFilter::All("all".to_string()),
                model: None,
                system_prompt: "You are a helpful AI assistant.".to_string(),
            }),
            _ => None,
        }
    }

    /// 列出所有可用的 Agent
    pub async fn list_agents(&self) -> Result<Vec<Agent>> {
        let mut agents = Vec::new();
        let mut seen_names = std::collections::HashSet::new();

        // 扫描所有路径
        for path in &self.search_paths {
            if let Ok(entries) = self.scan_directory(path).await {
                for agent in entries {
                    if !seen_names.contains(&agent.name) {
                        seen_names.insert(agent.name.clone());
                        agents.push(agent);
                    }
                }
            }
        }

        Ok(agents)
    }

    async fn scan_directory(&self, path: &Path) -> Result<Vec<Agent>> {
        let mut agents = Vec::new();

        if !path.exists() {
            return Ok(agents);
        }

        let mut entries = tokio::fs::read_dir(path).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                let content = tokio::fs::read_to_string(&path).await?;
                if let Ok(agent) = self.parse_agent(&content) {
                    agents.push(agent);
                }
            }
        }

        Ok(agents)
    }

    /// 解析 Agent 定义
    fn parse_agent(&self, content: &str) -> Result<Agent> {
        // 分离 YAML frontmatter 和 Markdown 正文
        let parts: Vec<&str> = content.splitn(3, "---").collect();
        if parts.len() < 3 {
            return Err(Error::InvalidAgentFormat);
        }

        let yaml_content = parts[1].trim();
        let markdown_body = parts[2].trim();

        // 解析 YAML
        let yaml: serde_yaml::Value = serde_yaml::from_str(yaml_content)
            .map_err(|e| Error::AgentParseError(e.to_string()))?;

        let name = yaml["name"].as_str()
            .ok_or_else(|| Error::AgentParseError("Missing name field".to_string()))?
            .to_string();

        let description = yaml["description"].as_str().map(|s| s.to_string());

        let tools = if let Some(tools_value) = yaml.get("tools") {
            if let Some(s) = tools_value.as_str() {
                if s == "all" {
                    ToolFilter::All("all".to_string())
                } else {
                    return Err(Error::AgentParseError("Invalid tools value".to_string()));
                }
            } else if let Some(arr) = tools_value.as_sequence() {
                let tools = arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();
                ToolFilter::Specific(tools)
            } else {
                return Err(Error::AgentParseError("Invalid tools format".to_string()));
            }
        } else {
            ToolFilter::All("all".to_string())  // 默认所有工具
        };

        let model = yaml["model"].as_str().map(|s| s.to_string());

        Ok(Agent {
            name,
            description,
            tools,
            model,
            system_prompt: markdown_body.to_string(),
        })
    }
}
```

### 决策 4: LRU 缓存配置 / Decision 4: LRU Cache Configuration

**选择**: 使用 lru crate，缓存大小为 100

**Choice**: Use lru crate with cache size of 100

**理由 / Rationale**:
- 100 个 Agent 足够大多数使用场景 / 100 agents is sufficient for most use cases
- LRU 自动淘汰不常用的 Agent / LRU automatically evicts infrequently used agents
- 减少文件 I/O / Reduce file I/O

### 决策 5: YAML 解析库选择 / Decision 5: YAML Parsing Library Selection

**选择**: 使用 serde_yaml

**Choice**: Use serde_yaml

**理由 / Rationale**:
- 与 serde 生态集成 / Integrated with serde ecosystem
- 成熟稳定 / Mature and stable
- yaml-rust2 的维护版本 / Maintained version of yaml-rust

## 数据流 / Data Flow

```
用户请求 Agent / User requests Agent
    ↓
AgentLoader::load_agent(name)
    ↓
检查缓存 / Check cache
    ↓ (未命中 / miss)
遍历 search_paths / Iterate through search_paths
    ↓
找到文件 / Find file
    ↓
读取文件内容 / Read file content
    ↓
解析 YAML frontmatter / Parse YAML frontmatter
    ↓
提取 Markdown 正文 / Extract Markdown body
    ↓
创建 Agent 结构体 / Create Agent struct
    ↓
存入缓存 / Store in cache
    ↓
返回 Agent / Return Agent
```

## Agent 定义示例 / Agent Definition Examples

### 通用助手 / General Purpose
```markdown
---
name: general-purpose
description: "General purpose AI assistant"
tools: all
---

You are a helpful AI assistant. You can help with various tasks including:
- Answering questions
- Writing code
- Analyzing problems
- Providing explanations
```

### 代码审查者 / Code Reviewer
```markdown
---
name: code-reviewer
description: "Reviews code for best practices and potential issues"
tools:
  - FileRead
  - Grep
  - Bash
model: claude-sonnet
---

You are an expert code reviewer. When reviewing code:
1. Check for security vulnerabilities
2. Look for performance issues
3. Ensure code follows best practices
4. Verify error handling
5. Check for potential bugs

Always provide constructive feedback and suggest improvements.
```

### 文档编写者 / Documentation Writer
```markdown
---
name: docs-writer
description: "Writes and formats documentation"
tools:
  - FileRead
  - FileWrite
  - Glob
---

You are a technical writer specializing in clear, concise documentation.
When writing documentation:
- Use clear and simple language
- Include code examples
- Organize content logically
- Follow markdown best practices
```

## 错误处理 / Error Handling

| 错误类型 / Error Type | 描述 / Description | 处理方式 / Handling |
|---------------------|-------------------|-------------------|
| `AgentNotFound` | 请求的 Agent 不存在 / Requested Agent does not exist | 列出可用 Agent / List available Agents |
| `InvalidAgentFormat` | 文件格式无效 / Invalid file format | 指出格式错误 / Point out format error |
| `AgentParseError` | YAML 解析失败 / YAML parsing failed | 显示解析错误详情 / Show parsing error details |

## 测试策略 / Testing Strategy

### 单元测试 / Unit Tests

```rust
#[tokio::test]
async fn test_parse_agent() {
    let loader = AgentLoader::new().unwrap();
    let content = r#"---
name: test-agent
tools: all
---

Test prompt"#;

    let agent = loader.parse_agent(content).unwrap();
    assert_eq!(agent.name, "test-agent");
    assert!(matches!(agent.tools, ToolFilter::All(_)));
}

#[tokio::test]
async fn test_agent_cache() {
    let mut loader = AgentLoader::new().unwrap();
    // 第一次加载
    let agent1 = loader.load_agent("general-purpose").await.unwrap();
    // 第二次应该从缓存加载
    let agent2 = loader.load_agent("general-purpose").await.unwrap();
    assert_eq!(agent1.name, agent2.name);
}
```

## Risks / Trade-offs

### 风险 1: YAML 注入 / Risk 1: YAML Injection

**风险 / Risk**: 恶意的 Agent 文件可能包含危险的 YAML 内容

**缓解措施 / Mitigation**:
- 仅在本地执行 Agent / Only execute agents locally
- 用户控制 Agent 来源 / Users control Agent sources
- 考虑添加 Agent 签名验证 / Consider adding Agent signature verification

### 风险 2: 缓存一致性问题 / Risk 2: Cache Consistency Issue

**风险 / Risk**: Agent 文件修改后缓存可能过期

**缓解措施 / Mitigation**:
- 提供清除缓存的命令 / Provide cache clearing command
- 考虑添加文件修改时间检查 / Consider adding file modification time checking
- 限制缓存大小以减少影响 / Limit cache size to reduce impact

## Open Questions

1. **是否需要支持子目录组织 Agent？**
   - 建议：支持，允许 Agent 分类
   - Rationale: Yes, allow Agent categorization

2. **是否需要 Agent 依赖管理？**
   - 建议：暂不实现，保持简单
   - Rationale: Don't implement for now, keep it simple

3. **是否需要 Agent 模板？**
   - 建议：未来添加，简化 Agent 创建
   - Rationale: Add in the future to simplify Agent creation

## 参考资源 / References

- [pulldown-cmark 文档](https://docs.rs/pulldown-cmark/)
- [yaml-rust2 文档](https://docs.rs/yaml-rust2/)
- [lru crate 文档](https://docs.rs/lru/)
- [TOML 配置格式](https://toml.io/)
