# 工作流程说明

本文档详细说明 Kode-Rust 项目的开发工作流程。

## 📁 项目路径上下文

**重要**: 在开发过程中需要频繁参考原始实现

- **原始 TypeScript 版本**: `/Users/gemini/Documents/backup/Kode-cli`
  - 参考实现细节
  - 查看配置格式
  - 了解 Agent 定义格式
  - 测试兼容性

- **当前 Rust 版本**: `/Users/gemini/Documents/backup/Kode-cli-rust`
  - 新实现的代码
  - 测试和文档

## 🔄 核心工作流

每个开发任务都应遵循以下 **6 步工作流**：

```
计划 → 执行 → 测试 → 总结 → 反思 → 更新
 ↓      ↓      ↓      ↓      ↓      ↓
Plan → Execute → Test → Summary → Reflect → Update
```

## 📋 详细流程

### 1️⃣ 计划阶段（Plan）

**目标**: 明确任务范围和实现方案

#### 输入
- [TODO.md](TODO.md) 中的待办任务
- [ARCHITECTURE.md](ARCHITECTURE.md) 相关设计
- [ROADMAP.md](ROADMAP.md) 阶段计划

#### 执行步骤
1. **理解任务**
   - 阅读任务描述
   - 确认任务目标
   - 理解验收标准

2. **技术调研**
   - 查看相关设计文档
   - 研究需要的 crate
   - 参考 TS 版本实现

3. **制定计划**
   - 列出子任务
   - 识别依赖关系
   - 预估工作量

4. **风险识别**
   - 技术难点
   - 时间风险
   - 依赖风险

#### 输出
- 清晰的任务分解
- 实现方案草图
- 风险清单

#### 示例

```markdown
## 计划：实现配置加载系统

### 任务目标
实现 Config 结构体，支持从 JSON 文件加载配置，支持多层配置合并。

### 子任务
1. 定义 Config 结构体（30min）
2. 实现 JSON 反序列化（20min）
3. 实现配置文件查找逻辑（40min）
4. 实现配置合并逻辑（1h）
5. 编写单元测试（1h）
6. 文档和示例（30min）

### 技术方案
- 使用 `serde` + `serde_json` 序列化
- 使用 `directories` crate 获取配置目录
- 配置合并：项目配置覆盖全局配置

### 风险
- 配置合并逻辑可能比较复杂
- 需要处理配置文件不存在的情况

### 预计时间
4 小时
```

---

### 2️⃣ 执行阶段（Execute）

**目标**: 高质量完成代码实现

#### 代码规范

##### 文件组织
```
crates/kode-core/src/config/
├── mod.rs          # 模块入口和主要实现
├── types.rs        # 数据类型定义
├── loader.rs       # 加载逻辑
├── merge.rs        # 合并逻辑
└── tests.rs        # 单元测试（或在各文件内）
```

##### 代码模板
```rust
//! 配置管理模块
//!
//! 提供配置文件的加载、合并和验证功能。

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// 应用配置
///
/// 支持从 JSON 文件加载，支持多层配置合并。
///
/// # Examples
///
/// ```
/// use kode_core::config::Config;
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let config = Config::load().await?;
///     println!("Default model: {}", config.default_model);
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    /// 模型配置列表
    pub model_profiles: HashMap<String, ModelProfile>,

    /// 默认模型名称
    pub default_model: String,

    // ... 其他字段
}

impl Config {
    /// 加载配置文件
    ///
    /// 按以下顺序加载并合并配置：
    /// 1. 全局配置 `~/.kode.json`
    /// 2. 项目配置 `./.kode.json`
    /// 3. 环境变量
    ///
    /// # Errors
    ///
    /// 如果配置文件格式错误，返回解析错误。
    /// 如果没有找到任何配置文件，返回默认配置。
    pub async fn load() -> Result<Self> {
        let global_config = Self::load_global().await.ok();
        let project_config = Self::load_project().await.ok();

        // 合并配置
        let mut config = global_config.unwrap_or_default();
        if let Some(project) = project_config {
            config.merge(project);
        }

        Ok(config)
    }

    /// 加载全局配置文件 `~/.kode.json`
    async fn load_global() -> Result<Self> {
        let path = Self::global_config_path()?;
        Self::load_from_file(&path).await
    }

