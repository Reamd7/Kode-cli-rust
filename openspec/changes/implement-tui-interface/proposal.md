# Change: 实现 TUI 界面 / Implement TUI Interface

## Why

TUI（Terminal User Interface）提供交互式命令行体验，是 Kode 的主要用户界面。

TUI (Terminal User Interface) provides interactive command-line experience, the primary user interface of Kode.

## What Changes

- 设置 ratatui 框架
- 实现 `App` 状态管理
- 实现 `AppState` 枚举（Repl, ToolExecution, etc.）
- 实现 `ReplState`（消息列表、输入框、滚动偏移）
- 实现 `ReplWidget`（消息列表 Widget、输入框 Widget）
- 实现基础事件循环
- 实现键盘输入处理

- Setup ratatui framework
- Implement `App` state management
- Implement `AppState` enum (Repl, ToolExecution, etc.)
- Implement `ReplState` (message list, input box, scroll offset)
- Implement `ReplWidget` (message list widget, input widget)
- Implement basic event loop
- Implement keyboard input handling

## Impact

**Affected specs:**
- tui-interface

**Affected code:**
- `crates/kode-ui/src/app.rs` (新建)
- `crates/kode-ui/src/state.rs` (新建)
- `crates/kode-ui/src/widgets/repl.rs` (新建)
- `crates/kode-ui/Cargo.toml` (添加 ratatui, crossterm 依赖)
