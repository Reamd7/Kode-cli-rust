# Message and Model Specification

消息与模型抽象规范 - 定义 AI 消息格式和模型适配器接口。

## Purpose

消息与模型抽象层定义了 AI 交互的核心数据结构和接口。Message 结构表示对话中的消息，ModelAdapter trait 定义了与不同 AI 提供商交互的统一接口，ModelProfile 管理模型配置。

## ADDED Requirements

### Requirement: 消息结构
The system SHALL define a Message structure representing conversational messages with roles and content.

系统必须定义 Message 结构来表示具有角色和内容的对话消息。

#### Scenario: 用户消息
- **WHEN** 创建用户消息
- **THEN** 消息角色为 "user"
- **AND** 内容为文本或工具使用结果

#### Scenario: 助手消息
- **WHEN** 创建助手消息
- **THEN** 消息角色为 "assistant"
- **AND** 内容为文本或工具调用

#### Scenario: 系统消息
- **WHEN** 创建系统消息
- **THEN** 消息角色为 "system"
- **AND** 内容为系统提示词

#### Scenario: 工具使用消息
- **WHEN** 消息包含工具调用
- **THEN** 内容包含工具名称和参数
- **AND** 支持序列化和反序列化

#### Scenario: 工具结果消息
- **WHEN** 消息包含工具执行结果
- **THEN** 内容包含工具调用 ID 和结果
- **AND** 支持成功和失败状态

### Requirement: 模型适配器接口
The system SHALL define a ModelAdapter trait for interacting with AI providers.

系统必须定义 ModelAdapter trait 用于与 AI 提供商交互。

#### Scenario: 发送消息
- **WHEN** 调用 send_message 方法
- **THEN** 发送消息列表到 AI 模型
- **AND** 返回助手响应消息
- **AND** 支持同步 API 调用

#### Scenario: 流式发送消息
- **WHEN** 调用 stream_message 方法
- **THEN** 返回流式响应
- **AND** 支持 SSE (Server-Sent Events) 格式
- **AND** 逐块返回响应内容

#### Scenario: 错误处理
- **WHEN** API 调用失败
- **THEN** 返回错误信息
- **AND** 错误包含网络错误和 API 错误

### Requirement: 模型配置管理
The system SHALL support ModelProfile for configuring AI models and ModelManager for managing multiple adapters.

系统必须支持 ModelProfile 配置 AI 模型和 ModelManager 管理多个适配器。

#### Scenario: 模型配置定义
- **WHEN** 定义模型配置
- **THEN** 必须包含模型名称和 API 密钥
- **AND** 必须指定提供商类型 (Anthropic/OpenAI/Custom)
- **AND** 可选包含 base URL、max_tokens、temperature 等参数

#### Scenario: 模型注册
- **WHEN** 向 ModelManager 注册模型
- **THEN** 使用唯一名称作为模型标识
- **AND** 存储模型配置和适配器实例

#### Scenario: 模型切换
- **WHEN** 请求使用指定模型
- **THEN** ModelManager 返回对应的适配器
- **AND** 支持运行时动态切换模型

#### Scenario: 模型指针
- **WHEN** 使用模型指针 (如 "main", "task")
- **THEN** 系统解析为实际的模型名称
- **AND** 支持配置文件中定义的指针映射

### Requirement: 错误类型
The system SHALL define typed errors for model operations.

系统必须为模型操作定义类型化错误。

#### Scenario: API 错误
- **WHEN** AI API 返回错误
- **THEN** 错误类型包含 HTTP 状态码
- **AND** 包含 API 返回的错误消息

#### Scenario: 网络错误
- **WHEN** 网络请求失败
- **THEN** 错误类型包含请求详情
- **AND** 支持重试逻辑判断

#### Scenario: 认证错误
- **WHEN** API 密钥无效
- **THEN** 返回明确的认证错误
- **AND** 不暴露敏感信息

## Non-Goals

本规范不包含：
- Token 计数和限制管理（见 context-management）
- 消息历史压缩和裁剪
- 多轮对话上下文管理
- 流式响应的 UI 渲染（见 tui-interface）
