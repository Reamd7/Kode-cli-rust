# Delta Spec: OpenAI-compatible Service

## ADDED Requirements

### Requirement: OpenAIService / OpenAIService
The system SHALL implement OpenAI-compatible API client.

系统应实现 OpenAI-compatible API 客户端。

#### Scenario: 服务初始化 / Service Initialization
- **WHEN** 创建 OpenAIService 时
- **THEN** 接受 api_key 和 base_url 参数
- **AND** base_url 可自定义（支持 DeepSeek 等）
- **AND** 创建 reqwest::Client 实例

- **WHEN** creating OpenAIService
- **THEN** accepts api_key and base_url parameters
- **AND** base_url is customizable (supports DeepSeek, etc.)
- **AND** creates reqwest::Client instance

#### Scenario: ModelAdapter 实现 / ModelAdapter Implementation
- **WHEN** 实现 ModelAdapter for OpenAIService 时
- **THEN** 实现 send_message() 方法
- **AND** 实现 stream_message() 方法
- **AND** 适配 OpenAI API 格式

- **WHEN** implementing ModelAdapter for OpenAIService
- **THEN** implements send_message() method
- **AND** implements stream_message() method
- **AND** adapts to OpenAI API format
