# Anthropic Service Specification

Anthropic API 服务规范 - 定义 Claude API 客户端实现。

## Purpose

Anthropic 服务提供了与 Anthropic Claude API 的集成。实现了 ModelAdapter trait，支持同步和流式 API 调用，处理认证、请求构建、响应解析和错误处理。

## ADDED Requirements

### Requirement: API 客户端初始化
The system SHALL initialize Anthropic API client with configuration.

系统必须使用配置初始化 Anthropic API 客户端。

#### Scenario: 使用 API 密钥初始化
- **WHEN** 创建 AnthropicService 实例
- **THEN** 必须提供有效的 API 密钥
- **AND** 默认使用官方 Anthropic API 端点

#### Scenario: 使用自定义端点
- **WHEN** 配置指定了 base_url
- **THEN** 客户端使用自定义端点
- **AND** 支持兼容 Anthropic API 的第三方服务

#### Scenario: 配置请求超时
- **WHEN** 初始化客户端
- **THEN** 设置合理的请求超时时间
- **AND** 支持流式请求的长连接

### Requirement: 同步消息发送
The system SHALL send non-streaming requests to Anthropic Messages API.

系统必须发送非流式请求到 Anthropic Messages API。

#### Scenario: 发送基本消息
- **WHEN** 调用 send_message
- **THEN** 构建符合 Anthropic API 格式的请求
- **AND** 发送 POST 请求到 /v1/messages 端点
- **AND** 解析响应返回 Message 对象

#### Scenario: 处理工具调用
- **WHEN** 模型响应包含工具使用
- **THEN** 正确解析 tool_use 内容块
- **AND** 返回包含工具调用的 Message

#### Scenario: 设置模型参数
- **WHEN** 发送请求
- **THEN** 支持设置 max_tokens、temperature 等参数
- **AND** 参数值来自 ModelProfile 配置

### Requirement: 流式消息发送
The system SHALL send streaming requests to Anthropic Messages API.

系统必须发送流式请求到 Anthropic Messages API。

#### Scenario: SSE 流式响应
- **WHEN** 调用 stream_message
- **THEN** 请求设置 stream: true
- **AND** 使用 Server-Sent Events 格式接收响应
- **AND** 逐事件返回 StreamChunk

#### Scenario: 处理流式事件
- **WHEN** 接收 SSE 事件
- **THEN** 正确解析 message_start, content_block_delta, message_delta 事件
- **AND** 累积内容块形成完整响应

#### Scenario: 处理流式错误
- **WHEN** 流式连接中断
- **THEN** 返回流式错误
- **AND** 支持重试逻辑

### Requirement: 错误处理
The system SHALL handle Anthropic API errors appropriately.

系统必须正确处理 Anthropic API 错误。

#### Scenario: 处理认证错误
- **WHEN** API 返回 401 状态码
- **THEN** 返回认证错误
- **AND** 错误消息提示检查 API 密钥

#### Scenario: 处理速率限制
- **WHEN** API 返回 429 状态码
- **THEN** 返回速率限制错误
- **AND** 错误包含重试时间

#### Scenario: 处理无效请求
- **WHEN** API 返回 400 状态码
- **THEN** 返回验证错误
- **AND** 错误包含 API 返回的详细信息

#### Scenario: 处理网络错误
- **WHEN** 请求无法连接到 API
- **THEN** 返回网络错误
- **AND** 错误包含连接失败原因

### Requirement: 请求构建
The system SHALL build requests compliant with Anthropic API format.

系统必须构建符合 Anthropic API 格式的请求。

#### Scenario: 消息格式转换
- **WHEN** 转换内部 Message 到 API 格式
- **THEN** role 映射为 "user", "assistant", "system"
- **AND** content 映射为正确的 content_block 结构

#### Scenario: 工具定义格式
- **WHEN** 请求包含工具定义
- **THEN** 工具定义符合 Anthropic 工具格式
- **AND** 包含 name, description, input_schema

#### Scenario: 系统提示词处理
- **WHEN** 消息包含系统提示
- **THEN** 系统提示放在单独的 system 字段
- **AND** 不在 messages 数组中

### Requirement: 响应解析
The system SHALL parse Anthropic API responses into internal Message format.

系统必须解析 Anthropic API 响应为内部 Message 格式。

#### Scenario: 解析文本响应
- **WHEN** 响应只包含文本内容
- **THEN** 创建包含文本的助手消息
- **AND** 使用响应的 id 作为消息 ID

#### Scenario: 解析工具调用
- **WHEN** 响应包含 tool_use 块
- **THEN** 每个工具调用转换为独立内容块
- **AND** 保留工具 ID、名称和参数

#### Scenario: 解析流式响应
- **WHEN** 解析流式事件
- **THEN** 累积 delta 内容块形成完整消息
- **AND** 正确处理 partial 和 final 消息

## Non-Goals

本规范不包含：
- Token 使用统计和计费（未来扩展）
- Prompt 缓存管理
- 批量请求和批处理
- 其他 AI 提供商的实现（见 message-model）
