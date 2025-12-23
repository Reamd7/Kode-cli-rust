# TUI 界面规范 / TUI Interface Specification

## Purpose

TUI (Terminal User Interface) 提供交互式终端界面，允许用户与 AI Agent 进行实时对话，查看流式响应，并管理工具执行。

The TUI (Terminal User Interface) provides an interactive terminal interface, allowing users to have real-time conversations with AI agents, view streaming responses, and manage tool execution.

## Requirements

### Requirement: REPL 界面 / REPL Interface
The system SHALL provide a Read-Eval-Print Loop interface for interactive conversations.

系统应提供 Read-Eval-Print Loop 界面用于交互式对话。

#### Scenario: 显示消息历史 / Display Message History
- **WHEN** 用户启动 REPL 时
- **THEN** 显示对话历史消息
- **AND** 区分用户消息和 AI 响应

- **WHEN** the user starts the REPL
- **THEN** display conversation history messages
- **AND** distinguish between user messages and AI responses

#### Scenario: 处理用户输入 / Handle User Input
- **WHEN** 用户输入消息时
- **THEN** 实时显示输入内容
- **AND** 支持多行输入
- **AND** 支持 readline 功能（历史、编辑）

- **WHEN** the user enters a message
- **THEN** display the input in real-time
- **AND** support multi-line input
- **AND** support readline features (history, editing)

#### Scenario: 滚动支持 / Scrolling Support
- **WHEN** 消息历史超过屏幕高度时
- **THEN** 允许用户滚动查看
- **AND** 自动滚动到最新消息

- **WHEN** message history exceeds screen height
- **THEN** allow the user to scroll to view
- **AND** automatically scroll to the latest message

### Requirement: 流式输出 / Streaming Output
The system SHALL display AI responses in real-time as they are streamed.

系统应实时显示 AI 的流式响应。

#### Scenario: 实时文本显示 / Real-time Text Display
- **WHEN** AI 返回流式响应时
- **THEN** 逐步显示文本内容
- **AND** 实现打字机效果

- **WHEN** the AI returns a streaming response
- **THEN** display text content progressively
- **AND** implement typewriter effect

#### Scenario: 工具调用显示 / Tool Call Display
- **WHEN** AI 调用工具时
- **THEN** 显示工具名称和参数
- **AND** 显示工具执行状态

- **WHEN** the AI calls a tool
- **THEN** display the tool name and parameters
- **AND** display the tool execution status

#### Scenario: 思考指示器 / Thinking Indicator
- **WHEN** AI 处理请求但尚未返回内容时
- **THEN** 显示思考中动画
- **AND** 流式开始后移除动画

- **WHEN** the AI is processing but hasn't returned content yet
- **THEN** display a thinking animation
- **AND** remove the animation when streaming starts

### Requirement: 权限请求 / Permission Requests
The system SHALL display permission dialogs for tool executions requiring user approval.

系统应显示权限对话框以请求用户批准需要授权的工具执行。

#### Scenario: 显示权限对话框 / Show Permission Dialog
- **WHEN** 工具执行需要用户批准时
- **THEN** 暂停正常渲染
- **AND** 显示权限请求对话框
- **AND** 显示工具详情（名称、参数）

- **WHEN** a tool execution requires user approval
- **THEN** pause normal rendering
- **AND** display a permission request dialog
- **AND** display tool details (name, parameters)

#### Scenario: 处理用户响应 / Handle User Response
- **WHEN** 用户响应权限请求时
- **THEN** y/yes 执行工具
- **AND** n/no 取消执行
- **AND** 恢复正常渲染

- **WHEN** the user responds to a permission request
- **THEN** execute the tool on y/yes
- **AND** cancel execution on n/no
- **AND** resume normal rendering

### Requirement: 状态栏 / Status Bar
The system SHALL display a status bar with current application state.

系统应显示包含当前应用状态的状态栏。

#### Scenario: 显示当前信息 / Show Current Information
- **WHEN** REPL 运行时
- **THEN** 状态栏显示当前模型名称
- **AND** 显示 token 使用量
- **AND** 显示连接状态

- **WHEN** the REPL is running
- **THEN** the status bar displays the current model name
- **AND** display token usage
- **AND** display connection status

#### Scenario: 更新状态 / Update Status
- **WHEN** 状态变化时（如模型切换）
- **THEN** 实时更新状态栏内容

- **WHEN** the status changes (such as model switch)
- **THEN** update the status bar content in real-time

### Requirement: 渲染性能 / Rendering Performance
The system SHALL maintain smooth rendering during streaming and updates.

系统应在流式和更新期间保持流畅的渲染。

#### Scenario: 帧率控制 / Frame Rate Control
- **WHEN** 流式内容快速到达时
- **THEN** 限制渲染帧率（如 30 FPS）
- **AND** 避免过度渲染

- **WHEN** streaming content arrives quickly
- **THEN** limit the rendering frame rate (e.g., 30 FPS)
- **AND** avoid over-rendering

#### Scenario: 部分重绘 / Partial Redraw
- **WHEN** 仅部分内容变化时
- **THEN** 仅重绘变化区域
- **AND** 减少渲染开销

- **WHEN** only part of the content changes
- **THEN** only redraw the changed area
- **AND** reduce rendering overhead

## Non-Goals

- 本规范不包含语法高亮（在高级特性中）
- 不包含代码块渲染（在高级特性中）
- 不包含 Markdown 格式化（在高级特性中）

- This specification does not include syntax highlighting (in advanced features)
- Does not include code block rendering (in advanced features)
- Does not include Markdown formatting (in advanced features)
