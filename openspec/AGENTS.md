# OpenSpec Instructions / OpenSpec 指令

Instructions for AI coding assistants using OpenSpec for spec-driven development.
/ 使用 OpenSpec 进行规范驱动开发的 AI 编码助手指令。

## 🌐 Language Rule / 语言规则

**IMPORTANT / 重要**: All responses MUST be in Chinese / 所有回复必须使用中文。

- **✅** Use Chinese for all user interactions / 所有用户交互使用中文
- **✅** Use Chinese for code comments / 代码注释使用中文
- **✅** Variable/function names in English are acceptable / 变量/函数名称使用英文
- **❌** Do NOT use other languages unless explicitly requested / 禁止使用其他语言（除非明确要求）

This rule applies to all interactions:
- Answering questions / 回答问题
- Code review feedback / 代码审查反馈
- Error messages / 错误信息
- Documentation / 文档说明
- Progress reports / 进度汇报

**Note / 注意**: OpenSpec 文档格式请参考 `openspec/project.md` 中的"OpenSpec 文档规范（双语格式）"部分。

## TL;DR Quick Checklist / 快速检查清单

- **⚠️ 优先级检查**: 在创建或实施变更前，先查看 `openspec/SPEC_PRIORITIES.md` 了解 spec 优先级和依赖关系
  / **⚠️ Priority Check**: Check `openspec/SPEC_PRIORITIES.md` before creating or implementing changes to understand spec priorities and dependencies
- Search existing work: `openspec spec list --long`, `openspec list` (use `rg` only for full-text search)
  / 搜索现有工作：`openspec spec list --long`，`openspec list`（仅在全文搜索时使用 `rg`）
- Decide scope: new capability vs modify existing capability
  / 决定范围：新能力 vs 修改现有能力
- Pick a unique `change-id`: kebab-case, verb-led (`add-`, `update-`, `remove-`, `refactor-`)
  / 选择唯一的 `change-id`：kebab-case，动词引导（`add-`、`update-`、`remove-`、`refactor-`）
- Scaffold: `proposal.md`, `tasks.md`, `design.md` (only if needed), and delta specs per affected capability
  / 搭建结构：`proposal.md`、`tasks.md`、`design.md`（仅在需要时）以及每个受影响能力的增量规范
- Write deltas: use `## ADDED|MODIFIED|REMOVED|RENAMED Requirements`; include at least one `#### Scenario:` per requirement
  / 编写增量：使用 `## ADDED|MODIFIED|REMOVED|RENAMED Requirements`；每个需求至少包含一个 `#### Scenario:`
- Validate: `openspec validate [change-id] --strict` and fix issues
  / 验证：`openspec validate [change-id] --strict` 并修复问题
- Request approval: Do not start implementation until proposal is approved
  / 请求批准：在提案获得批准之前不要开始实施

## Three-Stage Workflow / 三阶段工作流程

### Stage 1: Creating Changes / 创建变更阶段
Create proposal when you need to:
/ 在以下情况下创建提案：
- Add features or functionality
  / 添加功能或能力
- Make breaking changes (API, schema)
  / 进行破坏性变更（API、schema）
- Change architecture or patterns
  / 更改架构或模式
- Optimize performance (changes behavior)
  / 优化性能（改变行为）
- Update security patterns
  / 更新安全模式

Triggers (examples):
/ 触发器（示例）：
- "Help me create a change proposal"
  / "帮我创建变更提案"
- "Help me plan a change"
  / "帮我规划变更"
- "Help me create a proposal"
  / "帮我创建提案"
- "I want to create a spec proposal"
  / "我想创建规范提案"
- "I want to create a spec"
  / "我想创建规范"

Loose matching guidance:
/ 模糊匹配指南：
- Contains one of: `proposal`, `change`, `spec`
  / 包含以下之一：`proposal`、`change`、`spec`
- With one of: `create`, `plan`, `make`, `start`, `help`
  / 以及以下之一：`create`、`plan`、`make`、`start`、`help`

Skip proposal for:
/ 跳过提案的情况：
- Bug fixes (restore intended behavior)
  / 错误修复（恢复预期行为）
- Typos, formatting, comments
  / 拼写错误、格式化、注释
- Dependency updates (non-breaking)
  / 依赖更新（非破坏性）
- Configuration changes
  / 配置变更
- Tests for existing behavior
  / 现有行为的测试

