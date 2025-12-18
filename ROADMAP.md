# Kode-Rust 开发路线图

## 📁 项目路径上下文

在开发每个功能时，请参考原始实现：

- **原始 TypeScript 版本**: `/Users/gemini/Documents/backup/Kode-cli`
- **当前 Rust 版本**: `/Users/gemini/Documents/backup/Kode-cli-rust`

**关键文件参考**:
- 原版 `CLAUDE.md` - 开发指导和最佳实践
- 原版 `src/` - TypeScript 实现细节
- 原版 `.kode.json` - 配置格式示例
- 原版 `.kode/agents/` - Agent 定义示例

## 📅 总体时间线

**总开发周期**: 10 周
**预计完成时间**: 根据实际开发速度调整

```
Week 1-2   │ Phase 1: 基础架构
Week 3-5   │ Phase 2: 核心功能
Week 6-8   │ Phase 3: 高级特性
Week 9-10  │ Phase 4: 优化与发布
```

## 🎯 Phase 1: 基础架构 (Week 1-2)

**目标**: 建立项目框架，实现基本可运行的程序

### Week 1: 项目初始化与核心抽象

#### Day 1-2: 项目骨架
- [x] 创建 Cargo workspace
- [ ] 初始化所有 crate 目录结构
- [ ] 配置 CI/CD (GitHub Actions)
- [ ] 设置 rustfmt 和 clippy
- [ ] 编写初始 README 和文档

**可交付**: 项目骨架，可编译通过

#### Day 3-4: 配置系统 (kode-core/config)
- [ ] 定义 `Config` 结构体
- [ ] 实现 JSON 序列化/反序列化
- [ ] 实现配置文件加载逻辑
  - [ ] 全局配置 (`~/.kode.json`)
  - [ ] 项目配置 (`./.kode.json`)
  - [ ] 配置合并逻辑
- [ ] 环境变量支持
- [ ] 单元测试

**可交付**: 能够加载和解析 `.kode.json` 配置

#### Day 5-7: 核心数据结构 (kode-core)
- [ ] 实现 `Message` 结构
- [ ] 实现 `ModelAdapter` trait
- [ ] 实现 `ModelProfile` 和 `ModelManager`
- [ ] 实现基础错误类型
- [ ] 单元测试

**可交付**: 核心数据结构完成

### Week 2: 基础工具与服务

#### Day 8-10: Anthropic 服务 (kode-services)
- [ ] 实现 `AnthropicService` 结构
- [ ] 实现 `ModelAdapter` for `AnthropicService`
- [ ] 实现简单的请求/响应 (非流式)
- [ ] 错误处理
- [ ] 集成测试 (需要 API key)

**可交付**: 可以调用 Claude API 获取响应

#### Day 11-12: 基础工具 (kode-tools)
- [ ] 定义 `Tool` trait
- [ ] 实现 `ToolRegistry`
- [ ] 实现 `FileReadTool`
- [ ] 实现 `FileWriteTool`
- [ ] 实现工具参数验证
- [ ] 单元测试

**可交付**: 基础文件操作工具可用

#### Day 13-14: 简单 CLI (kode-cli)
- [ ] 使用 clap 实现命令行解析
- [ ] 实现简单的非交互模式
- [ ] 集成配置、模型、工具
- [ ] 端到端测试：用户输入 -> AI 响应 -> 工具执行

**可交付**: 最小可用版本 (MVP)
```bash
kode "读取 src/main.rs 文件"
# AI 调用 FileReadTool，返回文件内容
```

### Phase 1 里程碑
✅ **核心框架建立**
✅ **可以调用 AI API**
✅ **基础工具可用**
✅ **端到端流程打通**

---

## 🚀 Phase 2: 核心功能 (Week 3-5)

**目标**: 实现主要功能，达到可日常使用的程度

### Week 3: 流式响应与更多工具

#### Day 15-17: 流式响应 (kode-services)
- [ ] 实现 SSE 流式解析
- [ ] 实现 `stream_message` 方法
- [ ] 实现 `StreamChunk` 枚举
- [ ] 实现流式响应收集器
- [ ] 处理流式错误和重连
- [ ] 集成测试

**可交付**: 支持流式 API 调用

