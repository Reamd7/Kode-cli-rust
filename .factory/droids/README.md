# Spec-Comparer Droid 使用指南

## 概述

`spec-comparer` 是一个专门的 sub-agent（droid），用于对比 OpenSpec 规范与 TypeScript 仓库的实现，并更新相关的 openspec 文件。

## 启用 Droid

在 Factory CLI 设置中启用 Custom Droids：
1. 打开设置 (`Shift+Tab` → Settings)
2. 在 Experimental 部分启用 **Custom Droids**
3. 或者直接编辑 `~/.factory/settings.json`：
   ```json
   {
     "enableCustomDroids": true
   }
   ```

## 使用方法

### 方法 1: 通过 Task 工具调用

在对话中直接请求：
```
请使用 spec-comparer 对比 tool-system spec 与 TypeScript 实现
```

或使用 Task 工具：
```
Run the Task tool with subagent spec-comparer to analyze the tool-system spec
```

### 方法 2: 通过 /droids 命令

1. 运行 `/droids` 命令
2. 选择 `spec-comparer`
3. 按照提示提供 spec 名称

## 支持的 Spec

当前项目中可用的 spec：
- `config-loading` - 配置加载系统 ✅ 已完成
- `agent-system` - Agent 系统 ✅ 已完成
- `message-model` - 消息与模型 ⬜ 未开始
- `tool-system` - 工具系统 ⬜ 未开始
- `anthropic-service` - Anthropic 服务 ⬜ 未开始
- `openai-service` - OpenAI 服务 ⬜ 未开始
- `cli-commands` - CLI 命令 ⬜ 未开始
- `tui-interface` - TUI 界面 ⬜ 未开始
- `mcp-integration` - MCP 集成 ⬜ 未开始

## 工作流程

spec-comparer 执行以下步骤：

1. **读取 spec** - 从 `openspec/specs/<name>/spec.md` 读取规范
2. **查找实现** - 在 TypeScript 仓库中搜索对应代码
3. **对比分析** - 比较 spec requirements 与实际实现
4. **生成报告** - 输出详细的对比报告
5. **更新文件** - 根据对比结果更新 openspec 文件

## 输出格式

spec-comparer 会生成结构化的报告，包括：

- **已实现的功能** - ✅ 标记，包含代码位置
- **部分实现的功能** - ⚠️ 标记，说明缺失部分
- **未实现的功能** - ❌ 标记
- **更新建议** - 具体的文件修改建议
- **下一步行动** - 待办事项清单

## 示例输出

```markdown
# Spec 对比报告: tool-system

## 📋 Spec 概述
工具系统提供可扩展的工具框架...

## 🔍 实现对比

### 已实现的功能
- Tool trait 定义: ✅ 实现于 src/Tool.ts:15
- 工具注册表: ✅ 实现于 src/tools.ts:42
- FileReadTool: ✅ 实现于 src/tools/FileReadTool/

### 部分实现的功能
- 权限系统: ⚠️ 部分实现
  - 已实现: requiresPermission() 方法
  - 缺失: 细粒度的权限控制

### 未实现的功能
- (无)

## 📝 更新建议

### 需要更新的文件
1. `openspec/changes/implement-tool-system/tasks.md`
   - [x] 实现 Tool trait
   - [x] 实现工具注册表
   - [ ] 添加权限系统

## 🎯 下一步行动
- [ ] 更新 tasks.md 中的任务状态
- [ ] 补充 design.md 中的权限系统设计
```

## 配置详情

- **名称**: `spec-comparer`
- **模型**: `inherit` (使用与主会话相同的模型)
- **工具**: Read, Grep, Glob, Edit, MultiEdit, Create, Execute
- **位置**: `.factory/droids/spec-comparer.md`

## 注意事项

### 格式规范要求
spec-comparer 会严格遵守 OpenSpec 文档格式规范：

1. **双语格式**
   - 英文结构标题：`## Purpose`, `## Requirements`, `### Requirement:`, `#### Scenario:`
   - SHALL/MUST 关键字必须在英文描述中
   - 中文翻译用 `/` 前缀

2. **Scenario 格式**
   - 必须使用 `#### Scenario:` (4个井号)
   - 使用 `**WHEN**`, `**THEN**`, `**AND**` 格式

3. **Delta 格式**
   - `## ADDED Requirements` - 新增功能
   - `## MODIFIED Requirements` - 修改（包含完整文本）
   - `## REMOVED Requirements` - 废弃功能

4. **验证要求**
   - 更新后运行 `openspec validate --strict` 确保无错误

### 操作约束
1. **只读操作** - spec-comparer 不会修改 TypeScript 仓库中的代码
2. **确认更新** - 在修改 openspec 文件前会先展示对比结果并等待确认
3. **格式保持** - 更新文件时保持现有格式和结构
4. **文档优先** - 执行前必读 AGENTS.md、project.md、SPEC_PRIORITIES.md

## 故障排除

### Droid 未加载
- 检查 `enableCustomDroids` 是否为 `true`
- 确认文件路径：`.factory/droids/spec-comparer.md`
- 查看 Factory CLI 日志中的验证错误

### 对比结果不准确
- 确认 TypeScript 仓库路径正确
- 检查 spec 的 Reference 部分是否准确
- 尝试提供更具体的搜索关键词

## 相关文档

- [OpenSpec 工作流程](../../openspec/AGENTS.md)
- [Spec 优先级指南](../../openspec/SPEC_PRIORITIES.md)
- [项目规范](../../openspec/project.md)
