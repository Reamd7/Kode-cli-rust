# 实现任务 - Anthropic API 服务 / Implementation Tasks - Anthropic API Service

## 1. AnthropicService 结构 / AnthropicService Struct
- [x] 1.1 定义 AnthropicService 结构体
- [x] 1.2 实现 new() 构造函数
- [x] 1.3 实现 API key 管理
- [x] 1.4 实现 base URL 配置

## 2. ModelAdapter 实现 / ModelAdapter Implementation
- [x] 2.1 实现 send_message() 方法
- [x] 2.2 实现 stream_message() 方法
- [x] 2.3 实现 API 请求构建
- [x] 2.4 实现响应解析

## 3. 错误处理 / Error Handling
- [x] 3.1 定义 API 错误类型 (AnthropicError)
- [x] 3.2 实现错误重试逻辑 (定义了 should_retry 和 get_retry_delay 辅助函数)
- [x] 3.3 实现超时处理 (ClientBuilder::timeout)

## 4. 成本计算 / Cost Calculation
- [x] 4.1 实现 get_model_input_token_cost_usd 函数
- [x] 4.2 实现 get_model_output_token_cost_usd 函数
- [x] 4.3 在响应处理中计算并返回成本 ✅ 已修复 (2025-12-28)
- [x] 4.4 支持缓存 token 成本计算

## 5. 缓存控制 / Prompt Caching
- [x] 5.1 实现 apply_cache_control_with_limits 函数
- [x] 5.2 实现 cache_breakpoints 添加
- [x] 5.3 在 send_message/stream_message 中应用缓存控制
- [x] 5.4 支持 4 个缓存块限制

## 6. API Key 验证 / API Key Verification
- [x] 6.1 实现 verify_api_key 函数
- [x] 6.2 支持 Anthropic 兼容 API 验证
- [x] 6.3 返回验证结果

## 7. 获取模型列表 / Model List
- [x] 7.1 实现 fetch_anthropic_models 函数
- [x] 7.2 支持自定义 base_url
- [x] 7.3 返回模型列表

## 8. Thinking Token 支持 / Thinking Token Support
- [x] 8.1 实现 anthropic-beta header 添加
- [x] 8.2 实现 thinking 参数配置
- [x] 8.3 在 send_message 中支持 thinking
- [x] 8.4 在 stream_message 中支持 thinking

## 9. 测试 / Testing
- [x] 9.1 单元测试
- [x] 9.2 集成测试（需要 API key）✅ 已完成
- [x] 9.3 成本计算测试
- [x] 9.4 缓存控制测试
- [x] 9.5 API Key 验证测试
- [x] 9.6 Thinking token 测试

## 10. 待完成项 / Pending Items (Fact-Check 2025-12-28)

基于代码审查，以下任务标记为已完成但实际存在实现缺陷：

### 完成度评估: ~90%

### 🚨 P0 - 关键缺失功能

- [x] 10.1 **工具调用未集成到 ModelAdapter trait** ✅ 已完成 (2025-12-25)
  - 已添加 `send_message_with_tools()` 和 `stream_message_with_tools()` 到 trait
  - `AnthropicService` 已实现这两个方法
  - 可以通过 trait 统一调用带工具的请求

- [x] 10.2 **思考模式未实际应用** ✅ 已完成 (2025-12-25)
  - 实际上代码已经实现了思考模式支持
  - 在 `new()` 中根据 `config.thinking` 添加 beta header
  - 在 `build_request_body()` 中添加 thinking 参数
  - 只需要通过 `AnthropicConfig::thinking` 字段配置即可使用

- [x] 10.3 **流式响应中的工具调用处理不完整** ✅ 已完成 (2025-12-28)
  - 已添加 JSON buffer 缓存
  - 已跟踪 block 类型（text/tool_use）
  - 在 `content_block_stop` 时发送 `ToolUseComplete` 事件
  - 添加了 `StreamChunk::ToolUseComplete` 变体
  - ✅ 已修复: 添加 `tool_name` 和 `tool_use_id` 到 `ContentBlockStartEvent`

### ⚠️ P1 - 重要功能

- [x] 10.4 **客户端状态管理缺失** ✅ 已完成 (2025-12-25)
  - 已实现 `AnthropicClientManager`
  - 支持配置哈希缓存和自动重建客户端
  - 支持过期清理（5 分钟）
  - 提供 `clear()` 方法重置

