# 设计文档 - 文档迁移到 OpenSpec

## Context

### 当前状态

项目目前使用以下 Markdown 文档管理开发：

| 文档 | 用途 | 大小 |
|------|------|------|
| `TODO.md` | 任务清单，跟踪开发进度 | ~16KB |
| `ROADMAP.md` | 10 周开发路线图 | ~17KB |
| `ARCHITECTURE.md` | 系统架构设计 | ~16KB |
| `TECH_STACK.md` | 技术选型说明 | ~9KB |
| `WORKFLOW.md` | 开发工作流程 | ~13KB |
| `CLAUDE.md` | AI Agent 工作指南 | ~11KB |

### 问题

1. **手动维护困难** - 需求变更需要手动同步多个文档
2. **缺乏验证** - 无法自动检查文档与代码的一致性
3. **进度跟踪不精确** - TODO.md 的 checkbox 容易过时
4. **归档流程缺失** - 完成的功能没有标准化的归档方式

### OpenSpec 解决方案

OpenSpec 提供了：
- **结构化需求** - Requirements + Scenarios 格式
- **自动验证** - `openspec validate` 检查格式
- **变更追踪** - Proposal → Apply → Archive 流程
- **需求映射** - 清晰的 spec → code 关系

## Goals / Non-Goals

### Goals

- [x] 将现有规划内容转换为 OpenSpec 规范
- [x] 建立标准化的变更提案流程
- [x] 保留历史文档作为参考
- [x] 更新工作流以使用 OpenSpec

### Non-Goals

- [ ] 修改现有代码实现
- [ ] 改变项目架构设计
- [ ] 重写技术栈选型说明

## Decisions

### 决策 1: 双语规范格式

**选择**: 使用双语格式（英文结构 + 中文内容）

**理由**:
- OpenSpec 验证器要求英文结构标题（`## Purpose`, `## Requirements`, etc.）
- 项目是中文主导，需求描述使用中文便于理解
- `openspec/AGENTS.md` 明确要求双语格式

**示例**:
```markdown
## Purpose

配置系统负责加载和管理 Kode 的配置文件。

## Requirements

### Requirement: 配置文件加载
The system SHALL load configuration from ~/.kode.json and ./.kode.json.

系统应从 ~/.kode.json 和 ./.kode.json 加载配置。

#### Scenario: 加载全局配置
- **WHEN** 用户启动 Kode 应用
- **THEN** 系统自动从 ~/.kode.json 加载全局配置
- **AND** 如果文件不存在，使用默认配置
```

### 决策 2: 规范划分策略

**选择**: 按功能领域划分规范，而不是按开发阶段

**划分**:
1. `config-loading` - 配置系统（已实现）
2. `message-model` - 消息与模型抽象（规划中）
3. `anthropic-service` - Anthropic API（规划中）
4. `tool-system` - 工具系统（规划中）
5. `agent-system` - Agent 系统（规划中）
6. `mcp-integration` - MCP 集成（规划中）
7. `tui-interface` - TUI 界面（规划中）
8. `cli-commands` - CLI 命令（规划中）

**理由**:
- 每个规范是独立的功能单元
- 便于并行开发
- 符合 OpenSpec 的单一能力原则

### 决策 3: 已实现功能的处理

**选择**: 为已实现的功能创建 "ADDED" 类型的规范

**理由**:
- 建立规范基线
- 记录已实现的行为
- 后续变更使用 "MODIFIED" 类型

**示例**:
```markdown
## ADDED Requirements

### Requirement: 配置文件加载
The system SHALL load configuration...

(描述已实现的配置加载功能)
```

### 决策 4: 旧文档的归档策略

**选择**: 移动到 `docs/archive/` 目录，保留原始内容

**理由**:
- 保留历史记录作为参考
- 不删除有价值的设计讨论
- 清晰区分当前规范和历史文档

**归档文档**:
- `TODO.md` → `docs/archive/TODO.md`
- `ROADMAP.md` → `docs/archive/ROADMAP.md`
- `ARCHITECTURE.md` → `docs/archive/ARCHITECTURE.md`
- `TECH_STACK.md` → `docs/archive/TECH_STACK.md`

**保留文档**:
- `README.md` - 项目介绍
- `CONTRIBUTING.md` - 贡献指南
- `PROJECT_OVERVIEW.md` - 项目概览
- `WORKFLOW.md` - 更新为 OpenSpec 流程
- `CLAUDE.md` - 更新引用

## Specification Structure

### 规范目录结构

