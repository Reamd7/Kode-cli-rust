# 设计文档 - 配置加载系统 / Design Document - Config Loading System

## Context

配置系统是 Kode-Rust 的基础组件，负责从文件系统加载配置并与 TypeScript 版本保持兼容。

The configuration system is a foundational component of Kode-Rust, responsible for loading configuration from the file system and maintaining compatibility with the TypeScript version.

## Goals / Non-Goals

### Goals

- [ ] 实现全局配置加载（~/.kode.json）/ Implement global configuration loading (~/.kode.json)
- [ ] 实现项目配置加载（./.kode.json）/ Implement project configuration loading (./.kode.json)
- [ ] 实现配置合并逻辑 / Implement configuration merging logic
- [ ] 支持 JSON 序列化/反序列化 / Support JSON serialization/deserialization
- [ ] 与 TypeScript 版本配置格式兼容 / Compatible with TypeScript version configuration format

### Non-Goals

- [ ] 配置验证（如检查 API key 有效性）/ Configuration validation (e.g., checking API key validity)
- [ ] 配置迁移工具 / Configuration migration tools
- [ ] 环境变量覆盖 / Environment variable overriding

## Decisions

### 决策 1: 配置结构设计 / Decision 1: Configuration Structure Design

**选择**: 分离 GlobalConfig 和 ProjectConfig

**Choice**: Separate GlobalConfig and ProjectConfig

**理由 / Rationale**:
- 全局配置和项目配置有不同的字段 / Global and project configurations have different fields
- 明确配置的来源和优先级 / Clear source and priority of configuration
- 便于实现配置合并 / Easier to implement configuration merging

**实现 / Implementation**:
```rust
/// 全局配置 (~/.kode.json)
pub struct GlobalConfig {
    pub theme: Option<String>,
    pub model_profiles: Option<HashMap<String, ModelProfile>>,
    pub model_pointers: Option<ModelPointers>,
    pub default_model: Option<String>,
    pub mcp_servers: Option<HashMap<String, McpServerConfig>>,
    pub verbose: Option<bool>,
    pub auto_updater_status: Option<String>,
}

/// 项目配置 (./.kode.json)
pub struct ProjectConfig {
    pub allowed_tools: Option<Vec<String>>,
    pub allowed_commands: Option<Vec<String>>,
    pub allowed_paths: Option<Vec<String>>,
    pub enable_architect_tool: Option<bool>,
    pub mcp_servers: Option<HashMap<String, McpServerConfig>>,
    pub context: Option<HashMap<String, String>>,
}

/// 合并后的配置
pub struct Config {
    pub global: Option<GlobalConfig>,
    pub project: Option<ProjectConfig>,
}
```

### 决策 2: 字段命名策略 / Decision 2: Field Naming Strategy

**选择**: 使用 camelCase serde 属性

**Choice**: Use camelCase serde attributes

**理由 / Rationale**:
- 与 TypeScript 版本配置文件兼容 / Compatible with TypeScript version configuration files
- 符合 JSON 命名惯例 / Follows JSON naming conventions
- Rust 代码内部使用 snakeCase，序列化时自动转换 / Use snakeCase internally in Rust code, automatically convert during serialization

**实现 / Implementation**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelProfile {
    pub model_name: String,  // 序列化为 "modelName" / serializes as "modelName"
    pub base_url: Option<String>,  // 序列化为 "baseUrl" / serializes as "baseUrl"
    pub api_key: String,
    // ...
}
```

### 决策 3: 默认配置策略 / Decision 3: Default Configuration Strategy

**选择**: 文件不存在时返回默认配置而不是错误

**Choice**: Return default configuration when file does not exist instead of error

**理由 / Rationale**:
- 用户首次使用时无需创建配置文件 / Users don't need to create configuration files on first use
- 配置是可选的，所有功能都有合理的默认值 / Configuration is optional, all features have reasonable defaults
- 与 TypeScript 版本行为一致 / Consistent with TypeScript version behavior

**实现 / Implementation**:
```rust
impl Default for GlobalConfig {
    fn default() -> Self {
        GlobalConfig {
            theme: Some("dark".to_string()),
            model_profiles: Some(HashMap::new()),
            model_pointers: Some(ModelPointers::default()),
            // ...
        }
    }
}

pub async fn load(&self, path: &Path) -> Result<GlobalConfig, Error> {
    match fs::read_to_string(path).await {
        Ok(content) => {
            // 解析配置
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            Ok(GlobalConfig::default())
        }
        Err(e) => Err(Error::ConfigLoadError { ... }),
    }
}
```

### 决策 4: MCP 服务器配置 / Decision 4: MCP Server Configuration

**选择**: 使用枚举表示 STDIO 和 SSE 两种传输方式

**Choice**: Use enum to represent STDIO and SSE transport methods

**理由 / Rationale**:
- MCP 协议支持多种传输方式 / MCP protocol supports multiple transport methods
- 类型安全的配置 / Type-safe configuration
- 便于扩展新的传输方式 / Easy to extend with new transport methods

**实现 / Implementation**:
```rust
/// MCP 服务器配置（联合类型）
#[serde(untagged)]
pub enum McpServerConfig {
    Stdio(McpStdioServerConfig),
    Sse(McpSseServerConfig),
}

