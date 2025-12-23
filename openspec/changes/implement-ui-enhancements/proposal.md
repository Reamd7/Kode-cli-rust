# Change: 实现 UI 美化 / Implement UI Enhancements

## Why

美观的 UI 提升用户体验，使 Kode 更易用和吸引人。

Beautiful UI improves user experience, making Kode more usable and attractive.

## What Changes

- 实现语法高亮（使用 syntect）
  - 支持常见语言
  - 支持主题
- 实现代码块渲染
- 实现 Markdown 格式化
- 实现状态栏
  - 显示当前模型
  - 显示 token 使用量
  - 显示连接状态
- 实现加载动画
- 错误显示优化

- Implement syntax highlighting (using syntect)
  - Support common languages
  - Support themes
- Implement code block rendering
- Implement Markdown formatting
- Implement status bar
  - Show current model
  - Show token usage
  - Show connection status
- Implement loading animation
- Error display optimization

## Impact

**Affected specs:**
- tui-interface (MODIFIED)

**Affected code:**
- `crates/kode-ui/src/widgets/syntax.rs` (新建)
- `crates/kode-ui/src/widgets/status_bar.rs` (新建)
- `crates/kode-ui/src/widgets/loading.rs` (新建)
- `crates/kode-ui/Cargo.toml` (添加 syntect 依赖)
