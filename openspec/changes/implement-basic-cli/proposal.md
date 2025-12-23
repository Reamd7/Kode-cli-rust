# Change: 实现基础 CLI 命令 / Implement Basic CLI Commands

## Why

CLI 是用户与 Kode 交互的主要方式，需要实现基本的命令行解析和非交互模式。

CLI is the primary way users interact with Kode, requiring basic command-line parsing and non-interactive mode.

## What Changes

- 使用 clap 实现命令行解析
- 实现 `kode run` 命令（非交互模式）
- 实现 `kode config` 命令组
- 实现 `kode agents` 命令
- 实现端到端流程：用户输入 → AI 响应 → 工具执行

- Use clap for command-line parsing
- Implement `kode run` command (non-interactive mode)
- Implement `kode config` command group
- Implement `kode agents` command
- Implement end-to-end flow: user input → AI response → tool execution

## Impact

**Affected specs:**
- cli-commands

**Affected code:**
- `crates/kode-cli/src/main.rs` (修改)
- `crates/kode-cli/src/cli.rs` (新建)
- `crates/kode-cli/src/commands/` (新建)
- `crates/kode-cli/Cargo.toml` (添加 clap 依赖)
