# 设计文档 - 消息与模型抽象 / Design Document - Message and Model Abstraction

## Context

消息与模型抽象是 Kode-Rust 的核心接口层，负责统一不同 AI 提供商的 API 差异，提供一致的调用接口。

The message and model abstraction is a core interface layer in Kode-Rust, responsible for unifying API differences across different AI providers and providing a consistent calling interface.

### 当前状态 / Current State

消息和模型模块的骨架已创建，但尚未实现具体功能：

The skeleton for message and model modules has been created, but specific features are not yet implemented:

- `crates/kode-core/src/message/mod.rs` - 消息管理（待实现）/ Message management (to be implemented)
- `crates/kode-core/src/model/mod.rs` - 模型抽象（待实现）/ Model abstraction (to be implemented)
- `crates/kode-core/src/context/mod.rs` - 上下文管理（待实现）/ Context management (to be implemented)

## Goals / Non-Goals

### Goals

- [ ] 定义统一的消息结构 / Define unified message structure
- [ ] 实现 ModelAdapter trait / Implement ModelAdapter trait
- [ ] 实现消息上下文管理器 / Implement message context manager
- [ ] 支持流式响应 / Support streaming responses
- [ ] 支持多模型提供商 / Support multiple model providers

### Non-Goals

- [ ] 具体 AI API 的实现（在 kode-services 中）/ Specific AI API implementations (in kode-services)
- [ ] 消息持久化存储 / Message persistence storage
- [ ] 精确的 token 计数（使用估算即可）/ Precise token counting (estimation is sufficient)

## Decisions

### 决策 1: 消息结构设计 / Decision 1: Message Structure Design

**选择**: 使用枚举表示不同类型的内容块

**Choice**: Use enums to represent different types of content blocks

**理由 / Rationale**:
- Anthropic 和 OpenAI 都使用内容块数组 / Both Anthropic and OpenAI use content block arrays
- 类型安全 / Type-safe
- 便于扩展新的内容类型 / Easy to extend with new content types

**实现 / Implementation**:
```rust
/// 来自 ARCHITECTURE.md L166-179
#[async_trait]
pub trait ModelAdapter: Send + Sync {
    async fn send_message(
        &self,
        messages: Vec<Message>,
        tools: Vec<ToolSchema>,
    ) -> Result<Response>;

    async fn stream_message(
        &self,
        messages: Vec<Message>,
        tools: Vec<ToolSchema>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<StreamChunk>>>>>;
}

/// 来自 ARCHITECTURE.md L181-184
pub struct ModelManager {
    adapters: HashMap<String, Box<dyn ModelAdapter>>,
    current_model: String,
}
```

### 决策 2: 流式响应设计 / Decision 2: Streaming Response Design

**实现 / Implementation**:
```rust
/// 来自 ARCHITECTURE.md L286-301
pub struct StreamingResponse {
    stream: Pin<Box<dyn Stream<Item = Result<StreamChunk>>>>,
}

pub enum StreamChunk {
    ContentBlockStart { index: usize },
    ContentBlockDelta { delta: String },
    ContentBlockStop,
    ToolUse { tool_name: String, params: Value },
    MessageStop,
}

impl StreamingResponse {
    pub async fn collect_full_response(self) -> Result<Response>;
    pub fn into_stream(self) -> impl Stream<Item = Result<StreamChunk>>;
}
```

### 决策 3: 模型管理器设计 / Decision 3: Model Manager Design

**选择**: 使用 HashMap 存储模型适配器

**理由 / Rationale**:
- O(1) 查找性能 / O(1) lookup performance
- 支持动态添加模型 / Support dynamic model addition
- 简单的实现 / Simple implementation

**实现 / Implementation** (已在决策 1 中包含):
```rust
/// 来自 ARCHITECTURE.md L181-184
pub struct ModelManager {
    adapters: HashMap<String, Box<dyn ModelAdapter>>,
    current_model: String,
}
```

### 决策 4: 消息上下文管理 / Decision 4: Message Context Management

**来自 ARCHITECTURE.md L187-200**:
```rust
pub struct MessageContextManager {
    messages: Vec<Message>,
    max_tokens: usize,
    token_counter: Box<dyn TokenCounter>,
}

impl MessageContextManager {
    pub fn add_message(&mut self, msg: Message) -> Result<()>;
    pub fn trim_to_fit(&mut self) -> Result<()>;
    pub fn get_context(&self) -> &[Message];
}
```

### 决策 5: Token 计数策略 / Decision 5: Token Counting Strategy

**选择**: 使用字符数估算，不使用精确 tokenizer

**Choice**: Use character count estimation, not precise tokenizer

**理由 / Rationale**:
- 精确 tokenizer 需要加载大量数据 / Precise tokenizer requires loading large amounts of data
- 增加启动时间和内存占用 / Increases startup time and memory usage
- 字符估算对大多数场景足够准确 / Character estimation is accurate enough for most scenarios

