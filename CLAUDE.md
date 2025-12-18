# CLAUDE.md - AI Agent 工作指南

本文档为 AI Agent（包括 Claude Code）提供明确的工作指导，确保开发过程高效、规范。

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

在开始任何任务前，请确保已阅读并理解以下文档：

### 核心文档（必读）
1. **[README.md](README.md)** - 项目概览和快速开始
2. **[GOALS.md](GOALS.md)** - 详细目标和成功标准
3. **[ARCHITECTURE.md](ARCHITECTURE.md)** - 系统架构设计
4. **[ROADMAP.md](ROADMAP.md)** - 开发路线图
5. **[TECH_STACK.md](TECH_STACK.md)** - 技术选型理由

### 开发文档（需要时阅读）
6. **[CONTRIBUTING.md](CONTRIBUTING.md)** - 代码规范和贡献流程
7. **[PROJECT_OVERVIEW.md](PROJECT_OVERVIEW.md)** - 项目概览和文档索引
8. **[TODO.md](TODO.md)** - 当前任务清单（实时更新）
9. **[WORKFLOW.md](WORKFLOW.md)** - 工作流程说明

## 🎯 工作原则

### 1. 遵循规划
- ✅ **严格按照 ROADMAP.md 执行**
- ✅ **每个阶段完成后才进入下一阶段**
- ✅ **不跳过测试和文档**
- ❌ 不随意添加未规划的功能
- ❌ 不偏离既定架构

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

## 🔄 标准工作流

### 工作流程：计划 → 执行 → 测试 → 总结 → 反思 → 更新

每个任务都应遵循以下流程：

### 1️⃣ 计划阶段（Plan）

**任务开始前必须**:
1. 阅读 [TODO.md](TODO.md) 确认当前任务
2. 阅读相关设计文档（ARCHITECTURE.md 相关章节）
3. 理解任务在整体架构中的位置
4. 列出子任务清单
5. 识别潜在风险和依赖

**输出**:
- 明确的任务目标
- 详细的子任务列表
- 预估的工作量

### 2️⃣ 执行阶段（Execute）

**代码实现**:
```rust
// ✅ 好的示例：清晰的文档、错误处理、测试
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

**执行规范**:
- ✅ 编写完整的 rustdoc 注释
- ✅ 添加示例代码
- ✅ 使用 `?` 传播错误，添加上下文
- ✅ 同时编写单元测试
- ✅ 运行 `cargo fmt` 和 `cargo clippy`

### 3️⃣ 测试阶段（Test）

**必须执行的测试**:

```bash
# 1. 单元测试
cargo test

# 2. 格式检查
cargo fmt --check

# 3. Clippy 检查（无警告）
cargo clippy -- -D warnings

# 4. 构建检查
cargo build

# 5. 文档检查
cargo doc --no-deps

# 6. 集成测试（如果有）
cargo test --test integration_tests
```

**测试标准**:
- ✅ 所有测试通过
- ✅ 无 clippy 警告
- ✅ 代码已格式化
- ✅ 文档可以生成
- ✅ 覆盖主要代码路径

### 4️⃣ 总结阶段（Summary）

**完成任务后记录**:

在 [TODO.md](TODO.md) 中更新任务状态，并添加总结：

```markdown
## [完成] 实现配置加载系统

**完成时间**: 2025-12-18
**用时**: 3 小时

### 完成的工作
- ✅ 实现 `Config` 结构体
- ✅ 实现 JSON 序列化/反序列化
- ✅ 实现配置合并逻辑
- ✅ 添加 10 个单元测试
- ✅ 通过所有检查

### 关键代码
- `crates/kode-core/src/config/mod.rs` - 主要实现
- `crates/kode-core/src/config/tests.rs` - 测试

### 遇到的问题
- 无

### 后续建议
- 添加配置验证逻辑
- 添加配置迁移工具
```

### 5️⃣ 反思阶段（Reflect）

**自我检查清单**:

```markdown
## 反思检查清单

### 代码质量
- [ ] 代码遵循 Rust 最佳实践？
- [ ] 有没有过度设计？
- [ ] 有没有未处理的错误情况？
- [ ] 有没有性能瓶颈？
- [ ] 命名是否清晰易懂？

### 测试覆盖
- [ ] 关键路径都有测试？
- [ ] 边界情况都考虑了？
- [ ] 错误情况都测试了？

### 文档质量
- [ ] 公开 API 都有文档？
- [ ] 文档示例可以运行？
- [ ] 复杂逻辑有注释？

### 架构一致性
- [ ] 符合 ARCHITECTURE.md 的设计？
- [ ] 模块职责清晰？
- [ ] 依赖关系合理？

### 兼容性
- [ ] 与 TS 版本行为一致？
- [ ] 配置格式没有变化？
- [ ] Agent 格式没有变化？
```

**改进建议**:
- 记录可以优化的地方
- 记录技术债务
- 记录未来的扩展点

### 6️⃣ 更新阶段（Update）

**更新相关文档**:

1. **更新 TODO.md**
   - 标记任务为完成 ✅
   - 添加任务总结
   - 更新进度百分比

2. **更新 ROADMAP.md**
   - 更新阶段进度
   - 标记完成的里程碑
   - 调整后续计划（如需要）

3. **更新 ARCHITECTURE.md**（如果有架构变化）
   - 记录实际实现与设计的差异
   - 更新代码示例
   - 补充实现细节

4. **提交 Git**
   ```bash
   git add .
   git commit -m "feat(core): implement config loading system

   - Add Config struct with full serialization
   - Add config merge logic
   - Add 10 unit tests
   - All checks passing

   Refs: TODO.md Phase 1 - Day 3-4"
   ```

## 📝 文档更新规范

### TODO.md 格式

```markdown
# Kode-Rust 开发任务清单

