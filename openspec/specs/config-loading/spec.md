# 配置加载系统规范 / Config Loading System Specification

## Purpose

配置系统负责加载和管理 Kode 的配置文件，支持环境变量、CLI 参数、项目配置和全局配置的优先级分层。

The configuration system is responsible for loading and managing Kode configuration files, supporting hierarchical priority of environment variables, CLI parameters, project configuration, and global configuration.

## Requirements

### Requirement: 配置优先级 / Configuration Priority

系统应按以下优先级处理配置（从高到低）：

The system SHALL handle configuration with the following priority (from highest to lowest):

1. **环境变量 / Environment Variables** (最高优先级 / Highest Priority)
   - `OPENAI_API_KEY`
   - `ANTHROPIC_API_KEY`
   - `KODE_CONFIG_DIR`
   - `CLAUDE_CONFIG_DIR`

2. **运行时标志 / Runtime Flags** (CLI 参数)
   - `--verbose`
   - `--model`
   - 其他 CLI 参数

3. **项目配置 / Project Configuration**
   - 存储在全局配置的 `projects` 字段中
   - Key 为当前目录的绝对路径

4. **全局配置 / Global Configuration**
   - 从 `~/.kode.json` 加载
   - 可通过环境变量自定义路径

5. **默认值 / Default Values** (最低优先级 / Lowest Priority)
   - `DEFAULT_GLOBAL_CONFIG`
   - `DEFAULT_PROJECT_CONFIG`

### Requirement: 配置文件加载 / Configuration File Loading

系统应从 `~/.kode.json` 加载全局配置，项目配置存储在其中。

The system SHALL load global configuration from `~/.kode.json`, with project configurations stored within it.

#### Scenario: 加载全局配置 / Load Global Configuration
- **WHEN** 用户启动 Kode 应用时
- **THEN** 系统从 `~/.kode.json` 加载全局配置
- **AND** 如果文件不存在，返回默认配置
- **AND** 应用配置迁移逻辑（如需要）
- **AND** 环境变量 `KODE_CONFIG_DIR` 或 `CLAUDE_CONFIG_DIR` 可覆盖配置路径

- **WHEN** the user starts the Kode application
- **THEN** the system loads global configuration from `~/.kode.json`
- **AND** if the file does not exist, returns default configuration
- **AND** applies configuration migration logic (if needed)
- **AND** environment variable `KODE_CONFIG_DIR` or `CLAUDE_CONFIG_DIR` can override config path

#### Scenario: 加载项目配置 / Load Project Configuration
- **WHEN** 用户在项目目录中启动 Kode 时
- **THEN** 系统从全局配置的 `projects` 字段中查找当前目录的绝对路径
- **AND** 如果找到，使用该项目的配置
- **AND** 如果未找到，返回默认项目配置

- **WHEN** the user starts Kode in a project directory
- **THEN** the system looks up the absolute path of current directory in the `projects` field of global config
- **AND** if found, uses that project's configuration
- **AND** if not found, returns default project configuration

#### Scenario: 配置不存在时返回默认值 / Return Defaults When Config Missing
- **WHEN** 配置文件不存在或 JSON 解析失败时
- **THEN** 返回默认配置而不是抛出错误
- **AND** 记录适当的日志信息

- **WHEN** the configuration file does not exist or JSON parsing fails
- **THEN** return default configuration instead of throwing an error
- **AND** log appropriate information

### Requirement: 配置序列化 / Configuration Serialization

系统应支持 JSON 序列化和反序列化，使用 camelCase 字段名。

The system SHALL support JSON serialization and deserialization with camelCase field names.

#### Scenario: JSON 解析 / JSON Parsing
- **WHEN** 读取有效的 JSON 配置文件时
- **THEN** 正确解析所有字段
- **AND** 支持 camelCase 字段名（与 TypeScript 版本兼容）

- **WHEN** reading a valid JSON configuration file
- **THEN** correctly parse all fields
- **AND** support camelCase field names (compatible with TypeScript version)

#### Scenario: JSON 序列化 / JSON Serialization
- **WHEN** 保存配置到文件时
- **THEN** 只保存与默认值不同的字段
- **AND** 输出格式化的 JSON（2 空格缩进）
- **AND** 使用 camelCase 字段名

- **WHEN** saving configuration to a file
- **THEN** only save fields that differ from defaults
- **AND** output formatted JSON (2-space indent)
- **AND** use camelCase field names

### Requirement: 环境变量支持 / Environment Variables Support

系统应支持通过环境变量覆盖配置。

The system SHALL support overriding configuration through environment variables.

#### Scenario: API Key 环境变量 / API Key Environment Variables
- **WHEN** 需要获取 OpenAI API Key 时
- **THEN** 优先从 `OPENAI_API_KEY` 环境变量读取
- **AND** 如果未设置，返回 undefined

