# CLI Commands Specification

CLI 命令规范 - 定义命令行接口的实现。

## Purpose

CLI 命令提供命令行方式与 Kode 交互。使用 clap 解析命令行参数，支持交互模式和非交互模式，提供配置管理、Agent 列表等命令。

## ADDED Requirements

### Requirement: 命令行解析
The system SHALL parse command line arguments using clap.

系统必须使用 clap 解析命令行参数。

#### Scenario: 基本命令
- **WHEN** 用户运行 `kode [prompt]`
- **THEN** 直接处理 prompt（非交互模式）
- **AND** 无参数时进入交互模式

#### Scenario: 子命令
- **WHEN** 用户运行 `kode <subcommand> [args]`
- **THEN** 调用对应的子命令处理
- **AND** 支持嵌套子命令

#### Scenario: 帮助信息
- **WHEN** 用户运行 `kode --help`
- **THEN** 显示使用说明
- **AND** 列出所有可用命令

### Requirement: 配置命令组
The system SHALL provide config commands for managing configuration.

系统必须提供 config 命令管理配置。

#### Scenario: 列出配置
- **WHEN** 用户运行 `kode config list`
- **THEN** 显示当前配置
- **AND** 包含全局和项目配置

#### Scenario: 设置配置
- **WHEN** 用户运行 `kode config set <key> <value>`
- **THEN** 更新配置值
- **AND** 保存到配置文件

#### Scenario: 获取配置
- **WHEN** 用户运行 `kode config get <key>`
- **THEN** 显示指定配置的值
- **AND** 支持嵌套键路径

#### Scenario: 编辑配置
- **WHEN** 用户运行 `kode config edit`
- **THEN** 在编辑器中打开配置文件
- **AND** 使用默认编辑器或 EDITOR 环境变量

### Requirement: Agent 命令
The system SHALL provide agent commands for managing Agents.

系统必须提供 agent 命令管理 Agent。

#### Scenario: 列出 Agent
- **WHEN** 用户运行 `kode agents list`
- **THEN** 列出所有可用的 Agent
- **AND** 显示名称和描述

#### Scenario: Agent 详情
- **WHEN** 用户运行 `kode agents show <name>`
- **THEN** 显示 Agent 的详细信息
- **AND** 包含允许的工具和模型配置

#### Scenario: 验证 Agent
- **WHEN** 用户运行 `kode agents validate <name>`
- **THEN** 验证 Agent 定义
- **AND** 报告任何错误

### Requirement: 工具命令
The system SHALL provide tool commands for inspecting tools.

系统必须提供 tool 命令检查工具。

#### Scenario: 列出工具
- **WHEN** 用户运行 `kode tools list`
- **THEN** 列出所有可用工具
- **AND** 按来源分组（内置、MCP）

#### Scenario: 工具详情
- **WHEN** 用户运行 `kode tools show <name>`
- **THEN** 显示工具详细信息
- **AND** 包含参数 schema

### Requirement: 模型命令
The system SHALL provide model commands for managing models.

系统必须提供 model 命令管理模型。

#### Scenario: 列出模型
- **WHEN** 用户运行 `kode models list`
- **THEN** 列出配置的模型
- **AND** 显示模型提供商和名称

#### Scenario: 切换模型
- **WHEN** 用户运行 `kode models switch <name>`
- **THEN** 更改默认模型
- **AND** 验证模型存在

### Requirement: 非交互模式
The system SHALL support non-interactive execution.

系统必须支持非交互式执行。

#### Scenario: 直接执行
- **WHEN** 用户运行 `kode "prompt"`
- **THEN** 发送 prompt 到 AI
- **AND** 显示响应和工具执行结果
- **AND** 完成后退出

#### Scenario: 管道输入
- **WHEN** 用户通过管道输入
- **THEN** 读取标准输入作为 prompt
- **AND** 处理后退出

#### Scenario: JSON 输出
- **WHEN** 用户指定 `--output json`
- **THEN** 输出格式化为 JSON
- **AND** 便于脚本处理

### Requirement: 日志控制
The system SHALL support logging verbosity control.

系统必须支持日志详细程度控制。

#### Scenario: 详细输出
- **WHEN** 用户指定 `--verbose` 或 `-v`
- **THEN** 启用详细日志输出
- **AND** 显示调试信息

#### Scenario: 安静模式
- **WHEN** 用户指定 `--quiet` 或 `-q`
- **THEN** 只输出错误和结果
- **AND** 抑制额外信息

#### Scenario: 日志级别
- **WHEN** 用户指定 `--log-level <level>`
- **THEN** 设置日志级别
- **AND** 支持 trace, debug, info, warn, error

### Requirement: 命令别名
The system SHALL support command aliases.

系统必须支持命令别名。

#### Scenario: 短别名
- **WHEN** 定义命令别名
- **THEN** 支持常用的短形式
- **AND** 如 `a` for `agents`

#### Scenario: 自定义别名
- **WHEN** 用户配置自定义别名
- **THEN** 支持在配置文件中定义
- **AND** 覆盖默认别名

## Non-Goals

本规范不包含：
- Shell 补全脚本（未来扩展）
- 命令历史记录（在 TUI 中处理）
- 进度条和加载动画
- 远程命令执行
