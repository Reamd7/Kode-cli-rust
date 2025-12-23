# 实现任务 - 配置加载系统 / Implementation Tasks - Config Loading System

## 1. 配置结构定义 / Configuration Structure Definition

- [ ] 1.1 定义 `GlobalConfig` 结构体
  - [ ] 添加 theme 字段
  - [ ] 添加 model_profiles 字段
  - [ ] 添加 model_pointers 字段
  - [ ] 添加 default_model 字段
  - [ ] 添加 mcp_servers 字段
  - [ ] 添加 verbose 字段
  - [ ] 添加 auto_updater_status 字段
  - [ ] 添加 serde 序列化支持

- [ ] 1.2 定义 `ProjectConfig` 结构体
  - [ ] 添加 allowed_tools 字段
  - [ ] 添加 allowed_commands 字段
  - [ ] 添加 allowed_paths 字段
  - [ ] 添加 enable_architect_tool 字段
  - [ ] 添加 mcp_servers 字段
  - [ ] 添加 context 字段
  - [ ] 添加 serde 序列化支持

- [ ] 1.3 定义 `Config` 合并结构
  - [ ] 包含 global 配置
  - [ ] 包含 project 配置

- [ ] 1.4 定义 MCP 服务器配置
  - [ ] 定义 `McpServerConfig` 枚举 (untagged)
  - [ ] 定义 `McpStdioServerConfig` 结构
  - [ ] 定义 `McpSseServerConfig` 结构

## 2. 配置加载器实现 / Config Loader Implementation

- [ ] 2.1 实现 `ConfigLoader` 结构
  - [ ] 添加 cache 字段 (LRU)
  - [ ] 添加 directories 字段

- [ ] 2.2 实现全局配置加载
  - [ ] 实现 `load_global()` 方法
  - [ ] 处理文件不存在情况（返回默认配置）
  - [ ] 处理 JSON 解析错误
  - [ ] 使用 directories crate 获取配置路径

- [ ] 2.3 实现项目配置加载
  - [ ] 实现 `load_current_project()` 方法
  - [ ] 从当前目录向上查找 .kode.json
  - [ ] 处理文件不存在情况

- [ ] 2.4 实现配置合并
  - [ ] 实现 `merge()` 方法
  - [ ] 项目配置覆盖全局配置
  - [ ] MCP 服务器配置合并

- [ ] 2.5 实现配置保存
  - [ ] 实现 `save_global()` 方法
  - [ ] 实现 `save_project()` 方法
  - [ ] 使用格式化 JSON 提高可读性

## 3. 默认配置 / Default Configuration

- [ ] 3.1 实现 `Default for GlobalConfig`
  - [ ] 默认主题: dark
  - [ ] 空的 model_profiles
  - [ ] 默认 model_pointers
  - [ ] 其他默认值

- [ ] 3.2 实现 `Default for ProjectConfig`
  - [ ] 默认允许所有工具
  - [ ] 其他默认值

## 4. 错误处理 / Error Handling

- [ ] 4.1 定义错误类型
  - [ ] `ConfigLoadError` - 文件读取失败
  - [ ] `ConfigParseError` - JSON 解析失败
  - [ ] `ConfigSaveError` - 文件写入失败
  - [ ] `ConfigError` - 通用错误

- [ ] 4.2 实现错误上下文
  - [ ] 使用 anyhow::Context
  - [ ] 提供详细错误信息

## 5. 测试 / Testing

- [ ] 5.1 单元测试
  - [ ] 测试默认配置
  - [ ] 测试配置加载（文件存在）
  - [ ] 测试配置加载（文件不存在）
  - [ ] 测试 JSON 解析（有效配置）
  - [ ] 测试 JSON 解析（无效配置）
  - [ ] 测试配置合并
  - [ ] 测试 MCP 服务器配置解析
  - [ ] 测试 camelCase 序列化
  - [ ] 测试配置保存

- [ ] 5.2 集成测试
  - [ ] 测试完整加载流程
  - [ ] 测试与 TypeScript 版本配置兼容性

## 6. 文档 / Documentation

- [ ] 6.1 Rustdoc 注释
  - [ ] 所有公开结构体
  - [ ] 所有公开方法
  - [ ] 添加示例代码

- [ ] 6.2 验证
  - [ ] `cargo fmt --check`
  - [ ] `cargo clippy -- -D warnings`
  - [ ] `cargo test`
  - [ ] `cargo doc --no-deps`
