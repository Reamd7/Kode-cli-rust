# Anthropic 服务规范 / Anthropic Service Specification

## Purpose

Anthropic 服务提供与 Anthropic Claude API 的集成，实现消息发送和流式响应功能。

The Anthropic service provides integration with the Anthropic Claude API, implementing message sending and streaming response functionality.

## Requirements

### Requirement: API 客户端 / API Client
The system SHALL provide a client for interacting with the Anthropic Messages API.

系统应提供与 Anthropic Messages API 交互的客户端。

#### Scenario: 创建 API 客户端 / Create API Client
- **WHEN** 使用 API key 初始化客户端时
- **THEN** 创建配置好的 HTTP 客户端
- **AND** 使用默认的 Anthropic API 端点

- **WHEN** initializing the client with an API key
- **THEN** create a configured HTTP client
- **AND** use the default Anthropic API endpoint

#### Scenario: 自定义 API 端点 / Custom API Endpoint
- **WHEN** 用户指定自定义 base URL 时
- **THEN** 使用指定的端点而不是默认端点

- **WHEN** the user specifies a custom base URL
- **THEN** use the specified endpoint instead of the default

### Requirement: 消息发送 / Message Sending
The system SHALL send messages to the Anthropic API and return the response.

系统应向 Anthropic API 发送消息并返回响应。

#### Scenario: 发送文本消息 / Send Text Message
- **WHEN** 发送包含文本内容的消息时
- **THEN** API 返回助手的文本响应
- **AND** 响应包含完整的消息内容

- **WHEN** sending a message containing text content
- **THEN** the API returns the assistant's text response
- **AND** the response contains the complete message content

#### Scenario: 发送带工具的消息 / Send Message with Tools
- **WHEN** 发送消息时附带工具定义
- **THEN** API 可以调用定义的工具
- **AND** 响应包含工具使用信息

- **WHEN** sending a message with tool definitions
- **THEN** the API can call the defined tools
- **AND** the response contains tool use information

#### Scenario: 错误处理 / Error Handling
- **WHEN** API 返回错误时
- **THEN** 将 API 错误转换为应用错误
- **AND** 保留错误详情供用户查看

- **WHEN** the API returns an error
- **THEN** convert the API error to an application error
- **AND** preserve error details for user inspection

### Requirement: 流式响应 / Streaming Response
The system SHALL support streaming responses from the Anthropic API using Server-Sent Events.

系统应支持使用 Server-Sent Events 从 Anthropic API 获取流式响应。

#### Scenario: 建立流式连接 / Establish Streaming Connection
- **WHEN** 请求流式响应时
- **THEN** 与 API 建立 SSE 连接
- **AND** 逐步接收响应内容

- **WHEN** requesting a streaming response
- **THEN** establish an SSE connection with the API
- **AND** receive response content progressively

#### Scenario: 处理流式事件 / Handle Streaming Events
- **WHEN** 接收到 SSE 事件时
- **THEN** 正确解析事件类型
- **AND** 将内容块传递给消费者

- **WHEN** receiving SSE events
- **THEN** correctly parse the event type
- **AND** pass content chunks to the consumer

#### Scenario: 流式工具调用 / Streaming Tool Calls
- **WHEN** AI 在流式响应中调用工具时
- **THEN** 在流中返回工具使用信息
- **AND** 应用可以拦截并处理工具调用

- **WHEN** the AI calls a tool during streaming response
- **THEN** return tool use information in the stream
- **AND** the application can intercept and handle the tool call

### Requirement: 模型适配器实现 / Model Adapter Implementation
The system SHALL implement the ModelAdapter trait for Anthropic service.

系统应为 Anthropic 服务实现 ModelAdapter trait。

#### Scenario: 实现 send_message / Implement send_message
- **WHEN** 调用 send_message 方法时
- **THEN** 使用 Anthropic Messages API
- **AND** 返回统一格式的响应

- **WHEN** the send_message method is called
- **THEN** use the Anthropic Messages API
- **AND** return a response in unified format

#### Scenario: 实现 stream_message / Implement stream_message
- **WHEN** 调用 stream_message 方法时
- **THEN** 返回流式响应 Stream
- **AND** 支持 StreamChunk 格式的输出

- **WHEN** the stream_message method is called
- **THEN** return a streaming response Stream
- **AND** support StreamChunk formatted output

### Requirement: 请求头配置 / Request Header Configuration
The system SHALL correctly configure HTTP headers for Anthropic API requests.

系统应正确配置 Anthropic API 请求的 HTTP 头。

#### Scenario: 认证头 / Authentication Header
- **WHEN** 发送 API 请求时
- **THEN** 包含 x-api-key 头
- **AND** 包含 anthropic-version 头

- **WHEN** sending an API request
- **THEN** include the x-api-key header
- **AND** include the anthropic-version header

#### Scenario: 内容类型 / Content Type
- **WHEN** 发送请求体时
- **THEN** 设置正确的 Content-Type

- **WHEN** sending a request body
- **THEN** set the correct Content-Type

## Non-Goals

- 本规范不包含其他模型提供商（OpenAI、DeepSeek 等）
- 不包含 Anthropic API 的所有高级功能（如 prompt caching）
- 不包含 API 使用量统计

- This specification does not include other model providers (OpenAI, DeepSeek, etc.)
- Does not include all advanced Anthropic API features (like prompt caching)
- Does not include API usage statistics
