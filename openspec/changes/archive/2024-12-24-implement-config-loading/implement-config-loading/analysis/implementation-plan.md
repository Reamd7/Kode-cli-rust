# 配置系统重新实施计划

> **文档说明 / Document Notice**
>
> 本文档是 [implement-config-loading](../) 变更的实施计划文档，说明迁移策略和关键差异。
>
> This document is an implementation plan for the [implement-config-loading](../) change, explaining migration strategy and key differences.

---

## 当前状态

✅ **已完成的分析**:
1. 深入分析 TypeScript 版本配置系统（940 行代码）
2. 创建完整的分析文档
3. 重写 spec.md（包含 5 层配置优先级）
4. 明确配置文件路径和 API 函数

## 下一步工作

### 1. 重写 design.md
需要包含：
- 配置优先级架构设计
- 单文件架构设计
- 环境变量集成设计
- 配置迁移策略
- API 设计（30 个函数）

### 2. 重写 tasks.md
基于 30 个 API 函数创建详细的任务清单：
- 核心配置加载/保存（4 个函数）
- 环境变量处理（2 个函数）
- 配置迁移（1 个函数）
- 模型系统（2 个函数）
- CLI 工具（4 个函数）
- 配置键验证（2 个函数）
- 工具函数（6 个函数）
- GPT-5 支持（5 个函数）
- MCP 支持（4 个函数）

### 3. 从头实施
清空现有代码，基于 spec.md 重新实现。

## 关键差异总结

### 与之前实现的差异
1. **配置优先级**: 明确 5 层优先级（之前没有）
2. **环境变量**: 完整的环境变量支持（之前没有）
3. **API 函数**: 需要实现 30 个函数（之前只有 6 个）
4. **配置迁移**: 需要自动迁移逻辑（之前没有）
5. **保存优化**: 只保存差异字段（之前保存全部）

### TypeScript 版本的关键特性
1. 单文件架构（~/.kode.json）
2. 项目配置存储在 projects 字段中
3. 环境变量可覆盖配置路径
4. 错误容忍（不存在/解析失败都返回默认值）
5. 30 个公开 API 函数

## 文件清单

### 分析文档
- `docs/analysis/typescript-config-system.md` - 配置系统分析
- `docs/analysis/config-api-reference.md` - API 参考

### OpenSpec 文档（待更新）
- `openspec/specs/config-loading/spec.md` - ✅ 已更新
- `openspec/specs/config-loading/design.md` - ⏸️ 待更新
- `openspec/changes/implement-config-loading/tasks.md` - ⏸️ 待更新

### 实施文件（待重写）
- `crates/kode-core/src/config/types.rs` - 类型定义
- `crates/kode-core/src/config/loader.rs` - 配置加载器
- `crates/kode-core/src/config/env.rs` - 环境变量（新建）
- `crates/kode-core/src/config/mod.rs` - 模块导出

## 立即行动

1. 重写 design.md（包含架构设计）
2. 重写 tasks.md（包含 30 个 API 函数）
3. 创建实施分支并清空现有代码
4. 从头实施配置系统
