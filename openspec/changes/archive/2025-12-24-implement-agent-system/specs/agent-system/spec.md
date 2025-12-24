# Delta Spec: Agent System

## ADDED Requirements

### Requirement: Agent 结构 / Agent Structure
The system SHALL define agent structure for specialized AI assistants.

系统应定义用于专门化 AI 助手的 agent 结构。

#### Scenario: Agent 定义 / Agent Definition
- **WHEN** 定义 Agent 结构体时
- **THEN** 包含 name 字段
- **AND** 包含 description 字段
- **AND** 包含 tools 字段 (ToolFilter)
- **AND** 包含可选的 model 字段
- **AND** 包含 system_prompt 字段

- **WHEN** defining Agent struct
- **THEN** includes name field
- **AND** includes description field
- **AND** includes tools field (ToolFilter)
- **AND** includes optional model field
- **AND** includes system_prompt field

### Requirement: ToolFilter / ToolFilter
The system SHALL define tool filter for agent tool access control.

系统应定义 tool filter 用于 agent 工具访问控制。

#### Scenario: 工具过滤 / Tool Filtering
- **WHEN** 定义 ToolFilter 枚举时
- **THEN** 包含 All 变体（访问所有工具）
- **AND** 包含 Specific(Vec<String>) 变体（访问指定工具）

- **WHEN** defining ToolFilter enum
- **THEN** includes All variant (access all tools)
- **AND** includes Specific(Vec<String>) variant (access specified tools)

### Requirement: AgentLoader / AgentLoader
The system SHALL implement agent loader with 5-layer priority.

系统应实现 5 层优先级的 agent 加载器。

#### Scenario: 5 层加载 / 5-Layer Loading
- **WHEN** 加载 agents 时
- **THEN** 按优先级搜索以下目录：
  1. Built-in agents
  2. ~/.claude/agents/
  3. ~/.kode/agents/
  4. ./.claude/agents/
  5. ./.kode/agents/
- **AND** 使用 LRU 缓存
- **AND** 解析 Markdown + YAML frontmatter

- **WHEN** loading agents
- **THEN** searches directories in priority order:
  1. Built-in agents
  2. ~/.claude/agents/
  3. ~/.kode/agents/
  4. ./.claude/agents/
  5. ./.kode/agents/
- **AND** uses LRU cache
- **AND** parses Markdown + YAML frontmatter
