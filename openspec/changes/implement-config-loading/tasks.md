# 实施任务 - 配置加载系统 / Implementation Tasks - Config Loading System

**基于 TypeScript 版本完整实现（940 行，30 个 API 函数）**

**Based on TypeScript version complete implementation (940 lines, 30 API functions)**

---

## 📚 Knowledge Base / 知识库

本实施任务基于以下分析文档，请参考这些文档了解详细技术细节：

This implementation task is based on the following analysis documents, refer to them for detailed technical information:

- **[TypeScript 配置系统分析](./analysis/typescript-config-system.md)** - TS 版本完整架构分析
- **[配置 API 参考](./analysis/config-api-reference.md)** - 30 个 API 函数详细列表
- **[实施计划](./analysis/implementation-plan.md)** - 迁移策略和关键差异
- **[知识库索引](./analysis/README.md)** - 所有分析文档的导航

### 关键参考资料 / Key Reference Materials

- **TypeScript 源码**: `/Users/gemini/Documents/backup/Kode-cli/src/utils/config.ts` (940 行)
- **设计文档**: `../../specs/config-loading/design.md`
- **功能规范**: `../../specs/config-loading/spec.md`

---

## 1. 类型定义 / Type Definitions

### 1.1 基础类型 / Basic Types

- [ ] 定义 `NotificationChannel` 枚举
  - [ ] Iterm2
  - [ ] TerminalBell
  - [ ] Iterm2WithBell
  - [ ] NotificationsDisabled
  - [ ] 添加 serde 支持 (snake_case)
  - [ ] 实现 Default trait

- [ ] 定义 `AutoUpdaterStatus` 枚举
  - [ ] Disabled
  - [ ] Enabled
  - [ ] NoPermissions
  - [ ] NotConfigured
  - [ ] 添加 serde 支持 (lowercase)

- [ ] 定义 `AccountInfo` 结构
  - [ ] account_uuid: String
  - [ ] email_address: String
  - [ ] organization_uuid: Option<String>
  - [ ] 添加 serde 支持 (camelCase)

- [ ] 定义 `CustomApiKeyResponses` 结构
  - [ ] approved: Option<Vec<String>>
  - [ ] rejected: Option<Vec<String>>
  - [ ] 添加 serde 支持 (camelCase)
  - [ ] 实现 Default trait

### 1.2 MCP 类型 / MCP Types

- [ ] 定义 `McpStdioServerConfig` 结构
  - [ ] command: String
  - [ ] args: Vec<String>
  - [ ] env: Option<HashMap<String, String>>
  - [ ] 添加 serde 支持 (camelCase)

- [ ] 定义 `McpSseServerConfig` 结构
  - [ ] url: String
  - [ ] 添加 serde 支持 (camelCase)

- [ ] 定义 `McpServerConfig` 枚举 (untagged)
  - [ ] Stdio(McpStdioServerConfig)
  - [ ] Sse(McpSseServerConfig)
  - [ ] 添加 serde 支持 (camelCase, untagged)

### 1.3 模型类型 / Model Types

- [ ] 定义 `ProviderType` 枚举 (20 个提供商)
  - [ ] Anthropic, Openai
  - [ ] Mistral, Deepseek, Kimi, Qwen
  - [ ] Glm, Minimax, BaiduQianfan
  - [ ] Siliconflow, Bigdream, Opendev
  - [ ] Xai, Groq, Gemini
  - [ ] Ollama, Azure
  - [ ] Custom, CustomOpenai
  - [ ] 添加 serde 支持 (lowercase)

- [ ] 定义 `ModelProfile` 结构 (14 个字段)
  - [ ] name: String
  - [ ] provider: ProviderType
  - [ ] model_name: String
  - [ ] base_url: Option<String>
  - [ ] api_key: String
  - [ ] max_tokens: u32
  - [ ] context_length: u32
  - [ ] reasoning_effort: Option<String>
  - [ ] is_active: bool
  - [ ] created_at: u64
  - [ ] last_used: Option<u64>
  - [ ] is_gpt5: Option<bool>
  - [ ] validation_status: Option<String>
  - [ ] last_validation: Option<u64>
  - [ ] 添加 serde 支持 (camelCase)

