# Spec 优先级指南 / Spec Priorities Guide

> **OpenSpec 工作流集成文档**
>
> 本文档用于：
> - **apply 阶段**: 选择下一个要实施的变更提案时
> - **archive 阶段**: 验证变更是否可以安全归档时
> - **proposal 阶段**: 创建新变更时评估优先级和依赖关系
>
> **Workflow Integration Document**
>
> This document is used for:
> - **apply phase**: When selecting the next change proposal to implement
> - **archive phase**: When verifying if a change can be safely archived
> - **proposal phase**: When evaluating priorities and dependencies for new changes

最后更新 / Last Updated: 2025-12-24

## 📊 当前状态 / Current Status

| Spec | 优先级 | 状态 | 对应变更 | 备注 |
|------|--------|------|----------|------|
| config-loading | P0 | ✅ 已完成 | 2024-12-24-implement-config-loading | 配置加载系统 |
| agent-system | P1 | ✅ 已完成 | 2025-12-24-implement-agent-system | Agent 系统 |
| message-model | P0 | ✅ 已完成 | 2025-12-24-implement-message-model | 消息与模型抽象 |
| tool-system | P1 | ⬜ 未开始 | implement-tool-system | 工具系统 |
| anthropic-service | P1 | ⬜ 未开始 | implement-anthropic-service | Anthropic 服务 |
| openai-service | P1 | ⬜ 未开始 | implement-openai-service | OpenAI 服务 |
| cli-commands | P2 | ⬜ 未开始 | implement-cli-commands-full | CLI 命令 |
| basic-cli | P2 | ⬜ 未开始 | implement-basic-cli | 基础 CLI |
| tui-interface | P2 | ⬜ 未开始 | implement-tui-interface | TUI 界面 |
| mcp-integration | P3 | ⬜ 未开始 | implement-mcp-client | MCP 集成 |

## 📋 优先级分类

### 🔴 P0 - 核心基础 (Critical Foundation)
**必须最先实现的功能，是整个系统的基础**

这些功能是其他所有功能的依赖，没有它们无法构建任何其他内容。

#### 1. config-loading (配置加载系统) - **✅ 已完成**
- **状态**: 已归档为 `2024-12-24-implement-config-loading`
- **依赖关系**: 所有其他模块都依赖配置系统
- **关键文件**:
  - `GlobalConfig` - 全局配置类型
  - `ProjectConfig` - 项目配置类型
  - 配置加载/保存/迁移逻辑
- **参考实现**: `/Users/gemini/Documents/backup/Kode-cli/src/utils/config.ts` (940行)

#### 2. message-model (消息与模型抽象) - **⬜ 下一步建议**
- **依赖关系**: 依赖 config-loading，被所有服务依赖
- **实现原因**:
  - 定义了统一的消息格式
  - 定义了模型适配器接口
  - 是 AI 交互的核心抽象
- **关键文件**:
  - `Message` 类型系统
  - `ModelAdapter` trait
  - `ModelManager`
- **参考实现**: `/Users/gemini/Documents/backup/Kode-cli/src/types/conversation.ts`

---

### 🟠 P1 - 核心服务 (Core Services)
**实现后可以提供基本功能的服务**

这些服务实现了基础的 AI 对话能力，完成后可以运行最基本的应用。

#### 3. agent-system (Agent 系统) - **✅ 已完成**
- **状态**: 已归档为 `2025-12-24-implement-agent-system`
- **依赖关系**: 依赖 config-loading
- **关键文件**:
  - `Agent` 类型定义
  - `AgentLoader` (五层加载优先级)
  - `AgentStorage` (数据持久化)
- **参考实现**: `/Users/gemini/Documents/backup/Kode-cli/src/utils/agents.ts`

#### 4. anthropic-service (Anthropic 服务) - **⬜ 未开始**
- **依赖关系**: 依赖 message-model
- **实现原因**:
  - Anthropic Claude 是主要模型提供商
  - 实现基本的发送消息和流式响应
  - 是第一个需要实现的模型适配器
- **关键文件**:
  - Anthropic API 客户端
  - 流式响应处理
  - 错误处理和重试