    /// 从文件加载配置
    async fn load_from_file(path: &Path) -> Result<Self> {
        let content = tokio::fs::read_to_string(path)
            .await
            .context(format!("Failed to read config file: {:?}", path))?;

        let config: Self = serde_json::from_str(&content)
            .context("Failed to parse config file")?;

        Ok(config)
    }

    /// 合并另一个配置到当前配置
    ///
    /// 规则：其他配置的非空字段会覆盖当前配置
    pub fn merge(&mut self, other: Self) {
        // 实现合并逻辑
        if !other.default_model.is_empty() {
            self.default_model = other.default_model;
        }
        self.model_profiles.extend(other.model_profiles);
        // ...
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            model_profiles: HashMap::new(),
            default_model: String::from("claude"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.default_model, "claude");
    }

    #[tokio::test]
    async fn test_load_from_file() {
        // 测试实现
    }

    #[test]
    fn test_config_merge() {
        let mut config1 = Config::default();
        let config2 = Config {
            default_model: "gpt-4".to_string(),
            ..Default::default()
        };

        config1.merge(config2);
        assert_eq!(config1.default_model, "gpt-4");
    }
}
```

#### 开发要点

1. **使用 Anyhow 处理错误**
```rust
// ✅ 好
use anyhow::{Context, Result};

pub async fn load_config(path: &Path) -> Result<Config> {
    let content = tokio::fs::read_to_string(path)
        .await
        .context("Failed to read config")?;
    // ...
}

// ❌ 避免
pub async fn load_config(path: &Path) -> Result<Config> {
    let content = tokio::fs::read_to_string(path).await.unwrap();
    // ...
}
```

2. **异步优先**
```rust
// ✅ 使用 Tokio 异步 IO
use tokio::fs;

pub async fn read_file(path: &Path) -> Result<String> {
    fs::read_to_string(path).await
        .context("Failed to read file")
}

// ❌ 避免同步 IO
use std::fs;

pub fn read_file(path: &Path) -> Result<String> {
    fs::read_to_string(path)  // 阻塞！
        .context("Failed to read file")
}
```

3. **编写测试**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_unit_logic() {
        // 单元测试
    }

    #[tokio::test]
    async fn test_async_function() {
        // 异步测试
    }

    #[test]
    fn test_error_handling() {
        // 错误情况测试
    }
}
```

---

### 3️⃣ 测试阶段（Test）

**目标**: 确保代码质量和正确性

#### 测试检查清单

```bash
# 1. 运行所有测试
cargo test

# 2. 检查代码格式
cargo fmt --check

# 3. 运行 Clippy（无警告）
cargo clippy -- -D warnings

# 4. 构建项目
cargo build

# 5. 生成文档
cargo doc --no-deps

# 6. 运行特定测试（如需要）
cargo test config_tests -- --nocapture
```

#### 测试类型

##### 单元测试
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert!(!config.default_model.is_empty());
    }

    #[test]
    fn test_config_merge() {
        let mut c1 = Config::default();
        let c2 = Config {
            default_model: "new-model".to_string(),
            ..Default::default()
        };

        c1.merge(c2);
        assert_eq!(c1.default_model, "new-model");
    }
}
```

##### 集成测试
```rust
// tests/config_integration.rs
use kode_core::config::Config;
use std::path::PathBuf;

#[tokio::test]
async fn test_load_real_config() {
    let test_config = PathBuf::from("test_fixtures/config.json");
    let config = Config::load_from_file(&test_config).await;

    assert!(config.is_ok());
    let config = config.unwrap();
    assert_eq!(config.default_model, "claude");
}
```

#### 测试失败处理

1. **单元测试失败**
   - 仔细阅读错误信息
   - 使用 `--nocapture` 查看输出
   - 逐个修复，不跳过任何测试

2. **Clippy 警告**
   - 所有警告必须修复
   - 理解警告原因
   - 按 Clippy 建议改进代码

3. **格式检查失败**
   - 运行 `cargo fmt` 自动格式化
   - 提交前再次检查

---

### 4️⃣ 总结阶段（Summary）

**目标**: 记录完成的工作和关键信息

#### 更新 TODO.md

```markdown
## ✅ [Day 3-4] 配置系统

**完成时间**: 2025-12-18 18:00
**实际用时**: 4.5 小时
**预估用时**: 4 小时