- [ ] 定义 `ModelPointerType` 类型别名
  - [ ] "main" | "task" | "reasoning" | "quick"

- [ ] 定义 `ModelPointers` 结构
  - [ ] main: Option<String>
  - [ ] task: Option<String>
  - [ ] reasoning: Option<String>
  - [ ] quick: Option<String>
  - [ ] 添加 serde 支持 (camelCase)
  - [ ] 实现 Default trait

### 1.4 配置类型 / Configuration Types

- [ ] 定义 `ProjectConfig` 结构 (19 个字段)
  - [ ] allowed_tools: Vec<String>
  - [ ] context: HashMap<String, String>
  - [ ] context_files: Option<Vec<String>>
  - [ ] history: Vec<String>
  - [ ] dont_crawl_directory: Option<bool>
  - [ ] enable_architect_tool: Option<bool>
  - [ ] mcp_context_uris: Vec<String>
  - [ ] mcp_servers: Option<HashMap<String, McpServerConfig>>
  - [ ] approved_mcprc_servers: Option<Vec<String>>
  - [ ] rejected_mcprc_servers: Option<Vec<String>>
  - [ ] last_api_duration: Option<u64>
  - [ ] last_cost: Option<f64>
  - [ ] last_duration: Option<u64>
  - [ ] last_session_id: Option<String>
  - [ ] example_files: Option<Vec<String>>
  - [ ] example_files_generated_at: Option<u64>
  - [ ] has_trust_dialog_accepted: Option<bool>
  - [ ] has_completed_project_onboarding: Option<bool>
  - [ ] 添加 serde 支持 (camelCase)
  - [ ] 实现 Default trait

- [ ] 定义 `GlobalConfig` 结构 (24 个字段)
  - [ ] projects: Option<HashMap<String, ProjectConfig>>
  - [ ] num_startups: u32
  - [ ] auto_updater_status: Option<AutoUpdaterStatus>
  - [ ] user_id: Option<String>
  - [ ] theme: Option<String>
  - [ ] has_completed_onboarding: Option<bool>
  - [ ] last_onboarding_version: Option<String>
  - [ ] last_release_notes_seen: Option<String>
  - [ ] mcp_servers: Option<HashMap<String, McpServerConfig>>
  - [ ] preferred_notif_channel: NotificationChannel
  - [ ] verbose: bool
  - [ ] custom_api_key_responses: Option<CustomApiKeyResponses>
  - [ ] primary_provider: Option<ProviderType>
  - [ ] max_tokens: Option<u32>
  - [ ] has_acknowledged_cost_threshold: Option<bool>
  - [ ] oauth_account: Option<AccountInfo>
  - [ ] iterm2_key_binding_installed: Option<bool>
  - [ ] shift_enter_key_binding_installed: Option<bool>
  - [ ] proxy: Option<String>
  - [ ] stream: bool
  - [ ] model_profiles: Option<Vec<ModelProfile>>
  - [ ] model_pointers: Option<ModelPointers>
  - [ ] default_model_name: Option<String>
  - [ ] last_dismissed_update_version: Option<String>
  - [ ] 添加 serde 支持 (camelCase)
  - [ ] 实现 Default trait

---

## 2. 环境变量 / Environment Variables

### 2.1 环境变量模块 / Environment Variables Module

- [ ] 创建 `env.rs` 模块
  - [ ] 定义常量 `GLOBAL_CLAUDE_FILE` 路径逻辑
  - [ ] 支持环境变量覆盖（KODE_CONFIG_DIR, CLAUDE_CONFIG_DIR）
  - [ ] 定义 `CLAUDE_BASE_DIR`

- [ ] 实现 `get_config_file_path()` 函数
  - [ ] 检查 KODE_CONFIG_DIR 环境变量
  - [ ] 检查 CLAUDE_CONFIG_DIR 环境变量
  - [ ] 返回配置文件路径 (默认 ~/.kode.json)
  - [ ] 处理错误情况

- [ ] 实现 `get_openai_api_key()` 函数
  - [ ] 读取 OPENAI_API_KEY 环境变量
  - [ ] 返回 Option<String>
  - [ ] 添加单元测试

- [ ] 实现 `get_anthropic_api_key()` 函数
  - [ ] 读取 ANTHROPIC_API_KEY 环境变量
  - [ ] 返回 String (空字符串作为默认值)
  - [ ] 添加单元测试

