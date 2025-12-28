# Delta Spec: OpenAI-compatible Service

本 Delta Spec 定义了 OpenAI-compatible 服务的需求，基于原版 TypeScript 实现的分析。
这是一个全新的 spec，与 `anthropic-service` 平级。

This Delta Spec defines the requirements for the OpenAI-compatible service, based on analysis of the original TypeScript implementation.
This is a brand new spec, parallel to `anthropic-service`.

## ADDED Requirements

### Requirement: OpenAIService 结构体 / OpenAIService Struct
The system SHALL implement an OpenAIService struct that provides OpenAI-compatible API client functionality.

系统应实现 OpenAIService 结构体，提供 OpenAI-compatible API 客户端功能。

#### Scenario: 服务初始化 / Service Initialization
- **WHEN** 创建 OpenAIService 实例时
- **THEN** 接受 api_key、base_url、model_name、max_tokens 参数
- **AND** base_url 可自定义以支持不同提供商（OpenAI、DeepSeek、MiniMax 等）
- **AND** 创建并配置 reqwest::Client 实例
- **AND** 设置默认超时时间（60 秒）
- **AND** 配置默认请求头（Authorization、Content-Type）

- **WHEN** creating an OpenAIService instance
- **THEN** accepts api_key, base_url, model_name, max_tokens parameters
- **AND** base_url is customizable to support different providers (OpenAI, DeepSeek, MiniMax, etc.)
- **AND** creates and configures reqwest::Client instance
- **AND** sets default timeout (60 seconds)
- **AND** configures default request headers (Authorization, Content-Type)

#### Scenario: 从 ModelConfig 创建 / Creation from ModelConfig
- **WHEN** 使用 ModelConfig 创建 OpenAIService 时
- **THEN** 从 ModelConfig 中提取 api_key、base_url、model_name、max_tokens
- **AND** provider 字段可选（用于确定特殊逻辑）
- **AND** 如果 base_url 为 None，使用默认 OpenAI 端点
- **AND** 返回配置好的 OpenAIService 实例

- **WHEN** creating OpenAIService from ModelConfig
- **THEN** extracts api_key, base_url, model_name, max_tokens from ModelConfig
- **AND** provider field is optional (used for special logic determination)
- **AND** if base_url is None, uses default OpenAI endpoint
- **AND** returns configured OpenAIService instance

### Requirement: ModelAdapter Trait 实现 / ModelAdapter Trait Implementation
The system SHALL implement ModelAdapter trait for OpenAIService with full streaming and tool support.

系统应为 OpenAIService 实现 ModelAdapter trait，完整支持流式响应和工具调用。

#### Scenario: 非流式消息发送 / Non-streaming Message Sending
- **WHEN** 调用 send_message() 发送消息时
- **THEN** 将 Message 列表转换为 OpenAI Chat Completions 格式
- **AND** 发送 POST 请求到 {base_url}/chat/completions
- **AND** 包含 system_prompt（如果有）作为第一条消息
- **AND** 解析响应并提取 content、usage、model
- **AND** 返回 ModelResponse
- **AND** 如果失败，根据错误类型决定是否重试

- **WHEN** calling send_message() to send messages
- **THEN** converts Message list to OpenAI Chat Completions format
- **AND** sends POST request to {base_url}/chat/completions
- **AND** includes system_prompt (if any) as first message
- **AND** parses response and extracts content, usage, model
- **AND** returns ModelResponse
- **AND** if failed, decides whether to retry based on error type

#### Scenario: 工具调用支持 / Tool Call Support
- **WHEN** 调用 send_message_with_tools() 发送带工具的消息时
- **THEN** 将工具定义转换为 OpenAI Function Calling 格式
- **AND** 在请求中包含 tools 参数（tool_choice: "auto"）
- **AND** 解析响应中的 tool_calls
- **AND** 返回包含工具调用的 ModelResponse

- **WHEN** calling send_message_with_tools() to send messages with tools
- **THEN** converts tool definitions to OpenAI Function Calling format
- **AND** includes tools parameter in request (tool_choice: "auto")
- **AND** parses tool_calls from response
- **AND** returns ModelResponse containing tool calls