### 完成的工作
- ✅ 定义 Config 结构体
- ✅ 实现 JSON 序列化/反序列化
- ✅ 实现配置文件查找（全局 + 项目）
- ✅ 实现配置合并逻辑
- ✅ 添加环境变量支持
- ✅ 编写 12 个单元测试
- ✅ 编写 3 个集成测试
- ✅ 完整的 rustdoc 文档

### 关键代码位置
- `crates/kode-core/src/config/mod.rs` - 主要实现（250 行）
- `crates/kode-core/src/config/types.rs` - 类型定义（100 行）
- `crates/kode-core/src/config/tests.rs` - 单元测试（180 行）
- `tests/config_integration.rs` - 集成测试（60 行）

### 测试结果
- ✅ 15 个测试全部通过
- ✅ Clippy 无警告
- ✅ 代码覆盖率 85%

### 遇到的问题
1. 配置合并逻辑比预期复杂，花费额外 30 分钟
2. serde camelCase 序列化需要特别配置

### 解决方案
1. 参考 TS 版本实现，简化合并逻辑
2. 使用 `#[serde(rename_all = "camelCase")]`

### 后续建议
- 添加配置验证逻辑（检查必填字段）
- 添加配置迁移工具（版本升级）
- 考虑支持 TOML 格式（可选）
```

---

### 5️⃣ 反思阶段（Reflect）

**目标**: 识别改进机会和潜在问题

#### 反思检查清单

```markdown
## 反思：配置系统实现

### 代码质量 ✅
- [x] 代码遵循 Rust 最佳实践
- [x] 没有过度设计
- [x] 错误处理完善
- [x] 没有明显性能瓶颈
- [x] 命名清晰易懂

### 测试覆盖 ✅
- [x] 关键路径都有测试
- [x] 边界情况已考虑
- [x] 错误情况已测试
- [ ] 可以添加更多性能测试

### 文档质量 ✅
- [x] 公开 API 都有文档
- [x] 文档示例可运行
- [x] 复杂逻辑有注释

### 架构一致性 ✅
- [x] 符合 ARCHITECTURE.md 设计
- [x] 模块职责清晰
- [x] 依赖关系合理

### 兼容性 ✅
- [x] JSON 格式与 TS 版本一致
- [x] 字段命名（camelCase）一致
- [x] 配置行为一致

### 可以改进的地方
1. **性能**: 配置加载可以添加缓存
2. **功能**: 可以支持配置热重载
3. **测试**: 可以添加性能基准测试
4. **文档**: 可以添加更多使用示例

### 技术债务
- 暂无

### 经验教训
1. serde 的 `rename_all` 属性很有用
2. 配置合并逻辑应该先写测试
3. 环境变量支持可以使用 `envy` crate 简化
```

---

### 6️⃣ 更新阶段（Update）

**目标**: 同步更新所有相关文档和代码

#### 更新清单

1. **TODO.md**
   - ✅ 标记任务为完成
   - ✅ 添加完成总结
   - ✅ 更新进度百分比

2. **ROADMAP.md**
   - ✅ 更新阶段进度
   - ✅ 标记完成的任务
   - ⚠️ 调整后续计划（如有变化）

3. **ARCHITECTURE.md**
   - ⚠️ 仅在实现与设计有差异时更新
   - ⚠️ 补充实际实现细节

4. **Git 提交**
```bash
# 查看变更
git status
git diff

# 暂存文件
git add crates/kode-core/src/config/
git add tests/config_integration.rs
git add TODO.md
git add ROADMAP.md

# 提交（遵循 Conventional Commits）
git commit -m "feat(core): implement config loading system

- Add Config struct with JSON serialization
- Support global and project config files
- Implement config merge logic with env vars support
- Add 15 comprehensive tests (unit + integration)
- Complete rustdoc documentation

Performance:
- Config loading: <50ms (tested)
- Memory footprint: <1KB

Refs: TODO.md Phase 1 Day 3-4
Closes: #issue-number (如果有)"

# 推送（如果需要）
git push origin feature/config-system
```

#### Commit Message 规范

格式：
```
<type>(<scope>): <subject>

<body>