#### Day 18-19: 更多工具 (kode-tools)
- [ ] 实现 `BashTool`
  - [ ] 使用 `tokio::process::Command`
  - [ ] 超时控制
  - [ ] 输出捕获
- [ ] 实现 `FileEditTool`
  - [ ] 精确字符串替换
  - [ ] 错误处理（未找到字符串）
- [ ] 实现 `GlobTool`
  - [ ] 使用 `glob` crate
  - [ ] 模式匹配
- [ ] 单元测试

**可交付**: 核心工具集完成

#### Day 20-21: GrepTool (kode-tools)
- [ ] 研究 ripgrep 集成方式
- [ ] 实现基于 regex 的简单搜索
- [ ] 支持文件类型过滤
- [ ] 支持上下文行 (-A, -B, -C)
- [ ] 性能优化
- [ ] 单元测试

**可交付**: 代码搜索功能可用

### Week 4: Agent 系统

#### Day 22-24: Agent 定义解析 (kode-core/agent)
- [ ] 实现 Markdown + YAML frontmatter 解析
  - [ ] 使用 `pulldown-cmark` 解析 markdown
  - [ ] 使用 `yaml-rust2` 解析 frontmatter
- [ ] 定义 `Agent` 结构体
- [ ] 实现 `ToolFilter` 枚举
- [ ] 单元测试（解析各种 Agent 文件）

**可交付**: 可以解析 Agent 定义文件

#### Day 25-27: Agent 加载器 (kode-core/agent)
- [ ] 实现 5 层目录加载逻辑
  1. Built-in agents
  2. `~/.claude/agents/`
  3. `~/.kode/agents/`
  4. `./.claude/agents/`
  5. `./.kode/agents/`
- [ ] 实现 LRU 缓存
- [ ] 实现 `AgentLoader`
- [ ] 处理加载错误和降级
- [ ] 单元测试和集成测试

**可交付**: Agent 加载系统完成

#### Day 28: TaskTool (kode-tools)
- [ ] 实现 `TaskTool`
- [ ] 加载子 Agent 配置
- [ ] 过滤工具集
- [ ] 创建子上下文
- [ ] 递归调用 AI
- [ ] 返回结果
- [ ] 集成测试

**可交付**: Agent 委托功能可用

### Week 5: TUI 界面

#### Day 29-31: 基础 TUI (kode-ui)
- [ ] 设置 ratatui 框架
- [ ] 实现 `App` 状态管理
- [ ] 实现 `ReplState`
- [ ] 实现消息列表 Widget
- [ ] 实现输入框 Widget
- [ ] 实现基础事件循环
- [ ] 键盘输入处理

**可交付**: 基础 REPL 界面

#### Day 32-33: 流式输出 (kode-ui)
- [ ] 集成流式 API 调用
- [ ] 实时更新消息列表
- [ ] 实现打字机效果
- [ ] 性能优化（避免过度渲染）
- [ ] 集成测试

**可交付**: 实时流式输出

#### Day 34-35: 权限系统 (kode-core + kode-ui)
- [ ] 实现 `PermissionManager`
- [ ] 实现权限检查逻辑
- [ ] 实现权限请求 UI 对话框
- [ ] 集成到工具执行流程
- [ ] 测试危险操作拦截

**可交付**: 权限系统完成

### Phase 2 里程碑
✅ **流式响应**
✅ **核心工具完整**
✅ **Agent 系统可用**
✅ **交互式 TUI**
✅ **权限控制**
✅ **日常可用**

---

## 🎨 Phase 3: 高级特性 (Week 6-8)

**目标**: 完善功能，达到与 TS 版本功能对等

### Week 6: 多模型支持与 MCP

#### Day 36-38: OpenAI-compatible 支持 (kode-services)
- [ ] 实现 `OpenAIService`
- [ ] 实现 `ModelAdapter` for `OpenAIService`
- [ ] 支持自定义 base URL
- [ ] 支持流式响应
- [ ] 测试 OpenAI, DeepSeek 等

**可交付**: 多模型提供商支持

#### Day 39-40: 模型切换 (kode-core)
- [ ] 实现运行时模型切换
- [ ] 实现模型指针（default, task, etc.）
- [ ] UI 显示当前模型
- [ ] 命令：`/model switch <name>`
- [ ] 集成测试

**可交付**: 运行时切换模型