- [x] 10.5 **重试逻辑未实际应用** ✅ 已完成 (2025-12-25)
  - 已将重试逻辑移到 `send_message_with_retry` 内部方法
  - 已将重试逻辑移到 `stream_message_with_retry` 内部方法
  - 重试 429 和 5xx 错误，指数退避

- [x] 10.6 **思考内容块处理** ✅ 已完成 (2025-12-28)
  - ✅ 已修复: 添加 `thinking_tokens` 字段到 `ApiUsage` 和 `TokenUsage`
  - 可以追踪思考 token 使用量
  - 思考内容通过配置（`ThinkingConfig`）启用/禁用
  - 思考 token 使用情况通过 `usage.thinking_tokens` 追踪
  - 思考内容不作为最终响应输出（符合 Anthropic API 设计）
  - 流式响应中的思考 delta 被正确处理（通过 `content_block_delta`）

### 📝 P2 - 扩展功能

- [ ] 10.7 **Bedrock/Vertex 支持**
  - 问题: TypeScript 版本支持 `USE_BEDROCK` 和 `USE_VERTEX` 环境变量
  - Rust 状态: 完全未实现
  - 依赖: 需要额外的 AWS SDK 和 GCP 认证库
  - 备注: 可能需要独立的变更提案

### ✅ 已完成功能 (供参考)

- [x] 基础 API 交互 (send_message, stream_message)
- [x] 请求体构建和响应解析
- [x] 成本计算**函数** (含缓存 token 成本)
- [x] 缓存控制 (4 个缓存块限制)
- [x] API Key 验证
- [x] 模型列表获取
- [x] VCR 录制/回放支持 (`vcr.rs`)
- [x] 基础错误类型定义

### 🔧 需要修复的问题 (Fact-Check 2025-12-28)

- [x] **4.3 响应处理中返回成本** ✅ 已完成 (2025-12-28)
  - 文件: `crates/kode-services/src/anthropic.rs:243-257`
  - 修复: 调用 `get_model_total_cost_usd` 并将成本添加到 ModelResponse

- [x] **10.6 思考 token 追踪** ✅ 已完成 (2025-12-28)
  - 文件: `crates/kode-services/src/anthropic/types.rs:119-126`
  - 修复: 添加 `thinking_tokens: Option<usize>` 字段到 ApiUsage 和 TokenUsage

- [x] **10.3 流式工具调用完整信息** ✅ 已完成 (2025-12-28)
  - 文件: `crates/kode-services/src/anthropic/types.rs:158-166`
  - 修复: 添加 `tool_name: Option<String>` 和 `tool_use_id: Option<String>` 到 ContentBlockStartEvent
  - 更新流式响应处理逻辑

### 建议修复顺序

1. ✅ **修复工具调用集成** (10.1)
2. ✅ **应用思考模式** (10.2)
3. ✅ **完善流式工具调用** (10.3)
4. ✅ **实现客户端管理** (10.4)
5. ✅ **应用重试逻辑** (10.5)
6. ✅ **处理思考内容** (10.6)
7. ✅ **响应处理返回成本** (4.3)
8. ⬜ **Bedrock/Vertex** (10.7) - 可选扩展

### 当前状态总结 (2025-12-28)

**核心功能**: 基本完成 (100%)
- ✅ 基础 API 交互
- ✅ 工具调用 (非流式 + 流式，含完整工具信息)
- ✅ 思考模式配置
- ✅ 成本计算 + 响应中返回成本
- ✅ 缓存控制
- ✅ API Key 验证
- ✅ 模型列表
- ✅ VCR 支持
- ✅ 客户端状态管理
- ✅ 重试逻辑
- ✅ 思考 token 追踪

**待办事项**:
1. P2: Bedrock/Vertex 支持（可选扩展）

**已完成修复 (2025-12-28)**:
- ✅ 4.3: 在 parse_response 中计算并返回成本
- ✅ 10.3: 添加 tool_name 和 tool_use_id 到 ContentBlockStartEvent
- ✅ 10.6: 添加 thinking_tokens 字段到 ApiUsage 和 TokenUsage

**结论**: 所有 P0 和 P1 功能已完成，Anthropic API 服务实现完整。
