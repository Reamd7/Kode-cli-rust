# 设计文档 - MCP 集成 / Design Document - MCP Integration

## Context

MCP (Model Context Protocol) 是一个开放协议，允许 AI 应用连接到外部数据源和工具。Kode-Rust 需要实现 MCP 客户端以支持扩展工具集。

MCP (Model Context Protocol) is an open protocol that allows AI applications to connect to external data sources and tools. Kode-Rust needs to implement an MCP client to support an extensible tool set.

### 当前状态 / Current State

MCP 集成计划在 `crates/kode-services/src/mcp_client.rs` 中实现，尚未开始实现。

MCP integration is planned to be implemented in `crates/kode-services/src/mcp_client.rs`, implementation has not started yet.

## Goals / Non-Goals

### Goals

- [ ] 实现 MCP 客户端 / Implement MCP client
- [ ] 支持 STDIO 和 SSE 传输 / Support STDIO and SSE transports
- [ ] 实现工具发现和调用 / Implement tool discovery and invocation
- [ ] 实现服务器管理 / Implement server management
- [ ] 与 ToolRegistry 集成 / Integrate with ToolRegistry

### Non-Goals

- [ ] MCP 服务器实现 / MCP server implementation
- [ ] MCP 资源 (resources) 功能 / MCP resources feature
- [ ] MCP 提示词 (prompts) 功能 / MCP prompts feature

## Decisions

### 决策 1: MCP 协议库选择 / Decision 1: MCP Protocol Library Selection

**选择**: 手动实现 MCP JSON-RPC 协议

**Choice**: Manually implement MCP JSON-RPC protocol

**理由 / Rationale**:
- 没有成熟的 Rust MCP SDK / No mature Rust MCP SDK
- 协议相对简单 / Protocol is relatively simple
- 完全控制实现 / Full control over implementation

**实现 / Implementation**:
```rust
/// 来自 ARCHITECTURE.md L306-314
pub struct McpClient {
    servers: HashMap<String, McpServer>,
}

impl McpClient {
    pub async fn connect(&mut self, config: McpServerConfig) -> Result<()>;
    pub async fn list_tools(&self, server: &str) -> Result<Vec<ToolSchema>>;
    pub async fn call_tool(&self, server: &str, name: &str, params: Value) -> Result<Value>;
}
```

### 决策 2: MCP 传输方式 / Decision 2: MCP Transport Methods

**选择**: 定义 McpTransport trait

**Choice**: Define McpTransport trait

**理由 / Rationale**:
- 支持多种传输方式 / Support multiple transport methods
- 便于测试 / Easy to test
- 易于扩展 / Easy to extend

**设计 / Design**:
```rust
#[async_trait]
pub trait McpTransport: Send + Sync {
    /// 发送请求并接收响应
    async fn send_request(&self, request: McpRequest) -> Result<McpResponse>;

    /// 订阅服务器消息（用于 SSE）
    async fn subscribe(&mut self) -> Result<Pin<Box<dyn Stream<Item = Result<String>>>>>;

    /// 关闭连接
    async fn close(&self) -> Result<()>;
}

/// STDIO 传输
pub struct McpStdioTransport {
    child: tokio::process::Child,
    stdin: tokio::process::ChildStdin,
    stdout: tokio::process::ChildStdout,
}

/// SSE 传输
pub struct McpSseTransport {
    client: reqwest::Client,
    url: String,
    event_source: Pin<Box<dyn Stream<Item = Result<String>>>>,
}
```

### 决策 3: MCP 工具适配器 / Decision 3: MCP Tool Adapter

**选择**: 创建 McpToolWrapper 实现 Tool trait

**Choice**: Create McpToolWrapper implementing Tool trait

**理由 / Rationale**:
- MCP 工具可以像普通工具一样使用 / MCP tools can be used like regular tools
- 统一的接口 / Unified interface
- 自动参数传递 / Automatic parameter passing

**设计 / Design**:
```rust
pub struct McpToolWrapper {
    server_name: String,
    tool_name: String,
    description: String,
    parameters: JsonSchema,
    client: Arc<McpClient>,
}

#[async_trait]
impl Tool for McpToolWrapper {
    fn name(&self) -> &str {
        &self.tool_name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn schema(&self) -> JsonSchema {
        self.parameters.clone()
    }

    async fn execute(&self, params: Value, _context: &ToolContext) -> Result<ToolResult> {
        let result = self.client.call_tool(&self.server_name, &self.tool_name, params).await?;
        Ok(ToolResult {
            output: serde_json::to_string_pretty(&result)?,
            artifacts: vec![],
        })
    }
}
```

### 决策 4: MCP 客户端实现 / Decision 4: MCP Client Implementation

