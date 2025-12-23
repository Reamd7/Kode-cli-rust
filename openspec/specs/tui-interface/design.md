# 设计文档 - TUI 界面 / Design Document - TUI Interface

## Context

TUI 界面是 Kode-Rust 的主要用户交互方式，提供类似 Claude Code 的终端体验。

The TUI interface is the primary user interaction method for Kode-Rust, providing a terminal experience similar to Claude Code.

### 当前状态 / Current State

TUI 界面计划在 `crates/kode-ui/` 中实现，尚未开始实现。

The TUI interface is planned to be implemented in `crates/kode-ui/`, implementation has not started yet.

## Goals / Non-Goals

### Goals

- [ ] 实现 REPL 界面 / Implement REPL interface
- [ ] 实现流式输出显示 / Implement streaming output display
- [ ] 实现权限请求对话框 / Implement permission request dialog
- [ ] 实现状态栏 / Implement status bar
- [ ] 优化渲染性能 / Optimize rendering performance

### Non-Goals

- [ ] 语法高亮 / Syntax highlighting
- [ ] Markdown 格式化 / Markdown formatting
- [ ] 代码块渲染 / Code block rendering

## Decisions

### 决策 1: TUI 框架选择 / Decision 1: TUI Framework Selection

**选择**: 使用 Ratatui

**Choice**: Use Ratatui

**理由 / Rationale**:
- 项目已依赖 Ratatui / Project already depends on Ratatui
- 现代化、活跃维护 / Modern and actively maintained
- 丰富的 Widget 生态 / Rich Widget ecosystem

### 决策 2: 应用状态管理 / Decision 2: Application State Management

**实现 / Implementation**:
```rust
/// 来自 ARCHITECTURE.md L319-342
pub struct App {
    pub state: AppState,
    pub repl: ReplState,
    pub model_manager: Arc<ModelManager>,
    pub tool_registry: Arc<ToolRegistry>,
    pub config: Config,
}

pub enum AppState {
    Repl,
    ToolExecution,
    PermissionRequest,
    AgentWorking,
}

pub struct ReplState {
    pub messages: Vec<DisplayMessage>,
    pub input: String,
    pub scroll_offset: usize,
    pub waiting_for_response: bool,
}
```

#### REPL 界面组件
来自 ARCHITECTURE.md L344-365:
```rust
pub struct ReplWidget<'a> {
    messages: &'a [DisplayMessage],
    input: &'a str,
    scroll: usize,
}

impl Widget for ReplWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // 实现消息列表 + 输入框的渲染
    }
}
```

#### 组件系统
来自 ARCHITECTURE.md L360-365:
- `MessageList`: 消息滚动列表
- `InputBox`: 输入框
- `StatusBar`: 状态栏
- `ToolOutput`: 工具输出显示
- `PermissionDialog`: 权限请求对话框
- `ThinkingIndicator`: 思考中动画

/// 状态栏状态
pub struct StatusBarState {
    pub current_model: String,
    pub tokens_used: usize,
    pub connection_status: ConnectionStatus,
}

pub enum ConnectionStatus {
    Connected,
    Connecting,
    Disconnected,
    Error(String),
}
```

### 决策 3: 主循环设计 / Decision 3: Main Loop Design

**设计 / Design**:
```rust
use crossterm::event::{self, Event, KeyCode, KeyEvent};

pub async fn run_ui(mut app: App, model_manager: Arc<ModelManager>) -> Result<()> {
    let mut terminal = setup_terminal()?;
    let mut last_render = Instant::now();

    loop {
        // 处理事件
        if event::poll(Duration::from_millis(16))? {  // ~60 FPS
            if let Event::Key(key) = event::read()? {
                handle_key_event(key, &mut app).await?;
            }
        }

        // 检查是否应该退出
        if app.should_quit {
            break;
        }

        // 限制渲染帧率（30 FPS）
        if last_render.elapsed() > Duration::from_millis(33) {
            terminal.draw(|f| {
                render_app(f, &mut app);
            })?;
            last_render = Instant::now();
        }
    }

    restore_terminal(terminal)?;
    Ok(())
}

