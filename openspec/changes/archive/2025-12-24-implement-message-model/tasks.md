# 实现任务 - 消息模型系统 / Implementation Tasks - Message Model System

## ⚠️ 实现状态说明

当前任务列表分为**已实现**（✅）、**部分实现**（⚠️）和**未实现**（⬜）三类。

> **2024-12-24 更新**: 根据与 TypeScript 版本代码的对比分析，更新了功能差异表和待实现任务。

---

## 1. 数据结构定义 / Data Structure Definition

### ✅ 已实现

- [x] 1.1 定义 `Message` 结构体
  - [x] 定义 `Role` 枚举 (User, Assistant, System)
  - [x] 定义 `MessageContent` 枚举 (Text, Blocks)
  - [x] 定义 `ContentBlock` 枚举 (Text, ToolUse, ToolResult, Image)
  - [x] 添加 serde 序列化支持

- [x] 1.2 定义 `ToolUseBlock` 结构
  - [x] tool_use_id 字段
  - [x] tool_name 字段
  - [x] parameters 字段 (JSON Value)

- [x] 1.3 定义 `ToolResultBlock` 结构
  - [x] tool_use_id 字段
  - [x] content 字段
  - [x] is_error 字段

- [x] 1.4 定义 `ImageBlock` 结构
  - [x] image_type 字段
  - [x] media_type 字段
  - [x] data 字段

### ⬜ 未实现（根据 TS 版本对比发现的功能差异）

以下功能在 TypeScript 版本中存在，需要补充实现：

- [ ] 1.5 消息元数据（对应 TypeScript 的 `Message` 类型）
  - **TS 版本位置**: `/src/types/conversation.ts` L17-26
  - **TS 版本来源**: `@anthropic-ai/sdk` 的 `APIAssistantMessage` 和 `MessageParam`
  - **差异说明**: TS 版本使用 SDK 的类型，Rust 版本使用自定义类型
  - [ ] 添加 `id: Uuid` 字段（消息唯一标识）
  - [ ] 添加 `timestamp: DateTime<Utc>` 字段（创建时间）
  - [ ] 添加 `cost_usd: Option<f64>` 字段（由服务层填充）
  - [ ] 添加 `duration_ms: Option<u64>` 字段（由服务层填充）

- [ ] 1.6 ProgressMessage 类型（工具执行进度）
  - **TS 版本位置**: `/src/query.ts` L92-101
  - **功能说明**: 用于在工具执行期间显示进度更新
  - **Rust 设计建议**: 创建 `ProgressMessage` 枚举或结构体
  - [ ] 定义 `ProgressMessage` 结构
  - [ ] 包含 `content: AssistantMessage` 字段
  - [ ] 包含 `tool_use_id: String` 字段
  - [ ] 包含 `sibling_tool_use_ids: HashSet<String>` 字段
  - [ ] 包含 `tools: Vec<Tool>` 字段
  - [ ] 包含 Vec<NormalizedMessage `normalized_messages:>` 字段

- [ ] 1.7 用户消息选项（UserMessage options）
  - **TS 版本位置**: `/src/types/conversation.ts` L7-14
  - [ ] 添加 `is_koding_request: Option<bool>` 字段
  - [ ] 添加 `koding_context: Option<String>` 字段
  - [ ] 添加 `tool_use_result: Option<FullToolUseResult>` 字段

- [ ] 1.8 消息规范化功能
  - **TS 版本位置**: `/src/utils/messages.tsx` 中的 `normalizeMessagesForAPI`
  - **功能说明**: 将内部消息格式转换为 API 格式
  - [ ] 实现 `normalize_messages_for_api()` 函数
  - [ ] 处理 ContentBlock 到 API 格式的转换
  - [ ] 支持 thinking blocks 处理

- [ ] 1.9 对话恢复功能
  - **TS 版本位置**: `/src/utils/conversationRecovery.ts`
  - [ ] 实现对话状态序列化和反序列化
  - [ ] 支持从持久化文件恢复对话