<footer>
```

**Type**:
- `feat`: 新功能
- `fix`: Bug 修复
- `docs`: 文档变更
- `style`: 代码格式
- `refactor`: 重构
- `test`: 测试
- `chore`: 构建/工具

**示例**:
```
feat(core): add agent loader with 5-tier priority

- Implement Agent struct with YAML frontmatter parsing
- Support 5 directory priorities (built-in, user, project)
- Add LRU cache for performance
- Handle loading errors with graceful degradation

Performance: Agent loading <100ms (cold), <20ms (cached)

Refs: ROADMAP.md Phase 2 Week 4
```

---

## 🔁 工作流循环

### 迭代开发

每完成一个小功能就走一遍工作流：

```
Task 1: Config 结构体
  → Plan → Execute → Test → Summary → Reflect → Update

Task 2: Config 加载
  → Plan → Execute → Test → Summary → Reflect → Update

Task 3: Config 合并
  → Plan → Execute → Test → Summary → Reflect → Update
```

### 每日节奏

**上午**:
- 查看 TODO.md
- 计划今日任务
- 开始第一个任务

**工作中**:
- 遵循 6 步工作流
- 定期提交代码
- 及时更新进度

**下午/晚上**:
- 完成任务总结
- 更新 TODO.md
- 规划明日工作

---

## 📊 进度跟踪

### 使用 TODO.md

TODO.md 是项目的**唯一真相来源**，必须实时更新。

#### 状态符号

- `[ ]` - 未开始
- `[~]` - 进行中（可注明进度百分比）
- `[x]` - 已完成
- `[!]` - 被阻塞
- `[-]` - 已取消

#### 进度计算

```markdown
**总体进度**: 35% (7/20 任务完成)

- Phase 1: 70% (7/10)
- Phase 2: 0% (0/6)
- Phase 3: 0% (0/4)
```

---

## 🚨 异常处理

### 任务被阻塞

```markdown
## [!] [Day 5] MCP 客户端实现

**阻塞原因**: MCP 协议文档不够清晰
**开始时间**: 2025-12-18
**阻塞时间**: 2 小时

### 尝试的解决方案
1. 查阅 MCP 规范 - 部分解决
2. 参考 TS 版本实现 - 有帮助
3. 查找社区实现 - 未找到

### 需要的帮助
- 明确 MCP 工具调用的 JSON 格式
- 确认 MCP 服务器的启动方式

### 临时方案
先实现基础功能，MCP 放到 Phase 3
```

### 时间超支

```markdown
## [x] [Day 3-4] 配置系统

**预估时间**: 4 小时
**实际时间**: 6 小时
**超支原因**: 配置合并逻辑比预期复杂

### 影响
- Day 5-7 任务延后 2 小时
- Phase 1 总体延后半天

### 调整方案
- 缩短 Day 5 任务范围
- 周末补足进度
```

---

## 📈 质量保证

### 代码审查自检

每次提交前：

```markdown
- [ ] 功能完整实现
- [ ] 所有测试通过
- [ ] 无 Clippy 警告
- [ ] 代码已格式化
- [ ] 文档注释完整
- [ ] TODO.md 已更新
- [ ] Commit message 规范
```

### 定期回顾

**每周回顾**:
- 回顾本周完成的任务
- 评估进度和质量
- 调整下周计划

**阶段回顾**:
- 每个 Phase 结束后总结
- 记录经验教训
- 更新后续计划

---

## 🎯 效率提升

### 工具使用

```bash
# 快速测试单个模块
cargo test --package kode-core --lib config

# 监听文件变化自动测试
cargo watch -x test

# 生成测试覆盖率
cargo tarpaulin --out Html

# 性能分析
cargo flamegraph
```

### 常用命令别名

```bash
# 添加到 .bashrc 或 .zshrc
alias ct="cargo test"
alias cc="cargo clippy -- -D warnings"
alias cf="cargo fmt"
alias cb="cargo build"
alias cr="cargo run"
```

---

## 📝 总结

记住核心工作流：

1. **Plan** - 明确目标，制定方案
2. **Execute** - 高质量实现
3. **Test** - 保证正确性
4. **Summary** - 记录成果
5. **Reflect** - 识别改进
6. **Update** - 同步文档

坚持这个工作流，项目将：
- ✅ 进度可控
- ✅ 质量保证
- ✅ 文档同步
- ✅ 持续改进

**让我们开始构建 Kode-Rust！** 🚀