**更新时间**: 2025-12-18 20:30
**当前阶段**: Phase 1 - Week 1
**总体进度**: 15% (3/20 任务完成)

## 🚧 进行中

### [Day 3-4] 配置系统 (IN PROGRESS)
**负责人**: Claude
**开始时间**: 2025-12-18 10:00
**预计完成**: 2025-12-18 18:00

- [x] 定义 Config 结构体
- [x] 实现 JSON 序列化
- [~] 实现配置合并逻辑 (90%)
- [ ] 环境变量支持
- [ ] 单元测试

## ✅ 已完成

### [Day 1-2] 项目骨架
**完成时间**: 2025-12-18
**用时**: 2 小时

- [x] 创建 Cargo workspace
- [x] 初始化所有文档
- [x] 配置 CI/CD

**总结**: 项目基础框架搭建完成，所有规划文档就绪。

## 📋 待开始

### [Day 5-7] 核心数据结构
...
```

### 进度符号

- `[ ]` - 未开始
- `[~]` - 进行中（可注明百分比）
- `[x]` - 已完成
- `[!]` - 被阻塞
- `[-]` - 已取消

## 🚨 常见问题处理

### 遇到阻塞怎么办？

1. **技术难题**
   - 查阅 Rust 文档和相关 crate 文档
   - 搜索类似问题的解决方案
   - 简化问题，先实现基础版本
   - 在 TODO.md 中标记 `[!]` 并记录问题

2. **设计不明确**
   - 重新阅读 ARCHITECTURE.md
   - 参考 TS 版本实现
   - 提出具体问题，请求明确

3. **依赖问题**
   - 检查 Cargo.toml 版本
   - 查看 TECH_STACK.md 推荐的版本
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

3. **集成测试失败**
   - 检查模块间接口
   - 验证数据流
   - 添加调试日志

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
- 其他 Rust TUI 项目

## 📊 质量标准

### 代码质量指标

- **测试覆盖率**: 核心逻辑 > 80%
- **Clippy 警告**: 0
- **文档覆盖**: 公开 API 100%
- **性能**: 符合 GOALS.md 标准

### 代码审查清单

提交前自我审查：

```markdown
- [ ] 代码格式化 (`cargo fmt`)
- [ ] 无 Clippy 警告 (`cargo clippy`)
- [ ] 所有测试通过 (`cargo test`)
- [ ] 文档完整 (`cargo doc`)
- [ ] 遵循命名规范
- [ ] 错误处理正确
- [ ] 无 `unwrap()` 或 `expect()`（特殊情况除外）
- [ ] 性能考虑（无明显瓶颈）
- [ ] 兼容性检查（如适用）
```

## 🎯 每日工作流程

### 开始工作

1. 阅读 [TODO.md](TODO.md) 确认今日任务
2. 更新任务状态为 `[~]` 进行中
3. 记录开始时间

### 工作中

1. 遵循标准工作流（计划→执行→测试→总结→反思→更新）
2. 定期提交代码（功能完整时）
3. 及时更新进度

### 结束工作

1. 完成任务总结
2. 更新 TODO.md
3. 提交代码和文档
4. 规划明日任务

## 🔍 自检清单

每次提交前检查：

```markdown
## 提交前自检

### 功能
- [ ] 功能完整实现
- [ ] 边界情况处理
- [ ] 错误处理完善

### 测试
- [ ] 单元测试通过
- [ ] 集成测试通过（如有）
- [ ] 手动测试验证

### 代码质量
- [ ] 格式化完成
- [ ] 无 Clippy 警告
- [ ] 无编译警告
- [ ] 文档注释完整

### 文档
- [ ] TODO.md 已更新
- [ ] ROADMAP.md 已更新（如需要）
- [ ] 代码注释清晰

### Git
- [ ] Commit message 规范
- [ ] 包含相关文件
- [ ] 不包含临时文件
```

## 💡 最佳实践

### DO（推荐做法）

✅ **按阶段推进**: 完成 Phase 1 再进入 Phase 2
✅ **测试驱动**: 先写测试，再写实现
✅ **小步提交**: 功能完整即提交，不累积
✅ **文档同步**: 代码和文档同时更新
✅ **性能意识**: 使用异步、缓存、并发
✅ **错误处理**: 使用 `?`，添加上下文
✅ **类型安全**: 充分利用 Rust 类型系统

### DON'T（避免做法）

❌ **跳过测试**: 没有测试的代码不提交
❌ **随意设计**: 不符合 ARCHITECTURE.md 的设计
❌ **使用 unwrap**: 除非有充分理由并注释
❌ **忽略警告**: 所有警告都必须修复
❌ **破坏兼容**: 不修改配置和 Agent 格式
❌ **过度优化**: 先实现功能，再优化性能
❌ **忽略文档**: 公开 API 必须有文档

## 📞 获取帮助

如遇到问题：

1. **技术问题**: 查阅相关文档和 crate 文档
2. **设计问题**: 重新阅读 ARCHITECTURE.md
3. **流程问题**: 参考本文档
4. **其他问题**: 在 TODO.md 中记录，请求明确

## 🎉 总结

记住核心原则：

1. **遵循规划** - ROADMAP.md 是指南
2. **保证质量** - 测试、文档、规范
3. **持续改进** - 反思、总结、优化
4. **记录进度** - TODO.md 实时更新

让我们一起构建高质量的 Kode-Rust！🦀
