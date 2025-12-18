# Kode-Rust

> 🚀 高性能 AI Agent CLI 工具 - 使用 Rust 重写的 Kode

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)

**Kode-Rust** 是 [Kode-cli](https://github.com/shareAI-lab/kode) 的 Rust 完整重写版本，利用 Rust 的性能优势和类型安全特性，提供更快、更稳定的 AI 辅助开发体验。

## 📁 项目路径

- **原始 TypeScript 版本**: `/Users/gemini/Documents/backup/Kode-cli`
- **当前 Rust 版本**: `/Users/gemini/Documents/backup/Kode-cli-rust`

## 🎯 项目状态

**当前阶段**: 📝 规划与设计阶段

- [x] 完成需求调研
- [x] 完成技术选型
- [x] 完成架构设计
- [ ] Phase 1: 基础架构实现
- [ ] Phase 2: 核心功能开发
- [ ] Phase 3: 高级特性集成
- [ ] Phase 4: 优化与发布

查看完整路线图：[ROADMAP.md](ROADMAP.md)

## ✨ 核心特性

### 已规划功能

- 🤖 **多模型支持**: Anthropic Claude、OpenAI、DeepSeek 等
- 🔧 **强大的工具系统**: 文件操作、命令执行、代码搜索等
- 🎭 **灵活的 Agent 系统**: 支持自定义 Agent，任务委托
- 🔌 **MCP 协议集成**: 兼容 Model Context Protocol
- 🎨 **现代化 TUI 界面**: 基于 ratatui 的流畅交互体验
- ⚡ **流式响应**: 实时显示 AI 输出
- 🔐 **权限管理系统**: 安全的工具执行控制
- 💾 **配置兼容**: 完全兼容原 Kode-cli 的 `.kode.json`
- 📝 **Agent 兼容**: 支持相同的 markdown + YAML 格式

## 🎯 项目目标

详细目标请参考：[GOALS.md](GOALS.md)

### 核心目标

1. **完整功能移植**: 实现原 TypeScript 版本的所有核心功能
2. **性能提升**: 利用 Rust 的性能优势
   - 启动时间 < 100ms
   - 内存占用 < 50MB (idle)
   - UI 渲染 60 FPS
3. **完全兼容**: 配置文件、Agent 定义、MCP 协议与原版兼容
4. **更好的体验**: 更流畅的 TUI、更快的响应速度

## 🏗️ 架构设计

本项目采用模块化的 Cargo Workspace 架构：

```
kode-cli-rust/
├── crates/
│   ├── kode-core/      # 核心功能库（配置、Agent、上下文）
│   ├── kode-tools/     # 工具系统（Tool trait、工具实现）
│   ├── kode-services/  # 服务集成（API 客户端、MCP）
│   ├── kode-ui/        # TUI 界面（ratatui）
│   └── kode-cli/       # CLI 入口（命令行解析）
└── docs/               # 文档
```

详细架构说明：[ARCHITECTURE.md](ARCHITECTURE.md)

## 🛠️ 技术栈

详细技术选型：[TECH_STACK.md](TECH_STACK.md)

**核心技术**:
- **运行时**: Tokio (异步 IO)
- **TUI**: Ratatui (终端界面)
- **HTTP**: Reqwest (API 调用)
- **序列化**: Serde (配置、JSON)
- **错误处理**: Anyhow (应用错误)

## 🚀 快速开始

> ⚠️ 项目尚在开发中，以下命令仅为规划

### 安装

```bash
# 从源码构建
git clone https://github.com/shareAI-lab/kode-cli-rust.git
cd kode-cli-rust
cargo build --release

# 安装到系统
cargo install --path crates/kode-cli
```

### 配置

创建 `~/.kode.json`:

```json
{
  "modelProfiles": {
    "claude": {
      "provider": "anthropic",
      "apiKey": "sk-ant-...",
      "model": "claude-sonnet-4-5-20250929"
    }
  },
  "defaultModel": "claude"
}
```

### 使用

```bash
# 交互模式
kode

# 单次执行
kode "帮我重构这个函数"

# 查看配置
kode config list

# 查看可用 Agent
kode agents
```

## 📚 文档

- [项目目标](GOALS.md) - 详细的目标和需求
- [开发路线图](ROADMAP.md) - 分阶段开发计划
- [架构设计](ARCHITECTURE.md) - 系统架构说明
- [技术栈](TECH_STACK.md) - 技术选型理由
- [贡献指南](CONTRIBUTING.md) - 如何参与开发

## 🤝 贡献

欢迎贡献！请先阅读 [CONTRIBUTING.md](CONTRIBUTING.md)。

## 📋 开发计划

### Phase 1: 基础架构 (Week 1-2)
- [ ] Workspace 初始化
- [ ] 核心数据结构
- [ ] 配置系统
- [ ] 基础工具实现

### Phase 2: 核心功能 (Week 3-5)
- [ ] 多模型支持
- [ ] Agent 系统
- [ ] 工具系统完善
- [ ] TUI 界面

### Phase 3: 高级特性 (Week 6-8)
- [ ] MCP 集成
- [ ] 上下文管理
- [ ] UI 完善
- [ ] 性能优化

### Phase 4: 优化与发布 (Week 9-10)
- [ ] 测试覆盖
- [ ] 文档完善
- [ ] CI/CD
- [ ] 发布

详细计划：[ROADMAP.md](ROADMAP.md)

## 🌟 与原版对比

| 特性 | Kode-cli (TS) | Kode-Rust |
|------|---------------|-----------|
| 启动时间 | ~500ms | **~100ms** (目标) |
| 内存占用 | ~150MB | **~50MB** (目标) |
| 配置兼容 | ✅ | ✅ |
| Agent 兼容 | ✅ | ✅ |
| MCP 支持 | ✅ | ✅ |
| 类型安全 | TypeScript | **Rust** |
| 并发性能 | Node.js | **Tokio** |

## 📄 许可证

Apache License 2.0 - 详见 [LICENSE](LICENSE)

## 🙏 致谢

- 原始项目: [Kode-cli](https://github.com/shareAI-lab/kode)
- Rust 社区和生态系统
- Ratatui 项目

## 📮 联系方式

- Issues: [GitHub Issues](https://github.com/shareAI-lab/kode-cli-rust/issues)
- Email: ai-lab@foxmail.com

---

**注意**: 本项目目前处于积极开发阶段，API 和功能可能会有变化。