- **WHEN** OpenAI API Key is needed
- **THEN** read from `OPENAI_API_KEY` environment variable first
- **AND** if not set, return undefined

#### Scenario: 配置目录覆盖 / Config Directory Override
- **WHEN** 设置了 `KODE_CONFIG_DIR` 或 `CLAUDE_CONFIG_DIR` 环境变量时
- **THEN** 使用该目录作为配置文件路径
- **AND** 配置文件名为 `config.json`

- **WHEN** `KODE_CONFIG_DIR` or `CLAUDE_CONFIG_DIR` environment variable is set
- **THEN** use that directory as the configuration file path
- **AND** configuration file name is `config.json`

### Requirement: 配置保存 / Configuration Saving

系统应支持保存配置到适当的文件路径。

The system SHALL support saving configuration to the appropriate file path.

#### Scenario: 保存全局配置 / Save Global Configuration
- **WHEN** 用户修改全局配置并保存时
- **THEN** 写入到 `~/.kode.json`
- **AND** 如果目录不存在则自动创建
- **AND** 只保存与默认值不同的字段
- **AND** 遇到权限错误时静默跳过

- **WHEN** the user modifies and saves global configuration
- **THEN** write to `~/.kode.json`
- **AND** automatically create the directory if it does not exist
- **AND** only save fields that differ from defaults
- **AND** silently skip on permission errors

#### Scenario: 保存项目配置 / Save Project Configuration
- **WHEN** 用户修改项目配置并保存时
- **THEN** 将项目配置保存到全局配置的 `projects` 字段中
- **AND** 使用当前目录的绝对路径作为 key
- **AND** 保存整个全局配置到文件

- **WHEN** the user modifies and saves project configuration
- **THEN** save the project config in the `projects` field of global config
- **AND** use the absolute path of current directory as the key
- **AND** save the entire global config to file

### Requirement: 配置迁移 / Configuration Migration

系统应支持从旧版本配置格式迁移。

The system SHALL support migration from old version configuration formats.

#### Scenario: 移除 model profiles 的 id 字段 / Remove id Field from Model Profiles
- **WHEN** 加载包含旧格式 `modelProfiles` 的配置时
- **THEN** 自动移除 `id` 字段
- **AND** 更新 `modelPointers` 从 id 引用改为 `modelName` 引用
- **AND** 移除其他废弃字段（如 `defaultModelId`）

- **WHEN** loading configuration with old format `modelProfiles`
- **THEN** automatically remove the `id` field
- **AND** update `modelPointers` from id references to `modelName` references
- **AND** remove other deprecated fields (like `defaultModelId`)

## Reference / 参考资料

### TypeScript 版本实现参考 / TypeScript Implementation Reference

在实现本规范时，请参考原版 TypeScript 项目中的以下文件：

When implementing this specification, refer to the following files in the original TypeScript project:

#### 核心配置模块 / Core Configuration Module
- **配置加载与保存**: `/Users/gemini/Documents/backup/Kode-cli/src/utils/config.ts` (940 行)
  - 30 个公开 API 函数
  - 完整的配置加载、保存、迁移逻辑

- **环境变量处理**: `/Users/gemini/Documents/backup/Kode-cli/src/utils/env.ts`
  - `GLOBAL_CLAUDE_FILE` - 全局配置文件路径
  - `CLAUDE_BASE_DIR` - 基础目录配置

#### 配置类型定义 / Configuration Type Definitions
- **配置类型**: `/Users/gemini/Documents/backup/Kode-cli/src/utils/config.ts`
  - `GlobalConfig` - 全局配置类型 (24 个字段)
  - `ProjectConfig` - 项目配置类型 (19 个字段)
  - `ModelProfile` - 模型配置类型 (14 个字段)
  - `ProviderType` - 20 个提供商类型

### 实现要点 / Implementation Notes

1. **配置优先级**: 严格遵循 5 层优先级（环境变量 > CLI > 项目 > 全局 > 默认）
2. **配置存储**: 单文件架构，项目配置存储在 `GlobalConfig.projects` 中
3. **字段命名**: 使用 camelCase，与 TypeScript 版本完全兼容
4. **默认值处理**: 配置文件不存在或解析失败时返回默认配置，不抛错
5. **保存优化**: 只保存与默认值不同的字段
6. **错误容忍**: 权限错误、解析错误都应优雅处理

## Non-Goals

- 本规范不包含完整的配置验证逻辑（如 API key 有效性）
- 不包含配置文件的热重载
- 不包含配置文件编辑冲突处理
- 本版本不包含 .mcprc 文件支持（可后续添加）

- This specification does not include complete configuration validation logic (e.g., API key validity)
- Does not include hot reloading of configuration files
- Does not include configuration file edit conflict handling
- This version does not include .mcprc file support (can be added later)
