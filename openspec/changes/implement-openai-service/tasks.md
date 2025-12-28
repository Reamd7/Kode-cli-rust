# 实现任务 / Implementation Tasks

## 阶段 1：准备工作 / Phase 1: Preparation

### 1.1 理解需求和设计 / Understand Requirements and Design
- [ ] 1.1.1 阅读 `proposal.md` 理解变更目标
- [ ] 1.1.2 阅读 `design.md` 理解技术设计
- [ ] 1.1.3 阅读 `specs/anthropic-service/spec.md` 理解需求
- [ ] 1.1.4 阅读原版 TypeScript 实现（`/Users/gemini/Documents/backup/Kode-cli/src/services/openai.ts`）
- [ ] 1.1.5 阅读当前 Rust AnthropicService 实现（`crates/kode-services/src/anthropic.rs`）作为参考

### 1.2 环境设置 / Environment Setup
- [ ] 1.2.1 确认 Rust 版本 ≥ 1.75
- [ ] 1.2.2 确认依赖项已正确配置（`crates/kode-services/Cargo.toml`）
- [ ] 1.2.3 准备测试环境变量（`OPENAI_API_KEY`, `DEEPSEEK_API_KEY` 等）

## 阶段 2：基础结构实现 / Phase 2: Basic Structure Implementation

### 2.1 模块结构创建 / Module Structure Creation
- [ ] 2.1.1 创建 `crates/kode-services/src/openai/` 目录
- [ ] 2.1.2 创建 `crates/kode-services/src/openai/mod.rs`（模块导出）
- [ ] 2.1.3 在 `crates/kode-services/src/lib.rs` 中添加 `pub mod openai;`
- [ ] 2.1.4 验证编译通过（`cargo build --package kode-services`）

### 2.2 类型定义 / Type Definitions
- [ ] 2.2.1 创建 `crates/kode-services/src/openai/types.rs`
- [ ] 2.2.2 定义 `OpenAIConfig` 结构体（配置）
- [ ] 2.2.3 定义 `ChatCompletionRequest` 结构体（请求）
- [ ] 2.2.4 定义 `ChatMessage` 枚举（消息）
- [ ] 2.2.5 定义 `ChatCompletionResponse` 结构体（非流式响应）
- [ ] 2.2.6 定义 `ChatCompletionChunk` 结构体（流式响应块）
- [ ] 2.2.7 定义 `ToolDefinition`、`ToolCall` 等工具相关类型
- [ ] 2.2.8 定义 `Usage` 结构体（token 使用统计）
- [ ] 2.2.9 添加 serde 序列化/反序列化支持（camelCase 字段名）
- [ ] 2.2.10 编写单元测试验证序列化正确性

### 2.3 错误类型定义 / Error Type Definitions
- [ ] 2.3.1 创建 `crates/kode-services/src/openai/error.rs`
- [ ] 2.3.2 定义 `OpenAIError` 枚举（继承 thiserror::Error）
- [ ] 2.3.3 定义 `OpenAIErrorKind` 枚举（可修复错误类型）
- [ ] 2.3.4 实现错误转换（From<reqwest::Error>, From<serde_json::Error>）
- [ ] 2.3.5 添加错误上下文信息
- [ ] 2.3.6 编写单元测试验证错误处理

## 阶段 3：核心功能实现 / Phase 3: Core Functionality Implementation

### 3.1 OpenAIService 结构体 / OpenAIService Struct
- [ ] 3.1.1 创建 `crates/kode-services/src/openai/service.rs`
- [ ] 3.1.2 定义 `OpenAIService` 结构体（client + config）
- [ ] 3.1.3 实现 `OpenAIService::new()` 方法
- [ ] 3.1.4 实现 `OpenAIService::from_model_config()` 方法
- [ ] 3.1.5 配置 HTTP 客户端（headers、timeout、proxy 支持）
- [ ] 3.1.6 添加辅助方法（`base_url()`, `model_name()`）
- [ ] 3.1.7 编写单元测试验证初始化逻辑

### 3.2 消息转换 / Message Conversion
- [ ] 3.2.1 实现 `Message` → `ChatMessage` 转换逻辑
- [ ] 3.2.2 处理 system_prompt（作为第一条消息）
- [ ] 3.2.3 处理 user、assistant、tool 消息
- [ ] 3.2.4 处理 tool 消息的特殊格式（content 必须为字符串）
- [ ] 3.2.5 编写单元测试验证消息转换