## 2. ModelAdapter trait / Model Adapter Trait

### ✅ 已实现

- [x] 2.1 定义 `ModelAdapter` trait
  - [x] `send_message()` 方法（非流式）
  - [x] `stream_message()` 方法（流式）
  - [x] `model_name()` 方法
  - [x] `supports_streaming()` 方法
  - [x] 添加 `#[async_trait]`

- [x] 2.2 定义 `StreamChunk` 枚举
  - [x] ContentBlockStart { index }
  - [x] ContentBlockDelta { index, delta }
  - [x] ContentBlockStop { index }
  - [x] ToolUse { tool_name, tool_use_id, parameters }
  - [x] MessageStop { usage }
  - [x] Error { message }

- [x] 2.3 定义 `StreamingResponse` 包装器
  - [x] 实现 Stream trait
  - [x] 基于 tokio mpsc channel 的流式实现
  - [x] 提供 `channel()` 辅助方法

- [x] 2.4 定义 `TokenUsage` 结构
  - [x] input_tokens
  - [x] output_tokens
  - [x] total_tokens (可选)

### ⬜ 未实现

- [ ] 2.5 累积使用量追踪（对应 TypeScript 的 `cumulativeUsage`）
  - **TS 版本位置**: `/src/services/claude.ts` 中使用
  - [ ] 在 `StreamingResponse` 中累积 token 使用量
  - [ ] 提供 `cumulative_usage()` 方法

- [ ] 2.6 Thinking blocks 支持
  - **TS 版本位置**: `/src/query.ts` L117-131 注释中的规则说明
  - **功能说明**: Anthropic 的 thinking/redacted_thinking 块支持
  - [ ] 在 `StreamChunk` 中添加 `ThinkingStart`, `ThinkingDelta`, `ThinkingStop` 变体
  - [ ] 实现 thinking 块的序列化规则

- [ ] 2.7 工具构建方法
  - **TS 版本位置**: 存在于 TS 版本服务层
  - [ ] 评估是否需要在 trait 中添加 `build_tools()` 方法

## 3. 错误类型 / Error Types

### ✅ 已实现

- [x] 3.1 定义模型相关错误（在 `error.rs` 中）
  - [x] ModelRequestError - API 请求失败
  - [x] ModelResponseError - 响应解析失败
  - [x] ModelStreamError - 流式响应错误
  - [x] ModelNotConfigured - 模型未配置

## 4. 模型管理器 / Model Manager

### ⬜ 未实现（由 `implement-model-switching` 变更负责）

规范要求的以下功能将由独立的变更提案实现：

- [ ] 4.1 `ModelManager` 结构
  - [ ] 管理多个模型配置
  - [ ] 运行时模型切换
  - [ ] 模型配置验证

- [ ] 4.2 模型指针（Model Pointers）
  - [ ] default 指针
  - [ ] task 指针
  - [ ] architect 指针

## 5. 消息上下文管理 / Message Context Management

### ⬜ 未实现（由 `implement-context-management` 变更负责）

规范要求的以下功能将由独立的变更提案实现：

- [ ] 5.1 `MessageContextManager` 结构
  - [ ] 添加消息到上下文
  - [ ] Token 计数
  - [ ] 智能裁剪策略

- [ ] 5.2 Token 计数器
  - [ ] 字符估算
  - [ ] 集成 tokenizer

## 6. 测试 / Testing

### ✅ 已实现

- [x] 6.1 单元测试
  - [x] 测试 Message 序列化/反序列化
  - [x] 测试 Role 枚举
  - [x] 测试 ContentBlock 序列化
  - [x] 测试 ToolUse/ToolResult
  - [x] 测试 StreamChunk
  - [x] 测试 TokenUsage
  - [x] 测试 ModelAdapter trait（MockAdapter）

### ⬜ 未实现

- [ ] 6.2 集成测试
  - [ ] 与实际模型服务的集成测试（待服务实现后）

