# Change: 实现配置加载系统 / Implement Config Loading System

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
