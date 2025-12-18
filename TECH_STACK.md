# Kode-Rust 技术栈

本文档详细说明 Kode-Rust 项目的技术选型及其理由。

## 🦀 核心技术

### 编程语言: Rust

**版本**: 1.75+
**Edition**: 2021

**选择理由**:
1. **性能**: 接近 C/C++ 的性能，无 GC 开销
2. **内存安全**: 编译期保证内存安全，无需运行时检查
3. **并发**: 所有权系统保证并发安全
4. **生态**: 丰富的 crate 生态，成熟的工具链
5. **重写目标**: 项目目标是用 Rust 重写 TypeScript 版本

### 异步运行时: Tokio

**Crate**: `tokio = { version = "1.35", features = ["full"] }`

**选择理由**:
1. **事实标准**: Rust 异步生态的事实标准运行时
2. **完整功能**: 提供完整的异步 IO 支持
3. **性能优异**: 高效的任务调度和 IO 复用
4. **生态成熟**: 大量 crate 基于 Tokio 构建
5. **适合场景**: AI API 调用是网络密集型操作

**主要用途**:
- HTTP 请求 (AI API 调用)
- 流式响应处理
- 文件异步 IO
- 进程管理 (Bash 工具)
- 并发工具执行

### TUI 框架: Ratatui

**Crate**: `ratatui = "0.28"`
**辅助**: `crossterm = "0.27"` (终端控制)

**选择理由**:
1. **现代化**: tui-rs 的继任者，活跃维护
2. **组件化**: 类似 React 的 Widget 系统
3. **性能**: 高效的渲染算法
4. **灵活**: 完全控制布局和渲染
5. **生态**: 丰富的 Widget 库

**对比其他选项**:
- ~~cursive~~: 更高级但灵活性较低
- ~~crossterm 原生~~: 需要自己实现所有 UI 逻辑

## 🔧 核心依赖

### HTTP 客户端: Reqwest

**Crate**: `reqwest = { version = "0.12", features = ["json", "stream"] }`

**选择理由**:
1. **功能完整**: 支持 HTTP/2、TLS、代理等
2. **异步友好**: 基于 Tokio 构建
3. **流式支持**: 原生支持 SSE 和流式响应
4. **易用**: 简洁的 API
5. **成熟稳定**: Rust HTTP 客户端首选

**主要用途**:
- Anthropic API 调用
- OpenAI-compatible API 调用
- MCP 服务器通信 (HTTP transport)
- SSE 流式响应处理

### 序列化: Serde

**Crate**:
- `serde = { version = "1.0", features = ["derive"] }`
- `serde_json = "1.0"` (JSON)
- `serde_yaml = "0.9"` (YAML)

**选择理由**:
1. **事实标准**: Rust 序列化/反序列化标准
2. **性能**: 零拷贝反序列化
3. **类型安全**: 编译期类型检查
4. **灵活**: 支持自定义序列化逻辑
5. **生态**: 几乎所有格式都有 serde 支持

**主要用途**:
- 配置文件解析 (JSON)
- Agent 定义解析 (YAML frontmatter)
- API 请求/响应 (JSON)
- 工具参数验证

### 错误处理: Anyhow

**Crate**: `anyhow = "1.0"`

**选择理由**:
1. **简单**: 适合应用层错误处理
2. **灵活**: 支持任意错误类型
3. **上下文**: 方便添加错误上下文
4. **堆栈**: 自动错误链追踪
5. **快速开发**: 减少样板代码

**对比 thiserror**:
- `thiserror`: 适合库代码，定义精确错误类型
- `anyhow`: 适合应用代码，快速传播错误

**使用场景**: 所有应用层代码

## 📦 重要依赖

### CLI 解析: Clap

**Crate**: `clap = { version = "4.5", features = ["derive"] }`

**选择理由**:
1. **功能强大**: 支持子命令、参数验证、自动补全
2. **派生宏**: 声明式定义 CLI
3. **文档生成**: 自动生成帮助信息
4. **生态标准**: Rust CLI 工具首选

**主要用途**:
- 命令行参数解析
- 子命令定义 (`kode config`, `kode agents`)
- 帮助信息生成

### 配置目录: Directories

**Crate**: `directories = "5.0"`

**选择理由**:
1. **跨平台**: 自动处理不同平台的配置目录
2. **标准化**: 遵循各平台的目录规范
3. **简单**: 一行代码获取配置路径

**主要用途**:
- 获取 `~/.kode.json` 路径
- 获取 `~/.kode/agents/` 路径

### 文件操作

**Crate**:
- `glob = "0.3"` (模式匹配)
- `walkdir = "2.4"` (目录遍历)
- `ignore = "0.4"` (gitignore 支持)

**选择理由**:
1. **glob**: 简单的文件模式匹配
2. **walkdir**: 高效的递归目录遍历
3. **ignore**: 尊重 .gitignore，避免搜索无关文件

**主要用途**:
- `GlobTool` 实现
- Agent 目录扫描
- 项目上下文构建

### Markdown & YAML 解析

**Crate**:
- `pulldown-cmark = "0.11"` (Markdown)
- `yaml-rust2 = "0.8"` (YAML)

**选择理由**:
1. **pulldown-cmark**: 高性能、符合 CommonMark 规范
2. **yaml-rust2**: yaml-rust 的维护版本

**主要用途**:
- Agent 定义文件解析
- Markdown 格式化渲染

### 缓存: LRU

**Crate**: `lru = "0.12"`

**选择理由**:
1. **高效**: O(1) 查找和更新
2. **简单**: API 简洁
3. **标准**: 常用的缓存策略

**主要用途**:
- Agent 配置缓存
- 文件内容缓存 (可选)

