# Kode-Rust 开发任务清单

**更新时间**: 2025-12-18 20:30
**当前阶段**: Phase 1 - 规划完成，准备开发
**总体进度**: 5% (规划阶段完成)

## 📁 项目路径上下文

**重要**: 开发过程中经常需要参考原始实现

- **原始 TypeScript 版本**: `/Users/gemini/Documents/backup/Kode-cli`
  - 查看原版实现逻辑
  - 理解配置格式（`.kode.json`）
  - 理解 Agent 定义格式
  - 查看 CLAUDE.md 获取开发指导
  - 测试兼容性

- **当前 Rust 版本**: `/Users/gemini/Documents/backup/Kode-cli-rust`
  - 新实现的代码
  - 测试和文档

---

## 📊 总体进度概览

| 阶段 | 进度 | 状态 | 完成任务 | 总任务 |
|------|------|------|----------|--------|
| **Phase 1** | 0% | 🔴 未开始 | 0 | 10 |
| **Phase 2** | 0% | 🔴 未开始 | 0 | 15 |
| **Phase 3** | 0% | 🔴 未开始 | 0 | 12 |
| **Phase 4** | 0% | 🔴 未开始 | 0 | 8 |
| **总计** | 5% | 🟡 规划中 | 0 | 45 |

---

## 🎉 规划阶段完成

### ✅ [规划] 项目文档创建

**完成时间**: 2025-12-18 20:30
**总用时**: 约 3 小时

#### 已完成的文档
- ✅ README.md (4.9KB) - 项目介绍
- ✅ GOALS.md (9.1KB) - 项目目标
- ✅ ROADMAP.md (12KB) - 开发路线图
- ✅ ARCHITECTURE.md (16KB) - 架构设计
- ✅ TECH_STACK.md (9.3KB) - 技术选型
- ✅ CONTRIBUTING.md (9.4KB) - 贡献指南
- ✅ PROJECT_OVERVIEW.md (6.1KB) - 项目概览
- ✅ CLAUDE.md (11KB) - AI Agent 工作指南
- ✅ WORKFLOW.md (13KB) - 工作流程说明
- ✅ TODO.md (本文件) - 任务清单

**总计**: 约 90KB 的完整规划文档

#### 规划成果
1. 明确了项目目标和范围
2. 制定了 10 周开发计划
3. 设计了完整的系统架构
4. 确定了技术栈和依赖
5. 建立了工作流程和规范

---

## 🚧 Phase 1: 基础架构 (Week 1-2)

**目标**: 建立项目框架，实现基本可运行的程序
**预计时间**: 2 周（14 天）
**状态**: 🟡 进行中

### Week 1: 项目初始化与核心抽象

#### [x] Day 1-2: 项目骨架

**预估时间**: 1 天
**实际用时**: 0.5 天
**完成时间**: 2025-12-18 22:00
**状态**: ✅ 已完成

**子任务**:
- [x] 创建 Cargo workspace
- [x] 初始化 5 个 crate 目录结构
  - [x] kode-core
  - [x] kode-tools
  - [x] kode-services
  - [x] kode-ui
  - [x] kode-cli
- [x] 配置 Workspace Cargo.toml
- [x] 配置各 crate 的 Cargo.toml
- [x] 创建 .gitignore
- [x] 创建 LICENSE 文件
- [ ] 配置 GitHub Actions CI/CD (后续)
- [x] 设置 rustfmt 和 clippy 配置
- [x] 验证项目可以编译

**验收标准**:
- [x] `cargo build` 成功
- [x] `cargo test` 成功（空测试）
- [ ] CI 配置有效 (后续)

**完成的工作**:
- ✅ 创建完整的 Cargo workspace 结构
- ✅ 配置所有 5 个 crate 的 Cargo.toml
- ✅ 创建基础的模块结构和占位代码
- ✅ 配置 rustfmt.toml 和 clippy.toml
- ✅ 项目可以成功编译和运行
- ✅ 通过所有 Clippy 检查（无警告）
- ✅ 代码已格式化

**关键文件位置**:
- `Cargo.toml` - Workspace 配置（90 行）
- `crates/kode-core/` - 核心库基础结构
- `crates/kode-tools/` - 工具系统基础
- `crates/kode-services/` - 服务集成基础
- `crates/kode-ui/` - UI 库基础
- `crates/kode-cli/src/main.rs` - CLI 入口

**遇到的问题**:
1. workspace 依赖不能标记为 optional - 已修复
2. rustfmt 某些配置需要 nightly - 已调整
3. Clippy 建议使用 derive(Default) - 已采纳

**经验教训**:
1. Workspace 配置需要注意依赖的 optional 标记
2. 使用 derive 宏可以减少样板代码
3. 项目骨架搭建速度很快，为后续开发打好基础

---

#### [ ] Day 3-4: 配置系统 (kode-core/config)

**预估时间**: 2 天
**状态**: 未开始

**子任务**:
- [ ] 定义 `Config` 结构体
  - [ ] ModelProfile 定义
  - [ ] McpServer 定义
  - [ ] 所有配置字段
