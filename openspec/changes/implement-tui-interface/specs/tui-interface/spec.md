# Delta Spec: TUI Interface

## ADDED Requirements

### Requirement: App 结构 / App Structure
The system SHALL implement main application structure for TUI.

系统应实现 TUI 的主应用结构。

#### Scenario: App 状态管理 / App State Management
- **WHEN** 定义 App 结构体时
- **THEN** 包含 state 字段 (AppState)
- **AND** 包含 repl 字段 (ReplState)
- **AND** 包含 model_manager 字段
- **AND** 包含 tool_registry 字段
- **AND** 包含 config 字段

- **WHEN** defining App struct
- **THEN** includes state field (AppState)
- **AND** includes repl field (ReplState)
- **AND** includes model_manager field
- **AND** includes tool_registry field
- **AND** includes config field

### Requirement: AppState / AppState
The system SHALL define application states.

系统应定义应用状态。

#### Scenario: 状态枚举 / State Enum
- **WHEN** 定义 AppState 时
- **THEN** 包含 Repl 状态
- **AND** 包含 ToolExecution 状态
- **AND** 包含 PermissionRequest 状态
- **AND** 包含 AgentWorking 状态

- **WHEN** defining AppState
- **THEN** includes Repl state
- **AND** includes ToolExecution state
- **AND** includes PermissionRequest state
- **AND** includes AgentWorking state

### Requirement: ReplState / ReplState
The system SHALL define REPL state.

系统应定义 REPL 状态。

#### Scenario: REPL 状态 / REPL State
- **WHEN** 定义 ReplState 时
- **THEN** 包含 messages 字段 (Vec<DisplayMessage>)
- **AND** 包含 input 字段 (String)
- **AND** 包含 scroll_offset 字段
- **AND** 包含 waiting_for_response 字段

- **WHEN** defining ReplState
- **THEN** includes messages field (Vec<DisplayMessage>)
- **AND** includes input field (String)
- **AND** includes scroll_offset field
- **AND** includes waiting_for_response field

### Requirement: 事件循环 / Event Loop
The system SHALL implement main event loop.

系统应实现主事件循环。

#### Scenario: 键盘输入处理 / Keyboard Input Handling
- **WHEN** 用户按下按键时
- **THEN** 根据当前状态处理输入
- **AND** 更新 UI
- **AND** 重新渲染界面

- **WHEN** user presses a key
- **THEN** processes input based on current state
- **AND** updates UI
- **AND** re-renders interface
