# Change: 实现 OpenAI-compatible 服务 / Implement OpenAI-compatible Service

## Why

支持 OpenAI-compatible API 允许用户使用多种模型提供商（OpenAI, DeepSeek, MiniMax, 等），提高灵活性。这是 Kode-Rust 项目的核心功能之一，必须与原版 TypeScript 实现保持完全兼容。

Supporting OpenAI-compatible API allows users to use multiple model providers (OpenAI, DeepSeek, MiniMax, etc.), increasing flexibility. This is a core feature of the Kode-Rust project and must maintain full compatibility with the original TypeScript implementation.

### 背景与动机 / Background & Motivation

原版 TypeScript 实现已经验证了以下关键功能的必要性：
- **多提供商支持**：用户可以配置任意 OpenAI-compatible API 端点（DeepSeek、MiniMax、Azure OpenAI 等）
- **流式响应**：实时流式输出是 CLI 工具的核心体验
- **工具调用**：模型需要能够调用工具（Function Calling）
- **错误重试机制**：网络请求需要自动重试和退避策略
- **参数自适应**：不同模型（GPT-4、GPT-5、DeepSeek 等）有不同的参数要求
- **端点回退**：某些提供商可能使用不同的端点路径（如 `/chat/completions` vs `/text/chatcompletion_v2`）

The original TypeScript implementation has demonstrated the necessity of the following key features:
- **Multi-provider support**: Users can configure any OpenAI-compatible API endpoint (DeepSeek, MiniMax, Azure OpenAI, etc.)
- **Streaming response**: Real-time streaming output is a core experience for CLI tools
- **Tool calling**: Models need to be able to call tools (Function Calling)
- **Error retry mechanism**: Network requests need automatic retry and backoff strategies
- **Parameter adaptation**: Different models (GPT-4, GPT-5, DeepSeek, etc.) have different parameter requirements
- **Endpoint fallback**: Some providers may use different endpoint paths (e.g., `/chat/completions` vs `/text/chatcompletion_v2`)

## What Changes

### 核心功能实现 / Core Features Implementation

基于原版 TypeScript 实现（`/Users/gemini/Documents/backup/Kode-cli/src/services/openai.ts`），需要实现以下核心功能：

Based on the original TypeScript implementation (`/Users/gemini/Documents/backup/Kode-cli/src/services/openai.ts`), the following core features need to be implemented:

#### 1. OpenAIService 结构体 / OpenAIService Struct
- HTTP 客户端封装（基于 `reqwest::Client`）
- API 配置管理（api_key、base_url、model_name、max_tokens）
- 请求头管理（Authorization、Content-Type）
- 超时和重试配置

- HTTP client wrapper (based on `reqwest::Client`)
- API configuration management (api_key, base_url, model_name, max_tokens)
- Request header management (Authorization, Content-Type)
- Timeout and retry configuration

#### 2. ModelAdapter Trait 实现 / ModelAdapter Trait Implementation
实现 `ModelAdapter` trait 的所有必需方法：
- `send_message()` - 非流式消息发送
- `send_message_with_tools()` - 支持工具调用的非流式消息发送
- `stream_message()` - 流式消息发送
- `stream_message_with_tools()` - 支持工具调用的流式消息发送
- `model_name()` - 返回模型名称
- `supports_streaming()` - 返回是否支持流式响应

Implement all required methods of the `ModelAdapter` trait:
- `send_message()` - Non-streaming message sending
- `send_message_with_tools()` - Non-streaming message sending with tool calls
- `stream_message()` - Streaming message sending
- `stream_message_with_tools()` - Streaming message sending with tool calls
- `model_name()` - Return model name
- `supports_streaming()` - Return whether streaming is supported

#### 3. 流式响应处理 / Streaming Response Processing
- SSE（Server-Sent Events）流解析
- 增量内容块处理
- 工具调用流处理
- 使用统计收集
- 取消信号支持（AbortSignal）

- SSE (Server-Sent Events) stream parsing
- Incremental content block processing
- Tool call stream processing
- Usage statistics collection
- Cancellation signal support (AbortSignal)

#### 4. 模型参数适配 / Model Parameter Adaptation
针对不同模型的参数要求进行自适应转换：
- GPT-5/o1 系列：使用 `max_completion_tokens` 而非 `max_tokens`，temperature 固定为 1
- DeepSeek/其他提供商：使用标准 `max_tokens` 参数
- 工具描述长度限制（OpenAI API 限制为 1024 字符）
- 不支持参数的自动移除（如 `stream_options`、`citations`）

Adaptive transformation for different model parameter requirements:
- GPT-5/o1 series: Use `max_completion_tokens` instead of `max_tokens`, temperature fixed at 1
- DeepSeek/other providers: Use standard `max_tokens` parameter
- Tool description length limit (OpenAI API limits to 1024 characters)
- Automatic removal of unsupported parameters (e.g., `stream_options`, `citations`)

