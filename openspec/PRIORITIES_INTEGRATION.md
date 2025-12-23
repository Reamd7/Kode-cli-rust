# OpenSpec 优先级系统集成 / OpenSpec Priority System Integration

## 概述 / Overview

本文档说明如何将 Spec 优先级系统集成到 OpenSpec 工作流中。

This document explains how the Spec priority system is integrated into the OpenSpec workflow.

## 文件结构 / File Structure

```
openspec/
├── SPEC_PRIORITIES.md          # 优先级定义和工作流集成指南
├── AGENTS.md                   # 已更新：包含优先级检查步骤
├── PRIORITIES_INTEGRATION.md   # 本文档：集成说明
├── specs/                      # 功能规范
│   ├── config-loading/         # P0
│   ├── message-model/          # P0
│   ├── anthropic-service/      # P1
│   ├── agent-system/           # P1
│   ├── tool-system/            # P1
│   ├── cli-commands/           # P2
│   ├── tui-interface/          # P2
│   └── mcp-integration/        # P3
└── changes/                    # 变更提案
```

## 工作流集成点 / Workflow Integration Points

### 1. Proposal 阶段 / Proposal Stage

**触发点**: `/openspec:proposal` 或创建新的变更提案时

**集成方式**:
```
用户请求创建变更
    ↓
AI 检查 openspec/SPEC_PRIORITIES.md
    ↓
验证依赖 spec 是否已完成
    ↓
确认优先级合理
    ↓
创建变更提案
```

**关键检查**:
- 依赖的 P0/P1 spec 是否已完成？
- 是否按优先级顺序 (P0 → P1 → P2 → P3)？
- 是否有冲突的活跃变更？

### 2. Apply 阶段 / Apply Stage

**触发点**: `/openspec:apply <change-id>` 或实施变更时

**集成方式**:
```
有多个待实施的变更
    ↓
AI 检查 openspec/SPEC_PRIORITIES.md
    ↓
按优先级排序待实施变更
    ↓
选择最高优先级且依赖已满足的变更
    ↓
实施变更
```

**选择算法**:
```python
ready_changes = filter(depencies_satisfied, all_changes)
sorted_changes = sort_by_priority(ready_changes)
selected_change = sorted_changes[0]
```

### 3. Archive 阶段 / Archive Stage

**触发点**: `/openspec:archive <change-id>` 或归档变更时

**集成方式**:
```
请求归档变更
    ↓
AI 检查 openspec/SPEC_PRIORITIES.md
    ↓
验证归档前检查清单
    ↓
确认不影响其他活跃变更
    ↓
执行归档
```

**验证检查**:
- tasks.md 所有任务已完成？
- 所有测试通过？
- Spec delta 正确应用？
- 不破坏其他变更的依赖？

## 使用示例 / Usage Examples

### 示例 1: 创建新变更 / Creating New Change

```bash
# 用户请求
/openspec:proposal implement-mcp-full

# AI 检查
1. 查看 SPEC_PRIORITIES.md
2. MCP 集成是 P3，依赖 tool-system (P1)
3. 检查 tool-system 状态
4. 如果未完成，警告用户并建议先实现 tool-system
```

### 示例 2: 选择要实施的变更 / Selecting Changes

```bash
# 待实施变更
- implement-mcp-full (P3)
- implement-anthropic-service (P1)
- implement-config-loading (P0)

# AI 决策
1. 查看 SPEC_PRIORITIES.md
2. 优先级: P0 > P1 > P3
3. 选择 implement-config-loading
```

### 示例 3: 归档前验证 / Pre-Archive Validation

```bash
# 请求归档
/openspec:archive implement-config-loading

# AI 验证
1. 查看 SPEC_PRIORITIES.md 的归档检查清单
2. 验证所有任务完成
3. 运行测试套件
4. 验证 spec 更新
5. 确认不破坏依赖此 spec 的其他变更
```

## 维护指南 / Maintenance Guide

### 更新优先级 / Updating Priorities

当需要调整优先级时：

1. **更新 SPEC_PRIORITIES.md**
   ```markdown
   ## 📋 优先级分类
   ### 🔴 P0 - 核心基础
   - [添加/移除/调整 spec]
   ```

2. **更新依赖关系图**
   ```markdown
   ## 📊 依赖关系图
   [更新依赖树和完成状态]
   ```

3. **注明变更原因**
   ```markdown
   > 最后更新 / Last Updated: YYYY-MM-DD
   > 变更原因 / Change Reason: [说明原因]
   ```

### 添加新 Spec / Adding New Specs

当添加新功能规范时：

1. **评估优先级**
   - 依赖哪些现有 spec？
   - 被哪些 spec 依赖？
   - 对核心功能的重要性？

2. **更新 SPEC_PRIORITIES.md**
   ```markdown
   #### X. new-spec (优先级)
   - **依赖关系**: 依赖 spec-a, spec-b
   - **实现原因**: ...
   ```

3. **更新依赖关系图**
   ```markdown
   spec-a
       ↓
   new-spec ← 新增
   ```

## 快速参考 / Quick Reference

### 常用命令 / Common Commands

```bash
# 查看 spec 列表
npx openspec list --specs

# 查看活跃变更
npx openspec list

# 查看特定 spec
npx openspec show <spec-id>

# 验证所有规范
npx openspec validate --strict

# 查看优先级文档
cat openspec/SPEC_PRIORITIES.md
```

### 优先级速查表 / Priority Cheat Sheet

```
P0 (最高): config-loading, message-model
P1:      anthropic-service, agent-system, tool-system
P2:      cli-commands, tui-interface
P3:      mcp-integration
```

### 依赖链 / Dependency Chain

```
config-loading → message-model → P1 services → P2 UI → P3 advanced
```

## 故障排查 / Troubleshooting

### 问题: 变更无法创建 / Change Cannot Be Created

**可能原因**:
1. 依赖的 spec 未完成
2. 优先级不合理
3. 有冲突的活跃变更

**解决方案**:
1. 查看 `SPEC_PRIORITIES.md`
2. 确认依赖完成状态: `npx openspec list --specs`
3. 调整优先级或等待依赖完成

### 问题: 变更选择不明确 / Change Selection Ambiguous

**可能原因**:
1. 多个同优先级变更
2. 依赖关系复杂

**解决方案**:
1. 查看 `SPEC_PRIORITIES.md` 的选择算法
2. 优先实施被依赖的变更
3. 优先实施完成度高的变更

### 问题: 归档失败 / Archive Fails

**可能原因**:
1. tasks.md 未完成
2. 测试失败
3. 破坏其他变更依赖

**解决方案**:
1. 查看 `SPEC_PRIORITIES.md` 的归档检查清单
2. 完成所有任务
3. 修复测试
4. 确认不影响其他变更

## 总结 / Summary

优先级系统通过以下方式集成到 OpenSpec 工作流：

1. **文档中心化**: `SPEC_PRIORITIES.md` 作为单一真相来源
2. **工作流嵌入**: 在 AGENTS.md 的三个阶段都有检查点
3. **自动化决策**: AI 可以根据优先级自动选择变更
4. **可维护性**: 清晰的更新和维护流程

这确保了：
- ✅ 变更按正确顺序实施
- ✅ 依赖关系得到遵守
- ✅ 资源分配合理
- ✅ 减少阻塞和返工

---

**维护者**: 请在修改优先级或添加新 spec 时同步更新本文档。
