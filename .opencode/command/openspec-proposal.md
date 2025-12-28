---
description: 创建新的 OpenSpec 变更并进行严格验证 / Scaffold a new OpenSpec change and validate strictly.
argument-hint: request or feature description / 请求或功能描述
---
<!-- OPENSPEC:START -->
**Guardrails / 质量护栏**
- Favor straightforward, minimal implementations first and add complexity only when it is requested or clearly required.
  / 优先采用直接、最小化的实现，仅在明确需要或被要求时增加复杂性。
- Keep changes tightly scoped to the requested outcome.
  / 保持变更范围严格聚焦于既定目标。
- Refer to `openspec/AGENTS.md` (located inside the `openspec/` directory—run `ls openspec` or `openspec update` if you don't see it) if you need additional OpenSpec conventions or clarifications.
  / 如需更多 OpenSpec 约定或说明，请参阅 `openspec/AGENTS.md`（位于 `openspec/` 目录内——如果看不到，运行 `ls openspec` 或 `openspec update`）。
- Identify any vague or ambiguous details and ask the necessary follow-up questions before editing files.
  / 在编辑文件之前，识别任何模糊或不明确的细节并提出必要的后续问题。
- Do not write any code during the proposal stage. Only create design documents (proposal.md, tasks.md, design.md, and spec deltas). Implementation happens in the apply stage after approval.
  / 在提案阶段不要编写任何代码。仅创建设计文档（proposal.md、tasks.md、design.md 和规范增量）。实施在批准后的应用阶段进行。

**Steps / 步骤**
1. Review `openspec/project.md`, run `openspec list` and `openspec list --specs`, and inspect related code or docs (e.g., via `rg`/`ls`) to ground the proposal in current behaviour; note any gaps that require clarification.
   / 审查 `openspec/project.md`，运行 `openspec list` 和 `openspec list --specs`，并检查相关代码或文档（例如通过 `rg`/`ls`），使提案基于当前行为；记录任何需要澄清的差距。
2. Choose a unique verb-led `change-id` and scaffold `proposal.md`, `tasks.md`, and `design.md` (when needed) under `openspec/changes/<id>/`.
   / 选择唯一的动词引导的 `change-id`，并在 `openspec/changes/<id>/` 下搭建 `proposal.md`、`tasks.md` 和 `design.md`（需要时）。
3. Map the change into concrete capabilities or requirements, breaking multi-scope efforts into distinct spec deltas with clear relationships and sequencing.
   / 将变更映射到具体能力或需求，将多范围工作分解为具有明确关系和顺序的独立规范增量。
4. Capture architectural reasoning in `design.md` when the solution spans multiple systems, introduces new patterns, or demands trade-off discussion before committing to specs.
   / 当解决方案跨越多个系统、引入新模式或在提交规范前需要权衡讨论时，在 `design.md` 中捕获架构推理。
5. Draft spec deltas in `changes/<id>/specs/<capability>/spec.md` (one folder per capability) using `## ADDED|MODIFIED|REMOVED Requirements` with at least one `#### Scenario:` per requirement and cross-reference related capabilities when relevant.
   / 在 `changes/<id>/specs/<capability>/spec.md`（每个能力一个文件夹）中起草规范增量，使用 `## ADDED|MODIFIED|REMOVED Requirements`，每个需求至少包含一个 `#### Scenario:`，并在相关时交叉引用相关能力。
6. Draft `tasks.md` as an ordered list of small, verifiable work items that deliver user-visible progress, include validation (tests, tooling), and highlight dependencies or parallelizable work.
   / 将 `tasks.md` 起草为有序的小型可验证工作项列表，提供用户可见的进度，包括验证（测试、工具），并突出显示依赖关系或可并行的工作。
7. Validate with `openspec validate <id> --strict` and resolve every issue before sharing the proposal.
   / 使用 `openspec validate <id> --strict` 验证，并在共享提案之前解决每个问题。

**Reference / 参考**
- Use `openspec show <id> --json --deltas-only` or `openspec show <spec> --type spec` to inspect details when validation fails.
  / 验证失败时，使用 `openspec show <id> --json --deltas-only` 或 `openspec show <spec> --type spec` 检查详细信息。
- Search existing requirements with `rg -n "Requirement:|Scenario:" openspec/specs` before writing new ones.
  / 在编写新需求之前，使用 `rg -n "Requirement:|Scenario:" openspec/specs` 搜索现有需求。
- Explore the codebase with `rg <keyword>`, `ls`, or direct file reads so proposals align with current implementation realities.
  / 使用 `rg <keyword>`、`ls` 或直接文件读取探索代码库，使提案与当前实现现实保持一致。

$ARGUMENTS
<!-- OPENSPEC:END -->