### 日志: Tracing

**Crate**:
- `tracing = "0.1"`
- `tracing-subscriber = { version = "0.3", features = ["env-filter"] }`

**选择理由**:
1. **现代化**: 结构化日志
2. **异步友好**: 为异步场景设计
3. **灵活**: 丰富的过滤和格式化选项
4. **生态**: Tokio 官方推荐

**主要用途**:
- 调试日志
- 性能追踪
- 错误记录

### 工具库

**Crate**:
- `regex = "1.10"` (正则表达式)
- `chrono = "0.4"` (时间处理)
- `uuid = { version = "1.6", features = ["v4", "serde"] }` (唯一 ID)
- `lazy_static = "1.4"` (全局变量)
- `once_cell = "1.19"` (一次初始化)

**选择理由**: Rust 生态标准库

## 🎨 可选依赖

### 语法高亮: Syntect

**Crate**: `syntect = { version = "5.1", optional = true }`

**选择理由**:
1. **功能完整**: 支持大量语言
2. **主题**: 内置多种配色主题
3. **性能**: 高效的语法解析

**使用场景**: 代码块高亮显示

**可选原因**: 增加编译时间和二进制体积，仅在需要时启用

## 🔄 版本策略

### 依赖更新原则

1. **稳定性优先**: 使用稳定版本，避免 `*` 或 nightly
2. **安全更新**: 及时更新有安全漏洞的依赖
3. **主要版本锁定**: 锁定主要版本号（如 `1.x`）
4. **定期审查**: 每月检查依赖更新

### Rust 版本要求

**MSRV (Minimum Supported Rust Version)**: 1.75+

**理由**:
- 稳定特性需求
- 生态兼容性
- 不追求最新特性，保持广泛兼容

## 🏗️ 开发工具

### 代码质量

- `rustfmt`: 代码格式化
- `clippy`: Lint 检查
- `cargo-deny`: 依赖检查
- `cargo-audit`: 安全审计

### 测试工具

- `cargo-nextest`: 更快的测试运行器
- `cargo-tarpaulin`: 代码覆盖率
- `criterion`: 性能基准测试

### CI/CD

- GitHub Actions: 自动化测试和发布
- `cross`: 跨平台编译

## 📊 依赖图

```
kode-cli (bin)
    ├── kode-ui
    │   ├── ratatui
    │   ├── crossterm
    │   └── kode-core
    ├── kode-core
    │   ├── serde
    │   ├── directories
    │   └── anyhow
    ├── kode-tools
    │   ├── tokio
    │   ├── regex
    │   └── kode-core
    ├── kode-services
    │   ├── reqwest
    │   ├── tokio
    │   └── kode-core
    └── clap
```

## 🔍 技术选型对比

### TUI 框架

| 框架 | 优势 | 劣势 | 选择 |
|------|------|------|------|
| **Ratatui** | 灵活、现代、活跃 | 需要自己组织布局 | ✅ **选择** |
| Cursive | 高级抽象、简单 | 灵活性低、定制难 | ❌ |
| Crossterm 原生 | 完全控制 | 需要从零实现 UI | ❌ |

### 异步运行时

| 运行时 | 优势 | 劣势 | 选择 |
|--------|------|------|------|
| **Tokio** | 成熟、生态丰富、性能好 | 体积较大 | ✅ **选择** |
| async-std | 接近标准库 API | 生态较小 | ❌ |
| smol | 轻量 | 生态小、功能少 | ❌ |

### HTTP 客户端

| 客户端 | 优势 | 劣势 | 选择 |
|--------|------|------|------|
| **Reqwest** | 功能全、易用、流式支持 | 依赖较重 | ✅ **选择** |
| hyper | 底层、灵活 | API 复杂 | ❌ |
| ureq | 轻量、同步 | 不支持异步 | ❌ |

### 错误处理

| 库 | 适用场景 | 选择 |
|----|----------|------|
| **anyhow** | 应用层、快速开发 | ✅ **选择** |
| thiserror | 库代码、精确错误类型 | 考虑（未来） |
| eyre | anyhow 替代品 | ❌ |

## 🎯 性能考量

### 编译时间优化

1. **特性门控**: 可选特性使用 `optional = true`
2. **并行编译**: 使用 `codegen-units = 16` (开发)
3. **增量编译**: 默认启用
4. **缓存**: CI 中缓存依赖

### 运行时性能

1. **Release 优化**: `opt-level = 3`, `lto = true`
2. **异步 IO**: 避免阻塞操作
3. **零拷贝**: 使用引用避免复制
4. **并发**: 充分利用多核

### 二进制体积

- **Release 配置**: `strip = true` 去除符号
- **依赖选择**: 避免过重依赖
- **特性最小化**: 仅启用需要的特性

**目标体积**: < 20MB (release, stripped)

## 🔐 安全考虑

### 依赖安全

- 使用 `cargo-audit` 定期检查
- 避免使用废弃的 crate
- 优先使用知名维护良好的库

### 运行时安全

- 文件路径验证（防止路径遍历）
- 命令注入防护（Bash 工具）
- 权限检查（危险操作）
- 输入验证（用户输入、API 响应）

## 📝 总结

Kode-Rust 的技术栈经过精心选择，平衡了以下因素：

1. **性能**: Rust + Tokio + Reqwest 提供优异性能
2. **开发效率**: Serde + Anyhow + Clap 提高开发速度
3. **用户体验**: Ratatui 提供流畅的 TUI 体验
4. **生态成熟度**: 所有核心依赖都是生态标准
5. **可维护性**: 清晰的模块划分和依赖关系

**最后更新**: 2025-12-18
