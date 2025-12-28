---
description: 对比 TypeScript 和 Rust 版本的实现完成度，验证所有功能是否已完整移植
argument-hint: (可选) 指定对比的模块，如 "commands", "tools", "services", "all"（默认）
---

# 进度对比命令 / Compare Progress Command

**目标**: 检查 Rust 版本是否完整实现了 TypeScript 版本的所有功能

**前提条件**:
- TypeScript 版本路径: `/Users/gemini/Documents/backup/Kode-cli`
- Rust 版本路径: 当前工作目录

## 执行步骤

### 1. 读取 SPEC_PRIORITIES.md
首先读取 `openspec/SPEC_PRIORITIES.md` 获取所有已规划的 spec 和它们的完成状态。

### 2. 对比核心模块

对以下核心模块进行逐项对比：

#### A. Commands (命令)
- TypeScript: 读取 `/Users/gemini/Documents/backup/Kode-cli/src/commands/` 目录
- Rust: 读取当前仓库的 `crates/kode-cli/src/commands/` 目录
- 列出所有命令文件，对比功能

#### B. Tools (工具)
- TypeScript: 读取 `/Users/gemini/Documents/backup/Kode-cli/src/tools/` 目录
- Rust: 读取当前仓库的 `crates/kode-tools/src/` 目录
- 列出所有工具实现

#### C. Services (服务)
- TypeScript: 读取 `/Users/gemini/Documents/backup/Kode-cli/src/services/` 目录
- Rust: 读取当前仓库的 `crates/kode-services/src/` 目录
- 对比 Anthropic、OpenAI 等服务

#### D. Core (核心)
- TypeScript: 读取 `/Users/gemini/Documents/backup/Kode-cli/src/` 下的核心文件
  - `context.ts`
  - `messages.ts`
  - `query.ts`
  - `Tool.ts`
  - `permissions.ts`
- Rust: 对应 crates 下的实现
  - `crates/kode-core/src/context.rs`
  - `crates/kode-core/src/message.rs`
  - `crates/kode-core/src/agent.rs`
  - `crates/kode-tools/src/tool.rs`

### 3. 详细分析每个模块

对每个模块，输出：
- ✅ **已完成**: Rust 版本有对应实现
- ⬜ **未实现**: TypeScript 有但 Rust 没有
- ⚠️ **部分实现**: 功能存在但可能不完整
- 🆕 **新增**: Rust 有但 TypeScript 没有（不应该出现）

### 4. 生成报告格式

```
📊 Kode TypeScript → Rust 进度对比报告
================================================================================

📋 概览 / Overview
  TypeScript 版本: /Users/gemini/Documents/backup/Kode-cli
  Rust 版本: (当前目录)
  生成时间: {timestamp}

🎯 总体进度
  已完成: X%
  待实现: Y 个模块

================================================================================

📌 1. Commands (命令)
--------------------------------------------------------------------------------
TypeScript 发现: N 个命令
  - command1.ts
  - command2.ts
  ...

Rust 实现: M 个命令
  - command1.rs ✅
  - command3.rs ⬜ (未实现)

对比结果:
  ✅ 已完成: [列表]
  ⬜ 未实现: [列表]
  ⚠️ 需验证: [列表]

================================================================================

🛠️ 2. Tools (工具)
--------------------------------------------------------------------------------
[同样格式]

================================================================================

🌐 3. Services (服务)
--------------------------------------------------------------------------------
[同样格式]

================================================================================

📦 4. Core Modules (核心模块)
--------------------------------------------------------------------------------
[同样格式]

================================================================================

📝 详细建议
--------------------------------------------------------------------------------
基于 SPEC_PRIORITIES.md 的优先级，建议下一步实现：

1. [优先级最高的未完成模块]
2. [下一个优先级]
...

参考命令:
  openspec show <spec-id>  # 查看详细规范
  openspec proposal        # 创建新的变更提案
```

## 实现要点

### 文件对比逻辑
1. 使用 `LS` 工具列出目录内容
2. 对 TypeScript 文件，读取文件名并去除扩展名
3. 对 Rust 文件，同样处理
4. 对比文件名（TS 的 `something.ts` vs Rust 的 `something.rs`）

### 功能完整性判断
对于已实现的文件，读取前 20-50 行，检查：
- 是否有基本的结构定义（trait/struct/impl）
- 是否有主要的方法/函数
- 是否有 TODO 或未完成的标记

### 优先级映射
根据 SPEC_PRIORITIES.md 中的定义：
- P0 (核心基础): config-loading, message-model
- P1 (核心服务): agent-system, tool-system, anthropic-service, openai-service
- P2 (用户界面): basic-cli, tui-interface, cli-commands
- P3 (高级特性): mcp-integration

### 错误处理
- 如果目录不存在，明确说明
- 如果无法读取文件，记录警告但继续
- 提供对比的统计数字

## 输出要求

1. **清晰的视觉层次**: 使用 emoji 和分隔线区分不同部分
2. **可操作的建议**: 明确指出下一步应该做什么
3. **数据驱动**: 提供具体的数字和文件列表
4. **优先级感知**: 根据 SPEC_PRIORITIES.md 排序建议
5. **双语支持**: 关键部分提供中英文对照

## 注意事项

- 不做深度代码分析，只做存在性对比
- 不判断代码质量，只判断是否有实现
- 如果用户指定了特定模块（如 "commands"），只对比该模块
- 默认对比所有模块（使用 "all" 或不传参数）

$ARGUMENTS
