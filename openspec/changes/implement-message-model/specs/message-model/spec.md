# Delta Spec: Message Model System

## ADDED Requirements

### Requirement: Message 结构体 / Message Structure
The system SHALL define a message structure for AI conversations.

系统应定义用于 AI 对话的消息结构。

#### Scenario: Message 结构定义 / Message Structure Definition
- **WHEN** 定义 Message 结构体时
- **THEN** 包含 role 字段 (Role 枚举: User, Assistant, System)
- **AND** 包含 content 字段 (Content 枚举: Text, ToolUse, ToolResult)
- **AND** 支持 serde 序列化/反序列化

- **WHEN** defining Message struct
- **THEN** includes role field (Role enum: User, Assistant, System)
- **AND** includes content field (Content enum: Text, ToolUse, ToolResult)
- **AND** supports serde serialization/deserialization

### Requirement: ModelAdapter trait / ModelAdapter Trait
The system SHALL define a trait for model adapters.

系统应定义模型适配器的 trait。

#### Scenario: 非流式方法 / Non-streaming Method
- **WHEN** 定义 ModelAdapter trait 时
- **THEN** 包含 send_message() 方法
- **AND** 方法接受 messages 和 tools 参数
- **AND** 方法返回 Result<Response>

- **WHEN** defining ModelAdapter trait
- **THEN** includes send_message() method
- **AND** method accepts messages and tools parameters
- **AND** method returns Result<Response>

#### Scenario: 流式方法 / Streaming Method
- **WHEN** 定义 ModelAdapter trait 时
- **THEN** 包含 stream_message() 方法
- **AND** 方法返回 Pin<Box<dyn Stream<Item = Result<StreamChunk>>>>
- **AND** 使用 async_trait

- **WHEN** defining ModelAdapter trait
- **THEN** includes stream_message() method
- **AND** method returns Pin<Box<dyn Stream<Item = Result<StreamChunk>>>>
- **AND** uses async_trait

### Requirement: StreamChunk 枚举 / StreamChunk Enum
The system SHALL define stream chunk types for streaming responses.

系统应定义流式响应的流块类型。

#### Scenario: 流块类型 / Stream Chunk Types
- **WHEN** 定义 StreamChunk 枚举时
- **THEN** 包含 ContentBlockStart { index: usize }
- **AND** 包含 ContentBlockDelta { delta: String }
- **AND** 包含 ContentBlockStop
- **AND** 包含 ToolUse { tool_name: String, params: Value }
- **AND** 包含 MessageStop

- **WHEN** defining StreamChunk enum
- **THEN** includes ContentBlockStart { index: usize }
- **AND** includes ContentBlockDelta { delta: String }
- **AND** includes ContentBlockStop
- **AND** includes ToolUse { tool_name: String, params: Value }
- **AND** includes MessageStop