**Workflow / 工作流程**
1. **Check `openspec/SPEC_PRIORITIES.md`** - Verify dependencies and priority order for the intended spec
   / **检查 `openspec/SPEC_PRIORITIES.md`** - 验证目标规范的依赖关系和优先级顺序
2. Review `openspec/project.md`, `openspec list`, and `openspec list --specs` to understand current context.
   / 审查 `openspec/project.md`、`openspec list` 和 `openspec list --specs` 以了解当前上下文。
3. Choose a unique verb-led `change-id` and scaffold `proposal.md`, `tasks.md`, optional `design.md`, and spec deltas under `openspec/changes/<id>/`.
   / 选择唯一的动词引导的 `change-id`，并在 `openspec/changes/<id>/` 下搭建 `proposal.md`、`tasks.md`、可选的 `design.md` 和规范增量。
4. Draft spec deltas using `## ADDED|MODIFIED|REMOVED Requirements` with at least one `#### Scenario:` per requirement.
   / 使用 `## ADDED|MODIFIED|REMOVED Requirements` 起草规范增量，每个需求至少包含一个 `#### Scenario:`。
5. Run `openspec validate <id> --strict` and resolve any issues before sharing the proposal.
   / 运行 `openspec validate <id> --strict` 并在共享提案之前解决所有问题。

### Stage 2: Implementing Changes / 实施变更阶段
Track these steps as TODOs and complete them one by one.
/ 将这些步骤作为 TODO 跟踪并逐一完成。
1. **Check `openspec/SPEC_PRIORITIES.md`** - Confirm this change follows the correct priority order (dependencies first)
   / **检查 `openspec/SPEC_PRIORITIES.md`** - 确认此变更遵循正确的优先级顺序（依赖优先）
2. **Read proposal.md** - Understand what's being built
   / **阅读 proposal.md** - 了解要构建的内容
3. **Read design.md** (if exists) - Review technical decisions
   / **阅读 design.md**（如果存在）- 审查技术决策
4. **Read tasks.md** - Get implementation checklist
   / **阅读 tasks.md** - 获取实施检查清单
5. **Implement tasks sequentially** - Complete in order
   / **按顺序实施任务** - 按顺序完成
6. **Confirm completion** - Ensure every item in `tasks.md` is finished before updating statuses
   / **确认完成** - 在更新状态之前确保 `tasks.md` 中的每一项都已完成
7. **Update checklist** - After all work is done, set every task to `- [x]` so the list reflects reality
   / **更新检查清单** - 所有工作完成后，将每个任务设置为 `- [x]` 以反映实际情况
8. **Approval gate** - Do not start implementation until the proposal is reviewed and approved
   / **批准门控** - 在提案被审查和批准之前不要开始实施

### Stage 3: Archiving Changes / 归档变更阶段
After deployment, create separate PR to:
/ 部署后，创建单独的 PR 来：
- **Check `openspec/SPEC_PRIORITIES.md`** - Verify archive validation checklist
  / **检查 `openspec/SPEC_PRIORITIES.md`** - 验证归档验证检查清单
- Move `changes/[name]/` → `changes/archive/YYYY-MM-DD-[name]/`
  / 移动 `changes/[name]/` → `changes/archive/YYYY-MM-DD-[name]/`
- Update `specs/` if capabilities changed
  / 如果能力发生变化，更新 `specs/`
- Use `openspec archive <change-id> --skip-specs --yes` for tooling-only changes (always pass the change ID explicitly)
  / 对于仅工具的变更，使用 `openspec archive <change-id> --skip-specs --yes`（始终明确传递变更 ID）
- Run `openspec validate --strict` to confirm the archived change passes checks
  / 运行 `openspec validate --strict` 以确认归档的变更通过检查

## Before Any Task / 任务开始前

**Context Checklist / 上下文检查清单：**
- [ ] Read relevant specs in `specs/[capability]/spec.md`
  / [ ] 阅读 `specs/[capability]/spec.md` 中的相关规范
- [ ] Check pending changes in `changes/` for conflicts
  / [ ] 检查 `changes/` 中的待处理变更是否存在冲突
- [ ] Read `openspec/project.md` for conventions
  / [ ] 阅读 `openspec/project.md` 了解约定
- [ ] Run `openspec list` to see active changes
  / [ ] 运行 `openspec list` 查看活动变更
- [ ] Run `openspec list --specs` to see existing capabilities
  / [ ] 运行 `openspec list --specs` 查看现有能力