- **参考实现**: `/Users/gemini/Documents/backup/Kode-cli/src/services/claude.ts`

#### 5. openai-service (OpenAI 服务) - **⬜ 未开始**
- **依赖关系**: 依赖 message-model
- **实现原因**:
  - OpenAI 是重要的备用模型提供商
  - GPT 系列模型支持
  - 提供模型选择灵活性
- **关键文件**:
  - OpenAI API 客户端
  - 流式响应处理
- **参考实现**: `/Users/gemini/Documents/backup/Kode-cli/src/services/openai.ts`

#### 6. tool-system (工具系统) - **⬜ 未开始**
- **依赖关系**: 依赖 config-loading
- **实现原因**:
  - 工具是 AI 的能力扩展
  - 核心工具（Read, Write, Edit, Bash）是必需的
  - Tool trait 是扩展点
- **关键文件**:
  - `Tool` trait 定义
  - `ToolRegistry`
  - 核心工具实现
- **参考实现**:
  - `/Users/gemini/Documents/backup/Kode-cli/src/Tool.ts`
  - `/Users/gemini/Documents/backup/Kode-cli/src/tools.ts`

---

### 🟡 P2 - 用户界面 (User Interface)
**提供用户交互方式**

这些功能完成后，用户可以通过 CLI 和 TUI 与应用交互。

#### 6. cli-commands (CLI 命令) - **P2**
- **依赖关系**: 依赖所有 P0-P1 功能
- **实现原因**:
  - 提供命令行接口
  - 配置管理命令
  - Agent 和工具列表命令
- **关键文件**:
  - 命令行解析
  - 各种子命令实现
  - 帮助系统
- **参考实现**: `/Users/gemini/Documents/backup/Kode-cli/src/commands.ts`

#### 7. tui-interface (TUI 界面) - **P2**
- **依赖关系**: 依赖所有 P0-P1 功能
- **实现原因**:
  - 提供交互式终端界面
  - 流式响应显示
  - 权限请求对话框
- **关键文件**:
  - REPL 界面
  - 消息渲染
  - 权限对话框
  - 状态栏
- **参考实现**: `/Users/gemini/Documents/backup/Kode-cli/src/components/`

---

### 🟢 P3 - 高级特性 (Advanced Features)
**增强功能的特性**

这些是锦上添花的功能，可以在核心功能稳定后实现。

#### 8. mcp-integration (MCP 集成) - **P3**
- **依赖关系**: 依赖 tool-system
- **实现原因**:
  - MCP 是扩展协议
  - 支持动态工具加载
  - STDIO 和 SSE 传输
- **关键文件**:
  - MCP 客户端
  - 工具发现和调用
  - 服务器管理
- **参考实现**: `/Users/gemini/Documents/backup/Kode-cli/src/services/mcpClient.ts`

---

## 📊 依赖关系图

```
config-loading (P0) ✅ 已完成 / Completed
    ↓
message-model (P0) ⬜ 未开始 / Not Started ← **下一步建议 / Next Recommended**
    ↓
    ├─→ anthropic-service (P1) ⬜ 未开始 / Not Started
    ├─→ openai-service (P1) ⬜ 未开始 / Not Started
    ├─→ agent-system (P1) ✅ 已完成 / Completed
    └─→ tool-system (P1) ⬜ 未开始 / Not Started
            ↓
        cli-commands (P2) ⬜ 未开始 / Not Started
        basic-cli (P2) ⬜ 未开始 / Not Started
        tui-interface (P2) ⬜ 未开始 / Not Started
            ↓
        mcp-integration (P3) ⬜ 未开始 / Not Started
```

### 依赖检查命令 / Dependency Check Commands

```bash
# 检查 spec 完成状态 / Check spec completion status
openspec list --specs

# 检查活跃的变更 / Check active changes
openspec list

# 查看特定 spec 详情 / View specific spec details
openspec show <spec-id>
```

---

## 🎯 开发阶段建议

### 阶段 1: 基础设施 (Foundation)
**目标**: 建立项目基础，实现配置和核心抽象
- ✅ config-loading (P0) - **已完成**
- ⬜ message-model (P0) - **下一步建议**

