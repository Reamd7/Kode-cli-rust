# Change: 实现模型切换 / Implement Model Switching

## Why

运行时模型切换允许用户在不同场景使用不同的模型，提高灵活性。

Runtime model switching allows users to use different models in different scenarios, increasing flexibility.

## What Changes

- 实现运行时模型切换
- 实现模型指针（default, task, architect, etc.）
- UI 显示当前模型
- 实现 `/model switch <name>` 命令
- 实现模型配置验证

- Implement runtime model switching
- Implement model pointers (default, task, architect, etc.)
- UI shows current model
- Implement `/model switch <name>` command
- Implement model config validation

## Impact

**Affected specs:**
- message-model (MODIFIED)
- cli-commands (MODIFIED)
- tui-interface (MODIFIED)

**Affected code:**
- `crates/kode-core/src/model/manager.rs` (修改)
- `crates/kode-cli/src/commands/model.rs` (新建)
- `crates/kode-ui/src/widgets/status.rs` (修改)
