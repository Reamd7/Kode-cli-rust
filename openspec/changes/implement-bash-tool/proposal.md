# Change: 实现 Bash 工具 / Implement Bash Tool

## Why

Bash 工具允许 AI Agent 执行 shell 命令，是实现自动化任务的关键能力。

Bash tool allows AI agents to execute shell commands, a key capability for automation tasks.

## What Changes

- 实现 `BashTool`
- 使用 tokio::process::Command 执行命令
- 实现超时控制
- 捕获 stdout 和 stderr
- 实现权限检查（危险命令）

- Implement `BashTool`
- Use tokio::process::Command to execute commands
- Implement timeout control
- Capture stdout and stderr
- Implement permission checks (dangerous commands)

## Impact

**Affected specs:**
- tool-system (MODIFIED)

**Affected code:**
- `crates/kode-tools/src/bash.rs` (新建)