### 阶段 2: 核心服务 (Core Services)
**目标**: 实现基本的 AI 对话能力
- ✅ agent-system (P1) - **已完成**
- ⬜ anthropic-service (P1)
- ⬜ openai-service (P1)
- ⬜ tool-system (P1)

### 阶段 3: 用户界面 (User Interface)
**目标**: 提供完整的用户交互体验
- ⬜ basic-cli (P2)
- ⬜ cli-commands (P2)
- ⬜ tui-interface (P2)

### 阶段 4: 高级特性 (Advanced Features)
**目标**: 实现扩展功能
- ⬜ mcp-integration (P3)

---

## 📝 OpenSpec 工作流集成

### 1️⃣ Proposal 阶段：创建变更前 / Before Creating Changes

**使用命令**: `/openspec:proposal <description>`

在创建任何变更提案前，必须完成以下检查：

#### 依赖检查清单 / Dependency Checklist

```bash
# 1. 检查依赖的 spec 是否已完成 / Check if dependent specs are complete
openspec list --specs | grep <dependent-spec>

# 2. 查看依赖 spec 的完成状态 / Check completion status of dependent specs
openspec show <spec-id> --type spec

# 3. 确认没有冲突的活跃变更 / Confirm no conflicting active changes
openspec list
```

**检查项**:
- [ ] 该功能依赖的所有 P0-P1 spec 是否已完成？
- [ ] 是否按照优先级顺序（P0 → P1 → P2 → P3）？
- [ ] 是否有同名或冲突的活跃变更？
- [ ] 变更范围是否与 spec 优先级匹配？

**决策树**:
```
创建新变更？
├─ 依赖的 P0 spec 未完成？
│  └─ ❌ 先完成 P0 spec (config-loading, message-model)
├─ 依赖的 P1 spec 未完成？
│  └─ ❌ 先完成 P1 spec (anthropic-service, agent-system, tool-system)
├─ 该功能优先级是 P3 但 P1 未完成？
│  └─ ❌ 推迟到 P1 完成后
└─ 依赖已完成，优先级合理？
   └─ ✅ 可以创建变更提案
```

---

### 2️⃣ Apply 阶段：选择要实施的变更 / Selecting Changes to Implement

**使用命令**: `/openspec:apply <change-id>`

当有多个变更提案等待实施时，按以下优先级选择：

#### 变更选择优先级 / Change Selection Priority

1. **P0 依赖优先**: 首先实施 config-loading 和 message-model 相关的变更
2. **阻塞链优先**: 如果变更 A 阻塞了变更 B，优先实施 A
3. **完成度优先**: 优先实施已完成 80% 的变更，而不是启动新的
4. **依赖树自底向上**: 依赖树的叶子节点优先于根节点

#### 选择流程 / Selection Flow

```bash
# 1. 列出所有待实施的变更 / List all pending changes
openspec list

# 2. 查看变更详情和依赖 / View change details and dependencies
openspec show <change-id> --json --deltas-only

# 3. 检查依赖的 spec 状态 / Check dependent spec status
openspec show <spec-id> --type spec
```

**选择算法**:
```python
def select_next_change(pending_changes):
    # 1. 过滤出依赖已满足的变更
    ready = [c for c in pending_changes if dependencies_met(c)]

    # 2. 按优先级排序 (P0 > P1 > P2 > P3)
    sorted_by_priority = sort(ready, key=spec_priority)

    # 3. 在同优先级中，按依赖关系排序（被依赖的优先）
    sorted_by_dependency = sort_by_blocking(sorted_by_priority)

    return sorted_by_dependency[0]  # 返回最高优先级的变更
```

**示例场景**:
```
待实施变更:
- implement-mcp-client (P3, 依赖: tool-system)
- implement-anthropic-service (P1, 依赖: message-model)
- implement-tool-system (P1, 依赖: config-loading)

当前状态:
- config-loading: ✅ 已完成
- agent-system: ✅ 已完成
- message-model: ⬜ 未开始

优先级排序:
1. implement-message-model (P0, 无未完成依赖) ← **最高优先级**
2. implement-tool-system (P1, 依赖: config-loading ✅)
3. implement-anthropic-service (P1, 依赖: message-model ⬜)
4. implement-mcp-client (P3, 依赖: tool-system ⬜)

→ 下一步应实施: implement-message-model
```

