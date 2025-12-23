# 设计文档 - Anthropic 服务 / Design Document - Anthropic Service

## Context

Anthropic 服务是 Kode-Rust 的第一个 AI 模型集成，负责与 Anthropic Claude API 交互。

The Anthropic service is the first AI model integration in Kode-Rust, responsible for interacting with the Anthropic Claude API.

### 当前状态 / Current State

Anthropic 服务尚未实现，计划在 `crates/kode-services/src/anthropic.rs` 中实现。

The Anthropic service has not been implemented yet, planned to be implemented in `crates/kode-services/src/anthropic.rs`.

## TypeScript 版本参考 / TypeScript Version Reference

在实现本设计时，请参考原版 TypeScript 项目中的以下文件：

When implementing this design, refer to the following files in the original TypeScript project:

### Anthropic API 服务 / Anthropic API Service
- **Claude 服务**: `/Users/gemini/Documents/backup/Kode-cli/src/services/claude.ts`
  - API 客户端初始化
  - 消息发送和接收
  - 流式响应处理
  - 错误处理和重试

### API 适配器 / API Adapters
- **适配器目录**: `/Users/gemini/Documents/backup/Kode-cli/src/services/adapters/`
  - Anthropic 适配器实现
  - 消息格式转换
  - 工具调用处理

### 相关类型 / Related Types
- **请求上下文**: `/Users/gemini/Documents/backup/Kode-cli/src/types/RequestContext.ts`
  - API 请求参数
  - 模型配置
  - 流式响应选项

### 实现细节 / Implementation Details
1. **API 版本**: Anthropic Messages API (2023-06-01)
2. **流式响应**: Server-Sent Events (SSE)
3. **错误处理**: 区分可重试和不可重试错误
4. **超时控制**: 实现请求超时

## Goals / Non-Goals

### Goals

- [ ] 实现 AnthropicService 结构 / Implement AnthropicService struct
- [ ] 实现 ModelAdapter for AnthropicService / Implement ModelAdapter for AnthropicService
- [ ] 支持同步消息发送 / Support synchronous message sending
- [ ] 支持 SSE 流式响应 / Support SSE streaming responses
- [ ] 正确的错误处理 / Proper error handling

### Non-Goals

- [ ] Prompt caching 功能 / Prompt caching feature
- [ ] Batch API 支持 / Batch API support
- [ ] 使用量统计 / Usage statistics

## Decisions

### 决策 1: HTTP 客户端选择 / Decision 1: HTTP Client Selection

**选择**: 使用 Reqwest 作为 HTTP 客户端

**Choice**: Use Reqwest as the HTTP client

**理由 / Rationale**:
- 项目已依赖 Reqwest / Project already depends on Reqwest
- 原生支持异步和流式响应 / Native support for async and streaming responses
- 与 Tokio 运行时兼容 / Compatible with Tokio runtime

**实现 / Implementation**:
```rust
/// 来自 ARCHITECTURE.md L267-271
pub struct AnthropicService {
    client: reqwest::Client,
    api_key: String,
    base_url: String,
}

impl ModelAdapter for AnthropicService {
    async fn send_message(...) -> Result<Response> {
        // 实现完整的 Messages API 调用
    }

    async fn stream_message(...) -> Result<...> {
        // 实现 SSE 流式响应
    }
}
```

### 决策 2: API 请求结构 / Decision 2: API Request Structure

**选择**: 定义专门的请求结构体

**Choice**: Define dedicated request structs

**理由 / Rationale**:
- 类型安全 / Type-safe
- 清晰的 API 映射 / Clear API mapping
- 便于序列化 / Easy to serialize

**实现 / Implementation**:
```rust
#[derive(Serialize)]
struct AnthropicRequest {
    model: String,
    messages: Vec<AnthropicMessage>,
    max_tokens: u32,
    tools: Option<Vec<AnthropicTool>>,
    stream: Option<bool>,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum AnthropicMessageRole {
    User,
    Assistant,
}

#[derive(Serialize)]
struct AnthropicMessage {
    role: AnthropicMessageRole,
    content: Vec<AnthropicContent>,
}

#[derive(Serialize)]
#[serde(untagged)]
enum AnthropicContent {
    Text { text: String },
    Image { type_: String, source: ImageSource },
    ToolUse { type_: String, id: String, name: String, input: serde_json::Value },
    ToolResult { type_: String, tool_use_id: String, content: String },
}
```

