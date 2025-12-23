# Change: 实现 MCP 完整功能 / Implement MCP Full Features

## Why

完整的 MCP 支持包括服务器管理、健康检查、自动重连和动态工具加载，提供稳定的集成体验。

Full MCP support includes server management, health checks, auto-reconnect, and dynamic tool loading, providing stable integration experience.

## What Changes

- 实现 MCP 服务器管理
  - 启动/停止服务器
  - 健康检查
  - 自动重连
- 实现 MCP 工具动态加载
- 集成到 ToolRegistry
- 完整的 MCP 配置支持
- 集成测试

- Implement MCP server management
  - Start/stop servers
  - Health checks
  - Auto-reconnect
- Implement MCP tool dynamic loading
- Integrate into ToolRegistry
- Full MCP configuration support
- Integration tests

## Impact

**Affected specs:**
- mcp-integration (MODIFIED)
- tool-system (MODIFIED)

**Affected code:**
- `crates/kode-services/src/mcp/server.rs` (新建)
- `crates/kode-services/src/mcp/manager.rs` (新建)