---

## 3. 核心配置 / Core Configuration

### 3.1 配置加载器 / Configuration Loader

- [ ] 实现 `get_global_config()` 函数
  - [ ] 读取配置文件路径（考虑环境变量）
  - [ ] 读取配置文件内容
  - [ ] 文件不存在时返回默认配置
  - [ ] JSON 解析失败时返回默认配置
  - [ ] 应用配置迁移逻辑
  - [ ] 返回 GlobalConfig
  - [ ] 添加单元测试

- [ ] 实现 `get_current_project_config()` 函数
  - [ ] 调用 get_global_config()
  - [ ] 获取当前目录的绝对路径
  - [ ] 在 GlobalConfig.projects 中查找
  - [ ] 找到则返回项目配置
  - [ ] 未找到则返回默认项目配置
  - [ ] 添加单元测试

- [ ] 实现 `save_global_config()` 函数
  - [ ] 过滤掉与默认值相同的字段
  - [ ] 序列化为 JSON（2 空格缩进）
  - [ ] 创建配置目录（如不存在）
  - [ ] 写入文件
  - [ ] 处理权限错误（EACCES, EPERM, EROFS）
  - [ ] 添加单元测试

- [ ] 实现 `save_current_project_config()` 函数
  - [ ] 加载全局配置
  - [ ] 获取当前目录的绝对路径
  - [ ] 更新 GlobalConfig.projects 字段
  - [ ] 调用 save_global_config()
  - [ ] 添加单元测试

### 3.2 配置文件操作 / Configuration File Operations

- [ ] 实现 `save_config()` 通用函数
  - [ ] 接受配置对象和路径
  - [ ] 过滤默认值字段
  - [ ] 保存到文件
  - [ ] 错误处理

- [ ] 实现 `load_config()` 通用函数
  - [ ] 从指定路径加载
  - [ ] 处理文件不存在
  - [ ] 处理解析错误
  - [ ] 返回配置或默认值

---

## 4. 配置迁移 / Configuration Migration

### 4.1 模型配置迁移 / Model Configuration Migration

- [ ] 实现 `migrate_model_profiles()` 函数
  - [ ] 移除 ModelProfile.id 字段
  - [ ] 构建 id 到 modelName 的映射
  - [ ] 更新 modelPointers 从 id 改为 modelName
  - [ ] 移除 defaultModelId 等废弃字段
  - [ ] 迁移 defaultModelName 字段
  - [ ] 添加单元测试

---

## 5. 模型系统 / Model System

### 5.1 模型指针管理 / Model Pointer Management

- [ ] 实现 `set_all_pointers_to_model()` 函数
  - [ ] 加载全局配置
  - [ ] 设置所有指针（main, task, reasoning, quick）为指定模型
  - [ ] 更新 default_model_name
  - [ ] 保存配置
  - [ ] 添加单元测试

- [ ] 实现 `set_model_pointer()` 函数
  - [ ] 加载全局配置
  - [ ] 更新指定的模型指针
  - [ ] 保存配置
  - [ ] 触发 ModelManager 重新加载（如果需要）
  - [ ] 添加单元测试

---

## 6. CLI 工具 / CLI Tools

### 6.1 CLI 配置操作 / CLI Configuration Operations

- [ ] 定义 `GLOBAL_CONFIG_KEYS` 常量数组
  - [ ] 列出所有可修改的全局配置键

- [ ] 定义 `PROJECT_CONFIG_KEYS` 常量数组
  - [ ] 列出所有可修改的项目配置键

- [ ] 实现 `is_global_config_key()` 函数
  - [ ] 验证键是否在 GLOBAL_CONFIG_KEYS 中
  - [ ] 返回 bool

- [ ] 实现 `is_project_config_key()` 函数
  - [ ] 验证键是否在 PROJECT_CONFIG_KEYS 中
  - [ ] 返回 bool

- [ ] 实现 `get_config_for_cli()` 函数
  - [ ] 支持 global 参数
  - [ ] 验证配置键
  - [ ] 获取并返回配置值
  - [ ] 错误处理并退出进程

