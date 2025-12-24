# 项目上下文 (Project Context)

## 项目目的 (Purpose)

**Kode-Rust** 是一个用 Rust 编写的高性能 AI Agent CLI 工具 - 对 TypeScript 版本 [Kode-cli](https://github.com/shareAI-lab/kode) 的完整重写。

### 核心目标
1. **完整功能移植** - 实现原 TypeScript 版本的所有核心功能
2. **100% 兼容性** - 与 TS 版本的配置（`.kode.json`）和 Agent 定义（Markdown + YAML frontmatter）完全兼容
3. **卓越性能** - 启动 <100ms，空闲内存 <50MB，UI 渲染 60 FPS
4. **类型安全** - 利用 Rust 的所有权和类型系统保证可靠性

### 项目路径
- **原始 TypeScript 版本**: `/Users/gemini/Documents/backup/Kode-cli`
- **当前 Rust 版本**: `/Users/gemini/Documents/backup/Kode-cli-rust`

### 核心特性
- 多模型 AI 支持（Anthropic Claude、OpenAI-compatible APIs）
- 强大的工具系统（文件操作、命令执行、代码搜索）
- 灵活的 Agent 系统，支持动态加载
- MCP（Model Context Protocol）集成
- 现代化 TUI 界面（ratatui）
- 流式响应
- 权限管理

## 技术栈 (Tech Stack)

### 语言与运行时
- **Rust 1.75+** (Edition 2021)
- **Tokio** (1.35+) - 完整特性的异步运行时

### 核心框架
- **ratatui** (0.28) - TUI 框架
- **crossterm** (0.27) - 终端控制
- **reqwest** (0.12) - 支持 streaming 的 HTTP 客户端
- **serde** (1.0) - 序列化（JSON/YAML）
- **anyhow/thiserror** - 错误处理

### 关键依赖
- **clap** (4.5) - CLI 解析，支持 derive 宏
- **directories** (5.0) - 跨平台配置路径
- **pulldown-cmark** + **yaml-rust2** - Agent 定义解析
- **lru** (0.12) - Agent/配置缓存
- **regex** (1.10) - Grep 工具实现
- **glob** (0.3) + **walkdir** (2.4) - 文件操作
- **syntect** (5.1) - 语法高亮（可选）
- **tracing** - 结构化日志

### 工作区结构
```
kode-cli-rust/
├── crates/
│   ├── kode-core/      # 核心：配置、Agent、上下文、模型抽象
│   ├── kode-tools/     # 工具系统：Tool trait、工具实现
│   ├── kode-services/  # 外部服务：API 客户端、MCP 客户端
│   ├── kode-ui/        # TUI：REPL、组件、渲染
│   └── kode-cli/       # CLI 入口：命令行解析
```

## 项目规范 (Project Conventions)

### 代码风格 (Code Style)

#### Rust 规范
- 使用 `rustfmt` 格式化（提交前运行 `cargo fmt`）
- 零 clippy 警告（`cargo clippy -- -D warnings`）
- 遵循 Rust 命名规范：
  - 模块/类型：`PascalCase`
  - 函数/变量：`snake_case`
  - 常量：`SCREAMING_SNAKE_CASE`

#### 代码注释语言规范
- **✅ 代码注释使用中文** - 所有代码注释（包括行内注释和文档注释）使用中文
- **✅ 变量/函数名称使用英文** - 遵循 Rust 命名规范
- **✅ 文档注释使用中文** - rustdoc 注释的内容使用中文，但保留英文的标准章节标题（如 `# Arguments`、`# Errors`、`# Examples`）

```rust
/// 从指定路径加载配置。
///
/// # Arguments
/// * `path` - 配置文件路径
///
/// # Errors
/// 如果文件无法读取或解析，返回 `Error::ConfigLoadError`
///
/// # Examples
/// ```no_run
/// use kode_core::config::Config;
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let config = Config::load_from_file("~/.kode.json").await?;
/// # Ok(())
/// # }
/// ```
pub async fn load_from_file(path: &str) -> Result<Config> {
    // 读取文件内容
    let content = tokio::fs::read_to_string(path)
        .await
        .context(format!("无法读取配置文件: {}", path))?;
    // ...
}
```

#### 文档注释
- 公开 API 必须有 rustdoc 注释
- 包含 `# Arguments`、`# Errors`、`# Examples` 章节
- 使用 `///` 表示项文档，`//!` 表示模块文档

```rust
/// 从指定路径加载配置。
///
/// # Arguments
/// * `path` - 配置文件路径
///
/// # Errors
/// 如果文件无法读取或解析，返回 `Error::ConfigLoadError`
///
/// # Examples
/// ```no_run
/// use kode_core::config::Config;
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let config = Config::load_from_file("~/.kode.json").await?;
/// # Ok(())
/// # }
/// ```
pub async fn load_from_file(path: &str) -> Result<Config> {
    // ...
}
```

#### 错误处理
- 库代码（kode-core）使用 `thiserror::Error` 定义类型化错误
- 应用级错误传播使用 `anyhow`
- 转换错误时始终使用 `.context()` 添加上下文
- 避免使用 `unwrap()`/`expect()`（测试代码除外）

```rust
// 使用 ? 传播错误并添加上下文
let content = tokio::fs::read_to_string(path)
    .await
    .context(format!("Failed to read config from {}", path))?;
```

#### 异步/Await
- 所有 I/O 操作必须是异步的（Tokio）
- 使用 `async fn` 定义异步函数
- 使用 `tokio::test` 编写异步测试
- 使用 `futures::future::join_all` 并行化独立操作

### 架构模式 (Architecture Patterns)

#### 基于 Trait 的设计
- 核心抽象使用 trait：`Tool`、`ModelAdapter`、`Agent`
- Trait 使用 `async_trait` 并带有 `Send + Sync` 约束
- 实现存储在 `Arc` 中以实现共享所有权

```rust
#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    async fn execute(&self, params: Value, context: &ToolContext) -> Result<ToolResult>;
}
```

#### 模块组织
- **kode-core**：领域模型，除 serde 外无外部依赖
- **kode-tools**：工具实现，依赖 kode-core
- **kode-services**：API 客户端，依赖 kode-core
- **kode-ui**：TUI，依赖以上所有
- **kode-cli**：轻量级 CLI 层，编排组件

#### 配置分层
1. 全局配置：`~/.kode.json`
2. 项目配置：`./.kode.json`
3. 环境变量覆盖
4. 项目配置与全局配置合并（项目优先）

#### Agent 加载优先级
1. 内置 Agent（代码）
2. `~/.claude/agents/`（Claude Code 兼容）
3. `~/.kode/agents/`（Kode 用户目录）
4. `./.claude/agents/`（项目 Claude Code）
5. `./.kode/agents/`（项目 Kode）

### 测试策略 (Testing Strategy)

#### 测试类型
- **单元测试**：模块内联（`#[cfg(test)]` 模块）
- **集成测试**：工作区根目录的 `tests/` 目录
- **兼容性测试**：加载 TS 版本的配置/Agent

