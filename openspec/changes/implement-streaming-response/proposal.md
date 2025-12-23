# Change: 实现流式响应 / Implement Streaming Response

## Why

流式响应提供更好的用户体验，可以实时看到 AI 的输出，减少等待时间。

Streaming response provides better user experience by showing AI output in real-time, reducing waiting time.

## What Changes

- 实现 SSE 流式解析
- 实现 AnthropicService::stream_message 方法
- 实现 StreamChunk 枚举（ContentBlockStart, ContentBlockDelta, ToolUse, etc.）
- 实现 StreamingResponse 包装器
- 处理流式错误和重连

- Implement SSE streaming parsing
- Implement AnthropicService::stream_message method
- Implement StreamChunk enum (ContentBlockStart, ContentBlockDelta, ToolUse, etc.)
- Implement StreamingResponse wrapper
- Handle streaming errors and reconnection

## Impact

**Affected specs:**
- message-model (MODIFIED)
- anthropic-service (MODIFIED)

**Affected code:**
- `crates/kode-services/src/anthropic/streaming.rs` (新建)
- `crates/kode-core/src/model/types.rs` (修改)