#### Scenario: 流式消息发送 / Streaming Message Sending
- **WHEN** 调用 stream_message() 发送流式消息时
- **THEN** 在请求中设置 stream: true
- **AND** 使用 SSE（Server-Sent Events）格式解析响应流
- **AND** 为每个 content delta 生成 StreamChunk::ContentBlockDelta
- **AND** 在流结束时生成 StreamChunk::MessageStop（包含 usage）
- **AND** 返回 StreamingResponse
- **AND** 支持取消操作（通过 AbortSignal）

- **WHEN** calling stream_message() to send streaming messages
- **THEN** sets stream: true in request
- **AND** parses response stream using SSE (Server-Sent Events) format
- **AND** generates StreamChunk::ContentBlockDelta for each content delta
- **AND** generates StreamChunk::MessageStop at stream end (including usage)
- **AND** returns StreamingResponse
- **AND** supports cancellation (via AbortSignal)

#### Scenario: 流式工具调用 / Streaming Tool Calls
- **WHEN** 调用 stream_message_with_tools() 发送带工具的流式消息时
- **THEN** 在流中解析增量工具调用（delta.tool_calls）
- **AND** 累积工具调用参数（arguments 是分片的）
- **AND** 在流结束时返回完整的工具调用列表
- **AND** 生成 StreamChunk::ToolUseDelta 事件

- **WHEN** calling stream_message_with_tools() to send streaming messages with tools
- **THEN** parses incremental tool calls in stream (delta.tool_calls)
- **AND** accumulates tool call arguments (arguments are fragmented)
- **AND** returns complete tool call list at stream end
- **AND** generates StreamChunk::ToolUseDelta events

### Requirement: 模型参数适配 / Model Parameter Adaptation
The system SHALL automatically adapt request parameters based on model capabilities and requirements.

系统应自动根据模型能力和要求适配请求参数。

#### Scenario: GPT-5/o1 参数适配 / GPT-5/o1 Parameter Adaptation
- **WHEN** 检测到模型为 GPT-5、o1、o3 系列时
- **THEN** 自动将 max_tokens 转换为 max_completion_tokens
- **AND** 将 temperature 固定为 1（这些模型不支持其他值）
- **AND** 移除不支持的参数（frequency_penalty、presence_penalty、logit_bias）
- **AND** 添加 reasoning_effort 参数（如果支持）

