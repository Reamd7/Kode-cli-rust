# Delta Spec: Context Management

## ADDED Requirements

### Requirement: MessageContextManager / MessageContextManager
The system SHALL implement intelligent context window management.

系统应实现智能上下文窗口管理。

#### Scenario: Token 计数 / Token Counting
- **WHEN** 计算消息 token 时
- **THEN** 使用字符估算（简单实现）
- **AND** 或使用 tokenizer（高级实现）
- **AND** 返回准确 token 数量

- **WHEN** counting message tokens
- **THEN** uses character estimation (simple implementation)
- **AND** or uses tokenizer (advanced implementation)
- **AND** returns accurate token count

#### Scenario: 智能裁剪 / Smart Trimming
- **WHEN** 上下文超出限制时
- **THEN** 按优先级裁剪消息
- **AND** 保留系统消息
- **AND** 保留最近的消息
- **AND** 移除最早的工具结果

- **WHEN** context exceeds limit
- **THEN** trims messages by priority
- **AND** keeps system messages
- **AND** keeps recent messages
- **AND** removes earliest tool results

#### Scenario: 上下文优先级 / Context Priority
- **WHEN** 定义消息优先级时
- **THEN** 系统消息优先级最高
- **AND** 用户消息优先级较高
- **AND** 工具结果优先级较低

- **WHEN** defining message priority
- **THEN** system messages have highest priority
- **AND** user messages have higher priority
- **AND** tool results have lower priority
