# Delta Spec: Message Model System

## ADDED Requirements

### Requirement: 消息元数据 / Message Metadata
The system SHALL enhance the message structure with metadata for tracking.

系统应增强消息结构以支持元数据追踪。

#### Scenario: 消息唯一标识 / Message Unique Identifier
- **WHEN** 创建消息时
- **THEN** 生成唯一的 UUID 作为消息标识
- **AND** 记录创建时间戳

- **WHEN** creating a message
- **THEN** generate unique UUID as message identifier
- **AND** record creation timestamp

#### Scenario: 成本追踪 / Cost Tracking
- **WHEN** API 响应返回时
- **THEN** 追踪 token 成本（cost_usd）
- **AND** 追踪执行耗时（duration_ms）

- **WHEN** API response returns
- **THEN** track token cost (cost_usd)
- **AND** track execution time (duration_ms)

#### Scenario: 响应追踪 / Response Tracking
- **WHEN** 使用响应 ID 追踪多轮对话时
- **THEN** 记录 response_id 用于关联

- **WHEN** using response ID to track multi-turn conversations
- **THEN** record response_id for correlation

### Requirement: ProgressMessage 类型 / ProgressMessage Type
The system SHALL support a message type for tool execution progress.

系统应支持工具执行进度的消息类型。

#### Scenario: 进度消息结构 / Progress Message Structure
- **WHEN** 工具执行期间显示进度时
- **THEN** 使用 ProgressMessage 表示
- **AND** 包含原始消息内容
- **AND** 包含工具使用 ID 和关联工具使用 ID
- **AND** 包含可用工具列表和规范化消息列表

- **WHEN** showing progress during tool execution
- **THEN** use ProgressMessage to represent
- **AND** includes original message content
- **AND** includes tool_use_id and sibling_tool_use_ids
- **AND** includes available tools list and normalized messages list

### Requirement: 用户消息选项 / User Message Options
The system SHALL support options for user messages.

系统应支持用户消息的附加选项。

#### Scenario: Koding 模式 / Koding Mode
- **WHEN** 在 Koding 模式下发送输入时
- **THEN** 标记消息为 koding 请求
- **AND** 可选提供 koding 上下文信息

- **WHEN** sending input in Koding mode
- **THEN** mark message as koding request
- **AND** optionally provide koding context information

### Requirement: 错误标记 / Error Marking
The system SHALL support error marking for API responses.

系统应支持 API 响应的错误标记。

#### Scenario: API 错误消息 / API Error Message
- **WHEN** API 返回错误响应时
- **THEN** 标记消息为 API 错误（is_api_error_message）

- **WHEN** API returns error response
- **THEN** mark message as API error (is_api_error_message)

## MODIFIED Requirements

### Requirement: 消息数据结构 / Message Data Structure
The system SHALL define a unified message structure representing conversations with AI models.

系统应定义统一的消息结构来表示与 AI 模型的对话。

#### Scenario: 消息结构增强 / Message Structure Enhancement
- **WHEN** 创建消息时
- **THEN** 包含所有必需字段（id, role, content）
- **AND** 包含可选元数据（timestamp, cost_usd, duration_ms, response_id）

- **WHEN** creating a message
- **THEN** includes all required fields (id, role, content)
- **AND** includes optional metadata (timestamp, cost_usd, duration_ms, response_id)

## Non-Goals（本变更不包含）

以下功能由独立的变更提案负责：

- **模型管理器**（`implement-model-switching`）
  - 多模型配置管理
  - 运行时模型切换
  - 模型指针

- **消息上下文管理**（`implement-context-management`）
  - 上下文窗口管理
  - Token 讀数器
  - 智能裁剪策略

- **具体模型服务**（`implement-anthropic-service`, `implement-openai-service`）
  - Anthropic API 客户端
  - OpenAI API 客户端
  - 流式 SSE 解析

- **消息规范化功能**（未来增强）
  - normalize_messages_for_api() 函数
  - API 格式转换

- **对话恢复功能**（未来增强）
  - 对话状态序列化/ deserialization
  - 持久化支持

- **Thinking blocks 支持**（取决于模型支持）
  - ThinkingStart, ThinkingDelta, ThinkingStop 事件
  - Anthropic 的 thinking/redacted_thinking 块支持

- **二元反馈机制**（特殊功能）
  - 双模型响应比较机制
  - BinaryFeedbackResult 类型

- **累积使用量追踪**（由具体服务实现）
  - cumulativeUsage 字段
  - get_cumulative_usage() 方法
