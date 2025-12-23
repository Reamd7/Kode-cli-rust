# Context Management Specification

上下文管理规范 - 定义消息上下文管理和 token 计数机制。

## Purpose

上下文管理系统负责管理对话历史消息，智能裁剪以适应模型的 token 限制。提供准确的 token 计数、智能裁剪策略，确保在有限的上下文窗口内保留最重要的信息。

## ADDED Requirements

### Requirement: 消息上下文管理器
The system SHALL provide MessageContextManager for managing conversation history.

系统必须提供 MessageContextManager 管理对话历史。

#### Scenario: 添加消息
- **WHEN** 添加新消息到上下文
- **THEN** 消息被添加到历史列表末尾
- **AND** 计算当前总 token 数
- **AND** 检查是否超过限制

#### Scenario: 获取上下文
- **WHEN** 请求当前上下文
- **THEN** 返回所有在 token 限制内的消息
- **AND** 保持消息顺序

#### Scenario: 清空上下文
- **WHEN** 清空对话上下文
- **THEN** 移除所有历史消息
- **AND** 重置 token 计数

#### Scenario: 保留系统提示词
- **WHEN** 裁剪上下文时
- **THEN** 始终保留系统提示词
- **AND** 系统提示词不计入可裁剪部分

### Requirement: Token 计数
The system SHALL count tokens accurately for messages.

系统必须准确计算消息的 token 数。

#### Scenario: 基础字符计数
- **WHEN** 使用简单计数器
- **THEN** 按字符数估算 token（约 4 字符 = 1 token）
- **AND** 适用于快速近似计算

#### Scenario: 精确 Token 计数
- **WHEN** 使用 tokenizer
- **THEN** 使用与模型对应的 tokenizer
- **AND** 准确计算实际 token 数
- **AND** 支持不同模型的 tokenizer

#### Scenario: 工具调用计数
- **WHEN** 消息包含工具调用
- **THEN** 计算工具名称和参数的 token
- **AND** 计算工具结果的 token

#### Scenario: 增量更新
- **WHEN** 添加消息到现有上下文
- **THEN** 只计算新消息的 token
- **AND** 累加到总数

### Requirement: 智能裁剪策略
The system SHALL implement intelligent trimming strategies.

系统必须实现智能裁剪策略。

#### Scenario: FIFO 裁剪
- **WHEN** 上下文超过限制
- **THEN** 移除最旧的非系统消息
- **AND** 直到 token 数在限制内

#### Scenario: 保留重要消息
- **WHEN** 裁剪上下文
- **THEN** 保留标记为重要的消息
- **AND** 重要消息可以是用户标记或系统标记

#### Scenario: 滑动窗口
- **WHEN** 使用滑动窗口策略
- **THEN** 保留最近 N 条消息
- **AND** N 由 token 限制决定

#### Scenario: 摘要压缩
- **WHEN** 启用消息摘要
- **THEN** 将旧消息压缩为摘要
- **AND** 摘要由 AI 生成
- **AND** 保留原始消息的关键信息

### Requirement: 上下文窗口配置
The system SHALL support configurable context window sizes.

系统必须支持可配置的上下文窗口大小。

#### Scenario: 模型默认限制
- **WHEN** 模型有默认 token 限制
- **THEN** 使用模型的 context_length 配置
- **AND** 未配置时使用合理默认值

#### Scenario: 自定义限制
- **WHEN** 配置指定了 max_tokens
- **THEN** 使用配置的限制值
- **AND** 确保不超过模型硬限制

#### Scenario: 保留余量
- **WHEN** 计算可用 token
- **THEN** 预留空间给响应
- **AND** 预留约 20% 给输出

### Requirement: 上下文优先级
The system SHALL support prioritizing important messages.

系统必须支持优先级化重要消息。

#### Scenario: 用户消息优先
- **WHEN** 裁剪上下文
- **THEN** 优先保留最近的用户消息
- **AND** 确保用户意图不被丢失

#### Scenario: 工具结果优先
- **WHEN** 裁剪上下文
- **THEN** 保留相关的工具调用和结果
- **AND** 作为一对保留或移除

#### Scenario: 系统提示锁定
- **WHEN** 任何裁剪操作
- **THEN** 系统提示词始终保留
- **AND** 不参与裁剪

### Requirement: 上下文状态查询
The system SHALL provide context state information.

系统必须提供上下文状态信息。

#### Scenario: Token 使用统计
- **WHEN** 查询上下文状态
- **THEN** 返回当前 token 使用量
- **AND** 返回剩余可用 token

#### Scenario: 消息计数
- **WHEN** 查询上下文状态
- **THEN** 返回当前消息数量
- **AND** 区分系统、用户、助手消息

#### Scenario: 预估容量
- **WHEN** 添加消息前预估
- **THEN** 返回是否能添加新消息
- **AND** 预警即将超限

## Non-Goals

本规范不包含：
- 跨会话的持久化历史（未来扩展）
- 向量检索相似消息（RAG）
- 多轮对话的长期记忆
- 分布式上下文共享

## Performance Requirements

### Requirement: 性能目标
The system SHALL meet performance targets for context operations.

系统必须满足上下文操作的性能目标。

#### Scenario: 添加消息性能
- **WHEN** 添加单条消息
- **THEN** 操作完成时间 < 1ms
- **AND** 不阻塞主流程

#### Scenario: Token 计数性能
- **WHEN** 计算消息 token
- **THEN** 简单计数 < 0.1ms per 1K chars
- **AND** 精确计数 < 1ms per 1K tokens

#### Scenario: 裁剪性能
- **WHEN** 裁剪超限上下文
- **THEN** 操作完成时间 < 10ms
- **AND** 即使有大量消息

#### Scenario: 内存占用
- **WHEN** 上下文管理器运行
- **THEN** 内存占用与消息数成线性
- **AND** 不超过消息原始大小的 2x

## Compatibility Requirements

### Requirement: 配置兼容性
The system SHALL maintain compatibility with TS version context management.

系统必须与 TS 版本的上下文管理兼容。

#### Scenario: 相同的裁剪行为
- **WHEN** 相同的对话历史
- **THEN** 裁剪结果与 TS 版本一致
- **AND** 保留相同的消息

#### Scenario: Token 计数一致性
- **WHEN** 使用相同的 tokenizer
- **THEN** Token 计数结果一致
- **AND** 误差 < 1%

#### Scenario: 配置格式兼容
- **WHEN** 从 TS 版本迁移配置
- **THEN** contextLength 配置兼容
- **AND** 无需修改即可使用
