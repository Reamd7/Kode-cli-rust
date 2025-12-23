# MCP 集成规范 / MCP Integration Specification

## Purpose

MCP (Model Context Protocol) 集成允许 Kode 连接到外部 MCP 服务器，动态加载和使用服务器提供的工具。

The MCP (Model Context Protocol) integration allows Kode to connect to external MCP servers, dynamically loading and using tools provided by the servers.

## Requirements

### Requirement: MCP 客户端 / MCP Client
The system SHALL provide a client for connecting to MCP servers.

系统应提供连接到 MCP 服务器的客户端。

#### Scenario: STDIO 传输连接 / STDIO Transport Connection
- **WHEN** 使用 STDIO 传输连接到 MCP 服务器时
- **THEN** 启动服务器进程
- **AND** 通过标准输入/输出进行通信
- **AND** 建立 JSON-RPC 通信

- **WHEN** connecting to an MCP server using STDIO transport
- **THEN** start the server process
- **AND** communicate via standard input/output
- **AND** establish JSON-RPC communication

#### Scenario: SSE 传输连接 / SSE Transport Connection
- **WHEN** 使用 SSE 传输连接到 MCP 服务器时
- **THEN** 建立 HTTP 连接到服务器 URL
- **AND** 通过 Server-Sent Events 接收消息

- **WHEN** connecting to an MCP server using SSE transport
- **THEN** establish HTTP connection to the server URL
- **AND** receive messages via Server-Sent Events

#### Scenario: 服务器健康检查 / Server Health Check
- **WHEN** MCP 服务器连接建立后
- **THEN** 发送 initialize 请求
- **AND** 验证服务器响应

- **WHEN** the MCP server connection is established
- **THEN** send an initialize request
- **AND** verify the server response

### Requirement: 工具发现 / Tool Discovery
The system SHALL discover and list tools provided by MCP servers.

系统应发现并列出 MCP 服务器提供的工具。

#### Scenario: 列出服务器工具 / List Server Tools
- **WHEN** 请求 MCP 服务器的工具列表时
- **THEN** 发送 tools/list 请求
- **AND** 返回服务器提供的所有工具

- **WHEN** requesting an MCP server's tool list
- **THEN** send a tools/list request
- **AND** return all tools provided by the server

#### Scenario: 工具 Schema 转换 / Tool Schema Conversion
- **WHEN** MCP 工具定义被接收时
- **THEN** 将 MCP schema 转换为统一格式
- **AND** 保留工具名称和描述

- **WHEN** an MCP tool definition is received
- **THEN** convert the MCP schema to unified format
- **AND** preserve tool name and description

### Requirement: 工具调用 / Tool Invocation
The system SHALL invoke tools provided by MCP servers.

系统应调用 MCP 服务器提供的工具。

#### Scenario: 调用 MCP 工具 / Call MCP Tool
- **WHEN** 调用 MCP 服务器提供的工具时
- **THEN** 发送 tools/call 请求到服务器
- **AND** 传递工具参数
- **AND** 返回工具执行结果

- **WHEN** calling a tool provided by an MCP server
- **THEN** send a tools/call request to the server
- **AND** pass tool parameters
- **AND** return the tool execution result

#### Scenario: 处理工具错误 / Handle Tool Errors
- **WHEN** MCP 工具执行失败时
- **THEN** 返回包含错误详情的结果
- **AND** 不中断其他工具的执行

- **WHEN** an MCP tool execution fails
- **THEN** return a result containing error details
- **AND** don't interrupt execution of other tools

### Requirement: MCP 服务器管理 / MCP Server Management
The system SHALL manage multiple MCP server connections.

系统应管理多个 MCP 服务器连接。

#### Scenario: 启动服务器 / Start Server
- **WHEN** 根据配置启动 MCP 服务器时
- **THEN** 解析服务器配置
- **AND** 建立连接
- **AND** 执行健康检查

- **WHEN** starting an MCP server based on configuration
- **THEN** parse the server configuration
- **AND** establish the connection
- **AND** perform health check