**Before Creating Specs / 创建规范前：**
- Always check if capability already exists
  / 始终检查能力是否已存在
- Prefer modifying existing specs over creating duplicates
  / 优先修改现有规范而不是创建重复规范
- Use `openspec show [spec]` to review current state
  / 使用 `openspec show [spec]` 查看当前状态
- If request is ambiguous, ask 1–2 clarifying questions before scaffolding
  / 如果请求不明确，在搭建结构前提出 1-2 个澄清问题

### Search Guidance / 搜索指南
- Enumerate specs: `openspec spec list --long` (or `--json` for scripts)
  / 枚举规范：`openspec spec list --long`（或 `--json` 用于脚本）
- Enumerate changes: `openspec list` (or `openspec change list --json` - deprecated but available)
  / 枚举变更：`openspec list`（或 `openspec change list --json` - 已弃用但可用）
- Show details:
  / 显示详细信息：
  - Spec: `openspec show <spec-id> --type spec` (use `--json` for filters)
    / 规范：`openspec show <spec-id> --type spec`（使用 `--json` 进行过滤）
  - Change: `openspec show <change-id> --json --deltas-only`
    / 变更：`openspec show <change-id> --json --deltas-only`
- Full-text search (use ripgrep): `rg -n "Requirement:|Scenario:" openspec/specs`
  / 全文搜索（使用 ripgrep）：`rg -n "Requirement:|Scenario:" openspec/specs`

## Quick Start / 快速开始

### CLI Commands / CLI 命令

```bash
# Essential commands / 基本命令
openspec list                  # 列出活动变更 / List active changes
openspec list --specs          # 列出规范 / List specifications
openspec show [item]           # 显示变更或规范 / Display change or spec
openspec validate [item]       # 验证变更或规范 / Validate changes or specs
openspec archive <change-id> [--yes|-y]   # 部署后归档（添加 --yes 进行非交互运行）/ Archive after deployment (add --yes for non-interactive runs)

# Project management / 项目管理
openspec init [path]           # 初始化 OpenSpec / Initialize OpenSpec
openspec update [path]         # 更新指令文件 / Update instruction files

# Interactive mode / 交互模式
openspec show                  # 提示选择 / Prompts for selection
openspec validate              # 批量验证模式 / Bulk validation mode

# Debugging / 调试
openspec show [change] --json --deltas-only
openspec validate [change] --strict
```

### Command Flags / 命令标志

- `--json` - Machine-readable output / 机器可读输出
- `--type change|spec` - Disambiguate items / 消除项目歧义
- `--strict` - Comprehensive validation / 全面验证
- `--no-interactive` - Disable prompts / 禁用提示
- `--skip-specs` - Archive without spec updates / 归档时不更新规范
- `--yes`/`-y` - Skip confirmation prompts (non-interactive archive) / 跳过确认提示（非交互式归档）

## Directory Structure / 目录结构

```
openspec/
├── project.md              # 项目约定 / Project conventions
├── specs/                  # 当前事实 - 已构建的内容 / Current truth - what IS built
│   └── [capability]/       # 单一聚焦的能力 / Single focused capability
│       ├── spec.md         # 需求和场景 / Requirements and scenarios
│       └── design.md       # 技术模式 / Technical patterns
├── changes/                # 提案 - 应该改变什么 / Proposals - what SHOULD change
│   ├── [change-name]/
│   │   ├── proposal.md     # 为什么、什么、影响 / Why, what, impact
│   │   ├── tasks.md        # 实施检查清单 / Implementation checklist
│   │   ├── design.md       # 技术决策（可选；见标准）/ Technical decisions (optional; see criteria)
│   │   └── specs/          # 增量变更 / Delta changes
│   │       └── [capability]/
│   │           └── spec.md # ADDED/MODIFIED/REMOVED
│   └── archive/            # 已完成的变更 / Completed changes
```

## Creating Change Proposals / 创建变更提案

### Decision Tree / 决策树

```
New request? / 新请求？
├─ Bug fix restoring spec behavior? → Fix directly / 修复恢复规范行为的错误？→ 直接修复
├─ Typo/format/comment? → Fix directly / 拼写/格式/注释？→ 直接修复
├─ New feature/capability? → Create proposal / 新功能/能力？→ 创建提案
├─ Breaking change? → Create proposal / 破坏性变更？→ 创建提案
├─ Architecture change? → Create proposal / 架构变更？→ 创建提案
└─ Unclear? → Create proposal (safer) / 不明确？→ 创建提案（更安全）
```

