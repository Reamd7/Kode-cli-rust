# 设计文档 - 配置加载系统 / Design Document - Config Loading System

## Context

配置系统是 Kode-Rust 的基础组件，负责从文件系统加载配置、处理环境变量、支持配置迁移，并与 TypeScript 版本保持 100% 兼容。

The configuration system is a foundational component of Kode-Rust, responsible for loading configuration from the file system, handling environment variables, supporting configuration migration, and maintaining 100% compatibility with the TypeScript version.

## TypeScript 版本参考 / TypeScript Version Reference

在实现本设计时，请参考原版 TypeScript 项目中的以下文件：

When implementing this design, refer to the following files in the original TypeScript project:

### 核心配置实现 / Core Configuration Implementation
- **配置加载**: `/Users/gemini/Documents/backup/Kode-cli/src/utils/config.ts` (940 行)
  - `getGlobalConfig()` - 全局配置加载（带迁移）
  - `getCurrentProjectConfig()` - 项目配置加载
  - `saveGlobalConfig()` / `saveCurrentProjectConfig()` - 配置保存
  - 30 个公开 API 函数

### 环境变量处理 / Environment Variables
- **环境变量**: `/Users/gemini/Documents/backup/Kode-cli/src/utils/env.ts`
  - `GLOBAL_CLAUDE_FILE` - 全局配置文件路径
  - `CLAUDE_BASE_DIR` - 基础目录
  - `KODE_CONFIG_DIR` / `CLAUDE_CONFIG_DIR` - 环境变量覆盖

### 实现细节 / Implementation Details
1. **配置优先级**: 5 层优先级（环境变量 > CLI > 项目 > 全局 > 默认）
2. **配置存储**: 单文件架构 (~/.kode.json)
3. **字段命名**: TypeScript 版本使用 camelCase
4. **默认值处理**: 配置文件不存在时返回默认值
5. **错误处理**: 使用 `ConfigParseError` 处理解析错误
6. **保存优化**: 只保存与默认值不同的字段

## Goals / Non-Goals

### Goals

- [x] 实现单文件配置架构（~/.kode.json）/ Implement single-file configuration architecture (~/.kode.json)
- [ ] 项目配置存储在 GlobalConfig.projects 中 / Store project configs in GlobalConfig.projects
- [ ] 实现完整的配置优先级系统 / Implement complete configuration priority system
- [ ] 支持环境变量覆盖配置 / Support environment variable overrides
- [ ] 实现 30 个配置 API 函数 / Implement 30 configuration API functions
- [ ] 支持 JSON 序列化/反序列化 / Support JSON serialization/deserialization
- [ ] 与 TypeScript 版本配置格式完全兼容 / Fully compatible with TypeScript version configuration format
- [ ] 实现配置迁移逻辑 / Implement configuration migration logic

### Non-Goals

- [ ] 配置验证（如检查 API key 有效性）/ Configuration validation (e.g., checking API key validity)
- [ ] 配置文件的热重载 / Hot reloading of configuration files
- [ ] 环境变量覆盖的配置验证 / Validation of environment variable overrides
- [ ] .mcprc 文件支持（后续添加）/ .mcprc file support (add later)

## Decisions

### 决策 1: 配置优先级架构 / Decision 1: Configuration Priority Architecture

**选择**: 5 层配置优先级系统

**Choice**: 5-Layer Configuration Priority System

**优先级从高到低 / Priority from High to Low**:
1. 环境变量 (Environment Variables)
2. 运行时标志 (Runtime Flags/CLI)
3. 项目配置 (Project Configuration)
4. 全局配置 (Global Configuration)
5. 默认值 (Default Values)

**理由 / Rationale**:
- 提供最大的灵活性 / Provides maximum flexibility
- 符合 12-factor app 原则 / Follows 12-factor app principles
- 与 TypeScript 版本保持一致 / Consistent with TypeScript version
- 环境变量优先级最高，适合容器化部署 / Environment variables have highest priority, suitable for containerized deployment

