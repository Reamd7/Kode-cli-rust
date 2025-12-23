# Change: 将项目文档迁移至 OpenSpec 规范

## Why

当前项目使用多个 Markdown 文档（TODO.md、ROADMAP.md、ARCHITECTURE.md 等）来管理开发计划和规范，这种方式存在以下问题：

1. **缺乏结构化验证** - 文档内容无法自动验证是否符合规范
2. **与实现脱节** - 文档更新经常滞后于代码实现
3. **缺乏标准格式** - 需求描述格式不统一，难以追踪
4. **手动归档困难** - 完成的功能没有规范化的归档流程

通过迁移到 OpenSpec 规范，可以获得：
- 结构化的需求定义（Requirements + Scenarios）
- 自动验证（`openspec validate`）
- 标准化的变更提案流程（Proposal → Apply → Archive）
- 需求与实现的清晰映射

## What Changes

### 新增内容

- **创建 OpenSpec 规范文件**（`openspec/specs/*/spec.md`）
  - `config-loading` - 配置加载系统
  - `message-model` - 消息与模型抽象
  - `anthropic-service` - Anthropic API 服务
  - `tool-system` - 工具系统（Tool trait + Registry）
  - `agent-system` - Agent 定义与加载
  - `mcp-integration` - MCP 协议集成
  - `tui-interface` - TUI 用户界面
  - `cli-commands` - CLI 命令

### 归档内容

- 将现有规划文档移至 `docs/archive/` 目录：
  - `TODO.md` → `docs/archive/TODO.md`
  - `ROADMAP.md` → `docs/archive/ROADMAP.md`
  - `ARCHITECTURE.md` → `docs/archive/ARCHITECTURE.md`
  - `TECH_STACK.md` → `docs/archive/TECH_STACK.md`

### 保留文档

- `README.md` - 项目介绍（保持不变）
- `CLAUDE.md` - AI Agent 工作指南（更新引用 OpenSpec）
- `CONTRIBUTING.md` - 贡献指南（保持不变）
- `PROJECT_OVERVIEW.md` - 项目概览（保持不变）
- `WORKFLOW.md` - 更新为使用 OpenSpec 工作流

## Impact

### 影响的规范 (Affected Specs)

- **无**（这是首次创建 OpenSpec 规范）

### 影响的代码 (Affected Code)

- **无直接影响** - 这是文档迁移，不涉及代码变更
- **间接影响** - 后续开发将遵循 OpenSpec 流程

### 工作流变更

**之前的流程**:
```
1. 阅读 TODO.md/ROADMAP.md
2. 直接编码实现
3. 手动更新 TODO.md
```

**新的流程**:
```
1. 使用 openspec list 查看当前规范和变更
2. 创建 openspec proposal（如需新增功能）
3. 等待批准后实现 (openspec apply)
4. 归档完成的变更 (openspec archive)
```

### 文档结构变更

**之前的结构**:
```
Kode-cli-rust/
├── README.md
├── TODO.md          # 手动维护的任务清单
├── ROADMAP.md       # 手动维护的路线图
├── ARCHITECTURE.md  # 静态架构文档
└── ...
```

**新的结构**:
```
Kode-cli-rust/
├── README.md
├── openspec/
│   ├── project.md       # 项目规范（已有）
│   ├── specs/           # 当前已实现的功能规范
│   │   ├── config-loading/spec.md
│   │   ├── message-model/spec.md
│   │   └── ...
│   ├── changes/         # 进行中的变更提案
│   │   └── migrate-docs-to-specs/
│   └── archive/         # 已完成的变更
└── docs/archive/
    ├── TODO.md          # 归档的旧文档
    ├── ROADMAP.md
    └── ARCHITECTURE.md
```

## 迁移策略

### 阶段 1: 创建基础规范（本次变更）

1. 为已实现的功能创建 spec（基于当前代码状态）
2. 归档旧文档到 `docs/archive/`
3. 更新 WORKFLOW.md 和 CLAUDE.md

### 阶段 2: 创建功能规范（后续变更）

1. 为 ROADMAP 中的每个功能创建独立的 proposal
2. 按优先级逐步实现
3. 使用 OpenSpec 流程跟踪进度

## 验收标准

- [ ] 所有已实现的功能都有对应的 spec.md
- [ ] 所有 spec 通过 `openspec validate --strict` 验证
- [ ] 旧文档已归档到 `docs/archive/`
- [ ] CLAUDE.md 和 WORKFLOW.md 已更新
- [ ] 项目文档（README.md）更新了新的工作流说明