- [ ] 实现 `set_config_for_cli()` 函数
  - [ ] 支持 global 参数
  - [ ] 验证配置键
  - [ ] 验证配置值（如 autoUpdaterStatus）
  - [ ] 更新配置
  - [ ] 保存配置
  - [ ] 延迟退出以刷新输出

- [ ] 实现 `delete_config_for_cli()` 函数
  - [ ] 支持 global 参数
  - [ ] 验证配置键
  - [ ] 删除配置字段
  - [ ] 保存配置

- [ ] 实现 `list_config_for_cli()` 函数
  - [ ] 支持 global 参数
  - [ ] 使用 pick 过滤配置
  - [ ] 返回过滤后的配置

---

## 7. 配置验证 / Configuration Validation

- [ ] 实现 `is_auto_updater_disabled()` 函数
  - [ ] 加载全局配置
  - [ ] 检查 auto_updater_status 是否为 "disabled"
  - [ ] 返回 bool
  - [ ] 添加单元测试

- [ ] 实现 `check_has_trust_dialog_accepted()` 函数
  - [ ] 获取当前目录
  - [ ] 遍历目录树向上查找
  - [ ] 检查每个项目的 has_trust_dialog_accepted
  - [ ] 找到返回 true，否则返回 false

---

## 8. 工具函数 / Utility Functions

### 8.1 配置工具 / Configuration Utilities

- [ ] 实现 `normalize_api_key()` 函数
  - [ ] 截取 API Key 的最后 20 个字符
  - [ ] 用于安全显示
  - [ ] 添加单元测试

- [ ] 实现 `get_custom_api_key_status()` 函数
  - [ ] 加载全局配置
  - [ ] 检查 custom_api_key_responses
  - [ ] 返回 'approved' | 'rejected' | 'new'
  - [ ] 添加单元测试

- [ ] 实现 `get_or_create_user_id()` 函数
  - [ ] 加载全局配置
  - [ ] 如果 user_id 存在则返回
  - [ ] 否则生成随机 user_id (32 字节 hex)
  - [ ] 保存更新后的配置
  - [ ] 返回 user_id

- [ ] 实现 `enable_configs()` 函数
  - [ ] 设置 config_reading_allowed 标志
  - [ ] 首次调用时验证配置文件
  - [ ] 抛出 ConfigParseError 如果配置无效（仅首次）

---

## 9. GPT-5 支持 / GPT-5 Support

### 9.1 GPT-5 检测和验证 / GPT-5 Detection and Validation

- [ ] 实现 `is_gpt5_model_name()` 函数
  - [ ] 检查模型名称（小写转换）
  - [ ] 判断是否以 "gpt-5" 开头或包含 "gpt-5"
  - [ ] 返回 bool
  - [ ] 添加单元测试

- [ ] 实现 `validate_and_repair_gpt5_profile()` 函数
  - [ ] 检查模型名称是否为 GPT-5
  - [ ] 设置 is_gpt5 标志
  - [ ] 验证 reasoning_effort (默认 "medium")
  - [ ] 验证 context_length (最小 128000)
  - [ ] 验证 max_tokens (最小 4000，GPT-5-mini 4096，nano 2048)
  - [ ] 验证 provider (应为 openai/custom-openai/azure)
  - [ ] 设置默认 base_url (如需要)
  - [ ] 更新 validation_status 和 last_validation
  - [ ] 返回修复后的配置
  - [ ] 添加单元测试

- [ ] 实现 `validate_and_repair_all_gpt5_profiles()` 函数
  - [ ] 加载全局配置
  - [ ] 遍历所有 model_profiles
  - [ ] 对每个配置调用 validate_and_repair_gpt5_profile
  - [ ] 统计修复数量
  - [ ] 如有修复则保存配置
  - [ ] 返回修复统计
  - [ ] 添加单元测试

- [ ] 实现 `get_gpt5_config_recommendations()` 函数
  - [ ] 检查是否为 GPT-5 模型
  - [ ] 返回推荐配置
    - context_length: 128000
    - max_tokens: 8192 (mini: 4096, nano: 2048)
    - reasoning_effort: "medium" (mini: "low", nano: "minimal")
    - is_gpt5: true
  - [ ] 添加单元测试

