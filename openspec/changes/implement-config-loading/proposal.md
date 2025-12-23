# Change: 实现配置加载系统 / Implement Config Loading System

## 📚 Knowledge Base / 知识库

本变更基于深入的 TypeScript 版本分析，相关知识库文档：

This change is based on in-depth analysis of the TypeScript version, relevant knowledge base documents:

- **[TypeScript 配置系统分析](./analysis/typescript-config-system.md)** - 完整分析 TS 版本 940 行代码
  - Complete analysis of TS version 940 lines of code
  - 5 层配置优先级系统 / 5-layer configuration priority system
  - 配置文件路径和结构 / Config file paths and structure

- **[配置 API 参考](./analysis/config-api-reference.md)** - 30 个公开 API 函数文档
  - 30 public API functions reference
  - 按功能分组 / Organized by functionality
  - 函数签名和说明 / Function signatures and descriptions

- **[实施计划](./analysis/implementation-plan.md)** - 迁移策略和关键差异
  - Migration strategy and key differences
  - 与之前实现的对比 / Comparison with previous implementation
  - 实施建议 / Implementation recommendations

- **[知识库索引](./analysis/README.md)** - 所有分析文档的导航和说明
  - Navigation and overview of all analysis documents

## Why / 原因

配置系统是 Kode-Rust 的基础组件，需要从文件系统加载配置并与 TypeScript 版本保持兼容。

The configuration system is a foundational component of Kode-Rust, requiring file system loading and TypeScript version compatibility.

## What Changes / 变更内容

- 实现 `Config` 结构体，支持全局配置 (~/.kode.json) 和项目配置 (./.kode.json)
- 实现 JSON 序列化/反序列化，使用 camelCase 字段命名
- 实现配置合并逻辑（项目配置覆盖全局配置）
- 实现 MCP 服务器配置（STDIO 和 SSE 传输）
- 实现 `ConfigLoader` 异步 API
- 添加单元测试和集成测试

- Implement `Config` struct supporting global config (~/.kode.json) and project config (./.kode.json)
- Implement JSON serialization/deserialization with camelCase field naming
- Implement config merge logic (project config overrides global config)
- Implement MCP server configuration (STDIO and SSE transports)
- Implement `ConfigLoader` async API
- Add unit tests and integration tests

## Impact / 影响范围

**Affected specs / 影响的规范:**
- config-loading

**Affected code / 影响的代码:**
- `crates/kode-core/src/config/mod.rs` (新建)
- `crates/kode-core/src/config/types.rs` (新建)
- `crates/kode-core/src/config/loader.rs` (新建)
- `crates/kode-core/src/config/tests.rs` (新建)
- `crates/kode-core/Cargo.toml` (添加 serde、tokio、directories 依赖)
