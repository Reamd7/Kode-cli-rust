# MCP Integration Specification

MCP (Model Context Protocol) 集成规范 - 定义与 MCP 服务器的集成机制。

## Purpose

MCP 集成允许从外部服务器动态加载工具。支持 STDIO 和 SSE 两种通信方式，自动发现服务器暴露的工具，并集成到 ToolRegistry 中。

## ADDED Requirements

### Requirement: MCP 客户端连接
The system SHALL connect to MCP servers via STDIO or SSE.

系统必须通过 STDIO 或 SSE 连接到 MCP 服务器。

#### Scenario: STDIO 连接
- **WHEN** 连接到 STDIO 类型的 MCP 服务器
- **THEN** 启动配置的命令进程
- **AND** 通过标准输入/输出进行通信
- **AND** 使用 JSON-RPC 2.0 协议

#### Scenario: SSE 连接
- **WHEN** 连接到 SSE 类型的 MCP 服务器
- **THEN** 建立到指定 URL 的 HTTP 连接
- **AND** 通过 Server-Sent Events 接收消息
- **AND** 使用 JSON-RPC 2.0 协议

#### Scenario: 处理连接失败
- **WHEN** MCP 服务器连接失败
- **THEN** 返回连接错误
- **AND** 错误包含失败原因

### Requirement: 工具发现
The system SHALL discover tools exposed by MCP servers.

系统必须发现 MCP 服务器暴露的工具。

#### Scenario: 请求工具列表
- **WHEN** 连接到 MCP 服务器后
- **THEN** 发送 tools/list 请求
- **AND** 解析返回的工具定义

#### Scenario: 工具元数据
- **WHEN** 解析 MCP 工具
- **THEN** 提取工具名称和描述
- **AND** 提取 JSON Schema 参数定义

#### Scenario: 处理工具列表错误
- **WHEN** tools/list 请求失败
- **THEN** 服务器标记为错误状态
- **AND** 不暴露该服务器的工具

### Requirement: 工具调用
The system SHALL execute tools on MCP servers.

系统必须执行 MCP 服务器上的工具。

#### Scenario: 调用工具
- **WHEN** 调用 MCP 工具
- **THEN** 发送 tools/call 请求到服务器
- **AND** 传递工具名称和参数
- **AND** 返回工具执行结果

#### Scenario: 处理工具错误
- **WHEN** MCP 工具执行失败
- **THEN** 返回工具错误信息
- **AND** 错误包含服务器返回的详情

#### Scenario: 异步工具调用
- **WHEN** MCP 工具需要较长时间
- **THEN** 支持异步执行
- **AND** 不阻塞其他操作

### Requirement: 服务器管理
The system SHALL manage MCP server lifecycle.

系统必须管理 MCP 服务器的生命周期。

#### Scenario: 启动服务器
- **WHEN** 配置中定义 MCP 服务器
- **THEN** 应用启动时自动连接
- **AND** 发起连接并初始化

#### Scenario: 健康检查
- **WHEN** MCP 服务器运行中
- **THEN** 定期检查服务器健康状态
- **AND** 检测到无响应时标记为断开

#### Scenario: 自动重连
- **WHEN** MCP 服务器断开连接
- **THEN** 尝试自动重新连接
- **AND** 使用指数退避策略

#### Scenario: 停止服务器
- **WHEN** 应用关闭
- **THEN** 优雅地关闭 STDIO 进程
- **AND** 清理所有连接资源

### Requirement: 工具注册集成
The system SHALL integrate MCP tools with ToolRegistry.

系统必须将 MCP 工具集成到 ToolRegistry。

#### Scenario: 注册 MCP 工具
- **WHEN** MCP 工具被发现
- **THEN** 自动注册到 ToolRegistry
- **AND** 使用服务器名称作为工具前缀

#### Scenario: 工具命名
- **WHEN** 注册 MCP 工具
- **THEN** 使用格式 "server_name:tool_name"
- **AND** 避免名称冲突

#### Scenario: 过滤 MCP 工具
- **WHEN** Agent 有工具过滤
- **THEN** MCP 工具遵守过滤规则
- **AND** 只暴露允许的工具

### Requirement: 配置支持
The system SHALL support MCP server configuration.

系统必须支持 MCP 服务器配置。

#### Scenario: 全局 MCP 配置
- **WHEN** 在 ~/.kode.json 中配置 MCP 服务器
- **THEN** 所有项目可以使用这些服务器
- **AND** 配置包含服务器类型和参数

#### Scenario: 项目 MCP 配置
- **WHEN** 在 ./.kode.json 中配置 MCP 服务器
- **THEN** 只有当前项目可以使用
- **AND** 项目配置可以覆盖全局配置

#### Scenario: STDIO 配置
- **WHEN** 配置 STDIO 类型服务器
- **THEN** 必须指定 command 和 args
- **AND** 可选指定环境变量

#### Scenario: SSE 配置
- **WHEN** 配置 SSE 类型服务器
- **THEN** 必须指定 url
- **AND** 可选指定认证头

### Requirement: 资源访问
The system SHALL support MCP resources (future).

系统必须支持 MCP 资源（未来）。

#### Scenario: 列出资源
- **WHEN** MCP 服务器暴露资源
- **THEN** 支持列出可用资源
- **AND** 包含资源 URI 和元数据

#### Scenario: 读取资源
- **WHEN** 请求 MCP 资源
- **THEN** 通过 resources/read 请求获取内容
- **AND** 返回资源内容

## Non-Goals

本规范不包含：
- Prompt 模板（未来扩展）
- MCP 采样功能（未来扩展）
- 服务器之间的通信
- 自定义协议扩展