### 3.3 非流式消息发送 / Non-streaming Message Sending
- [ ] 3.3.1 实现 `send_message()` 方法
- [ ] 3.3.2 转换消息为 OpenAI 格式
- [ ] 3.3.3 发送 POST 请求到 /chat/completions
- [ ] 3.3.4 解析响应并提取 content、usage
- [ ] 3.3.5 转换为 `ModelResponse` 格式
- [ ] 3.3.6 实现基础错误处理
- [ ] 3.3.7 编写单元测试（mock HTTP 响应）

### 3.4 工具调用支持 / Tool Call Support
- [ ] 3.4.1 实现 `send_message_with_tools()` 方法
- [ ] 3.4.2 转换工具定义为 OpenAI Function Calling 格式
- [ ] 3.4.3 在请求中包含 tools 和 tool_choice 参数
- [ ] 3.4.4 解析响应中的 tool_calls
- [ ] 3.4.5 转换 tool_calls 为统一格式
- [ ] 3.4.6 编写单元测试验证工具调用

## 阶段 4：流式响应实现 / Phase 4: Streaming Response Implementation

### 4.1 SSE 流处理 / SSE Stream Processing
- [ ] 4.1.1 创建 `crates/kode-services/src/openai/streaming.rs`
- [ ] 4.1.2 定义 `SseStreamProcessor` 结构体
- [ ] 4.1.3 实现 `process_line()` 方法（解析 SSE 格式）
- [ ] 4.1.4 实现 `process_chunk()` 方法（处理数据块）
- [ ] 4.1.5 处理 `data: [DONE]` 标记
- [ ] 4.1.6 累积 content delta
- [ ] 4.1.7 累积 tool_calls（arguments 是分片的）
- [ ] 4.1.8 编写单元测试验证 SSE 解析

### 4.2 流式消息发送 / Streaming Message Sending
- [ ] 4.2.1 实现 `stream_message()` 方法
- [ ] 4.2.2 在请求中设置 `stream: true`
- [ ] 4.2.3 使用 `SseStreamProcessor` 处理响应流
- [ ] 4.2.4 生成 `StreamChunk::ContentBlockDelta` 事件
- [ ] 4.2.5 生成 `StreamChunk::MessageStop` 事件
- [ ] 4.2.6 实现 `response_to_streaming()` 辅助函数
- [ ] 4.2.7 支持 AbortSignal 取消操作
- [ ] 4.2.8 编写单元测试验证流式响应

### 4.3 流式工具调用 / Streaming Tool Calls
- [ ] 4.3.1 实现 `stream_message_with_tools()` 方法
- [ ] 4.3.2 处理流中的 delta.tool_calls
- [ ] 4.3.3 累积工具调用参数（处理分片）
- [ ] 4.3.4 生成 `StreamChunk::ToolUseDelta` 事件
- [ ] 4.3.5 在流结束时返回完整工具调用列表
- [ ] 4.3.6 编写单元测试验证流式工具调用

## 阶段 5：高级功能实现 / Phase 5: Advanced Features Implementation

### 5.1 参数适配逻辑 / Parameter Adaptation Logic
- [ ] 5.1.1 创建 `crates/kode-services/src/openai/adapter.rs`
- [ ] 5.1.2 定义 `ModelFeatures` 结构体
- [ ] 5.1.3 实现 `get_model_features()` 函数（检测 GPT-5/o1/o3）
- [ ] 5.1.4 实现 `apply_model_transformations()` 函数
- [ ] 5.1.5 实现 max_tokens → max_completion_tokens 转换
- [ ] 5.1.6 实现 temperature 固定为 1（特定模型）
- [ ] 5.1.7 移除不支持参数（frequency_penalty 等）
- [ ] 5.1.8 编写单元测试验证参数转换

### 5.2 可修复错误检测 / Fixable Error Detection
- [ ] 5.2.1 实现 `detect_fixable_error()` 函数
- [ ] 5.2.2 检测 max_tokens vs max_completion_tokens 错误
- [ ] 5.2.3 检测 temperature must be 1 错误
- [ ] 5.2.4 检测 stream_options not supported 错误
- [ ] 5.2.5 检测工具描述过长错误
- [ ] 5.2.6 编写单元测试验证错误检测

