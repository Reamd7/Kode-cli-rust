# 任务清单 - 文档迁移到 OpenSpec

## 1. 规范文件创建

- [x] 1.1 创建 `config-loading` 规范
  - [x] 1.1.1 编写 spec.md（配置加载需求）
  - [x] 1.1.2 编写 design.md（配置架构设计）
  - [x] 1.1.3 验证规范（openspec validate）

- [x] 1.2 创建 `message-model` 规范
  - [x] 1.2.1 编写 spec.md（消息与模型需求）
  - [x] 1.2.2 编写 design.md（模型抽象设计）
  - [x] 1.2.3 验证规范

- [x] 1.3 创建 `anthropic-service` 规范
  - [x] 1.3.1 编写 spec.md（API 服务需求）
  - [x] 1.3.2 编写 design.md（服务设计）
  - [x] 1.3.3 验证规范

- [x] 1.4 创建 `tool-system` 规范
  - [x] 1.4.1 编写 spec.md（工具系统需求）
  - [x] 1.4.2 编写 design.md（工具抽象设计）
  - [x] 1.4.3 验证规范

- [x] 1.5 创建 `agent-system` 规范
  - [x] 1.5.1 编写 spec.md（Agent 系统需求）
  - [x] 1.5.2 编写 design.md（Agent 架构设计）
  - [x] 1.5.3 验证规范

- [x] 1.6 创建 `mcp-integration` 规范
  - [x] 1.6.1 编写 spec.md（MCP 集成需求）
  - [x] 1.6.2 编写 design.md（MCP 设计）
  - [x] 1.6.3 验证规范

- [x] 1.7 创建 `tui-interface` 规范
  - [x] 1.7.1 编写 spec.md（TUI 界面需求）
  - [x] 1.7.2 编写 design.md（UI 设计）
  - [x] 1.7.3 验证规范

- [x] 1.8 创建 `cli-commands` 规范
  - [x] 1.8.1 编写 spec.md（CLI 命令需求）
  - [x] 1.8.2 编写 design.md（CLI 设计）
  - [x] 1.8.3 验证规范

## 2. 文档归档

- [x] 2.1 创建 `docs/archive/` 目录
- [x] 2.2 移动 `TODO.md` → `docs/archive/TODO.md`
- [x] 2.3 移动 `ROADMAP.md` → `docs/archive/ROADMAP.md`
- [x] 2.4 移动 `ARCHITECTURE.md` → `docs/archive/ARCHITECTURE.md`
- [x] 2.5 移动 `TECH_STACK.md` → `docs/archive/TECH_STACK.md`
- [x] 2.6 在归档文档中添加迁移说明

## 3. 工作流文档更新

- [x] 3.1 更新 `WORKFLOW.md`
  - [x] 3.1.1 添加 OpenSpec 工作流说明
  - [x] 3.1.2 更新命令示例
  - [x] 3.1.3 添加提案创建流程

- [x] 3.2 更新 `CLAUDE.md`
  - [x] 3.2.1 更新工作流程章节
  - [x] 3.2.2 添加 OpenSpec 指令引用
  - [x] 3.2.3 更新文档更新规范

- [x] 3.3 更新 `README.md`
  - [x] 3.3.1 添加 OpenSpec 开发流程说明
  - [x] 3.3.2 更新贡献指南链接

## 4. 验证与测试

- [x] 4.1 运行 `openspec validate --strict` 确保所有规范通过
- [x] 4.2 运行 `openspec list` 确认变更可被识别
- [x] 4.3 运行 `openspec show migrate-docs-to-specs` 确认详情正确
- [x] 4.4 修正 design.md 文档独立性问题
  - [x] 修正 config-loading design.md 中错误的完成状态标记（[x] → [ ]）
  - [x] 移除所有 design.md 中对归档文档的外部引用
  - [x] 替换为公开的技术文档链接（如 API 文档、crate 文档）
  - [x] 确保新生成的规范文档独立完整，无需参考旧文档
- [x] 4.5 补充 ARCHITECTURE.md 中的伪代码到 design.md
  - [x] agent-system/design.md: 添加 Agent、AgentLoader 代码（来自 L111-140）
  - [x] tool-system/design.md: 添加 Tool trait、ToolRegistry 代码（来自 L206-251）
  - [x] message-model/design.md: 添加 ModelAdapter、StreamChunk 代码（来自 L166-301）
  - [x] anthropic-service/design.md: 添加 AnthropicService 代码（来自 L267-281）
  - [x] mcp-integration/design.md: 添加 McpClient 代码（来自 L306-314）
  - [x] tui-interface/design.md: 添加 App、ReplWidget 代码（来自 L319-365）
  - [x] cli-commands/design.md: 添加 main 函数代码（来自 L369-393）
  - [x] 所有代码标注来源行号，方便追溯

## 5. 迁移 ROADMAP 和 TODO 到 OpenSpec Changes