pub struct McpStdioServerConfig {
    pub command: String,
    pub args: Vec<String>,
    pub env: Option<HashMap<String, String>>,
}

pub struct McpSseServerConfig {
    pub url: String,
}
```

### 决策 5: 异步 API / Decision 5: Async API

**选择**: 使用 Tokio 异步 API 进行文件操作

**Choice**: Use Tokio async API for file operations

**理由 / Rationale**:
- 项目整体使用异步架构 / Project uses async architecture overall
- 配置加载不会阻塞主线程 / Configuration loading won't block main thread
- 与其他模块（如 Agent 加载）保持一致 / Consistent with other modules (like Agent loading)

**实现 / Implementation**:
```rust
pub async fn load(&self, path: &Path) -> Result<GlobalConfig, Error> {
    let content = fs::read_to_string(path).await?;
    // ...
}

pub async fn save(&self, config: &GlobalConfig, path: &Path) -> Result<(), Error> {
    // ...
}
```

## 数据流 / Data Flow

```
用户启动应用 / User starts app
    ↓
ConfigLoader::load_global()
    ↓
读取 ~/.kode.json / Read ~/.kode.json
    ↓
解析 JSON / Parse JSON
    ↓ (如果不存在 / if not found)
返回 GlobalConfig::default()
    ↓
ConfigLoader::load_current_project()
    ↓
读取 ./.kode.json / Read ./.kode.json
    ↓
解析 JSON / Parse JSON
    ↓ (如果不存在 / if not found)
返回 ProjectConfig::default()
    ↓
ConfigLoader::merge(global, project)
    ↓
返回合并后的 Config / Return merged Config
```

## 错误处理 / Error Handling

| 错误类型 / Error Type | 描述 / Description | 处理方式 / Handling |
|---------------------|-------------------|-------------------|
| `ConfigLoadError` | 文件读取失败 / File read failure | 返回详细错误信息 / Return detailed error message |
| `ConfigParseError` | JSON 解析失败 / JSON parse failure | 返回解析错误和路径 / Return parse error and path |
| `ConfigSaveError` | 文件写入失败 / File write failure | 返回写入错误和路径 / Return write error and path |
| `ConfigError` | 通用配置错误 / Generic config error | 用于自定义错误消息 / Used for custom error messages |

## 测试策略 / Testing Strategy

### 单元测试 / Unit Tests

```rust
#[tokio::test]
async fn test_load_config_from_file() {
    // 测试从文件加载配置
}

#[tokio::test]
async fn test_load_nonexistent_config() {
    // 测试文件不存在时返回默认配置
}

#[tokio::test]
async fn test_save_config() {
    // 测试保存配置
}

#[tokio::test]
async fn test_merge_configs() {
    // 测试配置合并
}
```

## Risks / Trade-offs

### 风险 1: 兼容性问题 / Risk 1: Compatibility Issues

**风险 / Risk**: Rust 序列化可能与 TypeScript 版本不完全兼容

**缓解措施 / Mitigation**:
- 使用 `serde(rename_all = "camelCase")` 确保字段名一致
- 使用 `#[serde(untagged)]` 处理联合类型
- 定期测试与 TypeScript 版本的配置文件互操作性

### 风险 2: 配置文件损坏 / Risk 2: Corrupted Configuration Files

**风险 / Risk**: 用户手动编辑配置文件可能导致格式错误

**缓解措施 / Mitigation**:
- 提供清晰的错误消息指出具体错误位置
- 在保存时使用格式化 JSON 提高可读性
- 考虑添加配置验证功能（未来）

## Open Questions

1. **是否需要配置热重载？**
   - 建议：暂不实现，用户可以重启应用
   - Rationale: Don't implement for now, users can restart the app

2. **是否需要环境变量覆盖？**
   - 建议：未来添加，用于敏感信息（如 API keys）
   - Rationale: Add in the future for sensitive information like API keys

3. **是否需要配置文件验证？**
   - 建议：在保存时验证，加载时宽松处理
   - Rationale: Validate when saving, be lenient when loading

## 参考资源 / References

- [Serde 文档](https://serde.rs/)
- [Serde camelCase 命名](https://serde.rs/attr-rename-all)
- [Tokio 文档](https://tokio.rs/)
- [directories crate](https://docs.rs/directificates/)