#### Scenario: 停止服务器 / Stop Server
- **WHEN** 停止 MCP 服务器时
- **THEN** 终止服务器进程（STDIO）
- **AND** 清理资源
- **AND** 从可用工具中移除服务器的工具

- **WHEN** stopping an MCP server
- **THEN** terminate the server process (for STDIO)
- **AND** clean up resources
- **AND** remove the server's tools from available tools

#### Scenario: 服务器自动重连 / Server Auto-Reconnect
- **WHEN** MCP 服务器连接断开时
- **THEN** 尝试自动重新连接
- **AND** 指数退避重试策略

- **WHEN** the MCP server connection is lost
- **THEN** attempt to automatically reconnect
- **AND** use exponential backoff retry strategy

### Requirement: 配置集成 / Configuration Integration
The system SHALL load MCP server configuration from the application config.

系统应从应用配置加载 MCP 服务器配置。

#### Scenario: 加载服务器配置 / Load Server Configuration
- **WHEN** 应用启动时
- **THEN** 从配置文件读取 mcp_servers 配置
- **AND** 解析 STDIO 和 SSE 服务器配置

- **WHEN** the application starts
- **THEN** read mcp_servers configuration from the config file
- **AND** parse both STDIO and SSE server configurations

#### Scenario: 动态工具注册 / Dynamic Tool Registration
- **WHEN** MCP 服务器连接成功后
- **THEN** 自动将服务器的工具注册到 ToolRegistry
- **AND** 使用工具名称前缀标识服务器

- **WHEN** an MCP server connects successfully
- **THEN** automatically register the server's tools to ToolRegistry
- **AND** use a tool name prefix to identify the server

## Reference / 参考资料

### TypeScript 版本实现参考 / TypeScript Implementation Reference

在实现本规范时，请参考原版 TypeScript 项目中的以下文件：

When implementing this specification, refer to the following files in the original TypeScript project:

#### MCP 客户端核心 / MCP Client Core
- **MCP 客户端**: `/Users/gemini/Documents/backup/Kode-cli/src/services/mcpClient.ts`
  - MCP 服务器连接管理
  - STDIO 和 SSE 传输支持
  - 工具发现和调用
  - `getMCPTools()` - 获取所有 MCP 工具
  - `addMcpServer()` - 添加 MCP 服务器
  - `removeMcpServer()` - 移除 MCP 服务器

#### MCP 工具实现 / MCP Tool Implementation
- **MCP 工具**: `/Users/gemini/Documents/backup/Kode-cli/src/tools/MCPTool/MCPTool.ts`
  - MCP 工具包装类
  - 工具调用转发到 MCP 服务器
  - 错误处理和结果转换

#### MCP 配置 / MCP Configuration
- **MCP 配置类型**: `/Users/gemini/Documents/backup/Kode-cli/src/utils/config.ts`
  - `McpServerConfig` - MCP 服务器配置类型
  - `McpStdioServerConfig` - STDIO 服务器配置
  - `McpSSEServerConfig` - SSE 服务器配置
  - 环境变量解析: `parseEnvVars()`

#### MCP 批准机制 / MCP Approval Mechanism
- **MCP 服务器批准**: `/Users/gemini/Documents/backup/Kode-cli/src/services/mcpServerApproval.tsx`
  - 用户批准流程
  - 服务器信任管理

### 实现要点 / Implementation Notes

1. **传输协议**: 支持 STDIO（子进程）和 SSE（HTTP）两种传输方式
2. **JSON-RPC**: MCP 协议基于 JSON-RPC 2.0
3. **工具命名**: MCP 工具使用 `mcp_<server>_<tool>` 格式命名
4. **生命周期**: MCP 服务器在需要时启动，空闲时关闭
5. **配置合并**: MCP 服务器配置可以从 global、project、.mcprc 三个来源加载

## Non-Goals

- 本规范不包含 MCP 服务器的实现
- 不包含 MCP 协议的完整实现（仅工具相关部分）
- 不包含 MCP 资源和提示词功能

- This specification does not include MCP server implementation
- Does not include complete MCP protocol implementation (only tool-related parts)
- Does not include MCP resources and prompts features