**设计 / Design**:
```rust
pub struct McpClient {
    servers: HashMap<String, McpServerConnection>,
    next_id: Arc<AtomicU64>,
}

struct McpServerConnection {
    name: String,
    transport: Box<dyn McpTransport>,
    tools: Vec<McpToolDefinition>,
    state: ConnectionState,
}

enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Error(String),
}

impl McpClient {
    pub fn new() -> Self {
        Self {
            servers: HashMap::new(),
            next_id: Arc::new(AtomicU64::new(1)),
        }
    }

    /// 启动配置的服务器
    pub async fn start_servers(&mut self, servers: HashMap<String, McpServerConfig>) -> Result<()> {
        for (name, config) in servers {
            self.start_server(&name, config).await?;
        }
        Ok(())
    }

    /// 启动单个服务器
    pub async fn start_server(&mut self, name: &str, config: McpServerConfig) -> Result<()> {
        let transport: Box<dyn McpTransport> = match config {
            McpServerConfig::Stdio(stdio) => {
                Box::new(self.connect_stdio(stdio).await?)
            }
            McpServerConfig::Sse(sse) => {
                Box::new(self.connect_sse(sse).await?)
            }
        };

        // 初始化连接
        let init_request = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: self.next_id().to_string(),
            method: "initialize".to_string(),
            params: Some(json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": {
                    "name": "kode-rust",
                    "version": env!("CARGO_PKG_VERSION")
                }
            })),
        };

        let response = transport.send_request(init_request).await?;
        if response.error.is_some() {
            return Err(Error::McpInitError(response.error.unwrap().message));
        }

        // 发送 initialized 通知
        let notif = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: "".to_string(),  // 通知没有 ID
            method: "notifications/initialized".to_string(),
            params: None,
        };
        let _ = transport.send_request(notif).await;

        // 发现工具
        let tools_request = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: self.next_id().to_string(),
            method: "tools/list".to_string(),
            params: None,
        };

        let tools_response = transport.send_request(tools_request).await?;
        let tools = self.parse_tools_list(tools_response.result)?;

        self.servers.insert(name.to_string(), McpServerConnection {
            name: name.to_string(),
            transport,
            tools,
            state: ConnectionState::Connected,
        });

        Ok(())
    }

    /// 调用工具
    pub async fn call_tool(
        &self,
        server_name: &str,
        tool_name: &str,
        params: Value,
    ) -> Result<Value> {
        let server = self.servers.get(server_name)
            .ok_or_else(|| Error::McpServerNotFound(server_name.to_string()))?;

        let request = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: self.next_id().to_string(),
            method: "tools/call".to_string(),
            params: Some(json!({
                "name": tool_name,
                "arguments": params
            })),
        };

        let response = server.transport.send_request(request).await?;
        if let Some(error) = response.error {
            return Err(Error::McpToolError {
                tool: tool_name.to_string(),
                message: error.message,
            });
        }

        Ok(response.result.unwrap())
    }

    /// 获取所有服务器的工具
    pub fn get_all_tools(&self) -> Vec<Arc<dyn Tool>> {
        let mut tools = Vec::new();
        for server in self.servers.values() {
            for tool_def in &server.tools {
                tools.push(Arc::new(McpToolWrapper {
                    server_name: server.name.clone(),
                    tool_name: tool_def.name.clone(),
                    description: tool_def.description.clone(),
                    parameters: tool_def.input_schema.clone(),
                    client: Arc::new(self.clone()),
                }) as Arc<dyn Tool>);
            }
        }
        tools
    }

    fn next_id(&self) -> u64 {
        self.next_id.fetch_add(1, Ordering::SeqCst)
    }

    async fn connect_stdio(&self, config: McpStdioServerConfig) -> Result<McpStdioTransport> {
        let mut child = tokio::process::Command::new(&config.command)
            .args(&config.args)
            .envs(config.env.unwrap_or_default())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;

        let stdin = child.stdin.take().ok_or_else(|| Error::McpIoError("Failed to open stdin".to_string()))?;
        let stdout = child.stdout.take().ok_or_else(|| Error::McpIoError("Failed to open stdout".to_string()))?;

        Ok(McpStdioTransport::new(child, stdin, stdout))
    }

    async fn connect_sse(&self, config: McpSseServerConfig) -> Result<McpSseTransport> {
        Ok(McpSseTransport::new(config.url).await?)
    }

    fn parse_tools_list(&self, result: Option<Value>) -> Result<Vec<McpToolDefinition>> {
        let result = result.ok_or_else(|| Error::McpParseError("No result in tools/list response".to_string()))?;
        let tools = result["tools"].as_array()
            .ok_or_else(|| Error::McpParseError("Invalid tools array".to_string()))?;

        tools.iter().map(|v| serde_json::from_value(v.clone())
            .map_err(|e| Error::McpParseError(e.to_string()))
        ).collect()
    }
}
```

### 决策 5: 错误处理 / Decision 5: Error Handling

