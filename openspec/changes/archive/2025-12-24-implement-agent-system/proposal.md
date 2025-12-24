# Change: 实现 Agent 系统 / Implement Agent System

## Why

Agent 系统允许用户创建专门化的 AI 助手来完成特定任务，是 Kode 的核心功能。

Agent system allows users to create specialized AI assistants for specific tasks, a core feature of Kode.

## What Changes

- 实现 `Agent` 结构体（name, description, tools, model, system_prompt）
- 实现 `ToolFilter` 枚举（All, Specific）
- 实现 Markdown + YAML frontmatter 解析
- 实现 `AgentLoader`（5 层加载优先级）
- 实现 LRU 缓存
- 实现工具过滤逻辑

- Implement `Agent` struct (name, description, tools, model, system_prompt)
- Implement `ToolFilter` enum (All, Specific)
- Implement Markdown + YAML frontmatter parsing
- Implement `AgentLoader` (5-layer loading priority)
- Implement LRU cache
- Implement tool filtering logic

## Impact

**Affected specs:**
- agent-system

**Affected code:**
- `crates/kode-core/src/agent.rs` (新建)
- `crates/kode-core/src/agent/loader.rs` (新建)
- `crates/kode-core/Cargo.toml` (添加 pulldown-cmark, yaml-rust2, lru 依赖)
