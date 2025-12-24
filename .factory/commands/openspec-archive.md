---
description: 归档已部署的 OpenSpec 变更并更新规范 / Archive a deployed OpenSpec change and update specs.
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
1. Determine the change ID to archive / 确定要归档的变更 ID：
   - If this prompt already includes a specific change ID (for example inside a `<ChangeId>` block populated by slash-command arguments), use that value after trimming whitespace.
     / 如果此提示已包含特定变更 ID（例如在由斜杠命令参数填充的 `<ChangeId>` 块内），使用去除空格后的值。
   - If the conversation references a change loosely (for example by title or summary), run `openspec list` to surface likely IDs, share the relevant candidates, and confirm which one the user intends.
     / 如果对话中模糊引用了某个变更（例如通过标题或摘要），运行 `openspec list` 列出可能的 ID，分享相关候选项并确认用户意图。
   - Otherwise, review the conversation, run `openspec list`, and ask the user which change to archive; wait for a confirmed change ID before proceeding.
     / 否则，审查对话，运行 `openspec list`，并询问用户要归档哪个变更；在继续之前等待确认的变更 ID。
   - If you still cannot identify a single change ID, stop and tell the user you cannot archive anything yet.
     / 如果仍然无法识别单个变更 ID，停止并告知用户尚无法归档任何内容。
2. Validate the change ID by running `openspec list` (or `openspec show <id>`) and stop if the change is missing, already archived, or otherwise not ready to archive.
   / 通过运行 `openspec list`（或 `openspec show <id>`）验证变更 ID，如果变更缺失、已归档或未准备好归档，则停止。
3. Run `openspec archive <id> --yes` so the CLI moves the change and applies spec updates without prompts (use `--skip-specs` only for tooling-only work).
   / 运行 `openspec archive <id> --yes`，使 CLI 在无提示的情况下移动变更并应用规范更新（仅用于纯工具工作时使用 `--skip-specs`）。
4. Review the command output to confirm the target specs were updated and the change landed in `changes/archive/`.
   / 检查命令输出，确认目标规范已更新，变更已放入 `changes/archive/`。
5. **自动更新优先级文档 / Auto-update Priority Document**：
   - 从归档输出中提取涉及的 spec ID 列表
   - 更新 `openspec/SPEC_PRIORITIES.md` 中对应 spec 的状态为 `✅ 已完成`
   - 更新"对应变更"列为归档目录名（如 `2025-12-24-implement-agent-system`）
   - 更新"最后更新"时间戳为当前日期
   - Extract list of involved spec IDs from archive output
   - Update corresponding spec status in `openspec/SPEC_PRIORITIES.md` to `✅ 已完成`
   - Update "对应变更" column to archive directory name (e.g., `2025-12-24-implement-agent-system`)
   - Update "最后更新" timestamp to current date
6. Validate with `openspec validate --strict` and inspect with `openspec show <id>` if anything looks off.
   / 使用 `openspec validate --strict` 验证，如有异常使用 `openspec show <id>` 检查。

**Reference / 参考**
- Use `openspec list` to confirm change IDs before archiving.
  / 归档前使用 `openspec list` 确认变更 ID。
- Inspect refreshed specs with `openspec list --specs` and address any validation issues before handing off.
  / 使用 `openspec list --specs` 检查刷新后的规范，并在移交前解决任何验证问题。

$ARGUMENTS
<!-- OPENSPEC:END -->