### 5.3 错误处理和重试 / Error Handling and Retry
- [ ] 5.3.1 实现指数退避计算（`get_retry_delay()`）
- [ ] 5.3.2 添加随机抖动（±10%）
- [ ] 5.3.3 处理 retry-after header
- [ ] 5.3.4 实现最大重试次数限制（10 次）
- [ ] 5.3.5 区分可重试和不可重试错误
- [ ] 5.3.6 实现可修复错误的自动修复和重试
- [ ] 5.3.7 添加重试日志（尝试次数、延迟时间）
- [ ] 5.3.8 编写单元测试验证重试逻辑

### 5.4 端点回退支持 / Endpoint Fallback Support
- [ ] 5.4.1 实现 `try_with_endpoint_fallback()` 函数
- [ ] 5.4.2 支持 MiniMax 端点回退（/text/chatcompletion_v2 → /chat/completions）
- [ ] 5.4.3 处理 404 响应并尝试下一个端点
- [ ] 5.4.4 记录端点切换日志
- [ ] 5.4.5 支持可扩展的端点列表
- [ ] 5.4.6 编写单元测试验证端点回退

## 阶段 6：集成和测试 / Phase 6: Integration and Testing

### 6.1 单元测试完善 / Unit Test Completion
- [ ] 6.1.1 所有模块的单元测试覆盖率 > 80%
- [ ] 6.1.2 测试边界情况（空消息、无效参数等）
- [ ] 6.1.3 测试错误路径（网络错误、API 错误等）
- [ ] 6.1.4 使用 mock 框架（mockito 或类似）模拟 HTTP 响应
- [ ] 6.1.5 确保所有测试通过

### 6.2 集成测试 / Integration Tests
- [ ] 6.2.1 创建 `crates/kode-services/src/openai/tests.rs`
- [ ] 6.2.2 实现非流式请求集成测试（真实 OpenAI API）
- [ ] 6.2.3 实现流式请求集成测试（真实 OpenAI API）
- [ ] 6.2.4 实现工具调用集成测试（真实 OpenAI API）
- [ ] 6.2.5 使用环境变量配置凭证
- [ ] 6.2.6 测试默认使用 `#[ignore]` 属性
- [ ] 6.2.7 添加测试运行说明文档

### 6.3 兼容性测试 / Compatibility Tests
- [ ] 6.3.1 加载原版 TypeScript 配置文件
- [ ] 6.3.2 验证配置解析正确性
- [ ] 6.3.3 测试 DeepSeek 提供商（如果可用）
- [ ] 6.3.4 测试其他提供商（MiniMax 等）
- [ ] 6.3.5 比较响应格式与原版一致性
- [ ] 6.3.6 记录兼容性测试结果

### 6.4 性能测试 / Performance Tests
- [ ] 6.4.1 测试非流式请求延迟（目标 < 2s）
- [ ] 6.4.2 测试流式请求首字节延迟（目标 < 500ms）
- [ ] 6.4.3 测试内存占用（目标 < 10MB）
- [ ] 6.4.4 测试并发请求（多个 OpenAIService 实例）
- [ ] 6.4.5 记录性能基准测试结果

## 阶段 7：代码质量和文档 / Phase 7: Code Quality and Documentation

### 7.1 代码格式化 / Code Formatting
- [ ] 7.1.1 运行 `cargo fmt -- --check` 验证格式
- [ ] 7.1.2 确保所有代码符合 rustfmt 标准
- [ ] 7.1.3 检查导入顺序和分组
- [ ] 7.1.4 验证代码风格一致性

### 7.2 Clippy 检查 / Clippy Checks
- [ ] 7.2.1 运行 `cargo clippy -- -D warnings`
- [ ] 7.2.2 修复所有 clippy 警告
- [ ] 7.2.3 不使用 `#[allow(clippy::...)]`（除非有充分理由）
- [ ] 7.2.4 验证零警告状态

### 7.3 文档注释 / Documentation Comments
- [ ] 7.3.1 所有公开 API 添加 rustdoc 注释
- [ ] 7.3.2 包含 `# Arguments`、`# Errors`、`# Examples` 章节
- [ ] 7.3.3 添加使用示例代码
- [ ] 7.3.4 运行 `cargo doc` 验证文档生成
- [ ] 7.3.5 确保文档编译无警告

### 7.4 内联注释 / Inline Comments
- [ ] 7.4.1 关键逻辑添加中文注释
- [ ] 7.4.2 复杂算法添加解释说明
- [ ] 7.4.3 标注性能关键路径
- [ ] 7.4.4 标注安全敏感代码（API 密钥处理等）

## 阶段 8：验证和提交 / Phase 8: Validation and Submission

