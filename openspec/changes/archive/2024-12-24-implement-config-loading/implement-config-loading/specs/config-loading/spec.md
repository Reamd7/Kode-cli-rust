# Delta Spec: Config Loading System

## MODIFIED Requirements

### Requirement: Config 结构体定义 / Config Structure Definition
The system SHALL define configuration structures for global and project settings.

系统应定义全局和项目配置的结构体。

#### Scenario: 全局配置结构 / Global Config Structure
- **WHEN** 定义 GlobalConfig 结构时
- **THEN** 包含 theme (Option<String>)
- **AND** 包含 model_profiles (Option<HashMap<String, ModelProfile>>)
- **AND** 包含 model_pointers (Option<ModelPointers>)
- **AND** 包含 default_model (Option<String>)
- **AND** 包含 mcp_servers (Option<HashMap<String, McpServerConfig>>)
- **AND** 包含 verbose (Option<bool>)
- **AND** 包含 auto_updater_status (Option<String>)
- **AND** 使用 serde 序列化，camelCase 字段命名

- **WHEN** defining GlobalConfig struct
- **THEN** includes theme (Option<String>)
- **AND** includes model_profiles (Option<HashMap<String, ModelProfile>>)
- **AND** includes model_pointers (Option<ModelPointers>)
- **AND** includes default_model (Option<String>)
- **AND** includes mcp_servers (Option<HashMap<String, McpServerConfig>>)
- **AND** includes verbose (Option<bool>)
- **AND** includes auto_updater_status (Option<String>)
- **AND** uses serde serialization with camelCase field naming

#### Scenario: 项目配置结构 / Project Config Structure
- **WHEN** 定义 ProjectConfig 结构时
- **THEN** 包含 allowed_tools (Option<Vec<String>>)
- **AND** 包含 allowed_commands (Option<Vec<String>>)
- **AND** 包含 allowed_paths (Option<Vec<String>>)
- **AND** 包含 enable_architect_tool (Option<bool>)
- **AND** 包含 mcp_servers (Option<HashMap<String, McpServerConfig>>)
- **AND** 包含 context (Option<HashMap<String, String>>)
- **AND** 使用 serde 序列化，camelCase 字段命名

- **WHEN** defining ProjectConfig struct
- **THEN** includes allowed_tools (Option<Vec<String>>)
- **AND** includes allowed_commands (Option<Vec<String>>)
- **AND** includes allowed_paths (Option<Vec<String>>)
- **AND** includes enable_architect_tool (Option<bool>)
- **AND** includes mcp_servers (Option<HashMap<String, McpServerConfig>>)
- **AND** includes context (Option<HashMap<String, String>>)
- **AND** uses serde serialization with camelCase field naming

#### Scenario: MCP 服务器配置 / MCP Server Configuration
- **WHEN** 定义 McpServerConfig 时
- **THEN** 使用 #[serde(untagged)] 枚举
- **AND** 支持 McpStdioServerConfig (command, args, env)
- **AND** 支持 McpSseServerConfig (url)

- **WHEN** defining McpServerConfig
- **THEN** uses #[serde(untagged)] enum
- **AND** supports McpStdioServerConfig (command, args, env)
- **AND** supports McpSseServerConfig (url)

### Requirement: 配置加载 / Configuration Loading
The system SHALL load configuration from file system.

系统应从文件系统加载配置。

#### Scenario: 加载全局配置 / Load Global Configuration
- **WHEN** 调用 ConfigLoader::load_global() 时
- **THEN** 从 ~/.kode.json 读取配置
- **AND** 如果文件不存在，返回默认配置
- **AND** 如果文件存在，解析 JSON 并返回配置
- **AND** 如果解析失败，返回 ConfigParseError

- **WHEN** calling ConfigLoader::load_global()
- **THEN** reads config from ~/.kode.json
- **AND** if file doesn't exist, returns default config
- **AND** if file exists, parses JSON and returns config
- **AND** if parsing fails, returns ConfigParseError

#### Scenario: 加载项目配置 / Load Project Configuration
- **WHEN** 调用 ConfigLoader::load_current_project() 时
- **THEN** 从当前目录向上查找 .kode.json
- **AND** 如果找到文件，解析并返回配置
- **AND** 如果未找到文件，返回默认配置