#### 覆盖率目标
- 核心逻辑：> 80%
- 关键路径：100%
- 使用 `cargo-tarpaulin` 生成覆盖率报告

#### 测试组织
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_config_loading() {
        let config = Config::load_from_file("test_fixtures/config.json").await;
        assert!(config.is_ok());
    }
}
```

### Git 工作流 (Git Workflow)

#### 分支策略
- `master` - 主开发分支
- `feature/*` - 新功能
- `fix/*` - Bug 修复
- `refactor/*` - 重构

#### 提交信息
遵循 [Conventional Commits](https://www.conventionalcommits.org/)：
```
<type>(<scope>): <subject>

<body>
```

类型：
- `feat`：新功能
- `fix`：Bug 修复
- `docs`：文档
- `refactor`：重构
- `test`：测试
- `chore`：构建/工具

示例：
- `feat(tools): add GrepTool implementation`
- `fix(config): handle missing config file gracefully`
- `docs(readme): update installation instructions`

#### 提交前检查
1. `cargo fmt`
2. `cargo clippy -- -D warnings`
3. `cargo test`
4. 检查 git diff 确认无意外更改

### OpenSpec 文档规范 (双语格式)

项目使用 OpenSpec 进行规范驱动开发，所有 `spec.md` 文件必须采用**双语格式**以通过验证器。

#### 验证器要求 (OpenSpec 官方规范)
- 必须使用英文结构标题：`## Purpose`、`## Requirements`、`### Requirement:`、`#### Scenario:`
- 需求描述必须包含 `SHALL` 或 `MUST` 关键字
- 每个需求至少需要一个 Scenario 块

#### 双语格式示例

```markdown
## Purpose

中文描述功能目的...

## Requirements

### Requirement: 功能名称
The system SHALL provide [功能描述 in English with SHALL/MUST].

中文翻译说明...

#### Scenario: 场景名称
- **WHEN** 中文场景条件描述
- **THEN** 中文预期结果描述
- **AND** 中文附加条件（可选）
```

#### 文件格式规范

| 文件 | 格式要求 |
|------|----------|
| `spec.md` | 双语格式（英文结构 + 中文内容），通过 `openspec validate` 验证 |
| `design.md` | 可完全使用中文（无验证要求） |
| `proposal.md` | 建议使用中文（无验证要求） |
| `tasks.md` | 可使用中文 |

#### Delta 格式 (用于 `changes/*/specs/*/spec.md`)

```markdown
## ADDED Requirements    # 新增功能
## MODIFIED Requirements # 修改现有功能（包含完整更新文本）
## REMOVED Requirements  # 废弃功能
```

#### 验证命令

```bash
# 验证单个变更
npx openspec validate <change-id> --strict

# 列出所有变更
npx openspec list

# 查看变更详情
npx openspec show <change-id>
```

#### 重要提示
- **MODIFIED Requirements**: 必须包含完整的更新文本（不仅仅是变更部分）
- **Scenario 格式**: 必须使用 `#### Scenario:` (4 个井号)，不是 bullet 或粗体
- **ADDED vs MODIFIED**: 新增独立功能用 ADDED，修改现有需求行为用 MODIFIED

**参考**: [OpenSpec 官方文档](https://github.com/Fission-AI/OpenSpec)

## 领域上下文 (Domain Context)

### Agent 系统
- **Agents** 是以 Markdown + YAML frontmatter 定义的专用 AI 角色
- Agent 有过滤的工具访问（特定工具或 "all"）
- Agent 可通过 `TaskTool` 将任务委托给子 Agent
- Agent 定义从 5 个优先级目录加载

### 工具系统
- **Tools** 是 AI 可调用的能力（文件操作、Shell 命令等）
- 所有工具实现 `Tool` trait
- 工具执行前可能需要用户权限
- MCP 服务器可动态暴露工具

### 模型抽象
- **ModelProfiles** 定义 AI 模型配置
- 支持：Anthropic、OpenAI-compatible APIs
- 模型指针：`main`、`task`、`reasoning`、`quick`
- 运行时模型切换

### MCP (Model Context Protocol)
- 外部工具可通过 MCP 服务器提供
- 服务器通过 STDIO 或 SSE 通信
- MCP 工具动态发现和暴露
- 兼容标准 MCP 实现

## 重要约束 (Important Constraints)

### 兼容性约束
- **配置格式必须与 TS 版本完全匹配** - `camelCase` JSON 字段
- **Agent 格式必须与 TS 版本完全匹配** - Markdown + YAML frontmatter
- **MCP 协议必须符合标准**
- 不得破坏现有配置/Agent 格式

### 性能约束
- **冷启动**: < 100ms
- **空闲内存**: < 50MB
- **活跃内存**: < 200MB
- **UI 渲染**: 60 FPS
- **配置加载**: < 50ms
- **Agent 加载**: < 20ms（缓存），< 100ms（非缓存）

### 平台支持
- 主要平台：macOS、Linux
- 次要平台：Windows（尽力支持）
- 最低 Rust 版本：1.75+

### 安全约束
- 文件路径验证（防止路径遍历）
- 命令注入防护（Bash 工具）
- 危险操作的权限系统
- API 密钥存储在配置中（用户负责）

## 外部依赖 (External Dependencies)

### AI APIs
- **Anthropic Claude API** - 主要模型提供商
- **OpenAI-compatible APIs** - 备选提供商（DeepSeek 等）

### MCP 服务器
- `@modelcontextprotocol/server-filesystem` - 文件系统访问
- 任何标准 MCP 服务器

### 开发工具
- **ripgrep** (rg) - GrepTool 使用（外部二进制）
- 标准 Shell 命令 - BashTool 使用

### 参考实现
- 原始 TypeScript 版本位于 `/Users/gemini/Documents/backup/Kode-cli`
- 行为一致性问题参考 TS 实现

---

**最后更新**: 2025-12-23
