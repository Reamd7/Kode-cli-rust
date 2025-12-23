# TUI Interface Specification

TUI 界面规范 - 定义终端用户界面的实现。

## Purpose

TUI 界面提供交互式的终端体验。使用 ratatui 框架实现 REPL (Read-Eval-Print Loop) 界面，支持消息列表显示、流式输出、权限对话框和状态栏。

## ADDED Requirements

### Requirement: 应用状态管理
The system SHALL maintain application state for TUI.

系统必须维护 TUI 的应用状态。

#### Scenario: App 结构
- **WHEN** 初始化 TUI 应用
- **THEN** 创建 App 结构包含所有状态
- **AND** 包含消息历史、输入状态、UI 模式

#### Scenario: 状态更新
- **WHEN** 应用状态变化
- **THEN** 通过状态通道更新 UI
- **AND** 触发重绘

#### Scenario: 线程安全
- **WHEN** 多个线程访问状态
- **THEN** 使用 Arc 和 Mutex 保护
- **AND** 避免竞争条件

### Requirement: REPL 界面
The system SHALL provide a REPL interface for user interaction.

系统必须提供 REPL 界面用于用户交互。

#### Scenario: 消息列表显示
- **WHEN** 显示对话历史
- **THEN** 消息按时间顺序显示
- **AND** 用户消息右对齐
- **AND** 助手消息左对齐
- **AND** 工具调用显示为代码块

#### Scenario: 输入框
- **WHEN** 用户输入消息
- **THEN** 输入框显示在底部
- **AND** 支持多行输入
- **AND** 支持行编辑

#### Scenario: 滚动支持
- **WHEN** 消息超出显示区域
- **THEN** 用户可以滚动查看历史
- **AND** 新消息自动滚动到底部

### Requirement: 流式输出显示
The system SHALL display streaming AI responses in real-time.

系统必须实时显示流式 AI 响应。

#### Scenario: 逐字符显示
- **WHEN** 接收流式响应
- **THEN** 逐字符显示在界面上
- **AND** 创建打字机效果

#### Scenario: 流式更新
- **WHEN** 新内容到达
- **THEN** 更新当前消息
- **AND** 避免过度渲染

#### Scenario: 流式完成
- **WHEN** 流式响应结束
- **THEN** 固化消息内容
- **AND** 添加到消息历史

### Requirement: 权限对话框
The system SHALL show permission prompts for dangerous operations.

系统必须为危险操作显示权限提示。

#### Scenario: 显示权限请求
- **WHEN** 工具需要权限
- **THEN** 弹出权限对话框
- **AND** 显示工具名称和参数
- **AND** 提供允许/拒绝选项

#### Scenario: 记住选择
- **WHEN** 用户选择记住
- **THEN** 缓存权限决定
- **AND** 后续相同操作自动批准

#### Scenario: 会话范围
- **WHEN** 权限决定被记住
- **THEN** 仅在当前会话有效
- **AND** 应用关闭后清除

### Requirement: 状态栏
The system SHALL display status information.

系统必须显示状态信息。

#### Scenario: 显示当前模型
- **WHEN** TUI 运行中
- **THEN** 状态栏显示当前使用的模型
- **AND** 显示模型提供商

#### Scenario: 显示 token 使用
- **WHEN** 有 API 响应
- **THEN** 状态栏显示 token 使用量
- **AND** 包含输入和输出 token

#### Scenario: 显示连接状态
- **WHEN** 连接到服务器
- **THEN** 状态栏显示连接状态
- **AND** 断开时显示错误

### Requirement: Markdown 渲染
The system SHALL render Markdown content in TUI.

系统必须在 TUI 中渲染 Markdown 内容。

#### Scenario: 代码块渲染
- **WHEN** 响应包含代码块
- **THEN** 使用语法高亮
- **AND** 显示语言标识

#### Scenario: 格式化文本
- **WHEN** 响应包含 Markdown 格式
- **THEN** 渲染粗体和斜体
- **AND** 渲染列表和引用

#### Scenario: 长内容处理
- **WHEN** 内容超出显示区域
- **THEN** 支持滚动查看
- **AND** 折叠长代码块

### Requirement: 键盘处理
The system SHALL handle keyboard input.

系统必须处理键盘输入。

#### Scenario: 消息编辑
- **WHEN** 用户编辑输入
- **THEN** 支持方向键移动光标
- **AND** 支持删除字符
- **AND** 支持行编辑快捷键

#### Scenario: 发送消息
- **WHEN** 用户按 Enter
- **THEN** 发送单行消息
- **AND** Ctrl+Enter 或 Alt+Enter 发送多行

#### Scenario: 退出应用
- **WHEN** 用户按 Ctrl+C 或 Ctrl+D
- **THEN** 优雅退出应用
- **AND** 清理终端状态

#### Scenario: 命令模式
- **WHEN** 用户输入 /
- **THEN** 进入命令模式
- **AND** 支持斜杠命令

### Requirement: 终端控制
The system SHALL manage terminal state.

系统必须管理终端状态。

#### Scenario: 进入原始模式
- **WHEN** TUI 启动
- **THEN** 切换终端到原始模式
- **AND** 启用鼠标支持

#### Scenario: 退出原始模式
- **WHEN** TUI 关闭
- **THEN** 恢复终端正常模式
- **AND** 清屏

#### Scenario: 终端大小变化
- **WHEN** 终端窗口大小变化
- **THEN** 自动调整布局
- **AND** 重绘界面

## Non-Goals

本规范不包含：
- 图形用户界面（GUI）
- Web 界面
- 自定义主题系统（未来扩展）
- 多窗口布局（未来扩展）