#### 5. 错误处理和重试 / Error Handling and Retry
- 指数退避重试策略（base_delay: 1000ms, max_delay: 32000ms）
- 最大重试次数（默认 10 次）
- 可修复错误的自动检测和处理
- 速率限制错误的特殊处理（使用 retry-after header）
- 网络错误的重试机制

- Exponential backoff retry strategy (base_delay: 1000ms, max_delay: 32000ms)
- Maximum retry attempts (default 10)
- Automatic detection and handling of fixable errors
- Special handling for rate limit errors (using retry-after header)
- Network error retry mechanism

#### 6. 端点回退支持 / Endpoint Fallback Support
某些提供商使用非标准端点路径，需要支持回退：
- MiniMax: `/text/chatcompletion_v2` → `/chat/completions`
- 其他提供商可扩展

Some providers use non-standard endpoint paths, need to support fallback:
- MiniMax: `/text/chatcompletion_v2` → `/chat/completions`
- Other providers can be extended

#### 7. 多提供商兼容性 / Multi-Provider Compatibility
支持以下 OpenAI-compatible 提供商：
- OpenAI（官方）
- DeepSeek
- MiniMax
- Kimi（Moonshot）
- SiliconFlow
- Qwen（通义千问）
- GLM（智谱）
- Baidu Qianfan
- Azure OpenAI
- Mistral
- XAI (Grok)
- Groq
- 自定义 OpenAI-compatible API

Support the following OpenAI-compatible providers:
- OpenAI (official)
- DeepSeek
- MiniMax
- Kimi (Moonshot)
- SiliconFlow
- Qwen (Alibaba)
- GLM (Zhipu)
- Baidu Qianfan
- Azure OpenAI
- Mistral
- XAI (Grok)
- Groq
- Custom OpenAI-compatible APIs

#### 8. 测试覆盖 / Test Coverage
- 单元测试：各功能模块测试
- 集成测试：与真实 API 交互测试（使用环境变量配置凭证）
- 错误场景测试：重试、回退、参数转换等
- 兼容性测试：使用原版配置文件测试

- Unit tests: Individual module tests
- Integration tests: Real API interaction tests (using environment variables for credentials)
- Error scenario tests: Retry, fallback, parameter transformation, etc.
- Compatibility tests: Test using original configuration files

### 新增代码 / New Code

#### 新建文件 / New Files
- `crates/kode-services/src/openai/mod.rs` - OpenAI 服务模块
- `crates/kode-services/src/openai/service.rs` - OpenAIService 核心实现
- `crates/kode-services/src/openai/types.rs` - OpenAI API 类型定义
- `crates/kode-services/src/openai/error.rs` - OpenAI 特定错误类型
- `crates/kode-services/src/openai/streaming.rs` - 流式响应处理
- `crates/kode-services/src/openai/adapter.rs` - 模型参数适配逻辑
- `crates/kode-services/src/openai/tests.rs` - 集成测试

- `crates/kode-services/src/openai/mod.rs` - OpenAI service module
- `crates/kode-services/src/openai/service.rs` - OpenAIService core implementation
- `crates/kode-services/src/openai/types.rs` - OpenAI API type definitions
- `crates/kode-services/src/openai/error.rs` - OpenAI-specific error types
- `crates/kode-services/src/openai/streaming.rs` - Streaming response handling
- `crates/kode-services/src/openai/adapter.rs` - Model parameter adaptation logic
- `crates/kode-services/src/openai/tests.rs` - Integration tests

#### 修改文件 / Modified Files
- `crates/kode-services/src/lib.rs` - 添加 `pub mod openai;`
- `crates/kode-services/Cargo.toml` - 无需新增依赖（复用现有依赖）

- `crates/kode-services/src/lib.rs` - Add `pub mod openai;`
- `crates/kode-services/Cargo.toml` - No new dependencies needed (reuse existing dependencies)

### 模块结构 / Module Structure

```
crates/kode-services/src/openai/
├── mod.rs           # 模块导出 / Module exports
├── service.rs       # OpenAIService 核心实现 / OpenAIService core implementation
├── types.rs         # OpenAI API 类型定义 / OpenAI API type definitions
├── error.rs         # OpenAI 特定错误 / OpenAI-specific errors
├── streaming.rs     # SSE 流处理 / SSE stream processing
├── adapter.rs       # 参数适配逻辑 / Parameter adaptation logic
└── tests.rs         # 集成测试 / Integration tests
```

## Impact

### 影响的 Specs / Affected Specs

**新增的规范：**
- `openai-service` - OpenAI-compatible 服务规范（本次变更的核心）

