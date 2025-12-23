# Agent System Specification

Agent 系统规范 - 定义 Agent 的解析、加载和委托机制。

## Purpose

Agent 系统允许定义专用的 AI 角色，具有特定的工具访问权限和行为模式。Agent 使用 Markdown + YAML frontmatter 定义，支持从多个目录加载，可以通过 TaskTool 委托给子 Agent。Agent 格式与 TypeScript 版本和 Claude Code 完全兼容。

## ADDED Requirements

### Requirement: Agent 结构定义
The system SHALL define Agent structure with metadata and behavior configuration.

系统必须定义 Agent 结构，包含元数据和行为配置。

#### Scenario: Agent 字段定义
- **WHEN** 定义 Agent 结构
- **THEN** 必须包含 name: String
- **AND** 必须包含 description: String
- **AND** 必须包含 tools: ToolFilter
- **AND** 可选包含 model: Option<String>
- **AND** 必须包含 system_prompt: String

#### Scenario: ToolFilter 枚举
- **WHEN** 定义工具过滤器
- **THEN** 支持 ToolFilter::All（所有工具）
- **AND** 支持 ToolFilter::Specific(Vec<String>)（指定工具）
- **AND** Agent 只能使用过滤后的工具

#### Scenario: Agent 示例代码
- **WHEN** 在 Rust 代码中表示 Agent
- **THEN** 结构应符合：
```rust
pub struct Agent {
    pub name: String,
    pub description: String,
    pub tools: ToolFilter,
    pub model: Option<String>,
    pub system_prompt: String,
}

pub enum ToolFilter {
    All,
    Specific(Vec<String>),
}
```

### Requirement: Agent 定义格式
The system SHALL support Agent definitions in Markdown with YAML frontmatter.

系统必须支持使用 Markdown + YAML frontmatter 定义 Agent。

#### Scenario: 基本结构
- **WHEN** 解析 Agent 文件
- **THEN** YAML frontmatter 包含 agent 元数据
- **AND** Markdown 内容包含系统提示词

#### Scenario: 必需字段
- **WHEN** 定义 Agent
- **THEN** 必须包含 name 字段
- **AND** 必须包含 description 字段

#### Scenario: 工具过滤
- **WHEN** Agent 定义 allowedTools
- **THEN** 值为工具名称数组
- **OR** 值为 "all" 表示所有工具
- **AND** Agent 只能使用允许的工具

#### Scenario: 模型配置
- **WHEN** Agent 定义 model
- **THEN** 使用指定的模型指针或模型名称
- **AND** 未指定时使用默认模型

#### Scenario: 上下文注入
- **WHEN** Agent 定义 context
- **THEN** YAML 中的 context 字段注入到对话上下文
- **AND** 作为额外的系统信息提供

#### Scenario: Agent 定义示例
- **WHEN** 创建 Agent 文件
- **THEN** 格式应如下：
```markdown
---
name: code-reviewer
description: "Reviews code for best practices and potential issues"
tools: ["FileRead", "Grep", "Bash"]
model: "claude-sonnet"
---

You are an expert code reviewer. When reviewing code:
1. Check for security vulnerabilities
2. Look for performance issues
3. Ensure code follows best practices
...
```

### Requirement: 解析 Agent 文件
The system SHALL parse Agent definitions from Markdown files.

系统必须从 Markdown 文件解析 Agent 定义。

#### Scenario: 提取 frontmatter
- **WHEN** 解析 Agent 文件
- **THEN** 正确提取 YAML frontmatter
- **AND** 解析为结构化数据

#### Scenario: 提取提示词
- **WHEN** 解析 Agent 文件
- **THEN** Markdown 内容作为系统提示词
- **AND** 保留原始格式和换行

#### Scenario: 处理解析错误
- **WHEN** YAML frontmatter 无效
- **THEN** 返回解析错误
- **AND** 错误包含文件路径和具体问题

#### Scenario: 验证必需字段
- **WHEN** Agent 定义缺少 name
- **THEN** 返回验证错误
- **AND** 提示必需字段缺失

#### Scenario: 依赖解析库
- **WHEN** 解析 Agent 文件
- **THEN** 使用 pulldown-cmark 解析 Markdown
- **AND** 使用 yaml-rust2 解析 YAML frontmatter

### Requirement: Agent 加载优先级
The system SHALL load Agents from multiple directories with priority.

系统必须从多个目录按优先级加载 Agent。

#### Scenario: 加载优先级顺序
- **WHEN** 查找 Agent
- **THEN** 按以下顺序查找：
  1. 内置 Agent（代码中定义）
  2. ~/.claude/agents/
  3. ~/.kode/agents/
  4. ./.claude/agents/
  5. ./.kode/agents/
- **AND** 第一个找到的 Agent 被使用