**实现 / Implementation**:
```rust
// 伪代码示例
fn get_config_with_priority<T>(
    env_var: Option<&str>,
    cli_flag: Option<T>,
    project_config: Option<T>,
    global_config: Option<T>,
    default: T
) -> T {
    env_var
        .or(cli_flag)
        .or(project_config)
        .or(global_config)
        .unwrap_or(default)
}
```

### 决策 2: 配置存储架构 / Decision 2: Configuration Storage Architecture

**选择**: 单文件架构，项目配置嵌套在全局配置中

**Choice**: Single-file architecture with project configs nested in global config

**理由 / Rationale**:
- 与 TypeScript 版本完全兼容 / Fully compatible with TypeScript version
- 所有配置集中在一个文件中，便于管理 / All configs in one file for easier management
- 可以复用现有的配置文件 / Can reuse existing configuration files
- 符合原版实现的设计决策 / Aligns with original implementation design

**配置文件结构 / Configuration File Structure**:
```json
{
  "projects": {
    "/Users/user/project1": { "allowedTools": [...], ... },
    "/Users/user/project2": { "allowedTools": [...], ... }
  },
  "modelProfiles": [...],
  "modelPointers": { "main": "...", ... },
  "theme": "dark",
  ...
}
```

**实现 / Implementation**:
```rust
/// 全局配置 (~/.kode.json)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GlobalConfig {
    /// 项目配置集合 - key 是绝对路径
    pub projects: Option<HashMap<String, ProjectConfig>>,

    /// 模型配置列表（数组类型）
    pub model_profiles: Option<Vec<ModelProfile>>,

    /// 模型指针
    pub model_pointers: Option<ModelPointers>,

    /// 其他 20+ 个全局配置字段
    ...
}

/// 项目配置 (存储在 GlobalConfig.projects 中)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProjectConfig {
    /// 允许使用的工具列表
    #[serde(default)]
    pub allowed_tools: Vec<String>,

    /// 上下文信息
    #[serde(default)]
    pub context: HashMap<String, String>,

    /// 其他 17 个项目配置字段
    ...
}
```

### 决策 3: 环境变量集成 / Decision 3: Environment Variable Integration

**选择**: 支持关键配置项的环境变量覆盖

**Choice**: Support environment variable overrides for key configuration items

**支持的环境变量 / Supported Environment Variables**:
- `OPENAI_API_KEY` - OpenAI API 密钥
- `ANTHROPIC_API_KEY` - Anthropic API 密钥
- `KODE_CONFIG_DIR` - 自定义配置目录
- `CLAUDE_CONFIG_DIR` - 兼容 Claude Code 配置目录

**实现 / Implementation**:
```rust
pub fn get_openai_api_key() -> Option<String> {
    std::env::var("OPENAI_API_KEY").ok()
}

pub fn get_anthropic_api_key() -> String {
    std::env::var("ANTHROPIC_API_KEY").unwrap_or_default()
}

pub fn get_config_file_path() -> PathBuf {
    if let Ok(dir) = std::env::var("KODE_CONFIG_DIR") {
        return PathBuf::from(dir).join("config.json");
    }
    if let Ok(dir) = std::env::var("CLAUDE_CONFIG_DIR") {
        return PathBuf::from(dir).join("config.json");
    }
    dirs::home_dir()
        .unwrap()
        .join(".kode.json")
}
```

### 决策 4: 配置保存优化 / Decision 4: Configuration Save Optimization

**选择**: 只保存与默认值不同的字段

**Choice**: Only save fields that differ from default values

**理由 / Rationale**:
- 减少配置文件大小 / Reduce configuration file size
- 提高可读性 / Improve readability
- 与 TypeScript 版本行为一致 / Consistent with TypeScript version behavior

