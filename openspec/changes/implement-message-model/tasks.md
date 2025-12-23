# 实现任务 - 消息模型系统 / Implementation Tasks - Message Model System

## 1. 数据结构定义 / Data Structure Definition

- [ ] 1.1 定义 `Message` 结构体
  - [ ] 定义 `Role` 枚举 (User, Assistant, System)
  - [ ] 定义 `Content` 枚举 (Text, ToolUse, ToolResult)
  - [ ] 添加 serde 序列化支持

- [ ] 1.2 定义 `ToolUse` 结构
  - [ ] tool_name 字段
  - [ ] parameters 字段
  - [ ] tool_use_id 字段

- [ ] 1.3 定义 `ToolResult` 结构
  - [ ] tool_use_id 字段
  - [ ] content 字段
  - [ ] is_error 字段

## 2. ModelAdapter trait / ModelAdapter Trait

- [ ] 2.1 定义 `ModelAdapter` trait
  - [ ] `send_message()` 方法（非流式）
  - [ ] `stream_message()` 方法（流式）
  - [ ] 添加 `#[async_trait]`

- [ ] 2.2 定义 `StreamChunk` 枚举
  - [ ] ContentBlockStart 变体
  - [ ] ContentBlockDelta 变体
  - [ ] ContentBlockStop 变体
  - [ ] ToolUse 变体
  - [ ] MessageStop 变体

- [ ] 2.3 定义 `StreamingResponse` 包装器
  - [ ] 实现 Stream trait
  - [ ] 实现 TryStream trait

## 3. 错误类型 / Error Types

- [ ] 3.1 定义模型相关错误
  - [ ] ModelRequestError - API 请求失败
  - [ ] ModelResponseError - 响应解析失败
  - [ ] ModelStreamError - 流式响应错误

## 4. 测试 / Testing

- [ ] 4.1 单元测试
  - [ ] 测试 Message 序列化/反序列化
  - [ ] 测试 Role 枚举
  - [ ] 测试 Content 枚举
  - [ ] 测试 ToolUse/ToolResult

- [ ] 4.2 集成测试
  - [ ] 测试 ModelAdapter trait 约束

## 5. 文档 / Documentation

- [ ] 5.1 Rustdoc 注释
  - [ ] Message 结构
  - [ ] ModelAdapter trait
  - [ ] StreamChunk 枚举
