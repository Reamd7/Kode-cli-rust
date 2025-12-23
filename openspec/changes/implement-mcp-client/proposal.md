# Change: 实现 MCP 客户端基础 / Implement MCP Client Basics

## Why

MCP (Model Context Protocol) 集成允许 Kode 连接到外部 MCP 服务器，动态加载和使用服务器提供的工具。

MCP (Model Context Protocol) integration allows Kode to connect to external MCP servers, dynamically loading and using tools provided by the servers.

## What Changes

- 研究 MCP 协议规范
- 实现 MCP 客户端连接
- 实现工具发现（tools/list）
- 实现工具调用（tools/call）
- 错误处理

- Research MCP protocol specification
- Implement MCP client connection
- Implement tool discovery (tools/list)
- Implement tool calling (tools/call)
- Error handling

## Impact

**Affected specs:**
- mcp-integration

**Affected code:**
- `crates/kode-services/src/mcp.rs` (新建)
- `crates/kode-services/src/mcp/client.rs` (新建)
