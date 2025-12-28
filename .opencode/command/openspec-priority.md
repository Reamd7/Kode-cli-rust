---
description: 分析 OpenSpec 变更提案并推荐优先级最高的实施目标 / Analyze OpenSpec changes and recommend highest priority implementation target.
argument-hint: (可选) 可指定筛选条件，如 "P0" 或 "unblocked" / Optional filter like "P0" or "unblocked"
---
<!-- OPENSPEC:START -->
**Guardrails / 质量护栏**
- 严格依赖 openspec/SPEC_PRIORITIES.md 中定义的优先级规则
- 仅推荐依赖已满足的变更提案
- 当存在多个同等优先级的变更时，优先推荐阻塞其他变更的提案
- 推荐前必须验证依赖状态

**Steps / 步骤**
1. 读取 `openspec/SPEC_PRIORITIES.md` 理解优先级层级和依赖关系
2. 列出所有活跃的变更提案（openspec/changes/ 下未归档的变更）
3. 对每个变更：
   - 读取 proposal.md 获取依赖信息
   - 检查依赖的 spec 是否已完成（从 SPEC_PRIORITIES.md 的状态表）
   - 确定优先级（P0 > P1 > P2 > P3）
4. 应用选择算法：
   - 过滤掉依赖未满足的变更
   - 按优先级排序（P0 > P1 > P2 > P3）
   - 同优先级中，优先选择被其他变更依赖最多的（阻塞链优先）
5. 输出分析结果和推荐

**输出格式**
```
📊 OpenSpec 优先级分析 / Priority Analysis

活跃变更列表 / Active Changes:
| Change ID | Priority | Dependencies | Status | Description |
|-----------|----------|--------------|--------|-------------|
| ... | P0/P1/P2/P3 | ✅/⬜ | Ready/Blocked | ... |

推荐变更 / Recommended:
🎯 [change-id] - [简短描述]
   优先级: P0/P1/P2/P3
   依赖状态: ✅ 全部满足
   选择理由: [理由]

下一步 / Next Step:
openspec show <change-id> --type change
```

**Reference / 参考**
- 优先级定义: `openspec/SPEC_PRIORITIES.md`
- 查看变更详情: `openspec show <change-id> --type change`
- 列出所有变更: `openspec list`

$ARGUMENTS
<!-- OPENSPEC:END -->
