# 贡献指南

感谢您对 Kode-Rust 项目感兴趣！我们欢迎各种形式的贡献。

## 📁 项目路径

**重要**: 开发时需要参考原始 TypeScript 实现

- **原始 TS 版本**: `/Users/gemini/Documents/backup/Kode-cli`
  - 查看原版实现细节
  - 理解配置和 Agent 格式
  - 确保行为一致性

- **当前 Rust 版本**: `/Users/gemini/Documents/backup/Kode-cli-rust`
  - 新实现的代码
  - 参考 [CLAUDE.md](CLAUDE.md) 和 [WORKFLOW.md](WORKFLOW.md)

## 🤝 贡献方式

### 报告问题
- 使用 [GitHub Issues](https://github.com/shareAI-lab/kode-cli-rust/issues) 报告 bug
- 提供详细的复现步骤
- 包括系统信息（OS、Rust 版本等）
- 附上错误日志或截图

### 功能建议
- 在 [GitHub Discussions](https://github.com/shareAI-lab/kode-cli-rust/discussions) 讨论新功能
- 说明功能的使用场景和价值
- 考虑对现有功能的影响

### 贡献代码
- Fork 项目并创建分支
- 编写代码和测试
- 提交 Pull Request
- 参与 Code Review

### 改进文档
- 修复文档错误
- 补充使用示例
- 翻译文档
- 改进代码注释

## 🛠️ 开发环境设置

### 前置要求

- **Rust**: 1.75+ (推荐使用 rustup)
- **Git**: 用于版本控制
- **API Key**: Claude API key (用于测试)

### 安装步骤

1. **克隆项目**
```bash
git clone https://github.com/shareAI-lab/kode-cli-rust.git
cd kode-cli-rust
```

2. **检查 Rust 版本**
```bash
rustc --version  # 应该 >= 1.75
```

3. **构建项目**
```bash
cargo build
```

4. **运行测试**
```bash
cargo test
```

5. **配置 API Key**
```bash
# 创建配置文件
cat > ~/.kode.json <<EOF
{
  "modelProfiles": {
    "claude": {
      "provider": "anthropic",
      "apiKey": "sk-ant-your-api-key-here",
      "model": "claude-sonnet-4-5-20250929"
    }
  },
  "defaultModel": "claude"
}
EOF
```

## 📝 开发流程

### 1. 选择任务

- 查看 [Issues](https://github.com/shareAI-lab/kode-cli-rust/issues) 中的 `good first issue` 标签
- 或查看 [ROADMAP.md](ROADMAP.md) 选择未完成的功能
- 在 Issue 中评论表明你要处理这个任务

### 2. 创建分支

```bash
git checkout -b feature/your-feature-name
# 或
git checkout -b fix/bug-description
```

分支命名规范：
- `feature/xxx`: 新功能
- `fix/xxx`: Bug 修复
- `docs/xxx`: 文档改进
- `refactor/xxx`: 代码重构
- `test/xxx`: 测试相关

### 3. 开发

#### 代码规范

1. **使用 rustfmt 格式化代码**
```bash
cargo fmt
```

2. **使用 clippy 检查代码**
```bash
cargo clippy -- -D warnings
```

3. **编写测试**
- 新功能必须有测试
- Bug 修复应包含回归测试
- 单元测试放在模块内部
- 集成测试放在 `tests/` 目录

4. **编写文档**
- 公开 API 必须有 rustdoc 注释
- 复杂逻辑添加代码注释
- 更新相关的 markdown 文档

#### 代码风格

```rust
// ✅ 好的示例
/// 加载配置文件
///
/// # Arguments
/// * `path` - 配置文件路径
///
/// # Errors
/// 如果文件不存在或格式错误，返回 `Err`
pub async fn load_config(path: &Path) -> Result<Config> {
    let content = tokio::fs::read_to_string(path).await?;
    let config: Config = serde_json::from_str(&content)?;
    Ok(config)
}

// ❌ 不好的示例
pub async fn load_config(path: &Path) -> Result<Config> {  // 缺少文档
    let content = tokio::fs::read_to_string(path).await.unwrap();  // 不要使用 unwrap
    serde_json::from_str(&content).unwrap()
}
```

#### 错误处理

```rust
// ✅ 使用 ? 传播错误
pub fn parse_agent(content: &str) -> Result<Agent> {
    let parsed = yaml::parse(content)?;
    let agent = Agent::from_yaml(parsed)?;
    Ok(agent)
}

// ✅ 添加错误上下文
use anyhow::Context;

pub async fn load_agent(path: &Path) -> Result<Agent> {
    let content = tokio::fs::read_to_string(path)
        .await
        .context("Failed to read agent file")?;

    parse_agent(&content)
        .context(format!("Failed to parse agent: {:?}", path))?
}

// ❌ 不要使用 unwrap/expect（除非有充分理由）
let config = load_config().unwrap();  // 会 panic
```

#### 异步代码

```rust
// ✅ 使用 async/await
pub async fn fetch_data(url: &str) -> Result<String> {
    let response = reqwest::get(url).await?;
    let text = response.text().await?;
    Ok(text)
}

// ✅ 并发执行独立任务
use tokio::join;

pub async fn load_all_agents(paths: &[PathBuf]) -> Result<Vec<Agent>> {
    let futures = paths.iter().map(|p| load_agent(p));
    let results = futures::future::try_join_all(futures).await?;
    Ok(results)
}
```

### 4. 测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test config_tests

# 运行测试并显示输出
cargo test -- --nocapture

# 检查测试覆盖率（需要 cargo-tarpaulin）
cargo tarpaulin --out Html
```

### 5. 提交

#### Commit Message 规范

遵循 [Conventional Commits](https://www.conventionalcommits.org/) 规范：

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Type**:
- `feat`: 新功能
- `fix`: Bug 修复
- `docs`: 文档变更
- `style`: 代码格式（不影响功能）
- `refactor`: 重构
- `test`: 测试相关
- `chore`: 构建/工具变更

**示例**:
```bash
git commit -m "feat(tools): add GrepTool implementation"
git commit -m "fix(config): handle missing config file gracefully"
git commit -m "docs(readme): update installation instructions"
```

### 6. 提交 Pull Request

1. **推送分支**
```bash
git push origin feature/your-feature-name
```

2. **创建 PR**
- 访问 GitHub 仓库
- 点击 "New Pull Request"
- 选择你的分支
- 填写 PR 模板

3. **PR 描述模板**
```markdown
## 描述
简要描述这个 PR 做了什么

## 变更类型
- [ ] Bug 修复
- [ ] 新功能
- [ ] 文档更新
- [ ] 性能优化
- [ ] 代码重构

## 相关 Issue
Closes #123

## 测试
描述如何测试这些变更

## 截图（如适用）
添加截图展示变更效果

## Checklist
- [ ] 代码遵循项目规范
- [ ] 通过了所有测试
- [ ] 添加了新测试（如适用）
- [ ] 更新了文档
- [ ] 运行了 `cargo fmt`
- [ ] 运行了 `cargo clippy`
```

## 🧪 测试指南

### 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_loading() {
        let config = Config::default();
        assert_eq!(config.default_model, "claude");
    }

    #[tokio::test]
    async fn test_async_function() {
        let result = some_async_function().await;
        assert!(result.is_ok());
    }
}
```

### 集成测试

```rust
// tests/config_test.rs
use kode_core::config::Config;

#[tokio::test]
async fn test_load_real_config() {
    let config = Config::load_from_file("test_fixtures/config.json").await;
    assert!(config.is_ok());
}
```

### 测试覆盖率

目标：核心逻辑 > 80%

```bash
# 安装 tarpaulin
cargo install cargo-tarpaulin

# 生成覆盖率报告
cargo tarpaulin --out Html --output-dir coverage
```

## 📚 文档指南

### Rustdoc 注释

```rust
/// 短描述（一行）
///
/// 详细描述可以有多段
///
/// # Examples
///
/// ```
/// use kode_core::config::Config;
///
/// let config = Config::default();
/// assert!(config.model_profiles.contains_key("claude"));
/// ```
///
/// # Errors
///
/// 描述可能的错误情况
///
/// # Panics
///
/// 描述可能 panic 的情况（应该避免）
pub fn some_function() -> Result<()> {
    // ...
}
```

### Markdown 文档

- 使用清晰的标题结构
- 添加代码示例
- 包含截图或图表（如适用）
- 保持文档更新

## 🔍 Code Review 流程

### 提交者

- 确保 CI 通过
- 响应 reviewer 的评论
- 进行必要的修改
- 保持分支更新

### Reviewer

- 检查代码质量和风格
- 验证功能正确性
- 评估性能影响
- 提出建设性建议

### 合并标准

- [ ] CI 全部通过
- [ ] 至少 1 个 approve
- [ ] 无未解决的评论
- [ ] 文档已更新
- [ ] 测试已添加

## 🎯 特定模块贡献指南

### 添加新工具

1. 在 `crates/kode-tools/src/` 创建新文件
2. 实现 `Tool` trait
3. 在 `lib.rs` 中导出
4. 在 `ToolRegistry` 中注册
5. 添加单元测试
6. 更新文档

示例：
```rust
// crates/kode-tools/src/my_tool.rs
use async_trait::async_trait;
use crate::tool::{Tool, ToolSchema, ToolContext, ToolResult};

pub struct MyTool;

#[async_trait]
impl Tool for MyTool {
    fn name(&self) -> &str {
        "MyTool"
    }

    fn description(&self) -> &str {
        "Description of what this tool does"
    }

    fn schema(&self) -> ToolSchema {
        // 定义参数 schema
    }

    async fn execute(
        &self,
        params: Value,
        context: &ToolContext,
    ) -> Result<ToolResult> {
        // 实现工具逻辑
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_my_tool() {
        // 测试工具
    }
}
```

### 添加新模型适配器

1. 在 `crates/kode-services/src/` 创建新文件
2. 实现 `ModelAdapter` trait
3. 处理流式响应
4. 添加错误处理
5. 编写集成测试

### 添加 UI 组件

1. 在 `crates/kode-ui/src/components/` 创建组件
2. 实现 `Widget` trait
3. 处理用户输入
4. 优化渲染性能
5. 测试不同终端尺寸

## 🐛 调试技巧

### 启用日志

```bash
# 设置日志级别
export RUST_LOG=kode=debug,kode_core=trace

# 运行程序
cargo run
```

### 使用 rust-lldb/rust-gdb

```bash
# 构建 debug 版本
cargo build

# 使用调试器
rust-lldb target/debug/kode
```

### 性能分析

```bash
# 使用 flamegraph
cargo install flamegraph
cargo flamegraph

# 使用 perf (Linux)
perf record --call-graph dwarf cargo run
perf report
```

## 📧 获取帮助

- **GitHub Discussions**: 提问和讨论
- **GitHub Issues**: 报告问题
- **Email**: ai-lab@foxmail.com

## 📜 许可证

贡献的代码将采用 Apache License 2.0 许可证。

## 🙏 致谢

感谢所有贡献者！您的贡献让 Kode-Rust 变得更好。

---

**祝您贡献愉快！** 🎉
