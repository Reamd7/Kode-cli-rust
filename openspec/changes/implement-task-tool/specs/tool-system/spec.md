# Delta Spec: Task Tool

## ADDED Requirements

### Requirement: TaskTool / TaskTool
The system SHALL implement task tool for agent delegation.

系统应实现任务工具用于 agent 委托。

#### Scenario: Agent 委托 / Agent Delegation
- **WHEN** 执行 TaskTool 时
- **THEN** 接受 agent 参数（子 agent 名称）
- **AND** 加载指定的子 agent
- **AND** 根据 agent 定义的 tools 过滤工具集
- **AND** 创建子上下文
- **AND** 递归调用 AI
- **AND** 返回子 agent 的响应

- **WHEN** executing TaskTool
- **THEN** accepts agent parameter (sub-agent name)
- **AND** loads specified sub-agent
- **AND** filters tool set based on agent's tools definition
- **AND** creates sub-context
- **AND** recursively calls AI
- **AND** returns sub-agent response