#### Scenario: 项目 Agent 优先
- **WHEN** 项目目录和全局目录有同名 Agent
- **THEN** 项目目录的 Agent 优先
- **AND** 全局 Agent 被覆盖

#### Scenario: Claude 兼容性
- **WHEN** 从 ~/.claude/agents/ 加载
- **THEN** 支持加载 Claude Code 的 Agent
- **AND** 格式兼容

#### Scenario: AgentLoader 接口
- **WHEN** 定义 AgentLoader
- **THEN** 结构应包含：
```rust
pub struct AgentLoader {
    cache: LruCache<PathBuf, Agent>,
    directories: Vec<PathBuf>,
}

impl AgentLoader {
    pub async fn load_agents(&mut self) -> Result<Vec<Agent>>;
    pub async fn get_agent(&mut self, name: &str) -> Result<Agent>;
}
```

### Requirement: Agent 缓存
The system SHALL cache loaded Agents for performance.

系统必须缓存已加载的 Agent 以提高性能。

#### Scenario: LRU 缓存
- **WHEN** 加载 Agent
- **THEN** 使用 LRU 缓存存储解析的 Agent
- **AND** 缓存大小可配置
- **AND** 使用 lru crate 实现

#### Scenario: 缓存命中
- **WHEN** 再次请求已缓存的 Agent
- **THEN** 直接从缓存返回
- **AND** 不重新解析文件

#### Scenario: 缓存失效
- **WHEN** Agent 文件被修改
- **THEN** 下次加载时重新解析
- **AND** 缓存使用文件修改时间检测变化

#### Scenario: 性能目标
- **WHEN** 从缓存加载 Agent
- **THEN** 加载时间 < 20ms
- **WHEN** 未缓存加载 Agent
- **THEN** 加载时间 < 100ms

### Requirement: TaskTool 委托
The system SHALL support TaskTool for delegating to sub-Agents.

系统必须支持 TaskTool 用于委托给子 Agent。

#### Scenario: 加载子 Agent
- **WHEN** TaskTool 指定子 Agent 名称
- **THEN** 从 Agent 加载器获取子 Agent
- **AND** 使用子 Agent 的提示词和工具过滤

#### Scenario: 创建子上下文
- **WHEN** 委托给子 Agent
- **THEN** 创建独立的执行上下文
- **AND** 子上下文继承父上下文的配置

#### Scenario: 传递工具结果
- **WHEN** 子 Agent 完成
- **THEN** 结果返回给父 Agent
- **AND** 格式化为工具调用结果

#### Scenario: 嵌套委托
- **WHEN** 子 Agent 调用 TaskTool
- **THEN** 支持多层嵌套委托
- **AND** 每层有独立的工具过滤

#### Scenario: 委托流程
- **WHEN** Main Agent 收到 TaskTool 调用
- **THEN** 按以下流程执行：
  1. Load Sub-Agent config
  2. Filter tools by agent config
  3. Create sub-context with system prompt
  4. Model Adapter (with filtered tools)
  5. Sub-agent responds
  6. Return result to main agent
  7. Main agent continues

### Requirement: Agent 元数据
The system SHALL expose Agent metadata for discovery.

系统必须暴露 Agent 元数据用于发现。

#### Scenario: 列出可用 Agent
- **WHEN** 请求 Agent 列表
- **THEN** 返回所有已加载的 Agent
- **AND** 包含名称和描述

#### Scenario: Agent 详情
- **WHEN** 查询特定 Agent
- **THEN** 返回完整的元数据
- **AND** 包含允许的工具列表

#### Scenario: Agent 搜索
- **WHEN** 搜索 Agent
- **THEN** 支持按名称或描述搜索
- **AND** 返回匹配的 Agent 列表

### Requirement: 格式兼容性
The system SHALL maintain compatibility with TypeScript and Claude Code Agent formats.

系统必须与 TypeScript 和 Claude Code 的 Agent 格式保持兼容。

#### Scenario: TS 版本 Agent 可加载
- **WHEN** 加载 TS 版本生成的 Agent 文件
- **THEN** Agent 成功解析
- **AND** 所有字段正确映射

#### Scenario: Claude Code Agent 可加载
- **WHEN** 加载 Claude Code 的 Agent 文件
- **THEN** Agent 成功解析
- **AND** 格式兼容

#### Scenario: Rust 版本 Agent 可被 TS 读取
- **WHEN** Rust 版本创建 Agent 文件
- **THEN** TS 版本可以读取该 Agent
- **AND** 格式完全兼容

## Non-Goals

本规范不包含：
- Agent 之间的消息传递（未来扩展）
- Agent 的版本管理和迁移
- 动态 Agent 生成
- Agent 学习和适应
- Agent 的组合和链接（未来扩展）
- Agent 的权限系统独立于工具权限（未来扩展）