- **WHEN** calling ConfigLoader::load_current_project()
- **THEN** searches upward from current directory for .kode.json
- **AND** if file found, parses and returns config
- **AND** if file not found, returns default config

#### Scenario: 异步 API / Async API
- **WHEN** 实现配置加载方法时
- **THEN** 使用 async fn
- **AND** 使用 tokio::fs 进行文件操作
- **AND** 返回 impl Future<Output = Result<Config>>

- **WHEN** implementing config loading methods
- **THEN** use async fn
- **AND** use tokio::fs for file operations
- **AND** return impl Future<Output = Result<Config>>

### Requirement: 配置合并 / Configuration Merging
The system SHALL merge global and project configurations.

系统应合并全局和项目配置。

#### Scenario: 配置合并逻辑 / Config Merge Logic
- **WHEN** 调用 ConfigLoader::merge(global, project) 时
- **THEN** 返回包含 global 和 project 的 Config
- **AND** project 配置优先级高于 global 配置
- **AND** mcp_servers 进行合并（两边的服务器都可用）

- **WHEN** calling ConfigLoader::merge(global, project)
- **THEN** returns Config containing both global and project
- **AND** project config has higher priority than global config
- **AND** mcp_servers are merged (servers from both sides available)

### Requirement: 配置序列化 / Configuration Serialization
The system SHALL serialize configuration with camelCase naming.

系统应使用 camelCase 命名序列化配置。

#### Scenario: camelCase 字段命名 / camelCase Field Naming
- **WHEN** 序列化配置结构时
- **THEN** 使用 #[serde(rename_all = "camelCase")]
- **AND** Rust 代码使用 snake_case
- **AND** JSON 输出使用 camelCase
- **AND** 与 TypeScript 版本配置格式兼容

- **WHEN** serializing config structures
- **THEN** use #[serde(rename_all = "camelCase")]
- **AND** Rust code uses snake_case
- **AND** JSON output uses camelCase
- **AND** compatible with TypeScript version config format

### Requirement: 默认配置 / Default Configuration
The system SHALL provide default configuration values.

系统应提供默认配置值。

#### Scenario: 默认全局配置 / Default Global Config
- **WHEN** 配置文件不存在时
- **THEN** 返回 GlobalConfig::default()
- **AND** 默认主题为 "dark"
- **AND** 默认 model_profiles 为空 HashMap
- **AND** 默认 model_pointers 为 ModelPointers::default()

- **WHEN** config file doesn't exist
- **THEN** return GlobalConfig::default()
- **AND** default theme is "dark"
- **AND** default model_profiles is empty HashMap
- **AND** default model_pointers is ModelPointers::default()

#### Scenario: 默认项目配置 / Default Project Config
- **WHEN** 项目配置文件不存在时
- **THEN** 返回 ProjectConfig::default()
- **AND** 默认允许所有工具
- **AND** 其他字段为空或默认值

- **WHEN** project config file doesn't exist
- **THEN** return ProjectConfig::default()
- **AND** default allows all tools
- **AND** other fields are empty or default values

### Requirement: 配置错误处理 / Configuration Error Handling
The system SHALL handle configuration errors appropriately.

系统应正确处理配置错误。

#### Scenario: 文件读取错误 / File Read Error
- **WHEN** 文件读取失败（除 NotFound 外）时
- **THEN** 返回 ConfigLoadError
- **AND** 错误包含文件路径和原因

- **WHEN** file read fails (except NotFound)
- **THEN** return ConfigLoadError
- **AND** error includes file path and cause

#### Scenario: JSON 解析错误 / JSON Parse Error
- **WHEN** JSON 解析失败时
- **THEN** 返回 ConfigParseError
- **AND** 错误包含解析错误详情和文件路径

- **WHEN** JSON parsing fails
- **THEN** return ConfigParseError
- **AND** error includes parse error details and file path

#### Scenario: 文件写入错误 / File Write Error
- **WHEN** 配置保存失败时
- **THEN** 返回 ConfigSaveError
- **AND** 错误包含文件路径和原因

- **WHEN** config save fails
- **THEN** return ConfigSaveError
- **AND** error includes file path and cause