### 决策 3: 流式响应解析 / Decision 3: Streaming Response Parsing

**选择**: 手动解析 SSE 事件流

**Choice**: Manually parse SSE event streams

**理由 / Rationale**:
- Anthropic 使用自定义 SSE 格式 / Anthropic uses custom SSE format
- 需要精确控制事件处理 / Need precise control over event handling
- 避免引入额外依赖 / Avoid introducing additional dependencies

**实现 / Implementation**:
```rust
use futures_util::StreamExt;

pub async fn stream_message(
    &self,
    messages: Vec<Message>,
    tools: Vec<ToolSchema>,
) -> Result<Pin<Box<dyn Stream<Item = Result<StreamChunk>>>>> {
    let request = AnthropicRequest {
        model: self.model.clone(),
        messages: convert_messages(messages)?,
        max_tokens: 4096,
        tools: Some(convert_tools(tools)?),
        stream: Some(true),
    };

    let response = self.client
        .post(format!("{}/v1/messages", self.base_url))
        .header("x-api-key", &self.api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&request)
        .send()
        .await?;

    let stream = response.bytes_stream().map(|result| {
        match result {
            Ok(bytes) => parse_sse_event(&bytes),
            Err(e) => Err(Error::ApiError(e.to_string())),
        }
    });

    Ok(Box::pin(stream))
}

fn parse_sse_event(bytes: &[u8]) -> Result<StreamChunk> {
    let text = std::str::from_utf8(bytes)?;
    // 解析 "event: " 和 "data: " 行
    for line in text.lines() {
        if line.starts_with("data: ") {
            let data = &line[6..];
            if let Ok(event) = serde_json::from_str::<AnthropicEvent>(data) {
                return Ok(convert_event(event));
            }
        }
    }
    Ok(StreamChunk::ContentBlockDelta { delta: String::new() })
}
```

### 决策 4: 错误处理 / Decision 4: Error Handling

**选择**: 定义专门的错误类型

**Choice**: Define dedicated error types

**理由 / Rationale**:
- 清晰的错误分类 / Clear error categorization
- 便于错误恢复 / Easier error recovery
- 更好的用户体验 / Better user experience

**实现 / Implementation**:
```rust
#[derive(Error, Debug)]
pub enum AnthropicError {
    #[error("API request failed: {0}")]
    RequestError(String),

    #[error("API returned error: {message}")]
    ApiError {
        message: String,
        type_: Option<String>,
    },

    #[error("Rate limit exceeded")]
    RateLimitError,

    #[error("Invalid API key")]
    InvalidApiKey,

    #[error("Stream interrupted")]
    StreamInterrupted,
}

impl Fromreqwest::Error> for AnthropicError {
    fn from(e: reqwest::Error) -> Self {
        AnthropicError::RequestError(e.to_string())
    }
}
```

### 决策 5: 工具定义转换 / Decision 5: Tool Definition Conversion

**选择**: 实现自动转换工具 schema

**Choice**: Implement automatic tool schema conversion

**理由 / Rationale**:
- 需要统一格式和 Anthropic 格式的转换 / Need conversion between unified format and Anthropic format
- 支持复杂的工具参数定义 / Support complex tool parameter definitions

**实现 / Implementation**:
```rust
fn convert_tools(tools: Vec<ToolSchema>) -> Result<Vec<AnthropicTool>> {
    tools.into_iter().map(|tool| {
        Ok(AnthropicTool {
            name: tool.name,
            description: tool.description,
            input_schema: tool.parameters,
        })
    }).collect()
}

#[derive(Serialize)]
struct AnthropicTool {
    name: String,
    description: String,
    input_schema: serde_json::Value,
}
```

## 数据流 / Data Flow

### 同步请求 / Synchronous Request

```
ModelManager::send_message()
    ↓
AnthropicService::send_message(messages, tools)
    ↓
构建 AnthropicRequest / Build AnthropicRequest
    ↓
HTTP POST /v1/messages
    ↓
等待响应 / Wait for response
    ↓
解析 AnthropicResponse / Parse AnthropicResponse
    ↓
转换为统一 Response 格式 / Convert to unified Response format
    ↓
返回给调用者 / Return to caller
```

### 流式请求 / Streaming Request

