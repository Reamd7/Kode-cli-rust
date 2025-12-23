# Change: 实现基础文件工具 / Implement Basic File Tools

## Why

文件操作工具是 AI Agent 的基础能力，需要实现读取、写入和编辑文件的功能。

File operation tools are foundational capabilities for AI agents, requiring read, write, and edit file functionality.

## What Changes

- 定义 `Tool` trait
- 实现 `ToolRegistry`
- 实现 `FileReadTool`（支持行号范围）
- 实现 `FileWriteTool`（创建目录、权限检查）
- 实现 `FileEditTool`（字符串替换）
- 实现工具参数验证

- Define `Tool` trait
- Implement `ToolRegistry`
- Implement `FileReadTool` (support line ranges)
- Implement `FileWriteTool` (create directories, permission checks)
- Implement `FileEditTool` (string replacement)
- Implement tool parameter validation

## Impact

**Affected specs:**
- tool-system

**Affected code:**
- `crates/kode-tools/src/tool.rs` (新建)
- `crates/kode-tools/src/registry.rs` (新建)
- `crates/kode-tools/src/file_read.rs` (新建)
- `crates/kode-tools/src/file_write.rs` (新建)
- `crates/kode-tools/src/file_edit.rs` (新建)
- `crates/kode-tools/Cargo.toml` (添加 serde_json、async-trait 依赖)