### 8.1 最终验证 / Final Validation
- [ ] 8.1.1 运行 `cargo test --all` 确保所有测试通过
- [ ] 8.1.2 运行 `cargo fmt --check` 确保格式正确
- [ ] 8.1.3 运行 `cargo clippy -- -D warnings` 确保零警告
- [ ] 8.1.4 运行 `cargo build --release` 确保编译成功
- [ ] 8.1.5 运行 `cargo doc --no-deps` 确保文档生成

### 8.2 OpenSpec 验证 / OpenSpec Validation
- [ ] 8.2.1 运行 `npx openspec validate implement-openai-service --strict`
- [ ] 8.2.2 修复所有验证错误
- [ ] 8.2.3 确认所有需求已覆盖
- [ ] 8.2.4 确认所有场景已测试

### 8.3 代码审查准备 / Code Review Preparation
- [ ] 8.3.1 生成变更摘要（新增文件、修改文件）
- [ ] 8.3.2 准备测试结果报告
- [ ] 8.3.3 准备性能基准测试结果
- [ ] 8.3.4 准备兼容性测试结果
- [ ] 8.3.5 更新 `tasks.md` 标记所有任务完成

### 8.4 提交和归档 / Submission and Archiving
- [ ] 8.4.1 提交代码（遵循 Conventional Commits）
- [ ] 8.4.2 推送到远程仓库
- [ ] 8.4.3 创建 Pull Request（如果需要）
- [ ] 8.4.4 等待代码审查
- [ ] 8.4.5 根据反馈修改
- [ ] 8.4.6 合并到主分支
- [ ] 8.4.7 运行 `/openspec:archive` 归档变更

## 依赖关系 / Dependencies

### 并行任务 / Parallelizable Tasks
以下任务可以并行执行：
- 2.2（类型定义）和 2.3（错误类型）可以并行
- 3.2（消息转换）和 3.3（非流式消息发送）可以串行但与其他模块并行
- 4.1（SSE 流处理）和 5.1（参数适配）可以并行

### 串行任务 / Serial Tasks
以下任务必须串行执行：
- 2.1 → 2.2, 2.3 → 3.1 → 3.2 → 3.3 → 3.4
- 4.1 → 4.2 → 4.3
- 5.1 → 5.2 → 5.3 → 5.4
- 所有实现任务 → 阶段 6（测试）
- 阶段 7（代码质量）→ 阶段 8（验证和提交）

## 估计工作量 / Estimated Effort

**总估计：15-20 工作小时**

- 阶段 1（准备工作）：2 小时
- 阶段 2（基础结构）：2 小时
- 阶段 3（核心功能）：4 小时
- 阶段 4（流式响应）：4 小时
- 阶段 5（高级功能）：3 小时
- 阶段 6（集成和测试）：3 小时
- 阶段 7（代码质量）：1 小时
- 阶段 8（验证和提交）：1 小时

## 验收标准 / Acceptance Criteria

### 功能完整性
- ✅ 所有 spec 中的需求已实现
- ✅ 所有场景已测试并通过
- ✅ 非流式和流式请求都正常工作
- ✅ 工具调用功能完整实现
- ✅ 参数自适应正确工作

### 代码质量
- ✅ 所有测试通过（单元测试、集成测试）
- ✅ 测试覆盖率 > 80%
- ✅ 无 clippy 警告
- ✅ 代码已格式化
- ✅ 文档完整

### 性能和兼容性
- ✅ 性能符合目标（延迟、内存）
- ✅ 与原版 TypeScript 配置兼容
- ✅ 支持多个提供商（OpenAI、DeepSeek）
- ✅ 错误处理和重试机制工作正常

## 参考资料 / Reference Materials

### 原版实现
- `/Users/gemini/Documents/backup/Kode-cli/src/services/openai.ts` - OpenAI 服务核心实现
- `/Users/gemini/Documents/backup/Kode-cli/src/services/adapters/` - 适配器实现
- `/Users/gemini/Documents/backup/Kode-cli/src/services/modelAdapterFactory.ts` - 模型适配器工厂

### Rust 参考实现
- `crates/kode-services/src/anthropic.rs` - AnthropicService（可作为参考）
- `crates/kode-core/src/model/adapter.rs` - ModelAdapter trait 定义

### API 文档
- [OpenAI API Reference](https://platform.openai.com/docs/api-reference)
- [DeepSeek API Reference](https://platform.deepseek.com/api-docs/)
- [Reqwest Documentation](https://docs.rs/reqwest/)
- [Tokio Documentation](https://tokio.rs/)