### Proposal Structure / 提案结构

1. **Create directory / 创建目录：** `changes/[change-id]/` (kebab-case, verb-led, unique)

2. **Write proposal.md / 编写 proposal.md：**
```markdown
# Change: [Brief description of change] / 变更：[变更的简要描述]

## Why / 为什么
[1-2 sentences on problem/opportunity] / [关于问题/机会的1-2句话]

## What Changes / 变更内容
- [Bullet list of changes] / [变更的项目列表]
- [Mark breaking changes with **BREAKING**] / [用 **BREAKING** 标记破坏性变更]

## Impact / 影响
- Affected specs: [list capabilities] / 受影响的规范：[列出能力]
- Affected code: [key files/systems] / 受影响的代码：[关键文件/系统]
```

3. **Create spec deltas / 创建规范增量：** `specs/[capability]/spec.md`
```markdown
## ADDED Requirements / 新增需求
### Requirement: New Feature / 需求：新功能
The system SHALL provide... / 系统应提供...

#### Scenario: Success case / 场景：成功案例
- **WHEN** user performs action / 当用户执行操作时
- **THEN** expected result / 则预期结果

## MODIFIED Requirements / 修改需求
### Requirement: Existing Feature / 需求：现有功能
[Complete modified requirement] / [完整的修改需求]

## REMOVED Requirements / 移除需求
### Requirement: Old Feature / 需求：旧功能
**Reason**: [Why removing] / **原因**：[为什么移除]
**Migration**: [How to handle] / **迁移**：[如何处理]
```
If multiple capabilities are affected, create multiple delta files under `changes/[change-id]/specs/<capability>/spec.md`—one per capability.
/ 如果多个能力受到影响，在 `changes/[change-id]/specs/<capability>/spec.md` 下创建多个增量文件——每个能力一个。

4. **Create tasks.md / 创建 tasks.md：**
```markdown
## 1. Implementation / 实施
- [ ] 1.1 Create database schema / 创建数据库模式
- [ ] 1.2 Implement API endpoint / 实现 API 端点
- [ ] 1.3 Add frontend component / 添加前端组件
- [ ] 1.4 Write tests / 编写测试
```

5. **Create design.md when needed / 需要时创建 design.md：**
Create `design.md` if any of the following apply; otherwise omit it:
/ 如果以下任何情况适用，创建 `design.md`；否则省略：
- Cross-cutting change (multiple services/modules) or a new architectural pattern
  / 跨越性变更（多个服务/模块）或新的架构模式
- New external dependency or significant data model changes
  / 新的外部依赖或重要的数据模型变更
- Security, performance, or migration complexity
  / 安全性、性能或迁移复杂性
- Ambiguity that benefits from technical decisions before coding
  / 在编码前从技术决策中受益的不明确性

Minimal `design.md` skeleton:
/ 最小 `design.md` 骨架：
```markdown
## Context / 上下文
[Background, constraints, stakeholders] / [背景、约束、利益相关者]

## Goals / Non-Goals / 目标 / 非目标
- Goals: [...] / 目标：[...]
- Non-Goals: [...] / 非目标：[...]

## Decisions / 决策
- Decision: [What and why] / 决策：[什么和为什么]
- Alternatives considered: [Options + rationale] / 考虑的替代方案：[选项 + 理由]

## Risks / Trade-offs / 风险 / 权衡
- [Risk] → Mitigation / [风险] → 缓解措施

## Migration Plan / 迁移计划
[Steps, rollback] / [步骤、回滚]

## Open Questions / 未解决的问题
- [...] / [...]
```

## Spec File Format / 规范文件格式

### Critical: Scenario Formatting / 关键：场景格式

