# Delta Spec: MCP Client

## ADDED Requirements

### Requirement: McpClient / McpClient
The system SHALL implement MCP protocol client.

系统应实现 MCP 协议客户端。

#### Scenario: 客户端连接 / Client Connection
- **WHEN** 连接 MCP 服务器时
- **THEN** 支持 STDIO 传输
- **AND** 支持 SSE 传输
- **AND** 建立 JSON-RPC 通信

- **WHEN** connecting to MCP server
- **THEN** supports STDIO transport
- **AND** supports SSE transport
- **AND** establishes JSON-RPC communication

#### Scenario: 工具发现 / Tool Discovery
- **WHEN** 调用 list_tools() 时
- **THEN** 发送 tools/list 请求
- **AND** 返回服务器提供的工具列表
- **AND** 解析工具 schema

- **WHEN** calling list_tools()
- **THEN** sends tools/list request
- **AND** returns list of tools provided by server
- **AND** parses tool schemas

#### Scenario: 工具调用 / Tool Calling
- **WHEN** 调用 call_tool() 时
- **THEN** 发送 tools/call 请求
- **AND** 传递工具名称和参数
- **AND** 返回工具执行结果

- **WHEN** calling call_tool()
- **THEN** sends tools/call request
- **AND** passes tool name and parameters
- **AND** returns tool execution result
