# Change: 实现上下文管理 / Implement Context Management

## Why

智能上下文窗口管理可以优化 token 使用，在保持上下文完整性的同时控制成本。

Intelligent context window management optimizes token usage while maintaining context integrity and controlling costs.

## What Changes

- 实现 `MessageContextManager`
- 实现 token 计数器（字符估算 → tokenizer）
- 实现智能裁剪策略
- 实现上下文优先级
- 单元测试

- Implement `MessageContextManager`
- Implement token counter (character estimation → tokenizer)
- Implement smart trimming strategy
- Implement context priority
- Unit tests

## Impact

**Affected specs:**
- message-model (MODIFIED)

**Affected code:**
- `crates/kode-core/src/context.rs` (新建)