#### Day 41-42: MCP 客户端基础 (kode-services)
- [ ] 研究 MCP 协议规范
- [ ] 实现 MCP 客户端连接
- [ ] 实现工具发现
- [ ] 实现工具调用
- [ ] 错误处理
- [ ] 集成测试（连接示例 MCP 服务器）

**可交付**: MCP 基础集成

### Week 7: 上下文管理与 UI 完善

#### Day 43-45: 消息上下文管理 (kode-core)
- [ ] 实现 `MessageContextManager`
- [ ] 实现 token 计数器
  - [ ] 简单实现：按字符估算
  - [ ] 高级实现：使用 tokenizer
- [ ] 实现智能裁剪策略
- [ ] 实现上下文优先级
- [ ] 单元测试

**可交付**: 智能上下文窗口管理

#### Day 46-48: UI 美化 (kode-ui)
- [ ] 实现语法高亮
  - [ ] 使用 `syntect` crate
  - [ ] 支持常见语言
- [ ] 实现代码块渲染
- [ ] 实现 Markdown 格式化
- [ ] 实现状态栏
  - [ ] 显示当前模型
  - [ ] 显示 token 使用量
  - [ ] 显示连接状态
- [ ] 实现加载动画

**可交付**: 美观的 TUI 界面

#### Day 49: 错误显示优化 (kode-ui)
- [ ] 友好的错误消息格式
- [ ] 错误类型高亮
- [ ] 堆栈追踪（可选显示）
- [ ] 错误恢复提示

**可交付**: 用户友好的错误显示

### Week 8: MCP 完善与性能优化

#### Day 50-52: MCP 完善 (kode-services)
- [ ] 实现 MCP 服务器管理
  - [ ] 启动/停止服务器
  - [ ] 健康检查
  - [ ] 自动重连
- [ ] 实现 MCP 工具动态加载
- [ ] 集成到 `ToolRegistry`
- [ ] 完整的 MCP 配置支持
- [ ] 集成测试

**可交付**: 完整的 MCP 支持

#### Day 53-55: 性能优化
- [ ] 并发工具执行
  - [ ] 识别独立工具调用
  - [ ] 并行执行
  - [ ] 结果聚合
- [ ] 缓存优化
  - [ ] Agent 缓存预热
  - [ ] 配置缓存
  - [ ] 工具结果缓存（可选）
- [ ] 内存优化
  - [ ] 流式处理大文件
  - [ ] 及时释放资源
  - [ ] 上下文裁剪
- [ ] 性能基准测试

**可交付**: 性能优化完成

#### Day 56: CLI 命令完善 (kode-cli)
- [ ] 实现 `kode config` 命令组
  - [ ] `config list`: 显示配置
  - [ ] `config set`: 设置配置
  - [ ] `config get`: 获取配置
- [ ] 实现 `kode agents` 命令
  - [ ] 列出所有可用 Agent
  - [ ] 显示 Agent 详情
- [ ] 实现 `kode tools` 命令
  - [ ] 列出所有工具
  - [ ] 显示工具详情
- [ ] 完善 `--help` 信息

**可交付**: 完整的 CLI 命令

### Phase 3 里程碑
✅ **多模型支持**
✅ **MCP 完全集成**
✅ **上下文智能管理**
✅ **UI 完善美观**
✅ **性能优化**
✅ **功能完整**

---

## 🎓 Phase 4: 优化与发布 (Week 9-10)

**目标**: 测试、优化、文档、发布

### Week 9: 测试与文档

#### Day 57-59: 测试完善
- [ ] 单元测试覆盖检查
  - [ ] 目标：核心逻辑 > 80%
  - [ ] 补充缺失测试
- [ ] 集成测试
  - [ ] 完整工作流测试
  - [ ] 多场景测试
- [ ] 兼容性测试
  - [ ] 加载 TS 版本配置
  - [ ] 加载 TS 版本 Agent
  - [ ] MCP 互操作测试
- [ ] 性能基准测试
  - [ ] 启动时间
  - [ ] 内存占用
  - [ ] UI 渲染性能

**可交付**: 完整的测试套件

#### Day 60-62: 文档完善
- [ ] 更新 README.md
  - [ ] 完整的功能介绍
  - [ ] 安装指南
  - [ ] 使用示例
- [ ] 用户手册
  - [ ] 配置指南
  - [ ] Agent 编写指南
  - [ ] 工具使用说明
  - [ ] 常见问题 FAQ