fn render_app<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();

    // 分割布局
    let chunks = Layout::default()
        .direction(Vertical)
        .constraints([
            Constraint::Min(0),  // 主内容区
            Constraint::Length(1),  // 输入框
            Constraint::Length(1),  // 状态栏
        ])
        .split(size);

    // 渲染主内容
    render_main_content(f, app, chunks[0]);

    // 渲染输入框
    render_input_box(f, app, chunks[1]);

    // 渲染状态栏
    render_status_bar(f, app, chunks[2]);
}

fn render_main_content<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    // 如果显示权限对话框，渲染对话框
    if let Some(ref dialog) = app.permission_dialog {
        render_permission_dialog(f, app, area);
        return;
    }

    // 渲染消息列表
    let messages: Vec<ListItem> = app.repl.messages
        .iter()
        .skip(app.repl.scroll_offset)
        .map(|msg| {
            let content = format!("{}: {}", msg.role, msg.content);
            ListItem::new(content)
        })
        .collect();

    let list = List::new(messages)
        .block(Block::default().borders(Borders::ALL).title("Chat"));
    f.render_widget(list, area);
}
```

### 决策 4: 流式处理 / Decision 4: Streaming Handling

**设计 / Design**:
```rust
pub async fn handle_stream_chunk(app: &mut App, chunk: StreamChunk) {
    match chunk {
        StreamChunk::ContentBlockDelta { delta } => {
            app.current_response.push_str(&delta);
            app.waiting_for_response = true;
        }
        StreamChunk::ToolUse { tool_name, params } => {
            // 显示工具调用
            app.repl.messages.push(DisplayMessage {
                role: MessageRole::System,
                content: format!("Calling tool: {} with params: {}", tool_name, params),
                timestamp: Utc::now(),
            });
        }
        StreamChunk::MessageStop => {
            // 完成响应
            app.repl.messages.push(DisplayMessage {
                role: MessageRole::Assistant,
                content: app.current_response.clone(),
                timestamp: Utc::now(),
            });
            app.current_response.clear();
            app.waiting_for_response = false;
        }
        _ => {}
    }
}
```

### 决策 5: 权限对话框渲染 / Decision 5: Permission Dialog Rendering

**设计 / Design**:
```rust
fn render_permission_dialog<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let dialog = app.permission_dialog.as_ref().unwrap();

    let popup = Rect::new(
        area.width / 4,
        area.height / 4,
        area.width / 2,
        area.height / 2,
    );

    let content = vec![
        Line::from("Tool Execution Permission").style(Style::default().add_modifier(Modifier::BOLD)),
        Line::from(""),
        Line::from(format!("Tool: {}", dialog.tool_name)),
        Line::from(""),
        Line::from("Parameters:"),
        Line::from(format!("{}", serde_json::to_string_pretty(&dialog.params).unwrap())),
        Line::from(""),
        Line::from("Execute? (y/n)"),
    ];

    let paragraph = Paragraph::new(content)
        .block(Block::default()
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::DarkGray)))
        .wrap(Wrap { trim: true });

    f.render_widget(Clear, popup);  // 清除背景
    f.render_widget(paragraph, popup);
}
```

### 决策 6: 状态栏渲染 / Decision 6: Status Bar Rendering

**设计 / Design**:
```rust
fn render_status_bar<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let status = &app.status_bar;

    let connection_symbol = match status.connection_status {
        ConnectionStatus::Connected => "●",
        ConnectionStatus::Connecting => "○",
        ConnectionStatus::Disconnected => "◌",
        ConnectionStatus::Error(_) => "!",
    };

    let text = format!(
        "Model: {} | Tokens: {} | {}",
        status.current_model,
        status.tokens_used,
        connection_symbol
    );

    let bar = Paragraph::new(text)
        .style(Style::default().bg(Color::DarkBlue).fg(Color::White));

    f.render_widget(bar, area);
}
```

### 决策 7: 输入处理 / Decision 7: Input Handling

**设计 / Design**:
```rust
async fn handle_key_event(key: KeyEvent, app: &mut App) -> Result<()> {
    // 如果显示权限对话框，处理 y/n 响应
    if let Some(ref mut dialog) = app.permission_dialog {
        match key.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                dialog.approved = Some(true);
                app.permission_dialog = None;
            }
            KeyCode::Char('n') | KeyCode::Char('N') => {
                dialog.approved = Some(false);
                app.permission_dialog = None;
            }
            KeyCode::Esc => {
                app.permission_dialog = None;
            }
            _ => {}
        }
        return Ok(());
    }

    // 处理正常输入
    match key.code {
        KeyCode::Char(c) => {
            app.repl.input.insert(app.repl.cursor_position, c);
            app.repl.cursor_position += 1;
        }
        KeyCode::Backspace => {
            if app.repl.cursor_position > 0 {
                app.repl.input.remove(app.repl.cursor_position - 1);
                app.repl.cursor_position -= 1;
            }
        }
        KeyCode::Enter => {
            if !app.repl.input.trim().is_empty() {
                submit_message(app).await?;
            }
        }
        KeyCode::Ctrl('c') => {
            app.should_quit = true;
        }
        KeyCode::Up => {
            // 滚动历史
            if app.repl.scroll_offset > 0 {
                app.repl.scroll_offset -= 1;
            }
        }
        KeyCode::Down => {
            // 滚动历史
            app.repl.scroll_offset += 1;
        }
        _ => {}
    }

    Ok(())
}
```

## 布局设计 / Layout Design

```
┌─────────────────────────────────────────────────┐
│                                                 │
│              Main Content Area                  │
│            (Messages + Streaming)               │
│                                                 │
│                                                 │
├─────────────────────────────────────────────────┤
│ > User input here...                            │  Input Box
├─────────────────────────────────────────────────┤
│ Model: claude-sonnet | Tokens: 1234 | ●         │  Status Bar
└─────────────────────────────────────────────────┘
```

## 权限对话框布局 / Permission Dialog Layout

```
┌─────────────────────────────────────────────────┐
│         Tool Execution Permission               │
├─────────────────────────────────────────────────┤
│ Tool: write_file                                │
│                                                 │
│ Parameters:                                     │
│ {                                               │
│   "path": "example.txt",                        │
│   "content": "..."                              │
│ }                                               │
│                                                 │
│ Execute? (y/n)                                  │
└─────────────────────────────────────────────────┘
```

## 错误处理 / Error Handling

| 错误类型 / Error Type | 处理方式 / Handling |
|---------------------|-------------------|
| 终端大小变化 / Terminal resize | 自动重新布局 / Auto re-layout |
| 渲染错误 / Render error | 记录日志并继续 / Log and continue |
| 输入错误 / Input error | 忽略无效输入 / Ignore invalid input |

## 测试策略 / Testing Strategy

### 单元测试 / Unit Tests

```rust
#[test]
fn test_scroll_offset_bounds() {
    let mut repl = ReplState::new();
    repl.messages = create_test_messages(10);
    repl.scroll_offset = 5;

    // 验证滚动不会超出边界
    assert!(repl.scroll_offset < repl.messages.len());
}
```

## Risks / Trade-offs

### 风险 1: 终端兼容性 / Risk 1: Terminal Compatibility

**风险 / Risk**: 不同终端可能有不同的行为

**缓解措施 / Mitigation**:
- 使用 Crossterm 处理平台差异 / Use Crossterm for platform differences
- 测试常见终端 / Test common terminals

### 风险 2: 渲染性能 / Risk 2: Rendering Performance

**风险 / Risk**: 大量消息可能导致性能下降

**缓解措施 / Mitigation**:
- 限制显示的消息数量 / Limit displayed message count
- 使用虚拟滚动 / Use virtual scrolling
- 帧率限制 / Frame rate limiting

## Open Questions

1. **是否需要支持多窗口？**
   - 建议：暂不实现，保持简单
   - Rationale: Don't implement for now, keep it simple

2. **是否需要支持主题切换？**
   - 建议：支持，配置文件中定义
   - Rationale: Yes, define in config file

3. **是否需要支持鼠标交互？**
   - 建议：可选功能，未来添加
   - Rationale: Optional feature, add in the future

## 参考资源 / References

- [Ratatui 文档](https://ratatui.rs/)
- [Crossterm 文档](https://docs.rs/crossterm/)
- [Ratatui Widgets 示例](https://github.com/ratatui-org/ratatui/tree/master/examples)