- [ ] 实现 JSON 序列化/反序列化
  - [ ] serde 派生宏
  - [ ] camelCase 字段命名
- [ ] 实现配置文件加载逻辑
  - [ ] 全局配置 `~/.kode.json`
  - [ ] 项目配置 `./.kode.json`
  - [ ] 配置文件查找
- [ ] 实现配置合并逻辑
  - [ ] 项目配置覆盖全局配置
  - [ ] 字段级合并
- [ ] 环境变量支持（可选）
- [ ] 单元测试（至少 10 个）
  - [ ] 默认配置测试
  - [ ] 加载测试
  - [ ] 合并测试
  - [ ] 错误处理测试
- [ ] 集成测试（至少 3 个）
- [ ] Rustdoc 文档

**验收标准**:
- 可以加载原版 `.kode.json` 文件
- 配置合并逻辑正确
- 所有测试通过
- 无 clippy 警告

**文件位置**:
- `crates/kode-core/src/config/mod.rs`
- `crates/kode-core/src/config/types.rs`
- `crates/kode-core/src/config/tests.rs`

---

#### [ ] Day 5-7: 核心数据结构 (kode-core)

**预估时间**: 3 天
**状态**: 未开始

**子任务**:
- [ ] 实现 `Message` 结构体
  - [ ] Role (user, assistant, system)
  - [ ] Content (text, tool_use, tool_result)
  - [ ] 序列化支持
- [ ] 实现 `ModelAdapter` trait
  - [ ] send_message 方法
  - [ ] stream_message 方法
  - [ ] trait 文档
- [ ] 实现 `ModelProfile` 和 `ModelManager`
  - [ ] ModelProfile 结构
  - [ ] ModelManager 管理多个 adapter
  - [ ] 模型切换逻辑
- [ ] 实现基础错误类型
  - [ ] 使用 anyhow::Error
  - [ ] 常见错误情况
- [ ] 单元测试

**验收标准**:
- Message 可以正确序列化
- ModelAdapter trait 设计合理
- ModelManager 可以管理多个模型
- 所有测试通过

**文件位置**:
- `crates/kode-core/src/message.rs`
- `crates/kode-core/src/model/adapter.rs`
- `crates/kode-core/src/model/manager.rs`
- `crates/kode-core/src/error.rs`

---

### Week 2: 基础工具与服务

#### [ ] Day 8-10: Anthropic 服务 (kode-services)

**预估时间**: 3 天
**状态**: 未开始

**子任务**:
- [ ] 实现 `AnthropicService` 结构
  - [ ] reqwest::Client 封装
  - [ ] API key 管理
  - [ ] base URL 配置
- [ ] 实现 `ModelAdapter` for `AnthropicService`
  - [ ] send_message 实现（非流式）
  - [ ] 请求构建
  - [ ] 响应解析
- [ ] 错误处理
  - [ ] API 错误
  - [ ] 网络错误
  - [ ] 超时处理
- [ ] 集成测试（需要 API key）
  - [ ] 简单对话测试
  - [ ] 工具调用测试
  - [ ] 错误场景测试

**验收标准**:
- 可以成功调用 Claude API
- 可以解析 API 响应
- 错误处理完善
- 集成测试通过

**文件位置**:
- `crates/kode-services/src/anthropic.rs`
- `crates/kode-services/src/anthropic/types.rs`
- `tests/anthropic_integration.rs`

---

#### [ ] Day 11-12: 基础工具 (kode-tools)

**预估时间**: 2 天
**状态**: 未开始

**子任务**:
- [ ] 定义 `Tool` trait
  - [ ] name() 方法
  - [ ] description() 方法
  - [ ] schema() 方法
  - [ ] execute() 方法
  - [ ] requires_permission() 方法
- [ ] 实现 `ToolRegistry`
  - [ ] 工具注册
  - [ ] 工具查找
  - [ ] 工具列表
  - [ ] 工具过滤
- [ ] 实现 `FileReadTool`
  - [ ] 读取文件内容
  - [ ] 支持行号范围
  - [ ] 错误处理
- [ ] 实现 `FileWriteTool`
  - [ ] 写入文件内容
  - [ ] 创建目录
  - [ ] 权限检查
- [ ] 实现工具参数验证
  - [ ] JSON Schema 验证
- [ ] 单元测试

**验收标准**:
- Tool trait 设计合理
- ToolRegistry 可以管理工具
- FileReadTool 和 FileWriteTool 可用
- 所有测试通过

**文件位置**:
- `crates/kode-tools/src/tool.rs`
- `crates/kode-tools/src/registry.rs`
- `crates/kode-tools/src/file_read.rs`
- `crates/kode-tools/src/file_write.rs`

---

#### [ ] Day 13-14: 简单 CLI (kode-cli)

**预估时间**: 2 天
**状态**: 未开始

**子任务**:
- [ ] 使用 clap 实现命令行解析
  - [ ] 主命令结构
  - [ ] 子命令定义
  - [ ] 参数定义
- [ ] 实现简单的非交互模式
  - [ ] 接受用户输入
  - [ ] 调用 AI API
  - [ ] 执行工具
  - [ ] 输出结果
