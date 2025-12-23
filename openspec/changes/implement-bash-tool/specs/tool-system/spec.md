# Delta Spec: Bash Tool

## ADDED Requirements

### Requirement: BashTool / BashTool
The system SHALL implement bash command execution tool.

系统应实现 bash 命令执行工具。

#### Scenario: 命令执行 / Command Execution
- **WHEN** 执行 BashTool 时
- **THEN** 接受 command 参数
- **AND** 使用 tokio::process::Command 执行
- **AND** 捕获 stdout 和 stderr
- **AND** 支持超时控制
- **AND** 返回命令输出

- **WHEN** executing BashTool
- **THEN** accepts command parameter
- **AND** uses tokio::process::Command to execute
- **AND** captures stdout and stderr
- **AND** supports timeout control
- **AND** returns command output

#### Scenario: 危险命令权限检查 / Dangerous Command Permission Check
- **WHEN** 执行危险命令时（如 rm -rf）
- **THEN** 请求用户确认
- **AND** 在 TUI 中显示权限提示

- **WHEN** executing dangerous commands (e.g., rm -rf)
- **THEN** requests user confirmation
- **AND** shows permission prompt in TUI