**实现 / Implementation**:
```rust
pub fn save_optimized(config: &GlobalConfig, path: &Path) -> Result<()> {
    let default = GlobalConfig::default();

    // 过滤掉与默认值相同的字段
    let filtered_config = serde_json::to_value(config)?
        .as_object_mut()
        .map(|obj| {
            obj.retain(|key, value| {
                let default_value = serde_json::to_value(&default).unwrap();
                default_value.get(key) != Some(value)
            });
            obj.clone()
        });

    // 保存过滤后的配置
    let content = serde_json::to_string_pretty(&filtered_config)?;
    fs::write(path, content)?;

    Ok(())
}
```

### 决策 5: 配置迁移策略 / Decision 5: Configuration Migration Strategy

**选择**: 自动迁移旧配置格式

**Choice**: Automatically migrate old configuration formats

**迁移任务 / Migration Tasks**:
1. 移除 `ModelProfile.id` 字段 / Remove `ModelProfile.id` field
2. 更新 `modelPointers` 从 id 改为 `modelName` / Update `modelPointers` from id to `modelName`
3. 移除废弃字段 / Remove deprecated fields

**实现 / Implementation**:
```rust
pub fn migrate_config(config: GlobalConfig) -> GlobalConfig {
    // 实现迁移逻辑
    // TODO: 实施时完成
    config
}
```

### 决策 6: API 函数组织 / Decision 6: API Function Organization

**选择**: 按功能分组，实现 30 个公开 API

**Choice**: Group by functionality, implement 30 public APIs

**API 分组 / API Groups**:

#### 核心配置 (Core Configuration - 4 functions)
- `get_global_config()` - 获取全局配置
- `get_current_project_config()` - 获取当前项目配置
- `save_global_config()` - 保存全局配置
- `save_current_project_config()` - 保存项目配置

#### 环境变量 (Environment Variables - 2 functions)
- `get_openai_api_key()` - 获取 OpenAI API Key
- `get_anthropic_api_key()` - 获取 Anthropic API Key

#### 配置迁移 (Configuration Migration - 1 function)
- `migrate_model_profiles()` - 迁移模型配置

#### 模型系统 (Model System - 2 functions)
- `set_all_pointers_to_model()` - 设置所有指针
- `set_model_pointer()` - 设置单个指针

#### CLI 工具 (CLI Tools - 4 functions)
- `get_config_for_cli()` - CLI 获取配置
- `set_config_for_cli()` - CLI 设置配置
- `delete_config_for_cli()` - CLI 删除配置
- `list_config_for_cli()` - CLI 列出配置

#### 配置验证 (Configuration Validation - 2 functions)
- `is_global_config_key()` - 验证全局配置键
- `is_project_config_key()` - 验证项目配置键

#### 工具函数 (Utility Functions - 6 functions)
- `normalize_api_key()` - 规范化 API Key
- `get_custom_api_key_status()` - 获取自定义 Key 状态
- `is_auto_updater_disabled()` - 检查自动更新
- `check_has_trust_dialog_accepted()` - 检查信任对话框
- `get_or_create_user_id()` - 获取或创建用户 ID
- `enable_configs()` - 启用配置

#### GPT-5 支持 (GPT-5 Support - 5 functions)
- `is_gpt5_model_name()` - 判断是否 GPT-5
- `validate_and_repair_gpt5_profile()` - 验证和修复 GPT-5 配置
- `validate_and_repair_all_gpt5_profiles()` - 批量验证修复
- `get_gpt5_config_recommendations()` - GPT-5 配置建议
- `create_gpt5_model_profile()` - 创建 GPT-5 配置

#### MCP 支持 (MCP Support - 4 functions)
- `get_mcprc_config()` - 读取 .mcprc
- `clear_mcprc_config_for_testing()` - 测试工具
- `add_mcprc_server_for_testing()` - 测试工具
- `remove_mcprc_server_for_testing()` - 测试工具

