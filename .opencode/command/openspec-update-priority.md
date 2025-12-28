---
description: 自动扫描归档变更并更新 OpenSpec 优先级文档 / Automatically scan archived changes and update OpenSpec priority document
---
<!-- OPENSPEC:START -->
**Guardrails / 质量护栏**
- 自动扫描 `openspec/changes/archive/` 目录
- 自动对比 `openspec/SPEC_PRIORITIES.md` 中的状态
- 只更新落后的状态（未完成→已完成）
- 更新前创建备份文件
- 保持文档格式和结构不变

**Steps / 步骤**

1. **扫描归档目录 / Scan Archive Directory**
   - 列出 `openspec/changes/archive/` 下所有已归档的变更
   - 提取每个归档变更的名称（格式：`YYYY-MM-DD-change-name`）
   - 从变更名称中提取 spec ID（例如 `implement-agent-system` → `agent-system`）
   - 读取每个归档变更的 `specs/` 子目录，确定涉及的 spec

2. **读取当前优先级文档 / Read Current Priority Document**
   - 解析 `openspec/SPEC_PRIORITIES.md` 中的状态表格
   - 提取每个 spec 的当前状态和对应变更 ID

3. **对比状态 / Compare Status**
   - 对比已归档的变更和优先级文档中的状态
   - 识别需要更新的 spec：
     - 已归档但状态不是 `✅ 已完成` 的
     - 对应变更 ID 不匹配的（文档中是提案 ID，应该是归档名称）

4. **创建备份 / Create Backup**
   - 创建备份文件 `openspec/SPEC_PRIORITIES.md.backup-[YYYY-MM-DD]`
   - 输出备份文件路径

5. **自动更新状态 / Auto-update Status**
   - 将所有已归档 spec 的状态更新为 `✅ 已完成`
   - 将"对应变更"列更新为归档目录名（例如 `2025-12-24-implement-agent-system`）
   - 更新"最后更新"时间戳为当前日期
   - 保持表格格式和其他列不变

6. **输出更新报告 / Output Update Report**
   ```
   📊 优先级文档更新报告 / Priority Document Update Report

   发现的已归档变更 / Archived Changes Found:
   ✓ YYYY-MM-DD-change-name → spec-id

   需要更新的 spec / Specs to Update:
   ✓ spec-id: 状态 ⬜ 未开始 → ✅ 已完成
          对应变更: old-change-id → YYYY-MM-DD-change-name

   未在优先级文档中的 spec / Specs Not in Priority Document:
   ⚠ spec-id (已归档但文档中不存在)

   更新完成 / Update Complete
   备份位置 / Backup: openspec/SPEC_PRIORITIES.md.backup-YYYY-MM-DD
   ```

**示例输出 / Example Output**

```
📊 优先级文档更新报告

发现的已归档变更:
✓ 2024-12-24-implement-config-loading → config-loading
✓ 2025-12-24-implement-agent-system → agent-system
✓ 2025-12-24-implement-message-model → message-model

需要更新的 spec:
✓ message-model: 状态 ⬜ 未开始 → ✅ 已完成
              对应变更: implement-message-model → 2025-12-24-implement-message-model

更新完成！已更新 1 个 spec
备份: openspec/SPEC_PRIORITIES.md.backup-2025-12-24
```

**Reference / 参考**
- 归档目录: `openspec/changes/archive/`
- 优先级文档: `openspec/SPEC_PRIORITIES.md`
- 查看已归档变更: `ls openspec/changes/archive/`

$ARGUMENTS
<!-- OPENSPEC:END -->
