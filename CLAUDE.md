<!-- OPENSPEC:START -->
# OpenSpec Instructions

These instructions are for AI assistants working in this project.

Always open `@/openspec/AGENTS.md` when the request:
- Mentions planning or proposals (words like proposal, spec, change, plan)
- Introduces new capabilities, breaking changes, architecture shifts, or big performance/security work
- Sounds ambiguous and you need the authoritative spec before coding

Use `@/openspec/AGENTS.md` to learn:
- How to create and apply change proposals
- Spec format and conventions
- Project structure and guidelines

Keep this managed block so 'openspec update' can refresh the instructions.

<!-- OPENSPEC:END -->

# CLAUDE.md - AI Agent 工作指南

本文档为 AI Agent（包括 Claude Code）提供明确的工作指导，确保开发过程高效、规范。

> **完整工作流程**: 请参考 [openspec/AGENTS.md](openspec/AGENTS.md)

## 📋 项目上下文

### 项目简介
**Kode-Rust** 是使用 Rust 完整重写的 AI Agent CLI 工具，基于 TypeScript 版本的 [Kode-cli](https://github.com/shareAI-lab/kode)。

### 重要路径信息
- **原始 TS 版本仓库**: `/Users/gemini/Documents/backup/Kode-cli`
- **当前 Rust 版本仓库**: `/Users/gemini/Documents/backup/Kode-cli-rust`
- **参考原版实现**: 在实现功能时，可以参考 TS 版本的代码实现
- **配置兼容性测试**: 使用原版仓库的测试配置文件验证兼容性

### 核心目标
1. **完整功能移植**: 实现 TS 版本的所有核心功能
2. **性能提升**: 启动 <100ms，内存 <50MB
3. **100% 兼容**: 配置、Agent 定义、MCP 协议
4. **高质量代码**: 遵循 Rust 最佳实践

### 技术栈
- **语言**: Rust 1.75+ (Edition 2021)
- **运行时**: Tokio (异步 IO)
- **TUI**: Ratatui
- **HTTP**: Reqwest
- **架构**: Cargo Workspace (5 crates)

## 📚 必读文档

### 核心文档（必读）
1. **[README.md](README.md)** - 项目概览和快速开始
2. **[openspec/AGENTS.md](openspec/AGENTS.md)** - OpenSpec 工作流程（重要）
3. **[openspec/specs/](openspec/specs/)** - 功能规范
4. **[openspec/changes/](openspec/changes/)** - 进行中的变更提案

### 开发文档
5. **[CONTRIBUTING.md](CONTRIBUTING.md)** - 贡献指南

## 🎯 工作原则

### 1. OpenSpec 规范驱动开发
- ✅ **所有开发使用 OpenSpec 工作流**
- ✅ **遵循 openspec/specs/ 中的功能规范**
- ✅ **遵循 openspec/AGENTS.md 中的工作流程**
- ✅ **不跳过提案和批准阶段**
- ❌ 不随意添加未规划的功能
- ❌ 不偏离既定架构

**OpenSpec 三阶段流程**:
1. **创建变更** (`openspec proposal`)
2. **实施变更** (`/openspec:apply`)
3. **归档变更** (`/openspec:archive`)

详见 [openspec/AGENTS.md](openspec/AGENTS.md)

### 2. 代码质量
- ✅ **遵循 Rust 最佳实践**
- ✅ **每个功能都有单元测试**
- ✅ **关键逻辑有集成测试**
- ✅ **使用 rustfmt 格式化**
- ✅ **通过 clippy 检查（无警告）**
- ✅ **完整的文档注释（rustdoc）**

### 3. 兼容性优先
- ✅ **配置格式与 TS 版本完全一致**
- ✅ **Agent 定义格式与 TS 版本完全一致**
- ✅ **定期测试兼容性**
- ❌ 不修改配置格式
- ❌ 不修改 Agent 定义格式

### 4. 性能意识
- ✅ **使用异步 IO (Tokio)**
- ✅ **避免不必要的克隆**
- ✅ **使用缓存优化（LRU）**
- ✅ **并发执行独立任务**
- ❌ 不使用阻塞 IO
- ❌ 不过度分配内存

## 🔄 OpenSpec 工作流程

所有开发工作都遵循 OpenSpec 三阶段流程：

### Stage 1: 创建变更 (Creating Changes)

```bash
# 1. 查看现有规范
openspec list --specs
openspec show <spec-id>

# 2. 创建变更提案
# 创建 openspec/changes/<change-id>/
# - proposal.md (为什么、改什么、影响)
# - tasks.md (任务清单)
# - specs/<capability>/spec.md (Delta 规范)

# 3. 验证
openspec validate <change-id> --strict

# 4. 等待批准
```

### Stage 2: 实施变更 (Implementing Changes)

```bash
# 1. 阅读 proposal.md 了解需求
# 2. 阅读 design.md (如有) 了解技术决策
# 3. 按照 tasks.md 顺序实施
# 4. 完成后标记所有任务为 [x]

# 5. 测试验证
cargo test
cargo fmt --check
cargo clippy -- -D warnings
cargo build
```

### Stage 3: 归档变更 (Archiving Changes)

```bash
# 1. 部署完成后
# 2. 归档变更
openspec archive <change-id>

# 3. 更新 specs/ (如果功能变更)
# 4. 验证
openspec validate --strict
```

## 📝 代码实现规范

### Rust 代码示例

```rust
/// 加载配置文件
///
/// # Arguments
/// * `path` - 配置文件路径
///
/// # Examples
/// ```
/// let config = Config::load("~/.kode.json").await?;
/// ```
///
/// # Errors
/// 如果文件不存在或格式错误返回 Error
pub async fn load(path: &Path) -> Result<Config> {
    let content = tokio::fs::read_to_string(path)
        .await
        .context("Failed to read config file")?;

    let config: Config = serde_json::from_str(&content)
        .context("Failed to parse config")?;

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_load_config() {
        let config = Config::load("test_fixtures/config.json").await;
        assert!(config.is_ok());
    }
}
```

### 执行规范
- ✅ 编写完整的 rustdoc 注释
- ✅ 添加示例代码
- ✅ 使用 `?` 传播错误，添加上下文
- ✅ 同时编写单元测试
- ✅ 运行 `cargo fmt` 和 `cargo clippy`

## 🧪 测试标准

```bash
# 必须执行的测试
cargo test                  # 单元测试
cargo fmt --check          # 格式检查
cargo clippy -- -D warnings # Clippy 检查（无警告）
cargo build                # 构建检查
cargo doc --no-deps        # 文档检查
```

**测试标准**:
- ✅ 所有测试通过
- ✅ 无 clippy 警告
- ✅ 代码已格式化
- ✅ 文档可以生成
- ✅ 覆盖主要代码路径

## 📊 质量标准

### 代码质量指标
- **测试覆盖率**: 核心逻辑 > 80%
- **Clippy 警告**: 0
- **文档覆盖**: 公开 API 100%
- **性能**: 符合目标标准

### 提交前自检

```markdown
### 功能
- [ ] 功能完整实现
- [ ] 边界情况处理
- [ ] 错误处理完善

### 测试
- [ ] 单元测试通过
- [ ] 集成测试通过（如有）

### 代码质量
- [ ] 格式化完成 (`cargo fmt`)
- [ ] 无 Clippy 警告 (`cargo clippy`)
- [ ] 文档注释完整

### 文档
- [ ] tasks.md 已更新
- [ ] 相关规范已更新（如需要）

### Git
- [ ] Commit message 规范
- [ ] 包含相关文件
```

## 🚨 常见问题处理

### 遇到阻塞怎么办？

1. **技术难题**
   - 查阅 Rust 文档和相关 crate 文档
   - 搜索类似问题的解决方案
   - 简化问题，先实现基础版本
   - 在 tasks.md 中标记 `[!]` 并记录问题

2. **设计不明确**
   - 阅读相关功能规范
   - 阅读 specs/<capability>/design.md
   - 参考 TS 版本实现
   - 提出具体问题，请求明确

3. **依赖问题**
   - 检查 Cargo.toml 版本
   - 确认依赖兼容性

### 测试失败怎么办？

1. **单元测试失败**
   - 仔细阅读错误信息
   - 使用 `cargo test -- --nocapture` 查看输出
   - 逐个修复，不跳过

2. **Clippy 警告**
   - 所有警告都必须修复
   - 不使用 `#[allow(clippy::...)]`（除非有充分理由）
   - 参考 Clippy 建议改进代码

## 🎓 学习资源

### Rust 学习
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Async Book](https://rust-lang.github.io/async-book/)

### 相关 Crate 文档
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Ratatui Book](https://ratatui.rs/)
- [Reqwest Docs](https://docs.rs/reqwest/)
- [Serde Guide](https://serde.rs/)

### 参考项目
- 原版 [Kode-cli](https://github.com/shareAI-lab/kode)

## 💡 最佳实践

### DO（推荐做法）

✅ **OpenSpec 驱动**: 所有变更都通过 OpenSpec
✅ **测试驱动**: 先写测试，再写实现
✅ **小步提交**: 功能完整即提交，不累积
✅ **文档同步**: 代码和文档同时更新
✅ **性能意识**: 使用异步、缓存、并发
✅ **错误处理**: 使用 `?`，添加上下文
✅ **类型安全**: 充分利用 Rust 类型系统

### DON'T（避免做法）

❌ **跳过提案**: 没有变更提案不开发
❌ **随意设计**: 不符合功能规范的设计
❌ **使用 unwrap**: 除非有充分理由并注释
❌ **忽略警告**: 所有警告都必须修复
❌ **破坏兼容**: 不修改配置和 Agent 格式
❌ **过度优化**: 先实现功能，再优化性能
❌ **忽略文档**: 公开 API 必须有文档

## 📞 获取帮助

如遇到问题：

1. **OpenSpec 流程**: 参考 [openspec/AGENTS.md](openspec/AGENTS.md)
2. **技术问题**: 查阅相关文档和 crate 文档
3. **设计问题**: 阅读 openspec/specs/ 中的规范
4. **其他问题**: 在变更提案的 tasks.md 中记录

## 🎉 总结

记住核心原则：

1. **OpenSpec 驱动** - 所有开发通过变更提案
2. **保证质量** - 测试、文档、规范
3. **持续改进** - 反思、总结、优化
4. **记录进度** - tasks.md 实时更新

让我们一起构建高质量的 Kode-Rust！🦀