**实现 / Implementation**:
```rust
// 模块结构
pub mod config;
pub mod env;
pub mod migration;
pub mod model;
pub mod cli;
pub mod utils;
pub mod gpt5;
pub mod mcprc;

// 重新导出
pub use config::*;
pub use env::*;
pub use migration::*;
pub use model::*;
pub use cli::*;
pub use utils::*;
pub use gpt5::*;
pub use mcprc::*;
```

## Architecture Overview

### 配置加载流程 / Configuration Loading Flow

```
1. get_global_config()
   ├─ 读取环境变量 KODE_CONFIG_DIR / CLAUDE_CONFIG_DIR
   ├─ 确定配置文件路径 (~/.kode.json 或自定义路径)
   ├─ 读取配置文件
   ├─ 如果文件不存在，返回默认配置
   ├─ 如果解析失败，返回默认配置
   ├─ 应用配置迁移逻辑
   └─ 返回 GlobalConfig

2. get_current_project_config()
   ├─ 调用 get_global_config()
   ├─ 获取当前目录的绝对路径
   ├─ 在 GlobalConfig.projects 中查找
   ├─ 如果找到，返回项目配置
   └─ 如果未找到，返回默认项目配置

3. 环境变量处理
   ├─ get_openai_api_key() -> process.env.OPENAI_API_KEY
   ├─ get_anthropic_api_key() -> process.env.ANTHROPIC_API_KEY
   └─ 其他环境变量按需处理
```

### 配置保存流程 / Configuration Saving Flow

```
1. save_global_config()
   ├─ 过滤掉与默认值相同的字段
   ├─ 序列化为 JSON（2 空格缩进）
   ├─ 创建目录（如不存在）
   ├─ 写入文件
   └─ 处理权限错误（静默跳过）

2. save_current_project_config()
   ├─ 调用 get_global_config()
   ├─ 更新 GlobalConfig.projects[{current_dir}]
   ├─ 调用 save_global_config()
   └─ 保存整个全局配置
```

## Error Handling Strategy

### 错误类型 / Error Types

```rust
pub enum ConfigError {
    /// 文件读取失败
    ConfigLoadError { path: PathBuf, message: String },

    /// JSON 解析失败
    ConfigParseError { path: PathBuf, message: String },

    /// 文件写入失败
    ConfigSaveError { path: PathBuf, message: String },

    /// 通用配置错误
    ConfigError(String),
}
```

### 错误处理策略 / Error Handling Strategy

1. **文件不存在**: 返回默认配置，不抛错 / File not found: return default, no error
2. **解析失败**: 返回默认配置，记录日志 / Parse error: return default, log warning
3. **权限错误**: 静默跳过，记录日志 / Permission error: silently skip, log warning
4. **其他错误**: 返回错误给调用者 / Other errors: return to caller

## Testing Strategy

### 单元测试 / Unit Tests

- 测试配置加载（文件存在、不存在、解析失败）
- 测试配置保存
- 测试配置合并
- 测试环境变量处理
- 测试配置迁移
- 测试所有 30 个 API 函数

### 集成测试 / Integration Tests

- 测试与 TypeScript 版本配置文件兼容性
- 测试配置优先级
- 测试完整的加载和保存流程

## Migration Path

从当前的双文件架构迁移到单文件架构：

1. 保留现有配置文件作为备份
2. 实现新的单文件配置系统
3. 提供迁移工具将现有配置转换为新格式
4. 测试兼容性
5. 部署新版本

## Performance Considerations

- 使用 LRU 缓存避免重复读取配置文件（可选）
- 配置迁移只在加载时执行一次
- JSON 解析使用 serde_json（性能优秀）
- 异步 API 使用 Tokio

## Security Considerations

- API Key 不应在日志中输出完整值
- 配置文件权限应设置为用户只读（0600）
- 环境变量优先级最高，注意安全风险
