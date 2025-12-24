# Delta Spec: Message Model System

## ADDED Requirements

### Requirement: Message 结构体 / Message Structure
The system SHALL define a message structure for AI conversations.

系统应定义用于 AI 对话的消息结构。

#### Scenario: Message 结构定义 / Message Structure Definition
- **WHEN** 定义 Message 结构体时
- **THEN** 包含 role 字段 (Role 枚举: User, Assistant, System)
- **AND** 包含 content 字段 (MessageContent 枚举: Text, Blocks)
- **AND** 支持 serde 序列化/反序列化

- **WHEN** defining Message struct
- **THEN** includes role field (Role enum: User, Assistant, System)
- **AND** includes content field (MessageContent enum: Text, Blocks)
- **AND** supports serde serialization/deserialization

### Requirement: 内容块类型 / Content Block Types
The system SHALL support multiple content block types for rich message content.

系统应支持多种内容块类型以支持丰富的消息内容。

#### Scenario: 文本内容块 / Text Content Block
- **WHEN** 消息包含纯文本时
- **THEN** 使用 TextBlock 存储内容
- **AND** 支持多语言文本

- **WHEN** a message contains plain text
- **THEN** use TextBlock to store the content
- **AND** support multi-language text

#### Scenario: 工具使用块 / Tool Use Content Block
- **WHEN** 助手请求调用工具时
- **THEN** 使用 ToolUseBlock 表示
- **AND** 包含 tool_use_id, tool_name, parameters 字段

- **WHEN** the assistant requests to call a tool
- **THEN** use ToolUseBlock to represent it
- **AND** includes tool_use_id, tool_name, parameters fields

#### Scenario: 工具结果块 / Tool Result Content Block
- **WHEN** 工具执行完成时
- **THEN** 使用 ToolResultBlock 表示结果
- **AND** 包含 tool_use_id, content, is_error 字段

- **WHEN** a tool execution completes
- **THEN** use ToolResultBlock to represent the result
- **AND** includes tool_use_id, content, is_error fields

#### Scenario: 图片内容块 / Image Content Block
- **WHEN** 消息包含图片时
- **THEN** 使用 ImageBlock 存储
- **AND** 包含 image_type, media_type, data 字段

- **WHEN** a message contains an image
- **THEN** use ImageBlock to store it
- **AND** includes image_type, media_type, data fields

### Requirement: ModelAdapter trait / ModelAdapter Trait
The system SHALL define a trait for model adapters.

系统应定义模型适配器的 trait。

#### Scenario: 非流式方法 / Non-streaming Method
- **WHEN** 定义 ModelAdapter trait 时
- **THEN** 包含 send_message() 方法
- **AND** 方法接受 messages, system_prompt, max_tokens 参数
- **AND** 方法返回 Result<ModelResponse>

- **WHEN** defining ModelAdapter trait
- **THEN** includes send_message() method
- **AND** method accepts messages, system_prompt, max_tokens parameters
- **AND** method returns Result<ModelResponse>

#### Scenario: 流式方法 / Streaming Method
- **WHEN** 定义 ModelAdapter trait 时
- **THEN** 包含 stream_message() 方法
- **AND** 方法返回 Result<StreamingResponse>
- **AND** 使用 async_trait

- **WHEN** defining ModelAdapter trait
- **THEN** includes stream_message() method
- **AND** method returns Result<StreamingResponse>
- **AND** uses async_trait

#### Scenario: 模型名称和支持检查 / Model Name and Support Check
- **WHEN** 查询模型信息时
- **THEN** model_name() 返回模型名称
- **AND** supports_streaming() 返回是否支持流式

- **WHEN** querying model information
- **THEN** model_name() returns the model name
- **AND** supports_streaming() returns whether streaming is supported

### Requirement: StreamChunk 枚举 / StreamChunk Enum
The system SHALL define stream chunk types for streaming responses.

系统应定义流式响应的流块类型。

#### Scenario: 流块类型 / Stream Chunk Types
- **WHEN** 定义 StreamChunk 枚举时
- **THEN** 包含 ContentBlockStart { index }
- **AND** 包含 ContentBlockDelta { index, delta }
- **AND** 包含 ContentBlockStop { index }
- **AND** 包含 ToolUse { tool_name, tool_use_id, parameters }
- **AND** 包含 MessageStop { usage }
- **AND** 包含 Error { message }

- **WHEN** defining StreamChunk enum
- **THEN** includes ContentBlockStart { index }
- **AND** includes ContentBlockDelta { index, delta }
- **AND** includes ContentBlockStop { index }
- **AND** includes ToolUse { tool_name, tool_use_id, parameters }
- **AND** includes MessageStop { usage }
- **AND** includes Error { message }

### Requirement: TokenUsage 结构 / TokenUsage Struct
The system SHALL track token usage for API calls.

系统应跟踪 API 调用的 token 使用情况。

#### Scenario: Token 统计 / Token Statistics
- **WHEN** API 调用完成时
- **THEN** 返回 input_tokens 数量
- **AND** 返回 output_tokens 数量
- **AND** 可选返回 total_tokens 数量

- **WHEN** API call completes
- **THEN** return input_tokens count
- **AND** return output_tokens count
- **AND** optionally return total_tokens count

### Requirement: StreamingResponse 包装器 / StreamingResponse Wrapper
The system SHALL provide a wrapper for streaming responses.

系统应提供流式响应的包装器。

#### Scenario: 流式响应处理 / Streaming Response Handling
- **WHEN** 创建流式响应时
- **THEN** 基于 tokio mpsc channel 实现
- **AND** 实现 Stream trait
- **AND** 提供 channel() 辅助方法

- **WHEN** creating streaming response
- **THEN** based on tokio mpsc channel implementation
- **AND** implements Stream trait
- **AND** provides channel() helper method

## Non-Goals（本变更不包含）

以下功能由独立的变更提案负责：

- **模型管理器**（`implement-model-switching`）
  - 多模型配置管理
  - 运行时模型切换
  - 模型指针

- **消息上下文管理**（`implement-context-management`）
  - 上下文窗口管理
  - Token 计数器
  - 智能裁剪策略

- **具体模型服务**（`implement-anthropic-service`, `implement-openai-service`）
  - Anthropic API 客户端
  - OpenAI API 客户端
  - 流式 SSE 解析

- **Tool 角色消息**（可能由工具系统实现）
  - `role="tool"` 消息类型
  - 工具结果消息格式

- **消息元数据**
  - UUID 字段
  - 创建时间
  - 成本追踪（costUSD, durationMs）