- **WHEN** detecting model as GPT-5, o1, o3 series
- **THEN** automatically converts max_tokens to max_completion_tokens
- **AND** fixes temperature to 1 (these models don't support other values)
- **AND** removes unsupported parameters (frequency_penalty, presence_penalty, logit_bias)
- **AND** adds reasoning_effort parameter (if supported)

#### Scenario: 工具描述长度限制 / Tool Description Length Limit
- **WHEN** OpenAI API 返回工具描述过长错误时
- **THEN** 自动截断工具描述至 1024 字符
- **AND** 将截断的内容添加到 system message 中
- **AND** 使用特殊标签标记被截断的描述
- **AND** 自动重试请求

- **WHEN** OpenAI API returns tool description too long error
- **THEN** automatically truncates tool description to 1024 characters
- **AND** appends truncated content to system message
- **AND** uses special tags to mark truncated descriptions
- **AND** automatically retries request

#### Scenario: 不支持参数移除 / Unsupported Parameter Removal
- **WHEN** API 返回不支持参数错误时（如 stream_options、citations）
- **THEN** 自动检测并移除不支持的参数
- **AND** 记录参数移除日志
- **AND** 自动重试请求

- **WHEN** API returns unsupported parameter error (e.g., stream_options, citations)
- **THEN** automatically detects and removes unsupported parameters
- **AND** logs parameter removal
- **AND** automatically retries request

### Requirement: 错误处理和重试 / Error Handling and Retry
The system SHALL implement robust error handling with automatic retry for recoverable errors.

系统应实现强大的错误处理，对可恢复错误自动重试。

#### Scenario: 指数退避重试 / Exponential Backoff Retry
- **WHEN** 遇到可重试错误时
- **THEN** 使用指数退避策略计算延迟（base_delay: 1000ms, max_delay: 32000ms）
- **AND** 添加随机抖动（jitter: ±10%）
- **AND** 最大重试次数为 10 次
- **AND** 如果服务器返回 retry-after header，优先使用
- **AND** 每次重试前打印日志（包括尝试次数和延迟时间）

- **WHEN** encountering retryable error
- **THEN** uses exponential backoff strategy to calculate delay (base_delay: 1000ms, max_delay: 32000ms)
- **AND** adds random jitter (jitter: ±10%)
- **AND** maximum retry attempts is 10
- **AND** if server returns retry-after header, uses it preferentially
- **AND** prints log before each retry (including attempt number and delay)

#### Scenario: 速率限制处理 / Rate Limit Handling
- **WHEN** 收到 HTTP 429 (Rate Limit) 响应时
- **THEN** 从 retry-after header 读取等待时间（秒）
- **AND** 如果 retry-after 不存在，使用默认延迟
- **AND** 限制最大等待时间为 60 秒
- **AND** 等待指定时间后重试

- **WHEN** receiving HTTP 429 (Rate Limit) response
- **THEN** reads wait time from retry-after header (in seconds)
- **AND** if retry-after doesn't exist, uses default delay
- **AND** limits maximum wait time to 60 seconds
- **AND** waits specified time then retries

#### Scenario: 可修复错误自动修复 / Fixable Error Auto-correction
- **WHEN** 检测到可修复的参数错误时
- **THEN** 自动应用参数转换（如 max_tokens → max_completion_tokens）
- **AND** 记录转换日志
- **AND** 立即重试（不等待退避时间）
- **AND** 如果连续 3 次失败，停止自动修复

- **WHEN** detecting fixable parameter error
- **THEN** automatically applies parameter transformation (e.g., max_tokens → max_completion_tokens)
- **AND** logs transformation
- **AND** retries immediately (no backoff delay)
- **AND** if fails 3 times consecutively, stops auto-correction

#### Scenario: 不可重试错误 / Non-retryable Errors
- **WHEN** 遇到不可重试错误时（HTTP 401、403、400）
- **THEN** 立即返回错误，不重试
- **AND** 提供清晰的错误消息
- **AND** 包含 API 响应的详细信息（状态码、错误消息）

- **WHEN** encountering non-retryable error (HTTP 401, 403, 400)
- **THEN** returns error immediately, no retry
- **AND** provides clear error message
- **AND** includes detailed information from API response (status code, error message)

### Requirement: 端点回退支持 / Endpoint Fallback Support
The system SHALL support endpoint fallback for providers using non-standard endpoints.

系统应支持端点回退，以处理使用非标准端点的提供商。

#### Scenario: MiniMax 端点回退 / MiniMax Endpoint Fallback
- **WHEN** 检测到 provider 为 "minimax" 时
- **THEN** 首先尝试 /text/chatcompletion_v2 端点
- **AND** 如果返回 404，回退到 /chat/completions 端点
- **AND** 记录端点切换日志

- **WHEN** detecting provider as "minimax"
- **THEN** first tries /text/chatcompletion_v2 endpoint
- **AND** if returns 404, falls back to /chat/completions endpoint
- **AND** logs endpoint switch

#### Scenario: 其他提供商支持 / Other Provider Support
- **WHEN** 使用其他 OpenAI-compatible 提供商时
- **THEN** 默认使用 /chat/completions 端点
- **AND** 支持通过配置自定义端点路径
- **AND** 端点回退列表可扩展

- **WHEN** using other OpenAI-compatible providers
- **THEN** uses /chat/completions endpoint by default
- **AND** supports custom endpoint path via configuration
- **AND** endpoint fallback list is extensible

### Requirement: 多提供商兼容性 / Multi-Provider Compatibility
The system SHALL support multiple OpenAI-compatible API providers with identical behavior.

系统应支持多个 OpenAI-compatible API 提供商，行为一致。

#### Scenario: OpenAI 官方 / OpenAI Official
- **WHEN** 使用 api.openai.com 端点时
- **THEN** 使用标准 OpenAI API 格式
- **AND** 支持所有 OpenAI 功能（流式、工具调用、vision 等）
- **AND** 遵循 OpenAI 速率限制

- **WHEN** using api.openai.com endpoint
- **THEN** uses standard OpenAI API format
- **AND** supports all OpenAI features (streaming, tool calls, vision, etc.)
- **AND** follows OpenAI rate limits

#### Scenario: DeepSeek / DeepSeek
- **WHEN** 使用 DeepSeek API 时
- **THEN** 使用 api.deepseek.com 端点
- **AND** 使用标准 Chat Completions 格式
- **AND** 支持 DeepSeek 特定模型（deepseek-chat、deepseek-coder）

- **WHEN** using DeepSeek API
- **THEN** uses api.deepseek.com endpoint
- **AND** uses standard Chat Completions format
- **AND** supports DeepSeek-specific models (deepseek-chat, deepseek-coder)

#### Scenario: 自定义提供商 / Custom Provider
- **WHEN** 用户配置自定义 base_url 时
- **THEN** 使用配置的端点
- **AND** 假设使用标准 OpenAI API 格式
- **AND** 应用端点回退逻辑（如果适用）
- **AND** 记录请求和响应详情（便于调试）

- **WHEN** user configures custom base_url
- **THEN** uses configured endpoint
- **AND** assumes standard OpenAI API format
- **AND** applies endpoint fallback logic (if applicable)
- **AND** logs request and response details (for debugging)

### Requirement: 测试覆盖 / Test Coverage
The system SHALL have comprehensive test coverage including unit, integration, and compatibility tests.

系统应有全面的测试覆盖，包括单元测试、集成测试和兼容性测试。

#### Scenario: 单元测试 / Unit Tests
- **WHEN** 运行单元测试时
- **THEN** 测试模型特性检测（GPT-5 vs GPT-4）
- **AND** 测试参数转换（max_tokens → max_completion_tokens）
- **AND** 测试错误检测和分类
- **AND** 测试 SSE 流解析
- **AND** 所有测试通过且无 clippy 警告

- **WHEN** running unit tests
- **THEN** tests model feature detection (GPT-5 vs GPT-4)
- **AND** tests parameter transformation (max_tokens → max_completion_tokens)
- **AND** tests error detection and classification
- **AND** tests SSE stream parsing
- **AND** all tests pass with no clippy warnings

#### Scenario: 集成测试 / Integration Tests
- **WHEN** 运行集成测试时（使用环境变量配置凭证）
- **THEN** 测试非流式请求（真实 API）
- **AND** 测试流式请求（真实 API）
- **AND** 测试工具调用（真实 API）
- **AND** 测试多个提供商（OpenAI、DeepSeek）
- **AND** 测试默认使用 #[ignore] 属性（需手动运行）

- **WHEN** running integration tests (using environment variables for credentials)
- **THEN** tests non-streaming requests (real API)
- **AND** tests streaming requests (real API)
- **AND** tests tool calls (real API)
- **AND** tests multiple providers (OpenAI, DeepSeek)
- **AND** tests are ignored by default (require manual run)

#### Scenario: 兼容性测试 / Compatibility Tests
- **WHEN** 运行兼容性测试时
- **THEN** 加载原版 TypeScript 的配置文件
- **AND** 验证配置解析正确
- **AND** 使用相同配置发送请求
- **AND** 比较响应格式与原版一致

- **WHEN** running compatibility tests
- **THEN** loads original TypeScript configuration files
- **AND** verifies configuration parsing is correct
- **AND** sends requests using same configuration
- **AND** compares response format consistency with original version

#### Scenario: 错误场景测试 / Error Scenario Tests
- **WHEN** 测试错误场景时
- **THEN** 测试重试机制（模拟网络错误）
- **AND** 测试端点回退（模拟 404）
- **AND** 测试参数自动修复（模拟参数错误）
- **AND** 测试速率限制处理（模拟 429）
- **AND** 测试取消操作（模拟 AbortSignal）

- **WHEN** testing error scenarios
- **THEN** tests retry mechanism (simulates network error)
- **AND** tests endpoint fallback (simulates 404)
- **AND** tests parameter auto-correction (simulates parameter error)
- **AND** tests rate limit handling (simulates 429)
- **AND** tests cancellation (simulates AbortSignal)