## 7. 文档 / Documentation

### ✅ 已实现

- [x] 7.1 Rustdoc 注释
  - [x] Message 结构
  - [x] ModelAdapter trait
  - [x] StreamChunk 枚举
  - [x] TokenUsage 结构

- [x] 7.2 测试文档
  - [x] 示例代码在 Rustdoc 中

## 8. 兼容性说明 / Compatibility Notes

### 与 TypeScript 版本的主要差异

| TypeScript | Rust | 状态 | 说明 |
|-----------|------|------|------|
| `Message` 联合类型 (`UserMessage \| AssistantMessage \| ProgressMessage`) | `Message` 结构体 | ⚠️ 部分对等 | Rust 缺少 ProgressMessage 类型 |
| `type: 'user'/'assistant'` | `Role::User/Assistant` | ✅ 兼容 | 序列化格式相同 |
| UUID 追踪 | ⬜ 无 | ❌ 缺失 | TS 版本每个消息都有 `uuid: UUID` |
| `costUSD`, `durationMs` | ⬜ 无 | ❌ 缺失 | 应在服务层填充 |
| `ProgressMessage` | ⬜ 无 | ❌ 缺失 | 工具执行进度消息 |
| `cumulativeUsage` | ⬜ 无 | ❌ 缺失 | 累积 token 使用量 |
| `buildTools()` | ⬜ 无 | ⚠️ 待评估 | 工具构建方法 |
| `isApiErrorMessage` | ⬜ 无 | ❌ 缺失 | API 错误标记 |
| `responseId` | ⬜ 无 | ❌ 缺失 | 响应 ID 追踪 |
| `toolUseResult` | ⬜ 无 | ❌ 缺失 | 工具执行结果 |
| Thinking blocks | ⬜ 无 | ❌ 缺失 | thinking/redacted_thinking 块 |
| 二元反馈 (binary feedback) | ⬜ 无 | ❌ 缺失 | 双模型响应比较机制 |

### 数据来源对比

**TypeScript 版本使用外部 SDK**:
- `MessageParam`, `APIAssistantMessage` 来自 `@anthropic-ai/sdk`
- `ContentBlock`, `ToolUseBlock` 等来自 SDK
- 消息类型与 API 格式高度一致

**Rust 版本使用自定义类型**:
- `Message`, `Role`, `ContentBlock` 完全自定义
- 需要额外转换层与 API 格式兼容
- 设计更简洁但需要更多胶水代码

### 建议的改进优先级

1. **高优先级**: 补充 `id: Uuid` 和 `timestamp` 用于消息追踪
2. **高优先级**: 补充 `cost_usd` 和 `duration_ms` 用于成本追踪
3. **中优先级**: 实现 `ProgressMessage` 类型
4. **中优先级**: 实现 `normalize_messages_for_api()` 转换函数
5. **低优先级**: Thinking blocks 支持（取决于模型支持）

---

## 总结 / Summary

### 已完成（本变更范围）
- ✅ 核心消息类型定义 (Message, Role, ContentBlock)
- ✅ ModelAdapter trait 定义
- ✅ 流式响应类型 (StreamingResponse, StreamChunk)
- ✅ 错误类型扩展
- ✅ 基础单元测试

### 待补充（根据 TS 版本对比发现）
- ⚠️ 消息元数据 (UUID, timestamp, cost_usd, duration_ms)
- ⚠️ ProgressMessage 类型（工具执行进度）
- ⚠️ 消息规范化函数 (normalize_messages_for_api)
- ⚠️ 累积使用量追踪 (cumulativeUsage)
- ⚠️ Thinking blocks 支持
- ⚠️ 对话恢复功能

### 由后续变更完成
- ⬜ 模型管理器（`implement-model-switching`）
- ⬜ 上下文管理（`implement-context-management`）
- ⬜ 具体服务实现（`implement-anthropic-service`, `implement-openai-service`）
