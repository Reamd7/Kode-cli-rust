# Kode-Rust 项目概览

## 📁 项目文档结构

本项目包含完整的规划和设计文档，涵盖项目目标、技术选型、架构设计、开发路线等各个方面。

### 核心文档

| 文档 | 用途 | 读者 |
|------|------|------|
| [README.md](README.md) | 项目介绍、快速开始 | 所有人 |
| [GOALS.md](GOALS.md) | 详细的项目目标和成功标准 | 开发者、项目管理者 |
| [ROADMAP.md](ROADMAP.md) | 10周开发路线图 | 开发者、项目管理者 |
| [ARCHITECTURE.md](ARCHITECTURE.md) | 系统架构设计 | 开发者 |
| [TECH_STACK.md](TECH_STACK.md) | 技术栈选型理由 | 开发者 |
| [CONTRIBUTING.md](CONTRIBUTING.md) | 贡献指南 | 贡献者 |

## 🎯 项目定位

**Kode-Rust** 是使用 Rust 完整重写的 AI Agent CLI 工具，基于原 TypeScript 版本的 [Kode-cli](https://github.com/shareAI-lab/kode)。

### 项目路径
- **原始 TypeScript 版本**: `/Users/gemini/Documents/backup/Kode-cli`
- **当前 Rust 版本**: `/Users/gemini/Documents/backup/Kode-cli-rust`

**开发提示**: 实现功能时应频繁参考原 TS 版本代码，确保行为一致性和配置兼容性。

### 核心价值主张

1. **性能提升**: 利用 Rust 的性能优势
   - 启动时间 < 100ms（原版 ~500ms）
   - 内存占用 < 50MB（原版 ~150MB）
   - 更快的工具执行和 UI 渲染

2. **完全兼容**: 与原版配置和 Agent 定义 100% 兼容
   - 可直接使用原版 `.kode.json`
   - 可直接使用原版 Agent 定义
   - 支持相同的 MCP 协议

3. **功能完整**: 实现所有核心功能
   - 多模型支持
   - 强大的工具系统
   - 灵活的 Agent 系统
   - 现代化 TUI 界面

## 🏗️ 技术架构

### Cargo Workspace 结构

```
kode-cli-rust/
├── crates/
│   ├── kode-core      # 核心功能（配置、Agent、模型）
│   ├── kode-tools     # 工具系统
│   ├── kode-services  # API 集成、MCP
│   ├── kode-ui        # TUI 界面
│   └── kode-cli       # CLI 入口
└── docs/              # 文档
```

### 技术栈

- **运行时**: Tokio（异步 IO）
- **TUI**: Ratatui（终端界面）
- **HTTP**: Reqwest（API 调用）
- **序列化**: Serde（配置、JSON）

详见：[TECH_STACK.md](TECH_STACK.md)

## 📅 开发计划

### 10周开发路线

| 阶段 | 时间 | 目标 | 状态 |
|------|------|------|------|
| **Phase 1** | Week 1-2 | 基础架构 | 🟡 规划中 |
| **Phase 2** | Week 3-5 | 核心功能 | 🔴 未开始 |
| **Phase 3** | Week 6-8 | 高级特性 | 🔴 未开始 |
| **Phase 4** | Week 9-10 | 优化发布 | 🔴 未开始 |

详见：[ROADMAP.md](ROADMAP.md)

### 关键里程碑

- **M1 (Week 2)**: MVP - 可调用 AI 和执行基础工具
- **M2 (Week 5)**: 核心功能完整 - 可日常使用
- **M3 (Week 8)**: 功能完整 - 与 TS 版本功能对等
- **M4 (Week 10)**: 正式发布 - v0.1.0

## 🎯 项目目标

### 功能目标

- ✅ 多模型支持（Anthropic、OpenAI-compatible）
- ✅ 完整工具系统（文件、命令、搜索等）
- ✅ Agent 系统（动态加载、任务委托）
- ✅ MCP 协议集成
- ✅ 现代化 TUI 界面
- ✅ 权限管理系统

### 性能目标

- 启动时间 < 100ms
- 内存占用 < 50MB（idle）
- UI 渲染 60 FPS
- 工具执行开销 < 5ms

### 兼容性目标

- 100% 配置文件兼容
- 100% Agent 定义兼容
- 100% MCP 协议兼容

详见：[GOALS.md](GOALS.md)

## 🛠️ 快速开始

### 对于用户

> ⚠️ 项目尚在开发中，以下内容为规划

```bash
# 安装（未来）
cargo install kode-cli-rust

# 使用
kode
```

### 对于开发者

```bash
# 克隆项目
git clone https://github.com/shareAI-lab/kode-cli-rust.git
cd kode-cli-rust

# 构建
cargo build

# 运行测试
cargo test

# 运行（开发模式）
cargo run --bin kode
```

详见：[CONTRIBUTING.md](CONTRIBUTING.md)

## 📊 当前状态

### 已完成

- [x] 需求调研和分析
- [x] 技术栈选型
- [x] 架构设计
- [x] 详细规划文档
- [x] 开发路线图

### 进行中

- [ ] 项目初始化
- [ ] 基础框架搭建

### 待开始

- [ ] 核心功能开发
- [ ] 高级特性实现
- [ ] 测试和优化
- [ ] 文档完善
- [ ] 正式发布

## 📚 文档阅读指南

### 新手入门

1. 先读 [README.md](README.md) 了解项目概况
2. 再读 [GOALS.md](GOALS.md) 了解项目目标
3. 浏览 [ROADMAP.md](ROADMAP.md) 了解开发计划

### 开发者

1. 阅读 [ARCHITECTURE.md](ARCHITECTURE.md) 了解系统架构
2. 阅读 [TECH_STACK.md](TECH_STACK.md) 了解技术选型
3. 阅读 [CONTRIBUTING.md](CONTRIBUTING.md) 开始贡献

### 项目管理者

1. [GOALS.md](GOALS.md) - 了解目标和成功标准
2. [ROADMAP.md](ROADMAP.md) - 跟踪开发进度
3. [ARCHITECTURE.md](ARCHITECTURE.md) - 评估技术方案

## 🔄 与原版对比

### Kode-cli (TypeScript) vs Kode-Rust

| 方面 | Kode-cli | Kode-Rust | 提升 |
|------|----------|-----------|------|
| 启动时间 | ~500ms | ~100ms ⏱️ | **5x** |
| 内存占用 | ~150MB | ~50MB 💾 | **3x** |
| 配置兼容 | ✅ | ✅ | 100% |
| Agent兼容 | ✅ | ✅ | 100% |
| MCP支持 | ✅ | ✅ | 100% |
| 类型安全 | TypeScript | **Rust** 🦀 | 更强 |
| 并发性能 | Node.js | **Tokio** ⚡ | 更优 |

## 🚀 未来展望

### v0.1.x - MVP 阶段
- 基础功能可用
- 与 TS 版本功能对等
- 社区反馈收集

### v0.2.x - 优化阶段
- 性能优化
- 用户体验改进
- Bug 修复

### v1.0.0 - 正式发布
- 生产级稳定性
- 完整文档
- 活跃社区

### 后续可能的方向

1. **插件系统**: 动态加载工具
2. **Web UI**: 可选的浏览器界面
3. **IDE 集成**: LSP、VSCode 插件
4. **分布式 Agent**: 跨机器协作
5. **代码图谱**: 向量数据库、AI 索引

## 🤝 参与方式

### 报告问题
- GitHub Issues: 报告 bug、提出功能建议

### 贡献代码
- Fork & Pull Request
- 阅读 [CONTRIBUTING.md](CONTRIBUTING.md)

### 讨论交流
- GitHub Discussions: 技术讨论、问题解答

### 关注更新
- Star 项目
- Watch releases

## 📮 联系方式

- **GitHub**: [shareAI-lab/kode-cli-rust](https://github.com/shareAI-lab/kode-cli-rust)
- **Email**: ai-lab@foxmail.com
- **原项目**: [shareAI-lab/kode](https://github.com/shareAI-lab/kode)

## 📄 许可证

Apache License 2.0

## 🙏 致谢

- 原始项目 Kode-cli 及其开发者
- Rust 社区和生态系统
- Ratatui、Tokio 等优秀的开源项目

---

**项目状态**: 🟡 规划阶段
**最后更新**: 2025-12-18

欢迎关注项目进展！⭐
