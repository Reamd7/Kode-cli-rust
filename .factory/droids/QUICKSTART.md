# Spec-Comparer Droid - 快速开始

## 一句话介绍

`spec-comparer` 是一个专门的 sub-agent，用于对比 OpenSpec 规范与 TypeScript 仓库的实现，并自动更新相关文档。

## 启用（一次性操作）

### 方法 1: 通过设置界面
```
/droids
→ 选择 "Create a new Droid" 或查看已存在的 droid
```

### 方法 2: 手动启用
编辑 `~/.factory/settings.json`：
```json
{
  "enableCustomDroids": true
}
```

## 使用示例

### 示例 1: 对比 tool-system spec
```
请使用 spec-comparer 对比 tool-system spec
```

### 示例 2: 对比并更新 config-loading spec
```
使用 spec-comparer 分析 config-loading spec 与 TS 实现的差异
```

### 示例 3: 通过 Task 工具调用
```
Run the Task tool with subagent spec-comparer to analyze the message-model spec
```

## 它会做什么

1. **阅读规范** - 读取 `openspec/specs/<name>/spec.md`
2. **查找实现** - 在 TypeScript 仓库中搜索对应代码
3. **对比分析** - 识别已实现、部分实现、未实现的功能
4. **生成报告** - 输出详细的对比报告
5. **更新文档** - 更新 proposal.md、tasks.md、design.md

## 重要提醒

⚠️ **spec-comparer 会严格遵守 OpenSpec 格式规范**：
- 双语格式（中英文）
- Scenario 格式（#### Scenario:）
- SHALL/MUST 关键字位置
- Delta 格式（ADDED/MODIFIED/REMOVED）

✅ **更新前会展示对比结果并等待确认**

## 支持的 Spec

```
config-loading      ✅ 已完成
agent-system        ✅ 已完成
message-model       ⬜ 未开始
tool-system         ⬜ 未开始
anthropic-service   ⬜ 未开始
openai-service      ⬜ 未开始
cli-commands        ⬜ 未开始
tui-interface       ⬜ 未开始
mcp-integration     ⬜ 未开始
```

## 相关文档

- [详细使用指南](.factory/droids/README.md)
- [OpenSpec 工作流程](../openspec/AGENTS.md)
- [Spec 优先级](../openspec/SPEC_PRIORITIES.md)
