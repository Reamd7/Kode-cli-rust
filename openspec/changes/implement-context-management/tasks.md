# 实现任务 / Implementation Tasks

## 1. 准备工作 / Preparation
- [ ] 1.1 阅读 `openspec/specs/message-model/spec.md` 中 Context Management 相关需求
- [ ] 1.2 阅读 `openspec/changes/implement-context-management/proposal.md`
- [ ] 1.3 阅读 TypeScript 参考：`/Users/gemini/Documents/backup/Kode-cli/src/query.ts` 中的上下文管理部分

## 2. MessageContextManager 实现 / MessageContextManager Implementation

- [ ] 2.1 定义 `MessageContextManager` 结构
  - [ ] 存储消息列表
  - [ ] 管理 token 限制
  - [ ] 提供消息添加/删除接口

- [ ] 2.2 实现 Token 计数器
  - [ ] 字符估算实现（简单版本）
  - [ ] 集成 tokenizer（高级版本，可选）
  - [ ] `count_tokens(message: &Message) -> usize` 方法
  - [ ] 支持 TokenUsage 累计

- [ ] 2.3 实现智能裁剪策略
  - [ ] `trim_to_fit(max_tokens: usize)` 方法
  - [ ] 保留系统消息（高优先级）
  - [ ] 保留最近的用户消息（高优先级）
  - [ ] 移除最早的工具结果（低优先级）
  - [ ] 可配置的裁剪策略

- [ ] 2.4 实现上下文优先级
  - [ ] 定义 `MessagePriority` 枚举
  - [ ] System > User > ToolResult 优先级
  - [ ] 支持自定义优先级

- [ ] 2.5 实现 `add_message()` 方法
  - [ ] 追加消息到列表
  - [ ] 更新 token 计数
  - [ ] 自动触发裁剪（如果超限）

- [ ] 2.6 实现 `get_messages()` 方法
  - [ ] 返回裁剪后的消息列表
  - [ ] 确保不超过 token 限制

## 3. 上下文管理策略 / Context Management Strategies

- [ ] 3.1 实现保留策略
  - [ ] 保留策略：保留最近 N 条消息
  - [ ] 保留策略：保留重要消息
  - [ ] 滑动窗口策略

- [ ] 3.2 实现压缩策略（可选，未来增强）
  - [ ] 摘要压缩
  - [ ] 关键信息提取

## 4. 测试 / Testing

- [ ] 4.1 单元测试
  - [ ] 测试 Token 计数（字符估算）
  - [ ] 测试消息添加
  - [ ] 测试上下文裁剪
  - [ ] 测试优先级排序
  - [ ] 测试边界情况（空上下文、单条消息等）

- [ ] 4.2 集成测试
  - [ ] 测试与 Message 类型的集成
  - [ ] 测试与 ModelAdapter 的配合

## 5. 文档 / Documentation

- [ ] 5.1 Rustdoc 注释
  - [ ] MessageContextManager 结构
  - [ ] Token 计数相关方法
  - [ ] 裁剪策略说明

## 6. 验证 / Validation

- [ ] 6.1 运行测试（cargo test）
- [ ] 6.2 运行 clippy（cargo clippy -- -D warnings）
- [ ] 6.3 运行 fmt（cargo fmt）
- [ ] 6.4 验证 openspec（openspec validate --strict）