**CORRECT / 正确** (use #### headers / 使用 #### 标题)：
```markdown
#### Scenario: User login success / 场景：用户登录成功
- **WHEN** valid credentials provided / 当提供有效凭据时
- **THEN** return JWT token / 则返回 JWT 令牌
```

**WRONG / 错误** (don't use bullets or bold / 不要使用项目符号或粗体)：
```markdown
- **Scenario: User login**  ❌
**Scenario**: User login     ❌
### Scenario: User login      ❌
```

Every requirement MUST have at least one scenario.
/ 每个需求必须至少有一个场景。

### Requirement Wording / 需求措辞
- Use SHALL/MUST for normative requirements (avoid should/may unless intentionally non-normative)
  / 对规范性要求使用 SHALL/MUST（除非有意非规范性，否则避免使用 should/may）

### Reference Section (Kode-Rust Specific) / 参考部分（Kode-Rust 特定）

For Kode-Rust project, all spec.md files MUST include a Reference section at the end, before Non-Goals:
/ 对于 Kode-Rust 项目，所有 spec.md 文件必须在末尾（Non-Goals 之前）包含参考部分：

```markdown
## Reference / 参考资料

### TypeScript 版本实现参考 / TypeScript Implementation Reference

在实现本规范时，请参考原版 TypeScript 项目中的以下文件：

When implementing this specification, refer to the following files in the original TypeScript project:

#### [Module Name in Chinese] / [Module Name in English]
- **[File/Component]**: `/Users/gemini/Documents/backup/Kode-cli/src/path/to/file.ts`
  - [Function/Feature 1] - Brief description / [功能/特性 1] - 简要描述
  - [Function/Feature 2] - Brief description / [功能/特性 2] - 简要描述

### 实现要点 / Implementation Notes

1. **[Key Point 1]**: [Description] / [关键点 1]：[描述]
2. **[Key Point 2]**: [Description] / [关键点 2]：[描述]
```

For design.md files, add the TypeScript Version Reference section after the Context section:
/ 对于 design.md 文件，在 Context 部分后添加 TypeScript 版本参考部分：

```markdown
## TypeScript 版本参考 / TypeScript Version Reference

在实现本设计时，请参考原版 TypeScript 项目中的以下文件：

When implementing this design, refer to the following files in the original TypeScript project:

### [Module Name] / [Module Name]
- **[File/Component]**: `/Users/gemini/Documents/backup/Kode-cli/src/path/to/file.ts`
  - [Key implementation details] / [关键实现细节]

### 实现细节 / Implementation Details
1. **[Detail 1]**: [Description] / [细节 1]：[描述]
2. **[Detail 2]**: [Description] / [细节 2]：[描述]
```

**Purpose / 目的**: Ensure all implementations reference the original TypeScript codebase for compatibility and understanding.
/ 确保所有实现引用原始 TypeScript 代码库以保持兼容性和理解。

### Delta Operations / 增量操作

- `## ADDED Requirements` - New capabilities / 新增能力
- `## MODIFIED Requirements` - Changed behavior / 修改行为
- `## REMOVED Requirements` - Deprecated features / 弃用功能
- `## RENAMED Requirements` - Name changes / 名称变更

Headers matched with `trim(header)` - whitespace ignored.
/ 标题使用 `trim(header)` 匹配 - 忽略空白。

#### When to use ADDED vs MODIFIED / 何时使用 ADDED vs MODIFIED
- ADDED: Introduces a new capability or sub-capability that can stand alone as a requirement. Prefer ADDED when the change is orthogonal (e.g., adding "Slash Command Configuration") rather than altering the semantics of an existing requirement.
  / ADDED：引入可以独立作为需求的新能力或子能力。当变更是正交的（例如，添加"斜杠命令配置"）而不是更改现有需求的语义时，优先使用 ADDED。
- MODIFIED: Changes the behavior, scope, or acceptance criteria of an existing requirement. Always paste the full, updated requirement content (header + all scenarios). The archiver will replace the entire requirement with what you provide here; partial deltas will drop previous details.
  / MODIFIED：更改现有需求的行为、范围或验收标准。始终粘贴完整的更新需求内容（标题 + 所有场景）。归档器将用您提供的内容替换整个需求；部分增量将丢失先前的详细信息。
- RENAMED: Use when only the name changes. If you also change behavior, use RENAMED (name) plus MODIFIED (content) referencing the new name.
  / RENAMED：当仅名称更改时使用。如果还更改行为，请使用 RENAMED（名称）加上 MODIFIED（内容）引用新名称。

Common pitfall: Using MODIFIED to add a new concern without including the previous text. This causes loss of detail at archive time. If you aren't explicitly changing the existing requirement, add a new requirement under ADDED instead.
/ 常见陷阱：使用 MODIFIED 添加新关注点而不包含先前的文本。这会导致在归档时丢失详细信息。如果您没有明确更改现有需求，请在 ADDED 下添加新需求。

Authoring a MODIFIED requirement correctly:
/ 正确编写 MODIFIED 需求：
1) Locate the existing requirement in `openspec/specs/<capability>/spec.md`.
   / 在 `openspec/specs/<capability>/spec.md` 中找到现有需求。
2) Copy the entire requirement block (from `### Requirement: ...` through its scenarios).
   / 复制整个需求块（从 `### Requirement: ...` 到其场景）。