- [ ] 集成配置、模型、工具
  - [ ] 加载配置
  - [ ] 初始化模型
  - [ ] 注册工具
- [ ] 端到端测试
  - [ ] 用户输入 -> AI 响应
  - [ ] 工具调用 -> 工具执行
  - [ ] 完整流程测试

**验收标准**:
- 可以运行 `kode "帮我读取文件"`
- AI 可以调用 FileReadTool
- 工具执行成功并返回结果
- 端到端流程打通

**文件位置**:
- `crates/kode-cli/src/main.rs`
- `crates/kode-cli/src/cli.rs`
- `tests/e2e_basic.rs`

---

### Phase 1 里程碑

**M1: MVP 可用**
- ✅ 项目框架建立
- ✅ 可以加载配置
- ✅ 可以调用 AI API
- ✅ 基础工具可用
- ✅ 端到端流程打通

**交付物**:
- 可编译运行的 MVP
- 基础文档
- 单元测试 + 集成测试

---

## 🔴 Phase 2: 核心功能 (Week 3-5)

**目标**: 实现主要功能，达到可日常使用
**预计时间**: 3 周
**状态**: 🔴 未开始

### 待开发任务（15 个）
- [ ] 流式响应实现
- [ ] BashTool
- [ ] FileEditTool
- [ ] GrepTool
- [ ] GlobTool
- [ ] Agent 定义解析
- [ ] Agent 加载器
- [ ] TaskTool
- [ ] 基础 TUI 界面
- [ ] 流式输出显示
- [ ] 权限管理系统
- [ ] 更多模型支持
- [ ] 完整测试套件
- [ ] 文档完善
- [ ] 性能优化

详见 [ROADMAP.md](ROADMAP.md) Phase 2 章节。

---

## 🔴 Phase 3: 高级特性 (Week 6-8)

**目标**: 功能完整，与 TS 版本功能对等
**预计时间**: 3 周
**状态**: 🔴 未开始

### 待开发任务（12 个）
- [ ] OpenAI-compatible 适配器
- [ ] 多模型切换
- [ ] MCP 客户端
- [ ] MCP 工具加载
- [ ] 上下文管理
- [ ] UI 语法高亮
- [ ] UI 完善美化
- [ ] 性能优化
- [ ] CLI 命令完善
- [ ] 并发工具执行
- [ ] 缓存优化
- [ ] 完整测试

详见 [ROADMAP.md](ROADMAP.md) Phase 3 章节。

---

## 🔴 Phase 4: 优化与发布 (Week 9-10)

**目标**: 测试、优化、文档、发布
**预计时间**: 2 周
**状态**: 🔴 未开始

### 待开发任务（8 个）
- [ ] 测试完善
- [ ] 文档完善
- [ ] CI/CD 完善
- [ ] 跨平台构建
- [ ] 性能优化
- [ ] 代码审查
- [ ] 发布准备
- [ ] 正式发布 v0.1.0

详见 [ROADMAP.md](ROADMAP.md) Phase 4 章节。

---

## 📝 工作记录

### 2025-12-18

**时间**: 17:30 - 20:30
**阶段**: 规划阶段
**完成**: 所有规划文档

#### 工作内容
1. 需求调研和讨论
2. 技术选型确认
3. 创建完整规划文档
4. 建立工作流程

#### 产出文档
- README.md
- GOALS.md
- ROADMAP.md
- ARCHITECTURE.md
- TECH_STACK.md
- CONTRIBUTING.md
- PROJECT_OVERVIEW.md
- CLAUDE.md
- WORKFLOW.md
- TODO.md

#### 下一步
准备开始 Phase 1 Day 1-2: 项目骨架搭建

---

## 🎯 近期计划

### 本周计划（Week 1）
- [ ] Day 1-2: 项目骨架
- [ ] Day 3-4: 配置系统
- [ ] Day 5-7: 核心数据结构

### 下周计划（Week 2）
- [ ] Day 8-10: Anthropic 服务
- [ ] Day 11-12: 基础工具
- [ ] Day 13-14: 简单 CLI

---

## 📌 注意事项

### 开发规范
1. ✅ 严格遵循 ROADMAP.md
2. ✅ 每个任务完成后更新本文件
3. ✅ 所有代码必须有测试
4. ✅ 所有公开 API 必须有文档
5. ✅ 提交前运行完整检查

### 质量标准
- 测试覆盖率 > 80%
- Clippy 无警告
- 文档覆盖 100%
- 性能符合目标

### 工作流
遵循 6 步工作流（详见 [WORKFLOW.md](WORKFLOW.md)）：
1. Plan（计划）
2. Execute（执行）
3. Test（测试）
4. Summary（总结）
5. Reflect（反思）
6. Update（更新）

---

## 🚀 开始开发

准备好了吗？让我们开始构建 Kode-Rust！

**下一个任务**: Phase 1 Day 1-2 - 项目骨架搭建

阅读 [WORKFLOW.md](WORKFLOW.md) 了解详细工作流程。
阅读 [CLAUDE.md](CLAUDE.md) 了解 AI Agent 工作指南。

**让我们开始吧！** 🦀
