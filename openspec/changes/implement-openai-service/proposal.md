# Change: 实现 OpenAI-compatible 服务 / Implement OpenAI-compatible Service

## Why

支持 OpenAI-compatible API 允许用户使用多种模型提供商（OpenAI, DeepSeek, 等），提高灵活性。

Supporting OpenAI-compatible API allows users to use multiple model providers (OpenAI, DeepSeek, etc.), increasing flexibility.

## What Changes

- 实现 `OpenAIService` 结构体
- 实现 `ModelAdapter for OpenAIService`
- 支持自定义 base URL
- 支持流式响应
- 测试 OpenAI, DeepSeek 等提供商

- Implement `OpenAIService` struct
- Implement `ModelAdapter for OpenAIService`
- Support custom base URL
- Support streaming response
- Test OpenAI, DeepSeek and other providers

## Impact

**Affected specs:**
- anthropic-service (RENAMED → model-service)

**Affected code:**
- `crates/kode-services/src/openai.rs` (新建)
- `crates/kode-services/src/anthropic.rs` (重命名/重构)