3) Paste it under `## MODIFIED Requirements` and edit to reflect the new behavior.
   / 将其粘贴到 `## MODIFIED Requirements` 下并编辑以反映新行为。
4) Ensure the header text matches exactly (whitespace-insensitive) and keep at least one `#### Scenario:`.
   / 确保标题文本完全匹配（不区分空格）并保留至少一个 `#### Scenario:`。

Example for RENAMED:
/ RENAMED 示例：
```markdown
## RENAMED Requirements
- FROM: `### Requirement: Login`
- TO: `### Requirement: User Authentication`
```

## Troubleshooting / 故障排除

### Common Errors / 常见错误

**"Change must have at least one delta"** / "变更必须至少有一个增量"
- Check `changes/[name]/specs/` exists with .md files
  / 检查 `changes/[name]/specs/` 是否存在并包含 .md 文件
- Verify files have operation prefixes (## ADDED Requirements)
  / 验证文件是否有操作前缀（## ADDED Requirements）

**"Requirement must have at least one scenario"** / "需求必须至少有一个场景"
- Check scenarios use `#### Scenario:` format (4 hashtags)
  / 检查场景是否使用 `#### Scenario:` 格式（4 个井号）
- Don't use bullet points or bold for scenario headers
  / 不要为场景标题使用项目符号或粗体

**Silent scenario parsing failures** / 静默场景解析失败
- Exact format required: `#### Scenario: Name`
  / 需要精确格式：`#### Scenario: Name`
- Debug with: `openspec show [change] --json --deltas-only`
  / 使用以下命令调试：`openspec show [change] --json --deltas-only`

### Validation Tips / 验证技巧

```bash
# Always use strict mode for comprehensive checks / 始终使用严格模式进行全面检查
openspec validate [change] --strict

# Debug delta parsing / 调试增量解析
openspec show [change] --json | jq '.deltas'

# Check specific requirement / 检查特定需求
openspec show [spec] --json -r 1
```

## Happy Path Script / 快速路径脚本

```bash
# 1) Explore current state / 探索当前状态
openspec spec list --long
openspec list
# Optional full-text search / 可选的全文搜索：
# rg -n "Requirement:|Scenario:" openspec/specs
# rg -n "^#|Requirement:" openspec/changes

# 2) Choose change id and scaffold / 选择变更 ID 并搭建结构
CHANGE=add-two-factor-auth
mkdir -p openspec/changes/$CHANGE/{specs/auth}
printf "## Why\n...\n\n## What Changes\n- ...\n\n## Impact\n- ...\n" > openspec/changes/$CHANGE/proposal.md
printf "## 1. Implementation\n- [ ] 1.1 ...\n" > openspec/changes/$CHANGE/tasks.md

# 3) Add deltas (example) / 添加增量（示例）
cat > openspec/changes/$CHANGE/specs/auth/spec.md << 'EOF'
## ADDED Requirements
### Requirement: Two-Factor Authentication / 需求：双因素身份验证
Users MUST provide a second factor during login.
/ 用户在登录期间必须提供第二个因素。

#### Scenario: OTP required / 场景：需要 OTP
- **WHEN** valid credentials are provided / 当提供有效凭据时
- **THEN** an OTP challenge is required / 则需要 OTP 挑战
EOF

# 4) Validate / 验证
openspec validate $CHANGE --strict
```

## Multi-Capability Example / 多能力示例

```
openspec/changes/add-2fa-notify/
├── proposal.md
├── tasks.md
└── specs/
    ├── auth/
    │   └── spec.md   # ADDED: Two-Factor Authentication / 新增：双因素身份验证
    └── notifications/
        └── spec.md   # ADDED: OTP email notification / 新增：OTP 电子邮件通知
```

auth/spec.md
```markdown
## ADDED Requirements / 新增需求
### Requirement: Two-Factor Authentication / 需求：双因素身份验证
...
```

notifications/spec.md
```markdown
## ADDED Requirements / 新增需求
### Requirement: OTP Email Notification / 需求：OTP 电子邮件通知
...
```

## Best Practices / 最佳实践

