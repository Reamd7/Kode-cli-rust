---
description: 实施已批准的 OpenSpec 变更并同步任务 / Implement an approved OpenSpec change and keep tasks in sync.
argument-hint: change-id
---
<!-- OPENSPEC:START -->
**Guardrails / 质量护栏**
- Favor straightforward, minimal implementations first and add complexity only when it is requested or clearly required.
  / 优先采用直接、最小化的实现，仅在明确需要或被要求时增加复杂性。
- Keep changes tightly scoped to the requested outcome.
  / 保持变更范围严格聚焦于既定目标。
- Refer to `openspec/AGENTS.md` (located inside the `openspec/` directory—run `ls openspec` or `openspec update` if you don't see it) if you need additional OpenSpec conventions or clarifications.
  / 如需更多 OpenSpec 约定或说明，请参阅 `openspec/AGENTS.md`（位于 `openspec/` 目录内——如果看不到，运行 `ls openspec` 或 `openspec update`）。

**Steps / 步骤**
Track these steps as TODOs and complete them one by one.
/ 将这些步骤作为 TODO 跟踪并逐一完成。
1. Read `changes/<id>/proposal.md`, `design.md` (if present), and `tasks.md` to confirm scope and acceptance criteria.
   / 阅读 `changes/<id>/proposal.md`、`design.md`（如果存在）和 `tasks.md` 以确认范围和验收标准。
2. Work through tasks sequentially, keeping edits minimal and focused on the requested change.
   / 按顺序处理任务，保持编辑最小化并聚焦于请求的变更。
3. Confirm completion before updating statuses—make sure every item in `tasks.md` is finished.
   / 在更新状态之前确认完成——确保 `tasks.md` 中的每一项都已完成。
4. Update the checklist after all work is done so each task is marked `- [x]` and reflects reality.
   / 所有工作完成后更新检查清单，使每个任务标记为 `- [x]` 并反映实际情况。
5. Reference `openspec list` or `openspec show <item>` when additional context is required.
   / 需要额外上下文时，参考 `openspec list` 或 `openspec show <item>`。

**Reference / 参考**
- Use `openspec show <id> --json --deltas-only` if you need additional context from the proposal while implementing.
  / 如果在实施时需要提案的额外上下文，使用 `openspec show <id> --json --deltas-only`。

$ARGUMENTS
<!-- OPENSPEC:END -->
