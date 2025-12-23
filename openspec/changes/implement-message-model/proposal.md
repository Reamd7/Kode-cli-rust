# Change: 实现消息模型系统 / Implement Message Model System

## Why

消息模型是 Kode-Rust 的核心抽象，定义 AI 对话的数据结构和模型适配器接口。

The message model is a core abstraction of Kode-Rust, defining data structures for AI conversations and model adapter interfaces.

## What Changes

- 实现 `Message` 结构体（Role, Content）
- 实现 `ModelAdapter` trait
- 实现 `StreamChunk` 枚举（流式响应）
- 实现 `StreamingResponse` 包装器
- 添加错误类型定义

- Implement `Message` struct (Role, Content)
- Implement `ModelAdapter` trait
- Implement `StreamChunk` enum (streaming response)
- Implement `StreamingResponse` wrapper
- Add error type definitions

## Impact

**Affected specs:**
- message-model

**Affected code:**
- `crates/kode-core/src/message.rs` (新建)
- `crates/kode-core/src/model/adapter.rs` (新建)
- `crates/kode-core/src/model/types.rs` (新建)
- `crates/kode-core/src/error.rs` (新建)