```
ModelManager::stream_message()
    ↓
AnthropicService::stream_message(messages, tools)
    ↓
构建 AnthropicRequest (stream: true)
    ↓
HTTP POST /v1/messages
    ↓
返回 Stream<StreamChunk>
    ↓
消费者逐步处理事件 / Consumer processes events progressively
    ↓
实时更新 UI / Real-time UI update
```

## API 映射 / API Mapping

### 统一格式 → Anthropic 格式 / Unified → Anthropic

| 统一 / Unified | Anthropic |
|----------------|-----------|
| `MessageRole::User` | `{role: "user", content: [...]}` |
| `MessageRole::Assistant` | `{role: "assistant", content: [...]}` |
| `ContentBlock::Text(t)` | `{type: "text", text: t}` |
| `ContentBlock::ToolUse(t)` | `{type: "tool_use", id, name, input}` |
| `ContentBlock::ToolResult(r)` | `{type: "tool_result", tool_use_id, content}` |

### Anthropic 事件 → StreamChunk / Anthropic Event → StreamChunk

| Anthropic Event Type | StreamChunk |
|---------------------|-------------|
| `content_block_start` | `ContentBlockStart { index }` |
| `content_block_delta` | `ContentBlockDelta { index, delta }` |
| `content_block_stop` | `ContentBlockStop { index }` |
| `tool_use` | `ToolUse { tool_name, params }` |
| `message_stop` | `MessageStop` |

## 错误处理 / Error Handling

| Anthropic 错误 / Anthropic Error | 应用错误 / Application Error |
|----------------------------------|---------------------------|
| 401 Unauthorized | `InvalidApiKey` |
| 429 Rate Limit | `RateLimitError` |
| 400 Invalid Request | `ApiError` with details |
| 500 Server Error | `RequestError` with retry suggestion |

## 测试策略 / Testing Strategy

### 单元测试 / Unit Tests

```rust
#[test]
fn test_convert_message() {
    // 测试消息格式转换
}

#[test]
fn test_parse_sse_event() {
    // 测试 SSE 事件解析
}

#[test]
fn test_error_response() {
    // 测试错误响应处理
}
```

### 集成测试 / Integration Tests

```rust
#[tokio::test]
#[ignore]  // 需要 API key
async fn test_real_api_call() {
    let service = AnthropicService::new(
        std::env::var("ANTHROPIC_API_KEY").unwrap(),
        "claude-sonnet-4-5-20250929".to_string(),
    );

    let messages = vec![Message {
        role: MessageRole::User,
        content: vec![ContentBlock::Text(TextBlock {
            text: "Hello!".to_string(),
        })],
    }];

    let response = service.send_message(messages, vec![]).await.unwrap();
    assert!(!response.content.is_empty());
}
```

## Risks / Trade-offs

### 风险 1: API 兼容性变更 / Risk 1: API Compatibility Changes

**风险 / Risk**: Anthropic 可能更改 API 格式

**缓解措施 / Mitigation**:
- 锁定 API 版本 (anthropic-version header) / Lock API version (anthropic-version header)
- 监控 API 更新公告 / Monitor API update announcements
- 完善错误处理 / Comprehensive error handling

### 风险 2: 流式响应中断 / Risk 2: Streaming Response Interruption

**风险 / Risk**: 网络问题可能导致流中断

**缓解措施 / Mitigation**:
- 实现自动重连机制 / Implement automatic reconnection
- 设置合理的超时 / Set reasonable timeouts
- 提供错误恢复选项 / Provide error recovery options

## Open Questions

1. **是否支持 Prompt Caching？**
   - 建议：在性能优化阶段添加
   - Rationale: Add during performance optimization phase

2. **如何处理长时间运行的流？**
   - 建议：添加超时和取消机制
   - Rationale: Add timeout and cancellation mechanism

3. **是否需要支持 Batch API？**
   - 建议：暂不实现，使用场景有限
   - Rationale: Don't implement for now, limited use cases

## 参考资源 / References

- [Anthropic Messages API 文档](https://docs.anthropic.com/claude/reference/messages_post)
- [Anthropic Streaming 文档](https://docs.anthropic.com/claude/docs/streaming)
- [Reqwest 文档](https://docs.rs/reqwest/)
- [Tokio 文档](https://tokio.rs/)