### Simplicity First / 简单优先
- Default to <100 lines of new code
  / 默认情况下，新代码少于 100 行
- Single-file implementations until proven insufficient
  / 单文件实现，直到被证明不足
- Avoid frameworks without clear justification
  / 避免在没有明确理由的情况下使用框架
- Choose boring, proven patterns
  / 选择无聊但经过验证的模式

### Complexity Triggers / 复杂性触发器
Only add complexity with:
/ 仅在以下情况下添加复杂性：
- Performance data showing current solution too slow
  / 性能数据显示当前解决方案太慢
- Concrete scale requirements (>1000 users, >100MB data)
  / 具体的规模要求（>1000 用户，>100MB 数据）
- Multiple proven use cases requiring abstraction
  / 多个已证明需要抽象的用例

### Clear References / 清晰引用
- Use `file.ts:42` format for code locations
  / 使用 `file.ts:42` 格式表示代码位置
- Reference specs as `specs/auth/spec.md`
  / 将规范引用为 `specs/auth/spec.md`
- Link related changes and PRs
  / 链接相关的变更和 PR

### Capability Naming / 能力命名
- Use verb-noun: `user-auth`, `payment-capture`
  / 使用动词-名词：`user-auth`、`payment-capture`
- Single purpose per capability
  / 每个能力单一目的
- 10-minute understandability rule
  / 10 分钟可理解性规则
- Split if description needs "AND"
  / 如果描述需要 "AND"，则拆分

### Change ID Naming / 变更 ID 命名
- Use kebab-case, short and descriptive: `add-two-factor-auth`
  / 使用 kebab-case，简短且描述性：`add-two-factor-auth`
- Prefer verb-led prefixes: `add-`, `update-`, `remove-`, `refactor-`
  / 优先使用动词引导的前缀：`add-`、`update-`、`remove-`、`refactor-`
- Ensure uniqueness; if taken, append `-2`, `-3`, etc.
  / 确保唯一性；如果已被占用，附加 `-2`、`-3` 等

## Tool Selection Guide / 工具选择指南

| Task / 任务 | Tool / 工具 | Why / 原因 |
|------|------|-----|
| Find files by pattern / 按模式查找文件 | Glob | Fast pattern matching / 快速模式匹配 |
| Search code content / 搜索代码内容 | Grep | Optimized regex search / 优化的正则表达式搜索 |
| Read specific files / 读取特定文件 | Read | Direct file access / 直接文件访问 |
| Explore unknown scope / 探索未知范围 | Task | Multi-step investigation / 多步调查 |

## Error Recovery / 错误恢复

### Change Conflicts / 变更冲突
1. Run `openspec list` to see active changes
   / 运行 `openspec list` 查看活动变更
2. Check for overlapping specs
   / 检查重叠的规范
3. Coordinate with change owners
   / 与变更所有者协调
4. Consider combining proposals
   / 考虑合并提案

### Validation Failures / 验证失败
1. Run with `--strict` flag
   / 使用 `--strict` 标志运行
2. Check JSON output for details
   / 检查 JSON 输出以获取详细信息
3. Verify spec file format
   / 验证规范文件格式
4. Ensure scenarios properly formatted
   / 确保场景格式正确

### Missing Context / 缺少上下文
1. Read project.md first
   / 首先阅读 project.md
2. Check related specs
   / 检查相关规范
3. Review recent archives
   / 查看最近的归档
4. Ask for clarification
   / 请求澄清

## Quick Reference / 快速参考

### Stage Indicators / 阶段指示器
- `changes/` - Proposed, not yet built / 已提案，尚未构建
- `specs/` - Built and deployed / 已构建和部署
- `archive/` - Completed changes / 已完成的变更

### File Purposes / 文件用途
- `proposal.md` - Why and what / 为什么和什么
- `tasks.md` - Implementation steps / 实施步骤
- `design.md` - Technical decisions / 技术决策
- `spec.md` - Requirements and behavior / 需求和行为

### CLI Essentials / CLI 要点
```bash
openspec list              # What's in progress? / 进行中的工作？
openspec show [item]       # View details / 查看详细信息
openspec validate --strict # Is it correct? / 是否正确？
openspec archive <change-id> [--yes|-y]  # Mark complete (add --yes for automation) / 标记完成（为自动化添加 --yes）
```

Remember: Specs are truth. Changes are proposals. Keep them in sync.
/ 记住：规范是事实。变更是提案。保持它们同步。
