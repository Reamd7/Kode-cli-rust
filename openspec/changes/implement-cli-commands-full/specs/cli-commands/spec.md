# Delta Spec: CLI Commands Full

## MODIFIED Requirements

### Requirement: config 命令组 / Config Command Group
The system SHALL implement complete config management commands.

系统应实现完整的配置管理命令。

#### Scenario: config list / Config List
- **WHEN** 执行 kode config list 时
- **THEN** 显示全局配置
- **AND** 显示项目配置（如存在）
- **AND** 高亮显示合并后的配置

- **WHEN** executing kode config list
- **THEN** shows global configuration
- **AND** shows project configuration (if exists)
- **AND** highlights merged configuration

#### Scenario: config set / Config Set
- **WHEN** 执行 kode config set <key> <value> 时
- **THEN** 设置配置值
- **AND** 保存到配置文件
- **AND** 验证配置有效性

- **WHEN** executing kode config set <key> <value>
- **THEN** sets configuration value
- **AND** saves to config file
- **AND** validates configuration

#### Scenario: config get / Config Get
- **WHEN** 执行 kode config get <key> 时
- **THEN** 获取并显示配置值
- **AND** 显示配置来源（全局/项目）

- **WHEN** executing kode config get <key>
- **THEN** retrieves and displays configuration value
- **AND** shows config source (global/project)

### Requirement: agents 命令 / Agents Command
The system SHALL implement agents management commands.

系统应实现 agents 管理命令。

#### Scenario: agents list / Agents List
- **WHEN** 执行 kode agents list 时
- **THEN** 列出所有可用 agents
- **AND** 显示 agent 名称和描述
- **AND** 显示 agent 来源

- **WHEN** executing kode agents list
- **THEN** lists all available agents
- **AND** shows agent names and descriptions
- **AND** shows agent sources

#### Scenario: agents show / Agents Show
- **WHEN** 执行 kode agents show <name> 时
- **THEN** 显示 agent 完整信息
- **AND** 包括 system_prompt
- **AND** 包括可用工具

- **WHEN** executing kode agents show <name>
- **THEN** shows complete agent information
- **AND** includes system_prompt
- **AND** includes available tools

### Requirement: tools 命令 / Tools Command
The system SHALL implement tools listing commands.

系统应实现工具列表命令。

#### Scenario: tools list / Tools List
- **WHEN** 执行 kode tools list 时
- **THEN** 列出所有可用工具
- **AND** 显示工具名称和描述
- **AND** 显示工具参数 schema

- **WHEN** executing kode tools list
- **THEN** lists all available tools
- **AND** shows tool names and descriptions
- **AND** shows tool parameter schemas