**设计 / Design**:
```rust
pub trait TokenCounter: Send + Sync {
    fn count_tokens(&self, text: &str) -> usize;
}

/// 简单的字符估算计数器
pub struct SimpleTokenCounter;

impl TokenCounter for SimpleTokenCounter {
    fn count_tokens(&self, text: &str) -> usize {
        // 估算：英文约 4 字符/token，中文约 2 字符/token
        let chars = text.chars().count();
        let chinese_chars = text.chars().filter(|c| is_chinese(*c)).count();
        let non_chinese_chars = chars - chinese_chars;

        (chinese_chars / 2) + (non_chinese_chars / 4)
    }
}
```

## 数据流 / Data Flow

```
用户输入 / User Input
    ↓
Message { role: User, content: [...] }
    ↓
ModelManager::send_message(messages)
    ↓
ModelAdapter::send_message(messages, tools)
    ↓
HTTP 请求到 AI API / HTTP Request to AI API
    ↓
AI 响应 / AI Response
    ↓
Response { content: [...], tool_uses: [...] }
    ↓
添加到上下文 / Add to context
    ↓
更新 UI / Update UI
```

## 架构关系 / Architecture Relationships

```
┌─────────────────────────────────────────────────────┐
│                      kode-cli                       │
│  (CLI 入口，负责启动应用和协调各模块)                │
│  (CLI entry point, responsible for startup and      │
│   coordination)                                     │
└─────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────┐
│                    kode-core                        │
│  ┌──────────────┐  ┌──────────────┐  ┌───────────┐ │
│  │   message/   │  │    model/    │  │ context/  │ │
│  │ (消息结构)    │  │ (模型抽象)    │  │(上下文管理)│ │
│  └──────────────┘  └──────────────┘  └───────────┘ │
└─────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────┐
│                 kode-services                       │
│  ┌──────────────┐  ┌──────────────┐                │
│  │  anthropic/  │  │   openai/    │  (具体 API 实现) │
│  └──────────────┘  └──────────────┘  (Specific API) │
└─────────────────────────────────────────────────────┘
```

## 错误处理 / Error Handling

| 错误类型 / Error Type | 描述 / Description | 处理方式 / Handling |
|---------------------|-------------------|-------------------|
| `ModelNotFound` | 请求的模型不存在 / Requested model does not exist | 返回可用模型列表 / Return list of available models |
| `ApiError` | AI API 返回错误 / AI API returns error | 传递错误详情给用户 / Pass error details to user |
| `RateLimitError` | 超过速率限制 / Rate limit exceeded | 提示用户等待或重试 / Prompt user to wait or retry |
| `TokenLimitError` | 超过上下文长度 / Context length exceeded | 自动裁剪上下文 / Automatically trim context |

## 测试策略 / Testing Strategy

### 单元测试 / Unit Tests

```rust
#[test]
fn test_message_creation() {
    // 测试消息创建
}

#[tokio::test]
async fn test_token_counter() {
    // 测试 token 计数
}

#[tokio::test]
async fn test_context_trimming() {
    // 测试上下文裁剪
}
```

### Mock 测试 / Mock Tests

```rust
struct MockModelAdapter {
    responses: Vec<Response>,
}

#[async_trait]
impl ModelAdapter for MockModelAdapter {
    async fn send_message(&self, messages: Vec<Message>) -> Result<Response> {
        Ok(self.responses[0].clone())
    }
    // ...
}
```

## Risks / Trade-offs

### 风险 1: Token 估算不准确 / Risk 1: Inaccurate Token Estimation

**风险 / Risk**: 字符估算可能与实际 token 数相差较大

**缓解措施 / Mitigation**:
- 设置保守的上下文限制 / Set conservative context limits
- 考虑添加可选的精确 token 计数 / Consider adding optional precise token counting
- 监控实际使用情况并调整估算 / Monitor actual usage and adjust estimation

### 风险 2: 流式响应处理复杂 / Risk 2: Complex Streaming Response Handling

**风险 / Risk**: SSE 流可能中断、乱序或延迟

**缓解措施 / Mitigation**:
- 实现重连机制 / Implement reconnection mechanism
- 超时处理 / Timeout handling
- 错误恢复策略 / Error recovery strategy

## Open Questions

1. **是否需要支持函数调用（Function Calling）？**
   - 建议：使用工具调用（Tool Calling）统一接口
   - Rationale: Use Tool Calling as a unified interface

2. **是否需要消息批处理？**
   - 建议：暂不实现，单次请求足够
   - Rationale: Don't implement for now, single request is sufficient

3. **是否需要支持多模态输入？**
   - 建议：先支持图片，未来扩展视频
   - Rationale: Support images first, extend to video in the future

## 参考资源 / References

- [Anthropic Messages API](https://docs.anthropic.com/claude/reference/messages_post)
- [OpenAI Chat Completions API](https://platform.openai.com/docs/api-reference/chat)
- [async-trait 文档](https://docs.rs/async-trait/)
- [futures Stream 文档](https://docs.rs/futures/)