**设计 / Design**:
```rust
#[derive(Error, Debug)]
pub enum McpError {
    #[error("MCP server not found: {0}")]
    ServerNotFound(String),

    #[error("MCP initialization failed: {0}")]
    InitError(String),

    #[error("MCP tool call failed: {tool} - {message}")]
    ToolError {
        tool: String,
        message: String,
    },

    #[error("MCP parse error: {0}")]
    ParseError(String),

    #[error("MCP I/O error: {0}")]
    IoError(String),
}
```

## 数据流 / Data Flow

### 服务器启动流程 / Server Startup Flow

```
应用启动 / Application starts
    ↓
加载 mcp_servers 配置 / Load mcp_servers config
    ↓
遍历服务器配置 / Iterate server configs
    ↓
McpClient::start_server(name, config)
    ↓ (STDIO)
启动子进程 / Start child process
    ↓ (SSE)
建立 HTTP 连接 / Establish HTTP connection
    ↓
发送 initialize 请求 / Send initialize request
    ↓
发送 initialized 通知 / Send initialized notification
    ↓
调用 tools/list / Call tools/list
    ↓
解析工具定义 / Parse tool definitions
    ↓
创建 McpToolWrapper / Create McpToolWrapper
    ↓
注册到 ToolRegistry / Register to ToolRegistry
```

### 工具调用流程 / Tool Call Flow

```
AI 调用 MCP 工具 / AI calls MCP tool
    ↓
McpToolWrapper::execute(params, context)
    ↓
McpClient::call_tool(server, tool, params)
    ↓
构建 tools/call 请求 / Build tools/call request
    ↓
发送到服务器 / Send to server
    ↓
等待响应 / Wait for response
    ↓
解析结果 / Parse result
    ↓
返回 ToolResult / Return ToolResult
```

## 配置示例 / Configuration Examples

### STDIO 服务器 / STDIO Server

```json
{
  "mcpServers": {
    "filesystem": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-filesystem", "/Users/user/projects"],
      "env": {
        "NODE_ENV": "production"
      }
    }
  }
}
```

### SSE 服务器 / SSE Server

```json
{
  "mcpServers": {
    "remote-tools": {
      "url": "https://example.com/mcp/sse"
    }
  }
}
```

## 错误处理 / Error Handling

| 错误类型 / Error Type | 描述 / Description | 处理方式 / Handling |
|---------------------|-------------------|-------------------|
| `McpInitError` | 初始化失败 / Initialization failed | 标记服务器为错误状态 / Mark server as error state |
| `McpToolError` | 工具调用失败 / Tool call failed | 返回错误详情给 AI / Return error details to AI |
| `McpParseError` | 响应解析失败 / Response parse failed | 记录错误日志 / Log error |
| `McpIoError` | I/O 错误 / I/O error | 尝试重连 / Attempt reconnection |

## 测试策略 / Testing Strategy

### 单元测试 / Unit Tests

```rust
#[tokio::test]
async fn test_mcp_request_serialization() {
    let request = McpRequest {
        jsonrpc: "2.0".to_string(),
        id: "1".to_string(),
        method: "tools/list".to_string(),
        params: None,
    };

    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("\"method\":\"tools/list\""));
}
```

### 集成测试 / Integration Tests

```rust
#[tokio::test]
#[ignore]  // 需要 MCP 服务器
async fn test_real_mcp_server() {
    let mut client = McpClient::new();
    let config = McpServerConfig::Stdio(McpStdioServerConfig {
        command: "node".to_string(),
        args: vec!["test-server.js".to_string()],
        env: None,
    });

    client.start_server("test", config).await.unwrap();
    assert!(client.servers.contains_key("test"));
}
```

## Risks / Trade-offs

### 风险 1: 服务器进程管理 / Risk 1: Server Process Management

**风险 / Risk**: STDIO 服务器进程可能崩溃或挂起

**缓解措施 / Mitigation**:
- 监控进程状态 / Monitor process status
- 设置超时 / Set timeouts
- 自动重启机制 / Automatic restart mechanism

### 风险 2: 网络连接稳定性 / Risk 2: Network Connection Stability

**风险 / Risk**: SSE 连接可能断开

**缓解措施 / Mitigation**:
- 自动重连机制 / Automatic reconnection
- 心跳检测 / Heartbeat detection
- 指数退避 / Exponential backoff

## Open Questions

1. **是否需要支持 MCP 资源 (resources)？**
   - 建议：未来添加，优先工具
   - Rationale: Add in the future, prioritize tools

2. **是否需要支持 MCP 提示词 (prompts)？**
   - 建议：未来添加
   - Rationale: Add in the future

3. **如何处理并发工具调用？**
   - 建议：每个服务器顺序调用，服务器间并发
   - Rationale: Sequential calls per server, concurrent across servers

## 参考资源 / References

- [MCP 协议规范](https://spec.modelcontextprotocol.io/)
- [MCP TypeScript SDK](https://github.com/modelcontextprotocol/typescript-sdk)
- [tokio::process 文档](https://tokio.rs/tokio/process/)
- [reqwest SSE 文档](https://docs.rs/reqwest/reqwest/struct.Response.html#method.bytes_stream)
