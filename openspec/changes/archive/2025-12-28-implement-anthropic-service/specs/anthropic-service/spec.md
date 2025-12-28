# Delta Spec: Anthropic API Service

## ADDED Requirements

### Requirement: AnthropicService 结构 / AnthropicService Structure
The system SHALL implement Anthropic Claude API client.

系统应实现 Anthropic Claude API 客户端。

#### Scenario: 服务初始化 / Service Initialization
- **WHEN** 创建 AnthropicService 时
- **THEN** 接受 api_key 参数
- **AND** 接受可选的 base_url 参数
- **AND** 创建 reqwest::Client 实例

- **WHEN** creating AnthropicService
- **THEN** accepts api_key parameter
- **AND** accepts optional base_url parameter
- **AND** creates reqwest::Client instance

### Requirement: Messages API 实现 / Messages API Implementation
The system SHALL implement Anthropic Messages API.

系统应实现 Anthropic Messages API。

#### Scenario: 非流式请求 / Non-streaming Request
- **WHEN** 调用 send_message() 时
- **THEN** 发送 POST 请求到 /v1/messages
- **AND** 包含 x-api-key 请求头
- **AND** 包含 anthropic-version: 2023-06-01
- **AND** 返回解析后的 Response

- **WHEN** calling send_message()
- **THEN** sends POST request to /v1/messages
- **AND** includes x-api-key header
- **AND** includes anthropic-version: 2023-06-01
- **AND** returns parsed Response

### Requirement: 流式响应 / Streaming Response
The system SHALL implement SSE streaming for Anthropic API.

系统应实现 Anthropic API 的 SSE 流式响应。

#### Scenario: SSE 流解析 / SSE Stream Parsing
- **WHEN** 调用 stream_message() 时
- **THEN** 发送请求时设置 stream: true
- **AND** 解析 SSE 事件
- **AND** 返回 StreamItemStream<Result<StreamChunk>>
- **AND** 处理连接错误

- **WHEN** calling stream_message()
- **THEN** sends request with stream: true
- **AND** parses SSE events
- **AND** returns Stream<Item = Result<StreamChunk>>
- **AND** handles connection errors
