# Change: 实现 Task 工具 / Implement Task Tool

## Why

Task 工具允许 Agent 委托任务给子 Agent，是实现复杂任务分解的关键能力。

Task tool allows agents to delegate tasks to sub-agents, a key capability for complex task decomposition.

## What Changes

- 实现 `TaskTool`
- 加载指定的子 Agent
- 过滤工具集（基于 Agent 定义）
- 创建子上下文
- 递归调用 AI
- 返回子 Agent 的响应

- Implement `TaskTool`
- Load specified sub-agent
- Filter tool set (based on agent definition)
- Create sub-context
- Recursively call AI
- Return sub-agent response

## Impact

**Affected specs:**
- tool-system (MODIFIED)
- agent-system (MODIFIED)

**Affected code:**
- `crates/kode-tools/src/task.rs` (新建)
