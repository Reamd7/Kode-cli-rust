# Delta Spec: Streaming Response

## MODIFIED Requirements

### Requirement: ModelAdapter trait / ModelAdapter Trait
The system SHALL implement streaming response in ModelAdapter.

系统应在 ModelAdapter 中实现流式响应。

#### Scenario: SSE 流式响应 / SSE Streaming Response
- **WHEN** 实现 stream_message() 时
- **THEN** 返回 Pin<Box<dyn Stream<Item = Result<StreamChunk>>>>
- **AND** StreamChunk 包含多种事件类型
- **AND** 正确处理流结束

- **WHEN** implementing stream_message()
- **THEN** returns Pin<Box<dyn Stream<Item = Result<StreamChunk>>>>
- **AND** StreamChunk includes multiple event types
- **AND** properly handles stream end

### Requirement: StreamChunk 类型 / StreamChunk Types
The system SHALL define comprehensive stream chunk types.

系统应定义完整的流块类型。

#### Scenario: 完整流块 / Complete Stream Chunks
- **WHEN** 定义 StreamChunk 时
- **THEN** ContentBlockStart { index: usize }
- **AND** ContentBlockDelta { delta: String }
- **AND** ContentBlockStop
- **AND** ToolUse { tool_name: String, params: Value, tool_use_id: String }
- **AND** ToolResult { tool_use_id: String, content: String }
- **AND** MessageStop

- **WHEN** defining StreamChunk
- **THEN** ContentBlockStart { index: usize }
- **AND** ContentBlockDelta { delta: String }
- **AND** ContentBlockStop
- **AND** ToolUse { tool_name: String, params: Value, tool_use_id: String }
- **AND** ToolResult { tool_use_id: String, content: String }
- **AND** MessageStop