- [ ] 实现 `create_gpt5_model_profile()` 函数
  - [ ] 接受参数：name, model_name, api_key, base_url, provider
  - [ ] 调用 get_gpt5_config_recommendations
  - [ ] 创建 ModelProfile
  - [ ] 设置默认值和验证状态
  - [ ] 返回配置
  - [ ] 添加单元测试

---

## 10. MCP 支持 / MCP Support

### 10.1 .mcprc 文件支持 / .mcprc File Support

- [ ] 实现 `get_mcprc_config()` 函数
  - [ ] 构建当前目录的 .mcprc 路径
  - [ ] 检查文件是否存在
  - [ ] 读取文件内容
  - [ ] 解析 JSON
  - [ ] 返回配置或空 HashMap
  - [ ] 使用 memoize 优化（基于 cwd 和文件内容）
  - [ ] 添加单元测试

- [ ] 实现 `clear_mcprc_config_for_testing()` 函数
  - [ ] 仅在测试环境运行
  - [ ] 清空测试配置
  - [ ] 添加单元测试

- [ ] 实现 `add_mcprc_server_for_testing()` 函数
  - [ ] 仅在测试环境运行
  - [ ] 添加服务器配置
  - [ ] 添加单元测试

- [ ] 实现 `remove_mcprc_server_for_testing()` 函数
  - [ ] 仅在测试环境运行
  - [ ] 验证服务器存在
  - [ ] 删除服务器配置
  - [ ] 添加单元测试

---

## 11. 错误处理 / Error Handling

- [x] 定义 `ConfigError` 枚举
  - [x] ConfigLoadError { path, message }
  - [x] ConfigParseError { path, message }
  - [x] ConfigSaveError { path, message }
  - [x] ConfigError(String)

- [x] 实现 thiserror 支持
  - [x] 为所有错误变体添加错误消息

---

## 12. 测试 / Testing

### 12.1 单元测试 / Unit Tests

- [ ] 测试配置加载（文件存在）
- [ ] 测试配置加载（文件不存在）
- [ ] 测试配置加载（JSON 解析失败）
- [ ] 测试配置保存
- [ ] 测试配置保存（权限错误）
- [ ] 测试项目配置加载
- [ ] 测试项目配置保存
- [ ] 测试环境变量覆盖
- [ ] 测试配置迁移逻辑
- [ ] 测试所有 30 个 API 函数

### 12.2 集成测试 / Integration Tests

- [ ] 测试与 TypeScript 版本配置文件兼容性
- [ ] 测试配置优先级（环境变量 > CLI > 项目 > 全局 > 默认）
- [ ] 测试完整的加载和保存流程

---

## 13. 文档 / Documentation

- [ ] 为所有公开结构体添加 rustdoc 注释
- [ ] 为所有公开函数添加 rustdoc 注释
- [ ] 为所有公开字段添加 rustdoc 注释
- [ ] 添加使用示例

---

## 14. 验证 / Verification

- [ ] `cargo fmt --check` - 代码格式化检查
- [ ] `cargo clippy -- -D warnings` - Clippy 检查（无警告）
- [ ] `cargo test` - 所有测试通过
- [ ] `cargo doc --no-deps` - 文档生成

---

## 完成标准 / Completion Criteria

本变更实施完成的标准：

1. ✅ 所有 30 个 API 函数已实现
2. ✅ 所有类型定义已完成
3. ✅ 所有单元测试通过
4. ✅ 所有集成测试通过
5. ✅ 无 Clippy 警告
6. ✅ 代码格式化正确
7. ✅ 文档完整
8. ✅ 与 TypeScript 版本配置文件兼容

---

## 实施注意事项 / Implementation Notes

1. **严格遵循 TypeScript 实现**: 所有 API 函数的行为应与 TS 版本一致
2. **配置优先级**: 确保环境变量 > CLI > 项目 > 全局 > 默认值
3. **错误容忍**: 文件不存在或解析失败时返回默认配置，不抛错
4. **保存优化**: 只保存与默认值不同的字段
5. **camelCase 兼容**: 所有 JSON 字段使用 camelCase
6. **异步优先**: 所有文件操作使用 Tokio 异步 API
7. **测试覆盖**: 每个 API 函数都应有单元测试

---

**参考实现**: `/Users/gemini/Documents/backup/Kode-cli/src/utils/config.ts` (940 行)
