# CLI 命令规范 / CLI Commands Specification

## Purpose

CLI 命令提供命令行接口，允许用户通过终端与 Kode 交互，执行各种操作如配置管理、Agent 列表、工具列表等。

The CLI commands provide a command-line interface, allowing users to interact with Kode through the terminal to perform various operations such as configuration management, Agent listing, tool listing, etc.

## Requirements

### Requirement: 命令行解析 / Command Line Parsing
The system SHALL parse command-line arguments using a declarative CLI definition.

系统应使用声明式 CLI 定义解析命令行参数。

#### Scenario: 解析主命令 / Parse Main Command
- **WHEN** 用户运行 `kode [prompt]` 时
- **THEN** 如果提供 prompt，运行一次并退出
- **AND** 如果不提供 prompt，启动交互式 REPL

- **WHEN** the user runs `kode [prompt]`
- **THEN** if a prompt is provided, run once and exit
- **AND** if no prompt is provided, start interactive REPL

#### Scenario: 解析子命令 / Parse Subcommands
- **WHEN** 用户运行 `kode <subcommand> [args]` 时
- **THEN** 执行对应的子命令
- **AND** 验证必需参数

- **WHEN** the user runs `kode <subcommand> [args]`
- **THEN** execute the corresponding subcommand
- **AND** validate required arguments

#### Scenario: 显示帮助 / Show Help
- **WHEN** 用户运行 `kode --help` 或 `kode help` 时
- **THEN** 显示使用说明
- **AND** 列出所有可用命令

- **WHEN** the user runs `kode --help` or `kode help`
- **THEN** display usage instructions
- **AND** list all available commands

### Requirement: 配置命令 / Config Commands
The system SHALL provide commands for managing Kode configuration.

系统应提供管理 Kode 配置的命令。

#### Scenario: 列出配置 / List Configuration
- **WHEN** 用户运行 `kode config list` 时
- **THEN** 显示当前配置内容
- **AND** 区分全局配置和项目配置

- **WHEN** the user runs `kode config list`
- **THEN** display current configuration contents
- **AND** distinguish between global and project configuration

#### Scenario: 设置配置 / Set Configuration
- **WHEN** 用户运行 `kode config set <key> <value>` 时
- **THEN** 更新配置值
- **AND** 保存到配置文件
- **AND** 验证配置有效性

- **WHEN** the user runs `kode config set <key> <value>`
- **THEN** update the configuration value
- **AND** save to the configuration file
- **AND** validate configuration validity

#### Scenario: 获取配置值 / Get Configuration Value
- **WHEN** 用户运行 `kode config get <key>` 时
- **THEN** 显示指定配置键的值
- **AND** 如果键不存在则显示错误

- **WHEN** the user runs `kode config get <key>`
- **THEN** display the value of the specified configuration key
- **AND** display an error if the key does not exist

### Requirement: Agent 命令 / Agent Commands
The system SHALL provide commands for managing and listing Agents.

系统应提供管理和列出 Agent 的命令。

#### Scenario: 列出 Agent / List Agents
- **WHEN** 用户运行 `kode agents list` 时
- **THEN** 显示所有可用 Agent
- **AND** 显示每个 Agent 的名称和描述

- **WHEN** the user runs `kode agents list`
- **THEN** display all available Agents
- **AND** display the name and description of each Agent

#### Scenario: 显示 Agent 详情 / Show Agent Details
- **WHEN** 用户运行 `kode agents show <name>` 时
- **THEN** 显示 Agent 的完整定义
- **AND** 包括系统提示词和工具列表

- **WHEN** the user runs `kode agents show <name>`
- **THEN** display the complete Agent definition
- **AND** include the system prompt and tool list

#### Scenario: 验证 Agent / Validate Agent
- **WHEN** 用户运行 `kode agents validate <name>` 时
- **THEN** 验证 Agent 定义的有效性
- **AND** 报告任何语法错误

- **WHEN** the user runs `kode agents validate <name>`
- **THEN** validate the Agent definition
- **AND** report any syntax errors

### Requirement: 工具命令 / Tool Commands
The system SHALL provide commands for listing available tools.

系统应提供列出可用工具的命令。

#### Scenario: 列出工具 / List Tools
- **WHEN** 用户运行 `kode tools list` 时
- **THEN** 显示所有可用工具
- **AND** 显示每个工具的名称和描述

- **WHEN** the user runs `kode tools list`
- **THEN** display all available tools
- **AND** display the name and description of each tool

#### Scenario: 显示工具详情 / Show Tool Details
- **WHEN** 用户运行 `kode tools show <name>` 时
- **THEN** 显示工具的参数 schema
- **AND** 显示工具描述

- **WHEN** the user runs `kode tools show <name>`
- **THEN** display the tool's parameter schema
- **AND** display the tool description

### Requirement: 模型命令 / Model Commands
The system SHALL provide commands for managing model configurations.

系统应提供管理模型配置的命令。

#### Scenario: 列出模型 / List Models
- **WHEN** 用户运行 `kode models list` 时
- **THEN** 显示所有配置的模型
- **AND** 显示每个模型的提供商和名称

- **WHEN** the user runs `kode models list`
- **THEN** display all configured models
- **AND** display the provider and name of each model

#### Scenario: 切换模型 / Switch Model
- **WHEN** 用户运行 `kode models switch <name>` 时
- **THEN** 更新默认模型
- **AND** 保存到配置文件

- **WHEN** the user runs `kode models switch <name>`
- **THEN** update the default model
- **AND** save to the configuration file

### Requirement: 版本命令 / Version Command
The system SHALL provide a command to display version information.

系统应提供显示版本信息的命令。

#### Scenario: 显示版本 / Show Version
- **WHEN** 用户运行 `kode --version` 或 `kode version` 时
- **THEN** 显示 Kode 版本号
- **AND** 显示构建信息（如适用）

- **WHEN** the user runs `kode --version` or `kode version`
- **THEN** display the Kode version number
- **AND** display build information (if applicable)

## Reference / 参考资料

### TypeScript 版本实现参考 / TypeScript Implementation Reference

在实现本规范时，请参考原版 TypeScript 项目中的以下文件：

When implementing this specification, refer to the following files in the original TypeScript project:

#### CLI 命令系统 / CLI Command System
- **命令定义**: `/Users/gemini/Documents/backup/Kode-cli/src/commands.ts`
  - 命令注册和处理
  - 命令参数解析

#### 命令实现 / Command Implementations
- **命令目录**: `/Users/gemini/Documents/backup/Kode-cli/src/commands/`
  - 所有 CLI 命令的实现
  - 参数验证和处理
  - 命令执行逻辑

#### 入口点 / Entry Points
- **应用入口**: `/Users/gemini/Documents/backup/Kode-cli/src/entrypoints/`
  - CLI 入口点
  - 命令行参数解析
  - 应用初始化

### 实现要点 / Implementation Notes

1. **命令解析**: 使用命令行参数解析库（如 clap）
2. **错误处理**: 命令执行失败时提供清晰的错误信息
3. **帮助信息**: 为每个命令提供详细的帮助文档
4. **配置交互**: 命令需要与配置系统交互
5. **输出格式**: 保持与 TypeScript 版本一致的输出格式

## Non-Goals

- 本规范不包含交互式 TUI 命令（在 tui-interface 规范中）
- 不包含调试和诊断命令
- 不包含插件管理命令

- This specification does not include interactive TUI commands (in tui-interface specification)
- Does not include debug and diagnostic commands
- Does not include plugin management commands