---

### 3️⃣ Archive 阶段：归档前验证 / Pre-Archive Validation

**使用命令**: `/openspec:archive <change-id>`

在归档任何变更前，必须完成以下验证：

#### 归档验证清单 / Archive Validation Checklist

```bash
# 1. 验证变更的所有任务已完成 / Verify all tasks are completed
openspec show <change-id>
# 检查 tasks.md 中所有任务是否标记为 [x]

# 2. 运行测试套件 / Run test suite
cargo test
cargo clippy -- -D warnings
cargo fmt --check

# 3. 验证 spec 更新 / Validate spec updates
openspec validate --strict

# 4. 确认依赖链完整性 / Confirm dependency chain integrity
# 如果这个变更被其他变更依赖，确保不会破坏它们
```

**检查项**:
- [ ] `tasks.md` 中所有任务都标记为 `- [x]`？
- [ ] 所有测试通过 (`cargo test`)？
- [ ] 无 clippy 警告 (`cargo clippy`)？
- [ ] 代码已格式化 (`cargo fmt`)？
- [ ] Spec delta 正确应用到 specs/？
- [ ] 运行 `openspec validate --strict` 无错误？
- [ ] 不影响其他活跃变更的依赖？

**阻止归档的情况**:
```
❌ tasks.md 中有未完成的任务
❌ 测试失败或有 clippy 警告
❌ spec 验证失败
❌ 变更修改了被其他活跃变更依赖的接口
❌ 配置格式不兼容（对于 config-loading 相关变更）
```

---

### 4️⃣ 优先级动态调整 / Dynamic Priority Adjustment

某些情况下需要调整优先级：

#### 提升优先级的条件 / Conditions for Priority Promotion

- **安全漏洞**: 任何安全相关的变更立即提升到 P0
- **阻塞问题**: 阻塞多个其他变更的问题提升优先级
- **用户需求**: 核心用户急需的功能可以提升优先级
- **依赖变更**: 当高优先级 spec 完成后，依赖它的 P2/P3 功能可以提升

#### 降低优先级的条件 / Conditions for Priority Demotion

- **实现难度过高**: 暂时跳过，先实现简单功能
- **依赖未完成**: 被依赖的 spec 未完成时，推迟依赖者
- **替代方案**: 发现更简单的实现方式时，可以推迟复杂方案

---

## 🔍 快速参考

### Spec 完成度查询

```bash
# 查看所有 spec
openspec list --specs

# 查看特定 spec
openspec show <spec-id>

# 查看活跃的变更
openspec list
```

### Spec 文件位置

所有 spec 文件位于: `openspec/specs/<spec-id>/spec.md`

- `config-loading` - `openspec/specs/config-loading/spec.md` ✅
- `agent-system` - `openspec/specs/agent-system/spec.md` ✅
- `message-model` - `openspec/specs/message-model/spec.md` ⬜ **下一步**
- `tool-system` - `openspec/specs/tool-system/spec.md` ⬜
- `anthropic-service` - `openspec/specs/anthropic-service/spec.md` ⬜
- `openai-service` - `openspec/specs/openai-service/spec.md` ⬜
- `cli-commands` - `openspec/specs/cli-commands/spec.md` ⬜
- `tui-interface` - `openspec/specs/tui-interface/spec.md` ⬜
- `mcp-integration` - `openspec/specs/mcp-integration/spec.md` ⬜

---

## 📌 注意事项

1. **优先级不是绝对的**: 根据实际需求，可以并行开发独立的模块
2. **MVP 优先**: 优先实现最小可用产品所需的功能
3. **增量开发**: 每个 spec 都可以有多个 change proposal
4. **参考原版**: 所有实现都应参考 TypeScript 版本的代码

---

**维护说明**: 当添加新 spec 或修改优先级时，请更新此文档并注明修改原因。