**New specs:**
- `openai-service` - OpenAI-compatible service spec (core of this change)

**说明：**
OpenAI 服务作为独立的 spec，与 `anthropic-service` 平级，都实现了 `ModelAdapter` trait。
参考已归档的 `2025-12-28-implement-anthropic-service` 变更的结构模式。

**Notes:**
OpenAI service is an independent spec, parallel to `anthropic-service`, both implementing the `ModelAdapter` trait.
Reference the structure pattern from the archived `2025-12-28-implement-anthropic-service` change.

### 影响的代码 / Affected Code

**新增代码：**
- `crates/kode-services/src/openai/` - 整个 OpenAI 服务模块（约 1500-2000 行代码）

**New code:**
- `crates/kode-services/src/openai/` - Entire OpenAI service module (~1500-2000 lines of code)

**修改代码：**
- `crates/kode-services/src/lib.rs` - 添加 OpenAI 模块导出（约 5 行）

**Modified code:**
- `crates/kode-services/src/lib.rs` - Add OpenAI module export (~5 lines)

**不影响代码：**
- 现有的 `AnthropicService` 实现保持不变
- `ModelAdapter` trait 定义保持不变

**Unaffected code:**
- Existing `AnthropicService` implementation remains unchanged
- `ModelAdapter` trait definition remains unchanged

### 兼容性影响 / Compatibility Impact

**配置兼容性：**
- ✅ 100% 兼容原版 TypeScript 的配置格式
- ✅ 支持原版的所有配置参数（provider、baseURL、apiKey 等）

**Configuration compatibility:**
- ✅ 100% compatible with original TypeScript configuration format
- ✅ Supports all configuration parameters from the original version (provider, baseURL, apiKey, etc.)

**行为兼容性：**
- ✅ API 调用行为与原版一致
- ✅ 错误处理和重试策略与原版一致
- ✅ 流式响应格式与原版一致

**Behavioral compatibility:**
- ✅ API call behavior consistent with original version
- ✅ Error handling and retry strategy consistent with original version
- ✅ Streaming response format consistent with original version

### 性能影响 / Performance Impact

**预期性能：**
- 非流式请求：与 AnthropicService 相当
- 流式请求：首字节延迟 < 500ms（与原版一致）
- 内存占用：与 AnthropicService 相当（~10MB 基准）

**Expected performance:**
- Non-streaming requests: Comparable to AnthropicService
- Streaming requests: Time to first byte < 500ms (consistent with original)
- Memory usage: Comparable to AnthropicService (~10MB baseline)

### 安全影响 / Security Impact

**安全考虑：**
- ✅ API 密钥安全存储（不打印到日志）
- ✅ 支持 HTTPS 端点验证
- ✅ 代理支持（通过配置）
- ✅ 请求超时保护

**Security considerations:**
- ✅ API key secure storage (not printed to logs)
- ✅ HTTPS endpoint validation support
- ✅ Proxy support (via configuration)
- ✅ Request timeout protection

### 依赖影响 / Dependency Impact

**无需新增依赖：**
- ✅ 复用 `reqwest`（HTTP 客户端）
- ✅ 复用 `tokio`（异步运行时）
- ✅ 复用 `serde`（序列化）
- ✅ 复用 `serde_json`（JSON 处理）
- ✅ 复用 `anyhow`（错误处理）
- ✅ 复用 `tracing`（日志）

**No new dependencies needed:**
- ✅ Reuse `reqwest` (HTTP client)
- ✅ Reuse `tokio` (async runtime)
- ✅ Reuse `serde` (serialization)
- ✅ Reuse `serde_json` (JSON handling)
- ✅ Reuse `anyhow` (error handling)
- ✅ Reuse `tracing` (logging)

## Migration Strategy

### 实施步骤 / Implementation Steps

详见 `tasks.md` 文件，包含详细的任务分解和验证步骤。

See `tasks.md` file for detailed task breakdown and validation steps.

### 回滚计划 / Rollback Plan

如果实施出现问题：
- OpenAI 模块是独立的，不影响现有 AnthropicService
- 可以通过移除 `pub mod openai;` 快速回滚
- 配置文件中的 OpenAI 配置会被忽略（不导致启动失败）

If implementation issues arise:
- The OpenAI module is independent and does not affect the existing AnthropicService
- Can quickly rollback by removing `pub mod openai;`
- OpenAI configuration in config files will be ignored (won't cause startup failure)

## Testing Strategy

### 测试策略 / Testing Strategy

详见 `design.md` 文档，包含：
- 单元测试策略
- 集成测试策略
- 兼容性测试策略
- 性能测试策略

See `design.md` document for:
- Unit testing strategy
- Integration testing strategy
- Compatibility testing strategy
- Performance testing strategy
