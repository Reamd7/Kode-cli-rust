# Agent 系统规范 / Agent System Specification

## Purpose

Agent 系统负责加载、管理和执行 AI Agent 定义，允许用户创建专门化的 AI 助手来完成特定任务。

The Agent system is responsible for loading, managing, and executing AI Agent definitions, allowing users to create specialized AI assistants for specific tasks.
## Requirements
### Requirement: Agent 定义 / Agent Definition
The system SHALL support defining agents using Markdown with YAML frontmatter.

系统应支持使用 Markdown 和 YAML frontmatter 定义 Agent。

#### Scenario: 解析 Agent 定义 / Parse Agent Definition
- **WHEN** 读取包含 YAML frontmatter 的 Markdown 文件时
- **THEN** 解析 Agent 元数据（name、description、tools、model）
- **AND** 提取 Markdown 正文作为 system_prompt

- **WHEN** reading a Markdown file with YAML frontmatter
- **THEN** parse Agent metadata (name, description, tools, model)
- **AND** extract Markdown body as system_prompt

#### Scenario: 验证必需字段 / Validate Required Fields
- **WHEN** Agent 定义缺少必需字段时
- **THEN** 返回验证错误
- **AND** 指出缺少的字段

- **WHEN** an Agent definition is missing required fields
- **THEN** return a validation error
- **AND** indicate the missing fields

#### Scenario: 工具过滤器解析 / Tool Filter Parsing
- **WHEN** Agent 定义了 tools 字段时
- **THEN** 支持字符串 "all" 表示所有工具
- **AND** 支持字符串数组表示特定工具

- **WHEN** an Agent defines the tools field
- **THEN** support the string "all" to indicate all tools
- **AND** support an array of strings to indicate specific tools

### Requirement: Agent 加载 / Agent Loading
The system SHALL load agents from a hierarchical directory structure.

系统应从分层目录结构加载 Agent。

#### Scenario: 五层加载优先级 / Five-Layer Loading Priority
- **WHEN** 请求名为 "example" 的 Agent 时
- **THEN** 按以下顺序搜索：
  1. Built-in agents
  2. `~/.claude/agents/`
  3. `~/.kode/agents/`
  4. `./.claude/agents/`
  5. `./.kode/agents/`
- **AND** 使用第一个找到的定义

- **WHEN** requesting an Agent named "example"
- **THEN** search in the following order:
  1. Built-in agents
  2. `~/.claude/agents/`
  3. `~/.kode/agents/`
  4. `./.claude/agents/`
  5. `./.kode/agents/`
- **AND** use the first definition found

#### Scenario: 列出所有 Agent / List All Agents
- **WHEN** 请求所有可用 Agent 时
- **THEN** 扫描所有 Agent 目录
- **AND** 返回去重后的 Agent 列表

- **WHEN** requesting all available Agents
- **THEN** scan all Agent directories
- **AND** return a deduplicated list of Agents

#### Scenario: Agent 缓存 / Agent Caching
- **WHEN** 再次加载已加载的 Agent 时
- **THEN** 从缓存返回 Agent 定义
- **AND** 使用 LRU 缓存策略

- **WHEN** loading an already-loaded Agent again
- **THEN** return the Agent definition from cache
- **AND** use LRU cache strategy

### Requirement: Agent 执行 / Agent Execution
The system SHALL execute agents with their specific configuration.

系统应使用 Agent 的特定配置执行 Agent。

#### Scenario: 使用 Agent 的系统提示词 / Use Agent's System Prompt
- **WHEN** 执行 Agent 时
- **THEN** 使用 Agent 定义的 system_prompt
- **AND** 将其作为第一条消息添加到上下文

- **WHEN** executing an Agent
- **THEN** use the Agent's defined system_prompt
- **AND** add it as the first message to the context

#### Scenario: 使用 Agent 的工具集 / Use Agent's Tool Set
- **WHEN** Agent 定义了工具过滤器时
- **THEN** 仅向 Agent 暴露允许的工具
- **AND** 隐藏其他工具

- **WHEN** an Agent defines a tool filter
- **THEN** only expose allowed tools to the Agent
- **AND** hide other tools

#### Scenario: 使用 Agent 的模型 / Use Agent's Model
- **WHEN** Agent 定义了特定模型时
- **THEN** 使用指定的模型而不是默认模型

- **WHEN** an Agent defines a specific model
- **THEN** use the specified model instead of the default

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

## Reference / 参考资料

### TypeScript 版本实现参考 / TypeScript Implementation Reference

在实现本规范时，请参考原版 TypeScript 项目中的以下文件：

When implementing this specification, refer to the following files in the original TypeScript project:

#### Agent 管理模块 / Agent Management Module
- **Agent 加载与解析**: `/Users/gemini/Documents/backup/Kode-cli/src/utils/agents.ts`
  - Agent 定义解析逻辑
  - YAML frontmatter 处理
  - 五层加载优先级实现

#### Agent 目录结构 / Agent Directory Structure
- **内置 Agents**: `/Users/gemini/Documents/backup/Kode-cli/src/built-in-agents/`
- **用户 Agents**: `~/.claude/agents/` 和 `~/.kode/agents/`
- **项目 Agents**: `./.claude/agents/` 和 `./.kode/agents/`

#### Agent 类型定义 / Agent Type Definitions
- **Agent 接口**: `/Users/gemini/Documents/backup/Kode-cli/src/types/`
  - Agent 元数据结构
  - Agent 工具过滤器类型

### 实现要点 / Implementation Notes

1. **YAML 解析**: TypeScript 版本使用 frontmatter 解析 Agent 的 YAML 元数据
2. **五层加载**: 内置 -> 全局 Claude -> 全局 Kode -> 项目 Claude -> 项目 Kode
3. **工具过滤**: 支持 "all" 字符串或工具名称数组
4. **缓存策略**: 使用 LRU 缓存避免重复加载

## Non-Goals

- 本规范不包含 Agent 的热重载
- 不包含 Agent 版本控制
- 不包含 Agent 依赖管理

- This specification does not include Agent hot reloading
- Does not include Agent version control
- Does not include Agent dependency management