- [ ] 开发者文档
  - [ ] 架构说明
  - [ ] 贡献指南
  - [ ] 代码规范
- [ ] API 文档
  - [ ] rustdoc 注释
  - [ ] 生成文档站点

**可交付**: 完整的项目文档

#### Day 63: CI/CD 完善
- [ ] GitHub Actions 配置
  - [ ] 自动测试
  - [ ] 自动构建
  - [ ] 代码覆盖率
- [ ] Release 流程
  - [ ] 自动版本号
  - [ ] 自动生成 CHANGELOG
  - [ ] 自动发布 binaries

**可交付**: 完整的 CI/CD

### Week 10: 优化与发布

#### Day 64-66: 跨平台支持
- [ ] Linux 测试和优化
- [ ] macOS 测试和优化
- [ ] Windows 测试和优化
- [ ] 跨平台构建
  - [ ] x86_64-unknown-linux-gnu
  - [ ] x86_64-apple-darwin
  - [ ] aarch64-apple-darwin
  - [ ] x86_64-pc-windows-msvc

**可交付**: 跨平台 binaries

#### Day 67-68: 最终优化
- [ ] 代码审查
- [ ] 性能最终调优
- [ ] 内存泄漏检查
- [ ] 边界情况处理
- [ ] Clippy 警告清理
- [ ] 代码格式化

**可交付**: 生产就绪代码

#### Day 69-70: 发布准备
- [ ] 版本号确定 (v0.1.0)
- [ ] CHANGELOG 编写
- [ ] Release notes 编写
- [ ] 发布到 GitHub Releases
- [ ] 发布到 crates.io
- [ ] 社区公告
  - [ ] GitHub Discussions
  - [ ] Reddit r/rust
  - [ ] Twitter/X

**可交付**: 正式发布 v0.1.0

### Phase 4 里程碑
✅ **测试覆盖完整**
✅ **文档完善**
✅ **跨平台支持**
✅ **CI/CD 就绪**
✅ **正式发布**

---

## 📊 关键里程碑总结

| 里程碑 | 时间点 | 可交付成果 |
|--------|--------|-----------|
| **M1: MVP** | Week 2 | 最小可用版本，可调用 AI 和执行基础工具 |
| **M2: 核心功能** | Week 5 | 完整工具、Agent 系统、TUI 界面 |
| **M3: 功能完整** | Week 8 | 多模型、MCP、性能优化 |
| **M4: 正式发布** | Week 10 | 测试完善、文档齐全、跨平台发布 |

## 🔄 迭代与调整

### 风险与应对

1. **技术难点超预期**
   - **风险**: 某些功能（如 MCP）实现复杂度超预期
   - **应对**: 优先级调整，将次要功能推迟到后续版本

2. **兼容性问题**
   - **风险**: 与 TS 版本配置不完全兼容
   - **应对**: 早期进行兼容性测试，及时调整

3. **性能目标未达成**
   - **风险**: 性能优化效果不明显
   - **应对**: 引入性能基准测试，持续优化

4. **时间延误**
   - **风险**: 某个阶段延误影响后续计划
   - **应对**: 预留 buffer 时间，灵活调整优先级

### 优先级原则

1. **核心功能 > 高级特性**
2. **兼容性 > 新特性**
3. **稳定性 > 性能**
4. **用户体验 > 代码优雅**

### 版本规划

- **v0.1.0**: MVP，基础可用
- **v0.2.0**: 核心功能完整
- **v0.3.0**: 高级特性
- **v1.0.0**: 生产就绪，功能对等

## 📝 进度跟踪

### 更新频率
- 每日更新任务清单
- 每周回顾进度
- 每阶段总结经验

### 沟通机制
- GitHub Issues 跟踪任务
- GitHub Discussions 讨论技术方案
- README 更新项目状态

## 🎉 发布后计划

### v0.1.x 系列
- Bug 修复
- 小功能改进
- 文档补充

### v0.2.x 系列
- 性能进一步优化
- 用户反馈功能
- 社区贡献整合

### v1.0.0 目标
- 生产级稳定性
- 完整功能对等
- 丰富的文档和示例
- 活跃的社区

---

**最后更新**: 2025-12-18
**状态**: 🟡 规划阶段