```
openspec/
├── project.md              # 项目上下文（已有）
├── AGENTS.md               # Agent 指令（已有）
├── specs/                  # 当前已实现的功能
│   ├── config-loading/
│   │   ├── spec.md         # 配置加载规范
│   │   └── design.md       # 配置架构设计
│   ├── message-model/
│   │   ├── spec.md
│   │   └── design.md
│   ├── anthropic-service/
│   ├── tool-system/
│   ├── agent-system/
│   ├── mcp-integration/
│   ├── tui-interface/
│   └── cli-commands/
├── changes/                # 进行中的变更
│   └── migrate-docs-to-specs/
│       ├── proposal.md
│       ├── tasks.md
│       ├── design.md       # 本文件
│       └── specs/
│           └── README.md   # 变更说明
└── archive/                # 已完成的变更（暂时为空）
```

### 规范文件模板

每个 `spec.md` 应包含：

```markdown
# [功能名称] 规范

## Purpose

[中文描述功能目的]

## Requirements

### Requirement: [需求名称]
The system SHALL [需求描述 in English with SHALL].

[中文详细说明]

#### Scenario: [场景名称]
- **WHEN** [场景条件]
- **THEN** [预期结果]
- **AND** [附加条件，可选]

## Non-Goals

[本规范不包含的内容]
```

## Migration Plan

### 步骤 1: 创建规范目录结构

```bash
# 创建所有规范目录
mkdir -p openspec/specs/{config-loading,message-model,anthropic-service,tool-system,agent-system,mcp-integration,tui-interface,cli-commands}
```

### 步骤 2: 编写规范文件

为每个功能创建 `spec.md` 和 `design.md`。

#### 规范内容来源映射

| 规范 | 来源文档 |
|------|----------|
| `config-loading` | ROADMAP.md Day 3-4, ARCHITECTURE.md 配置章节 |
| `message-model` | ROADMAP.md Day 5-7, ARCHITECTURE.md 消息模型章节 |
| `anthropic-service` | ROADMAP.md Day 8-10, TECH_STACK.md HTTP 客户端 |
| `tool-system` | ROADMAP.md Day 11-12, ARCHITECTURE.md 工具系统 |
| `agent-system` | ROADMAP.md Day 22-27, ARCHITECTURE.md Agent 系统 |
| `mcp-integration` | ROADMAP.md Day 41-42, 50-52 |
| `tui-interface` | ROADMAP.md Day 29-35 |
| `cli-commands` | ROADMAP.md Day 56, CLAUDE.md |

### 步骤 3: 归档旧文档

```bash
# 创建归档目录
mkdir -p docs/archive

# 移动文档
mv TODO.md docs/archive/
mv ROADMAP.md docs/archive/
mv ARCHITECTURE.md docs/archive/
mv TECH_STACK.md docs/archive/
```

### 步骤 4: 更新工作流文档

1. 更新 `WORKFLOW.md`:
   - 添加 OpenSpec 三阶段工作流
   - 更新命令示例
   - 添加验证流程

2. 更新 `CLAUDE.md`:
   - 更新 "OpenSpec Instructions" 章节
   - 添加规范编写规范

3. 更新 `README.md`:
   - 添加 OpenSpec 开发流程说明

### 步骤 5: 验证

```bash
# 验证所有规范
npx openspec validate migrate-docs-to-specs --strict

# 列出变更
npx openspec list

# 查看变更详情
npx openspec show migrate-docs-to-specs
```

## Risks / Trade-offs

### 风险 1: 文档同步问题

**风险**: OpenSpec 规范与旧文档内容可能不一致

**缓解措施**:
- 以现有代码实现为准
- 旧文档仅作为设计参考
- 规范描述当前实际行为

### 风险 2: 学习曲线

**风险**: 团队需要学习 OpenSpec 工作流

**缓解措施**:
- CLAUDE.md 包含详细指令
- 提供命令参考表
- 渐进式采用

### 风险 3: 维护成本

**风险**: 需要同时维护 spec 和代码

**缓解措施**:
- 使用 `openspec archive` 自动归档
- 规范变更强制代码审查
- 自动验证减少人工检查

## Open Questions

1. **是否为所有 ROADMAP 功能创建规范？**
   - 建议：先为已实现和规划中的功能创建，后续功能逐步添加

2. **design.md 是否必需？**
   - 建议：复杂功能需要，简单功能可选

3. **如何处理跨功能的依赖？**
   - 建议：在 spec.md 中使用 "See also" 引用相关规范

## 参考资源

- [OpenSpec AGENTS.md](../AGENTS.md) - Agent 指令
- [OpenSpec project.md](../project.md) - 项目规范
- [现有 TODO.md](../../TODO.md) - 任务清单（待归档）
- [现有 ROADMAP.md](../../ROADMAP.md) - 路线图（待归档）
