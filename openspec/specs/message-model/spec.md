# 消息与模型抽象规范 / Message and Model Abstraction Specification

## Purpose

消息与模型抽象是 Kode 与 AI 模型交互的核心接口，定义了统一的消息格式和模型适配器接口。

The message and model abstraction is the core interface for Kode to interact with AI models, defining unified message formats and model adapter interfaces.

## Requirements

### Requirement: 消息数据结构 / Message Data Structure
The system SHALL define a unified message structure representing conversations with AI models.

系统应定义统一的消息结构来表示与 AI 模型的对话。

#### Scenario: 用户消息 / User Message
- **WHEN** 用户发送输入时
- **THEN** 创建包含 role="user" 的消息
- **AND** 消息包含文本内容和可选的图片

- **WHEN** the user sends input
- **THEN** create a message with role="user"
- **AND** the message contains text content and optional images

#### Scenario: 助手消息 / Assistant Message
- **WHEN** AI 模型返回响应时
- **THEN** 创建包含 role="assistant" 的消息
- **AND** 消息包含文本内容或工具调用

- **WHEN** the AI model returns a response
- **THEN** create a message with role="assistant"
- **AND** the message contains text content or tool calls

#### Scenario: 系统消息 / System Message
- **WHEN** 设置系统提示词时
- **THEN** 创建包含 role="system" 的消息
- **AND** 消息包含系统提示内容

- **WHEN** setting the system prompt
- **THEN** create a message with role="system"
- **AND** the message contains the system prompt content

### Requirement: 内容块类型 / Content Block Types
The system SHALL support multiple content block types for rich message content.

系统应支持多种内容块类型以支持丰富的消息内容。

#### Scenario: 文本内容 / Text Content
- **WHEN** 消息包含纯文本时
- **THEN** 使用 TextBlock 存储内容
- **AND** 支持多语言文本

- **WHEN** a message contains plain text
- **THEN** use TextBlock to store the content
- **AND** support multi-language text

#### Scenario: 工具使用 / Tool Use
- **WHEN** 助手请求调用工具时
- **THEN** 使用 ToolUseBlock 表示
- **AND** 包含工具名称和参数

- **WHEN** the assistant requests to call a tool
- **THEN** use ToolUseBlock to represent it
- **AND** include tool name and parameters

#### Scenario: 图片内容 / Image Content
- **WHEN** 消息包含图片时
- **THEN** 使用 ImageBlock 存储
- **AND** 支持多种图片格式

- **WHEN** a message contains an image
- **THEN** use ImageBlock to store it
- **AND** support multiple image formats

#### Scenario: 工具结果 / Tool Result
- **WHEN** 工具执行完成时
- **THEN** 使用 ToolResultBlock 表示结果
- **AND** 包含工具执行结果和状态

- **WHEN** a tool execution completes
- **THEN** use ToolResultBlock to represent the result
- **AND** include the tool execution result and status

### Requirement: 模型适配器接口 / Model Adapter Interface
The system SHALL provide a unified interface for interacting with different AI model providers.

系统应提供统一的接口来与不同的 AI 模型提供商交互。

#### Scenario: 发送消息 / Send Message
- **WHEN** 调用模型的 send_message 方法时
- **THEN** 发送消息到 AI 模型 API
- **AND** 返回模型响应
- **AND** 支持同步和异步调用

- **WHEN** the model's send_message method is called
- **THEN** send messages to the AI model API
- **AND** return the model response
- **AND** support both synchronous and asynchronous calls

#### Scenario: 流式响应 / Stream Response
- **WHEN** 调用模型的 stream_message 方法时
- **THEN** 建立流式连接
- **AND** 逐步返回内容块
- **AND** 支持实时显示

- **WHEN** the model's stream_message method is called
- **THEN** establish a streaming connection
- **AND** return content chunks progressively
- **AND** support real-time display

#### Scenario: 流块类型 / Stream Chunk Types
- **WHEN** 接收流式响应时
- **THEN** 支持多种流块事件类型
- **AND** 包括内容块开始、增量、结束
- **AND** 包括工具使用请求
- **AND** 包括消息结束和使用统计

- **WHEN** receiving streaming response
- **THEN** support multiple stream chunk event types
- **AND** include content block start, delta, stop
- **AND** include tool use requests
- **AND** include message stop and usage statistics

### Requirement: Token 使用统计 / Token Usage
The system SHALL track token usage for API calls.

系统应跟踪 API 调用的 token 使用情况。

#### Scenario: Token 统计
- **WHEN** API 调用完成时
- **THEN** 返回输入 token 数量
- **AND** 返回输出 token 数量
- **AND** 可选返回总 token 数量

- **WHEN** API call completes
- **THEN** return the number of input tokens
- **AND** return the number of output tokens
- **AND** optionally return total token count

## Reference / 参考资料

### TypeScript 版本实现参考 / TypeScript Implementation Reference

在实现本规范时，请参考原版 TypeScript 项目中的以下文件：

When implementing this specification, refer to the following files in the original TypeScript project:

#### 消息类型定义 / Message Type Definitions
- **消息类型**: `/Users/gemini/Documents/backup/Kode-cli/src/types/conversation.ts`
  - `Message` - 消息联合类型
  - `UserMessage` - 用户消息结构
  - `AssistantMessage` - 助手消息结构

#### 查询处理 / Query Processing
- **查询模块**: `/Users/gemini/Documents/backup/Kode-cli/src/query.ts`
  - 消息构建和发送
  - 工具调用处理
  - 响应流处理

#### 模型适配器工厂 / Model Adapter Factory
- **适配器工厂**: `/Users/gemini/Documents/backup/Kode-cli/src/services/modelAdapterFactory.ts`
  - 模型提供商选择
  - 适配器实例化

#### 流式响应 / Streaming Response
- **适配器基类**: `/Users/gemini/Documents/backup/Kode-cli/src/services/adapters/base.ts`
  - `StreamingEvent` 类型
  - `TokenUsage` 接口
  - 累积使用量追踪

### 实现要点 / Implementation Notes

1. **消息格式**: 遵循 Anthropic Messages API 的消息格式
2. **工具调用**: 使用 `tool_use` 和 `tool_result` 内容块
3. **流式处理**: 支持流式和非流式两种响应模式
4. **序列化**: 所有类型必须支持 serde 序列化/反序列化

## Non-Goals

- **模型管理器**（由 `implement-model-switching` 变更负责）
  - 多模型配置管理
  - 运行时模型切换
  - 模型指针（default, task, architect 等）

- **消息上下文管理**（由 `implement-context-management` 变更负责）
  - 消息上下文窗口管理
  - Token 计数器
  - 智能裁剪策略
  - 上下文优先级

- **具体模型 API 实现**（由 `implement-anthropic-service`、`implement-openai-service` 等变更负责）

- **消息持久化**（未来可能添加）

- **Model manager** (handled by `implement-model-switching` change)
  - Multiple model profile management
  - Runtime model switching
  - Model pointers (default, task, architect, etc.)

- **Message context management** (handled by `implement-context-management` change)
  - Message context window management
  - Token counter
  - Smart trimming strategy
  - Context priority

- **Specific model API implementations** (handled by `implement-anthropic-service`, `implement-openai-service` changes)

- **Message persistence** (may be added in the future)
