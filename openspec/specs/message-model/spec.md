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

#### Scenario: 工具结果消息 / Tool Result Message
- **WHEN** 工具执行完成时
- **THEN** 创建包含 role="tool" 的消息
- **AND** 消息包含工具执行结果

- **WHEN** a tool execution completes
- **THEN** create a message with role="tool"
- **AND** the message contains the tool execution result

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

### Requirement: 模型管理器 / Model Manager
The system SHALL manage multiple model profiles and support runtime model switching.

系统应管理多个模型配置并支持运行时模型切换。

#### Scenario: 模型切换 / Model Switching
- **WHEN** 用户请求切换模型时
- **THEN** 更新当前模型配置
- **AND** 验证新模型配置有效

- **WHEN** the user requests to switch models
- **THEN** update the current model configuration
- **AND** validate the new model configuration

#### Scenario: 获取模型 / Get Model
- **WHEN** 请求特定模型时
- **THEN** 返回对应的模型适配器
- **AND** 如果模型不存在则返回错误

- **WHEN** a specific model is requested
- **THEN** return the corresponding model adapter
- **AND** return an error if the model does not exist

### Requirement: 消息上下文管理 / Message Context Management
The system SHALL manage conversation context with token limit awareness.

系统应管理对话上下文并考虑 token 限制。

#### Scenario: 添加消息 / Add Message
- **WHEN** 添加新消息到上下文时
- **THEN** 消息被追加到消息列表
- **AND** 更新 token 计数

- **WHEN** adding a new message to context
- **THEN** the message is appended to the message list
- **AND** update the token count

#### Scenario: 上下文裁剪 / Context Trimming
- **WHEN** 上下文超过 token 限制时
- **THEN** 按策略裁剪旧消息
- **AND** 保留系统消息

- **WHEN** the context exceeds token limit
- **THEN** trim old messages according to strategy
- **AND** preserve system messages

## Reference / 参考资料

### TypeScript 版本实现参考 / TypeScript Implementation Reference

在实现本规范时，请参考原版 TypeScript 项目中的以下文件：

When implementing this specification, refer to the following files in the original TypeScript project:

#### 消息类型定义 / Message Type Definitions
- **消息类型**: `/Users/gemini/Documents/backup/Kode-cli/src/types/conversation.ts`
  - `Message` - 消息联合类型
  - `UserMessage` - 用户消息结构
  - `AssistantMessage` - 助手消息结构
  - `ProgressMessage` - 进度消息结构

#### 查询处理 / Query Processing
- **查询模块**: `/Users/gemini/Documents/backup/Kode-cli/src/query.ts`
  - 消息构建和发送
  - 工具调用处理
  - 响应流处理
  - 上下文管理

#### 模型适配器工厂 / Model Adapter Factory
- **适配器工厂**: `/Users/gemini/Documents/backup/Kode-cli/src/services/modelAdapterFactory.ts`
  - 模型提供商选择
  - 适配器实例化
  - 支持的提供商列表

#### 多提供商支持 / Multi-Provider Support
- **OpenAI 服务**: `/Users/gemini/Documents/backup/Kode-cli/src/services/openai.ts`
- **其他提供商**: 支持 Mistral、DeepSeek、Kimi、Qwen 等

### 实现要点 / Implementation Notes

1. **消息格式**: 遵循 Anthropic Messages API 的消息格式
2. **工具调用**: 使用 `tool_use` 和 `tool_result` 内容块
3. **流式处理**: 支持流式和非流式两种响应模式
4. **Token 计算**: 使用 @anthropic-ai/sdk/tokenizer 进行 token 计算
5. **上下文管理**: 实现智能的上下文窗口管理

## Non-Goals

- 本规范不包含具体的模型 API 实现
- 不包含 token 计数器的具体实现
- 不包含消息持久化

- This specification does not include specific model API implementations
- Does not include specific token counter implementation
- Does not include message persistence
