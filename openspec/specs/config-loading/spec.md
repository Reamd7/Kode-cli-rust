# 配置加载系统规范 / Config Loading System Specification

## Purpose

配置系统负责加载和管理 Kode 的配置文件，支持全局配置和项目配置的分层加载与合并。

The configuration system is responsible for loading and managing Kode configuration files, supporting hierarchical loading and merging of global and project configurations.

## Requirements

### Requirement: 配置文件加载 / Configuration File Loading
The system SHALL load configuration from ~/.kode.json and ./.kode.json with proper merge behavior.

系统应从 ~/.kode.json 和 ./.kode.json 加载配置，并正确合并配置。

#### Scenario: 加载全局配置 / Load Global Configuration
- **WHEN** 用户启动 Kode 应用时
- **THEN** 系统自动从 ~/.kode.json 加载全局配置
- **AND** 如果文件不存在，使用默认配置

- **WHEN** the user starts the Kode application
- **THEN** the system automatically loads global configuration from ~/.kode.json
- **AND** if the file does not exist, default configuration is used

#### Scenario: 加载项目配置 / Load Project Configuration
- **WHEN** 用户在项目目录中启动 Kode 时
- **THEN** 系统从 ./.kode.json 加载项目配置
- **AND** 项目配置覆盖全局配置的相同字段

- **WHEN** the user starts Kode in a project directory
- **THEN** the system loads project configuration from ./.kode.json
- **AND** project configuration overrides the same fields in global configuration

#### Scenario: 配置不存在时返回默认值 / Return Defaults When Config Missing
- **WHEN** 配置文件不存在时
- **THEN** 返回默认配置而不是错误
- **AND** 默认配置包含 theme="dark" 和其他基础设置

- **WHEN** a configuration file does not exist
- **THEN** return default configuration instead of an error
- **AND** default configuration includes theme="dark" and other basic settings

### Requirement: 配置序列化 / Configuration Serialization
The system SHALL support JSON serialization and deserialization with camelCase field names.

系统应支持 JSON 序列化和反序列化，使用 camelCase 字段名。

#### Scenario: JSON 解析 / JSON Parsing
- **WHEN** 读取有效的 JSON 配置文件时
- **THEN** 正确解析所有字段
- **AND** 支持 camelCase 和 snake_case 字段名

- **WHEN** reading a valid JSON configuration file
- **THEN** correctly parse all fields
- **AND** support both camelCase and snakeCase field names

#### Scenario: JSON 序列化 / JSON Serialization
- **WHEN** 保存配置到文件时
- **THEN** 输出格式化的 JSON
- **AND** 使用 camelCase 字段名以兼容 TypeScript 版本

- **WHEN** saving configuration to a file
- **THEN** output formatted JSON
- **AND** use camelCase field names for compatibility with the TypeScript version

### Requirement: 配置合并 / Configuration Merging
The system SHALL merge project configuration with global configuration, with project values taking precedence.

系统应将项目配置与全局配置合并，项目配置值优先。

#### Scenario: MCP 服务器配置合并 / MCP Server Configuration Merging
- **WHEN** 全局配置和项目配置都定义了 MCP 服务器时
- **THEN** 合并两个配置中的服务器列表
- **AND** 同名服务器使用项目配置的值

- **WHEN** both global and project configurations define MCP servers
- **THEN** merge server lists from both configurations
- **AND** for servers with the same name, use project configuration values

#### Scenario: 模型配置合并 / Model Configuration Merging
- **WHEN** 全局配置定义了模型配置时
- **THEN** 项目配置可以添加额外的模型配置
- **AND** 项目配置可以覆盖特定的模型指针

- **WHEN** global configuration defines model profiles
- **THEN** project configuration can add additional model profiles
- **AND** project configuration can override specific model pointers

### Requirement: 配置保存 / Configuration Saving
The system SHALL support saving configuration to the appropriate file path.

系统应支持保存配置到适当的文件路径。

#### Scenario: 保存全局配置 / Save Global Configuration
- **WHEN** 用户修改全局配置并保存时
- **THEN** 写入到 ~/.kode.json
- **AND** 如果目录不存在则自动创建

- **WHEN** the user modifies and saves global configuration
- **THEN** write to ~/.kode.json
- **AND** automatically create the directory if it does not exist

#### Scenario: 保存项目配置 / Save Project Configuration
- **WHEN** 用户修改项目配置并保存时
- **THEN** 写入到 ./.kode.json
- **AND** 保持格式化和可读性

- **WHEN** the user modifies and saves project configuration
- **THEN** write to ./.kode.json
- **AND** maintain formatting and readability

## Reference / 参考资料

### TypeScript 版本实现参考 / TypeScript Implementation Reference

在实现本规范时，请参考原版 TypeScript 项目中的以下文件：

When implementing this specification, refer to the following files in the original TypeScript project:

#### 核心配置模块 / Core Configuration Module
- **配置加载与合并**: `/Users/gemini/Documents/backup/Kode-cli/src/utils/config.ts`
  - `getGlobalConfig()` - 全局配置加载
  - `getCurrentProjectConfig()` - 项目配置加载
  - `saveGlobalConfig()` - 保存全局配置
  - `saveCurrentProjectConfig()` - 保存项目配置
  - `mergeConfigs()` - 配置合并逻辑

#### 配置类型定义 / Configuration Type Definitions
- **配置类型**: `/Users/gemini/Documents/backup/Kode-cli/src/utils/config.ts`
  - `ProjectConfig` - 项目配置类型
  - `McpServerConfig` - MCP 服务器配置类型
  - `ProviderType` - 提供商类型定义

#### 配置相关工具 / Configuration Utilities
- **环境变量处理**: `/Users/gemini/Documents/backup/Kode-cli/src/utils/env.ts`
  - `GLOBAL_CLAUDE_FILE` - 全局配置文件路径常量

### 实现要点 / Implementation Notes

1. **配置字段命名**: TypeScript 版本使用 camelCase，Rust 版本需要保持兼容
2. **默认值处理**: 参考 `DEFAULT_PROJECT_CONFIG` 的默认值定义
3. **配置合并策略**: 项目配置覆盖全局配置的同名字段
4. **错误处理**: 配置文件不存在时应返回默认值而非错误

## Non-Goals

- 本规范不包含配置验证逻辑
- 不包含配置迁移工具
- 不包含配置文件的热重载
- 不包含环境变量覆盖（未来可能添加）

- This specification does not include configuration validation logic
- Does not include configuration migration tools
- Does not include hot reloading of configuration files
- Does not include environment variable overriding (may be added in the future)
