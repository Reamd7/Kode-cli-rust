# Change: 实现 Anthropic API 服务 / Implement Anthropic API Service

## Why

Anthropic Claude API 是 Kode-Rust 的主要 AI 服务提供商，需要实现完整的 API 客户端。

Anthropic Claude API is the primary AI service provider for Kode-Rust, requiring a complete API client implementation.

## What Changes

- 实现 `AnthropicService` 结构体
- 实现 `ModelAdapter for AnthropicService`
- 实现非流式 send_message 方法
- 实现流式 stream_message 方法（SSE）
- 实现 API 错误处理
- 添加集成测试

- Implement `AnthropicService` struct
- Implement `ModelAdapter for AnthropicService`
- Implement non-streaming send_message method
- Implement streaming stream_message method (SSE)
- Implement API error handling
- Add integration tests

## Impact

**Affected specs:**
- anthropic-service

**Affected code:**
- `crates/kode-services/src/anthropic.rs` (新建)
- `crates/kode-services/src/anthropic/types.rs` (新建)
- `crates/kode-services/src/anthropic/error.rs` (新建)
- `crates/kode-services/Cargo.toml` (添加 reqwest 依赖)
