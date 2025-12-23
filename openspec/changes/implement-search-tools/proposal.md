# Change: 实现搜索工具 / Implement Search Tools

## Why

搜索工具允许 AI Agent 搜索文件内容和匹配文件模式，是代码理解和导航的关键能力。

Search tools allow AI agents to search file content and match file patterns, key capabilities for code understanding and navigation.

## What Changes

- 实现 `GrepTool`（基于 regex）
  - 支持正则表达式搜索
  - 支持文件类型过滤
  - 支持上下文行 (-A, -B, -C)
- 实现 `GlobTool`（使用 glob crate）
  - 支持通配符模式
  - 支持文件路径匹配

- Implement `GrepTool` (based on regex)
  - Support regex search
  - Support file type filtering
  - Support context lines (-A, -B, -C)
- Implement `GlobTool` (using glob crate)
  - Support wildcard patterns
  - Support file path matching

## Impact

**Affected specs:**
- tool-system (MODIFIED)

**Affected code:**
- `crates/kode-tools/src/grep.rs` (新建)
- `crates/kode-tools/src/glob.rs` (新建)
