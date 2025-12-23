# Delta Spec: Basic CLI Commands

## ADDED Requirements

### Requirement: CLI 结构 / CLI Structure
The system SHALL implement command-line interface using clap.

系统应使用 clap 实现命令行界面。

#### Scenario: 命令解析 / Command Parsing
- **WHEN** 解析命令行参数时
- **THEN** 使用 clap derive 宏
- **AND** 支持子命令：run, config, agents
- **AND** run 命令接受可选的 prompt 参数

- **WHEN** parsing command-line arguments
- **THEN** uses clap derive macros
- **AND** supports subcommands: run, config, agents
- **AND** run command accepts optional prompt parameter

### Requirement: run 命令 / Run Command
The system SHALL implement run command for non-interactive mode.

系统应实现 run 命令用于非交互模式。

#### Scenario: 非交互模式 / Non-interactive Mode
- **WHEN** 执行 kode run "prompt" 时
- **THEN** 加载配置
- **AND** 调用 AI API
- **AND** 执行工具调用
- **AND** 输出结果
- **AND** 退出

- **WHEN** executing kode run "prompt"
- **THEN** loads configuration
- **AND** calls AI API
- **AND** executes tool calls
- **AND** outputs result
- **AND** exits

### Requirement: config 命令 / Config Command
The system SHALL implement config commands.

系统应实现 config 命令。

#### Scenario: config 子命令 / Config Subcommands
- **WHEN** 执行 kode config list 时
- **THEN** 显示当前配置
- **AND** 支持显示全局和项目配置

- **WHEN** executing kode config list
- **THEN** shows current configuration
- **AND** supports showing global and project config