- [x] 5.1 创建 Phase 1 功能变更提案
  - [x] implement-config-loading (配置系统)
  - [x] implement-message-model (消息模型)
  - [x] implement-anthropic-service (Anthropic API)
  - [x] implement-file-tools (基础文件工具)
  - [x] implement-basic-cli (基础 CLI)

- [x] 5.2 创建 Phase 2 功能变更提案
  - [x] implement-streaming-response (流式响应)
  - [x] implement-bash-tool (Bash 工具)
  - [x] implement-search-tools (搜索工具)
  - [x] implement-agent-system (Agent 系统)
  - [x] implement-task-tool (Task 工具)
  - [x] implement-tui-interface (TUI 界面)
  - [x] implement-permission-system (权限系统)

- [x] 5.3 创建 Phase 3 功能变更提案
  - [x] implement-openai-service (OpenAI 服务)
  - [x] implement-model-switching (模型切换)
  - [x] implement-mcp-client (MCP 客户端基础)
  - [x] implement-context-management (上下文管理)
  - [x] implement-ui-enhancements (UI 美化)
  - [x] implement-mcp-full (MCP 完整功能)
  - [x] implement-performance-optimization (性能优化)

- [x] 5.4 创建 Phase 4 功能变更提案
  - [x] implement-cli-commands-full (CLI 命令完善)
  - [x] implement-comprehensive-tests (综合测试)
  - [x] implement-documentation (文档完善)
  - [x] implement-cicd (CI/CD)
  - [x] implement-cross-platform (跨平台支持)
  - [x] prepare-release-v0.1.0 (发布准备)

- [x] 5.5 删除归档文档
  - [x] 确认所有内容已迁移到 OpenSpec
  - [x] 删除 `docs/archive/ARCHITECTURE.md` (已迁移到 specs/*/design.md)
  - [x] 删除 `docs/archive/ROADMAP.md` (已迁移到 openspec/changes/*)
  - [x] 删除 `docs/archive/TODO.md` (已迁移到 openspec/changes/*/tasks.md)
  - [x] 删除 `docs/archive/TECH_STACK.md` (选型决策已在代码和设计中体现)
  - [x] 删除空的 `docs/archive/` 目录

- [x] 5.6 删除冗余项目文档
  - [x] 删除 `PROJECT_OVERVIEW.md` (与 README 重叠)
  - [x] 删除 `GOALS.md` (内容应在 openspec/project.md)
  - [x] 删除 `WORKFLOW.md` (完全使用 OpenSpec，抛弃传统瀑布流)

- [x] 5.7 更新文档引用
  - [x] 更新 `README.md` - 移除归档文档引用，指向 OpenSpec
  - [x] 更新 `CLAUDE.md` - 删除归档文档链接和传统瀑布流
  - [x] 验证无对归档文档的引用

## 6. 提交与审查

- [x] 6.1 提交 Git 变更
- [x] 6.2 创建 Pull Request
- [x] 6.3 等待审查批准

---

## 完成总结

**完成时间**: 2025-12-23

### 完成的工作

#### 1. 创建了 8 个功能规范
- config-loading - 配置加载系统
- message-model - 消息模型和适配器
- anthropic-service - Anthropic API 服务
- tool-system - 工具系统
- agent-system - Agent 系统
- mcp-integration - MCP 集成
- tui-interface - TUI 界面
- cli-commands - CLI 命令

#### 2. 创建了 26 个功能变更提案
- Phase 1 (5 个): 基础架构
- Phase 2 (7 个): 核心功能
- Phase 3 (7 个): 高级特性
- Phase 4 (6 个): 优化与发布
- + 1 个迁移变更

#### 3. 删除了所有归档和冗余文档
- docs/archive/* (4 个文档)
- PROJECT_OVERVIEW.md
- GOALS.md
- WORKFLOW.md

#### 4. 更新了文档引用
- README.md
- CLAUDE.md
- 删除所有归档文档引用

#### 5. 验证结果
- ✅ 34/34 OpenSpec items 验证通过
- ✅ 所有文档独立完整
- ✅ 无归档文档引用

### 关键成果

1. **完全使用 OpenSpec 管理项目**
   - 所有功能规划在 `openspec/changes/`
   - 所有技术规范在 `openspec/specs/`
   - 抛弃传统瀑布流，完全采用 OpenSpec 工作流

2. **文档清晰分离**
   - 用户文档: README.md
   - 开发文档: CLAUDE.md, CONTRIBUTING.md
   - 规范文档: openspec/

3. **无冗余和过时内容**
   - 删除所有归档文档
   - 删除冗余项目文档
   - 清理所有归档引用

### 最终文档结构

```
Kode-cli-rust/
├── README.md              # 项目入口 (~6KB)
├── CONTRIBUTING.md        # 贡献指南 (~10KB)
├── AGENTS.md              # OpenSpec 指南 (~0.7KB)
├── CLAUDE.md              # AI 工作指南 (~9KB)
│
└── openspec/
    ├── project.md         # 项目约定
    ├── specs/             # 8 个功能规范
    └── changes/           # 26 个变更提案
```
