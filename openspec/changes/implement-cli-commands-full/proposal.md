# Change: 实现 CLI 命令完善 / Implement CLI Commands Full

## Why

完整的 CLI 命令提供更好的用户体验，允许用户通过命令行管理配置和查看信息。

Complete CLI commands provide better user experience, allowing users to manage config and view info via command line.

## What Changes

- 实现 `kode config` 命令组
  - `config list`: 显示配置
  - `config set`: 设置配置
  - `config get`: 获取配置
- 实现 `kode agents` 命令
  - 列出所有可用 Agent
  - 显示 Agent 详情
- 实现 `kode tools` 命令
  - 列出所有工具
  - 显示工具详情
- 完善 `--help` 信息

- Implement `kode config` command group
  - `config list`: Show config
  - `config set`: Set config value
  - `config get`: Get config value
- Implement `kode agents` command
  - List all available agents
  - Show agent details
- Implement `kode tools` command
  - List all tools
  - Show tool details
- Improve `--help` information

## Impact

**Affected specs:**
- cli-commands (MODIFIED)

**Affected code:**
- `crates/kode-cli/src/commands/config.rs` (新建)
- `crates/kode-cli/src/commands/agents.rs` (新建)
- `crates/kode-cli/src/commands/tools.rs` (新建)
