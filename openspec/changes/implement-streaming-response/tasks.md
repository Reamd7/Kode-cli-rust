# 实现任务 / Implementation Tasks

## 当前状态 / Current Status

**已完成部分 / Completed**:
- ✅ 基础 SSE 流式解析器已实现 (`anthropic.rs:696-895`)
- ✅ StreamChunk 类型定义完成 (`kode-core/src/model/types.rs`)
- ✅ StreamingResponse 包装器完成 (`kode-core/src/model/streaming.rs`)
- ✅ 基本的 `stream_message` 方法已实现
- ✅ 工具调用的流式处理已支持

**待完善部分 / Pending**:
- ⬜ 缺少完整的错误处理和重连机制
- ⬜ 缺少 AbortSignal 支持（用户中断）
- ⬜ 缺少 TTFT (Time To First Token) 统计
- ⬜ 缺少流式响应的单元测试
- ⬜ 缺少流式响应的集成测试
- ⬜ 缺少 debug 日志系统（参考原版 debugLogger）
- ⬜ 缺少流式事件类型文档注释

---

## 1. 准备工作 / Preparation ✅
- [x] 1.1 阅读相关规范文档
  - [x] 阅读 `openspec/specs/anthropic-service/spec.md`
  - [x] 理解流式响应规范要求
- [x] 1.2 分析原版 TypeScript 实现
  - [x] 分析 `/Users/gemini/Documents/backup/Kode-cli/src/services/claude.ts` (2000+ 行)
  - [x] 分析 `/Users/gemini/Documents/backup/Kode-cli/src/services/adapters/responsesStreaming.ts`
  - [x] 分析 `/Users/gemini/Documents/backup/Kode-cli/src/services/adapters/base.ts`
- [x] 1.3 理解需求和场景
  - [x] 流式响应提供实时输出，改善用户体验
  - [x] 支持 SSE (Server-Sent Events) 格式
  - [x] 支持文本和工具调用的混合流

---

## 2. 核心功能实现 / Implementation ⚠️ (部分完成)

### 2.1 SSE 事件解析器 ✅
- [x] 2.1.1 实现 `ServerSentEvent` 类型
  - [x] 定义事件类型字段 (`type: content_block_start/delta/stop, message_delta`)
  - [x] 定义索引字段 (`index`)
  - [x] 定义增量字段 (`delta.text_delta`, `delta.input_json_delta`)
  - [x] 定义使用统计字段 (`usage`)
- [x] 2.1.2 实现字节流解析逻辑
  - [x] 识别 SSE 事件边界 (`\n\n`)
  - [x] 解析 `data:` 前缀的 JSON
  - [x] 处理 `[DONE]` 终止信号
  - [x] 缓冲不完整的事件数据

### 2.2 流式响应类型系统 ✅
- [x] 2.2.1 实现 `StreamChunk` 枚举 (`kode-core/src/model/types.rs`)
  - [x] `ContentBlockStart { index }`
  - [x] `ContentBlockDelta { index, delta }`
  - [x] `ContentBlockStop { index }`
  - [x] `ToolUse { tool_name, tool_use_id, parameters }`
  - [x] `ToolUseComplete { index, parameters }`
  - [x] `MessageStop { usage }`
  - [x] `Error { message }`
- [x] 2.2.2 实现 `TokenUsage` 结构体
  - [x] `input_tokens`
  - [x] `output_tokens`
  - [x] `total_tokens` (可选)
  - [x] `thinking_tokens` (可选)
- [x] 2.2.3 实现 `StreamingResponse` 包装器
  - [x] 包装 `Pin<Box<dyn Stream>>`
  - [x] 提供 `channel()` 方法创建生产者-消费者
  - [x] 实现 `Stream` trait

### 2.3 流式消息方法 ⚠️ (部分完成)
- [x] 2.3.1 实现 `stream_message` 基础方法
  - [x] 调用 `stream_message_with_retry`
  - [x] 返回 `StreamingResponse`
- [x] 2.3.2 实现 `stream_message_with_retry` 方法
  - [x] 构建请求体
  - [x] 设置 `Accept: text/event-stream` 头
  - [x] 发送 POST 请求
  - [x] 调用 `handle_streaming_response`
- [x] 2.3.3 实现 `handle_streaming_response` 方法
  - [x] 创建通道对 `(tx, rx)`
  - [x] 启动异步任务处理字节流
  - [x] 解析 SSE 事件
  - [x] 发送 `StreamChunk` 到通道

### 2.4 工具调用流式支持 ✅
- [x] 2.4.1 处理 `tool_use` 类型内容块
  - [x] 检测 `content_block_start` 的 `tool_use` 类型
  - [x] 初始化 JSON 缓冲区 (`json_buffers: HashMap<usize, String>`)
  - [x] 发送 `ToolUse` 事件（工具名、ID）
- [x] 2.4.2 处理工具参数增量
  - [x] 累积 `input_json_delta` 到缓冲区
  - [x] 发送 `ContentBlockDelta` 事件（参数片段）
- [x] 2.4.3 完成工具调用
  - [x] 在 `content_block_stop` 时发送 `ToolUseComplete`
  - [x] 清理缓冲区

---

## 3. 高级功能实现 / Advanced Features ❌ (未开始)

### 3.1 错误处理和重连 ❌
- [ ] 3.1.1 实现流式错误检测
  - [ ] 检测连接中断
    - [ ] 捕获 `reqwest::Error` 的网络错误类型（anthropic.rs:906）
    - [ ] 识别连接超时、DNS 失败、连接重置等错误
    - [ ] 记录错误类型到日志
  - [ ] 检测超时
    - [ ] 添加 `timeout` 参数到流式方法
    - [ ] 使用 `tokio::time::timeout` 包装流循环
    - [ ] 超时后发送错误事件并关闭流
  - [ ] 检测无效 JSON
    - [ ] 捕获 `serde_json::from_str` 的解析错误（anthropic.rs:795）
    - [ ] 记录原始 JSON 字符串到错误日志
    - [ ] 继续处理后续事件（容错）
  - [ ] 参考 TS 版本: `claude.ts:withRetry` 函数的错误处理逻辑
- [ ] 3.1.2 实现自动重连机制
  - [ ] 复用现有的重试逻辑
    - [ ] 参考 `anthropic.rs:403-500` 的 `send_message_with_retry` 实现
    - [ ] 提取重试逻辑为通用函数 `with_retry_backoff`
  - [ ] 指数退避重试
    - [ ] 实现指数退避：`delay = 500ms * 2^attempt`
    - [ ] 最大延迟限制：32 秒
    - [ ] 参考 `anthropic/error.rs:42-46` 的 `get_retry_delay` 函数
  - [ ] 最大重试次数限制
    - [ ] 默认最大重试 3 次
    - [ ] 可配置重试次数
  - [ ] 可重试的错误判断
    - [ ] 使用 `should_retry` 函数判断是否重试（anthropic/error.rs:35-40）
    - [ ] 仅对临时错误重试（超时、连接错误、5xx）
    - [ ] 认证错误、4xx 不重试
  - [ ] 注意：流式响应重连较复杂，暂不实现保留位置功能
- [ ] 3.1.3 发送错误事件到流
  - [ ] 通过通道发送 `StreamChunk::Error`（anthropic.rs:758, 908）
  - [ ] 错误消息格式：`"Stream error: {reason}"`
  - [ ] 记录详细错误日志
    - [ ] 使用 `tracing::error!` 级别
    - [ ] 包含错误类型、消息、堆栈（如果有）
  - [ ] 参考 TS 版本: `claude.ts:1521-1525` 的错误处理

### 3.2 AbortSignal 支持 ✅
- [x] 3.2.1 添加 `signal: Option<AbortSignal>` 参数
  - [x] `stream_message` 接受 signal 参数（通过内部方法）
  - [x] `stream_message_with_tools` 接受 signal 参数（通过内部方法）
- [x] 3.2.2 实现中断检查
  - [x] 在流循环中使用 `tokio::select!` 检查中断信号
  - [x] 发送 `Error("Request was cancelled")` 事件
  - [x] 清理资源（关闭通道）
- [x] 3.2.3 实现 AbortSignal 和 AbortHandle 类型
  - [x] 在 `anthropic/types.rs` 中定义类型
  - [x] 添加单元测试验证功能
- [x] 3.2.4 集成到流式方法
  - [x] `stream_message_with_tools_internal` 接受 `abort_handle: Option<AbortHandle>`
  - [x] 使用 tokio::sync::oneshot 通道进行中断通知
  - [x] 通过 tokio::select! 同时等待流数据和中断信号

### 3.3 性能监控 ✅ (已完成)
- [x] 3.3.1 TTFT (Time To First Token) 统计
  - [x] 添加 `StreamMetrics` 结构体存储性能指标
    ```rust
    pub struct StreamMetrics {
        pub start_time: Instant,
        pub first_token_time: Option<Instant>,
        pub end_time: Option<Instant>,
        pub chunk_count: usize,
        pub error_count: usize,
    }
    ```
  - [x] 在流开始时记录 `start_time`（anthropic.rs:740）
  - [x] 在第一个 `ContentBlockDelta` 时记录 `first_token_time`（anthropic.rs:837-845）
  - [x] 计算 TTFT = `first_token_time - start_time`
  - [x] 在流结束时记录 `end_time` 并计算总时长
  - [x] 参考 TS 版本: `claude.ts:1482` (`start = Date.now()`), `claude.ts:1565-1575` (`ttftMs`)
  - [x] 在 debug 日志中输出 TTFT 信息（anthropic.rs:841-843）
- [x] 3.3.2 流式事件计数
  - [x] 添加 `chunk_count: usize` 字段追踪接收的 chunk 总数
  - [x] 在每次成功解析 SSE 事件后递增（anthropic.rs:846）
  - [x] 添加 `error_count: usize` 字段追踪错误次数
  - [x] 在发送错误事件时递增（anthropic.rs:936）
  - [x] 参考 TS 版本: `claude.ts:1542-1543` (`chunkCount`, `errorCount`)
  - [x] 在完成日志中输出统计信息（anthropic.rs:906）
- [x] 3.3.3 流总时长统计
  - [x] 在流结束时记录 `end_time: Instant`（anthropic.rs:897, 943）
  - [x] 计算总时长 = `end_time.duration_since(start_time)`
  - [x] 计算流式时长 = `end_time.duration_since(first_token_time)`
  - [x] 参考 TS 版本: `claude.ts:1575` (`durationMs`)
  - [x] 在 debug 日志中输出时长信息（anthropic.rs:905-907）

### 3.4 Debug 日志系统 ⬜ (部分完成)
- [x] 3.4.1 定义统一的日志事件常量
  - [x] 使用统一的日志事件命名前缀 `ANTHROPIC_STREAM_*`
  - [x] 参考 TS 版本: `claude.ts:1479-1523` (`debugLogger.api` 调用)
- [x] 3.4.2 记录流开始事件
  - [x] 在发起请求前记录 `ANTHROPIC_STREAM_START` 事件（anthropic.rs:753-755）
  - [x] 使用 `tracing::debug!` 级别
- [x] 3.4.3 记录首个 token 事件
  - [x] 在第一个 `ContentBlockDelta` 时记录 `ANTHROPIC_STREAM_FIRST_TOKEN`（anthropic.rs:840-844）
  - [x] 包含 TTFT 毫秒数和 chunk 索引
  - [x] 参考 TS 版本: `claude.ts:1565-1575`
- [x] 3.4.4 记录流完成事件
  - [x] 在 `message_delta` 时记录 `ANTHROPIC_STREAM_COMPLETE`（anthropic.rs:900-908）
  - [x] 包含 TTFT、总时长、chunk 数量、错误数量
  - [x] 参考 TS 版本: `claude.ts:1565-1575`
- [x] 3.4.5 记录流中断事件
  - [x] 在 abort 触发时记录 `Streaming request was aborted`（anthropic.rs:762）
  - [x] 使用 `tracing::debug!` 级别
  - [x] 参考 TS 版本: `claude.ts:1521-1525`
- [ ] 3.4.6 记录 JSON 解析错误
  - [ ] 在工具调用 JSON 解析失败时记录错误（anthropic.rs:860-877）
  - [ ] 使用 `tracing::error!` 级别
  - [ ] 参考 TS 版本: `claude.ts:1560-1568`

---

## 4. 测试 / Tests ❌ (未开始)

### 4.1 单元测试 ❌
- [ ] 4.1.1 测试 `StreamChunk` 序列化/反序列化
  - [ ] 测试所有 chunk 类型的 JSON 转换
  - [ ] 测试边界情况（空字符串、特殊字符）
  - [ ] 参考: `types.rs:92-119` (已有简单测试)
- [ ] 4.1.2 测试 `StreamingResponse` 通道
  - [ ] 测试发送和接收 chunk
  - [ ] 测试通道关闭行为
  - [ ] 测试并发发送
  - [ ] 参考: `streaming.rs:58-88` (已有简单测试)
- [ ] 4.1.3 测试 SSE 解析器
  - [ ] 测试完整事件解析
  - [ ] 测试分块到达的事件重组
  - [ ] 测试 `[DONE]` 处理
  - [ ] 测试无效 JSON 容错

### 4.2 集成测试 ❌
- [ ] 4.2.1 测试 Anthropic API 流式响应
  - [ ] 使用测试 API key
  - [ ] 发送简单文本消息
  - [ ] 验证接收的 chunk 序列
  - [ ] 验证最终的 TokenUsage
- [ ] 4.2.2 测试工具调用流式响应
  - [ ] 定义测试工具
  - [ ] 发送带工具的消息
  - [ ] 验证工具调用事件的顺序
  - [ ] 验证工具参数的完整性
- [ ] 4.2.3 测试错误处理
  - [ ] 模拟网络中断
  - [ ] 模拟超时
  - [ ] 验证错误事件发送
- [ ] 4.2.4 测试 AbortSignal
  - [ ] 发起流式请求
  - [ ] 在中途中断
  - [ ] 验证资源清理

### 4.3 性能测试 ❌
- [ ] 4.3.1 测试 TTFT
  - [ ] 测量多次请求的 TTFT
  - [ ] 验证 TTFT < 2s (目标)
- [ ] 4.3.2 测试吞吐量
  - [ ] 测试大消息的流式性能
  - [ ] 测试并发流式请求
- [ ] 4.3.3 内存测试
  - [ ] 验证流式响应不会无限增长内存
  - [ ] 验证通道缓冲区大小合理

---

## 5. 文档 / Documentation ⬜ (部分完成)

### 5.1 代码注释 ✅ (部分完成)
- [x] 5.1.1 添加模块级文档
  - [x] `streaming.rs`: 流式响应包装器说明
  - [x] `types.rs`: 模型类型定义说明
  - [ ] `anthropic.rs`: 流式处理逻辑说明（需要补充）
- [ ] 5.1.2 添加函数文档
  - [ ] `stream_message` 方法
  - [ ] `handle_streaming_response` 方法
  - [ ] SSE 解析逻辑
- [ ] 5.1.3 添加使用示例
  - [ ] 如何发起流式请求
  - [ ] 如何处理流式响应
  - [ ] 如何中断流式请求

### 5.2 更新相关文档 ❌
- [ ] 5.2.1 更新 anthropic-service spec.md
  - [ ] 标记流式响应需求为已实现
  - [ ] 补充实现细节和注意事项
- [ ] 5.2.2 更新 message-model spec.md
  - [ ] 补充 `StreamChunk` 类型定义
  - [ ] 补充 `StreamingResponse` 使用说明
- [ ] 5.2.3 创建流式响应使用指南（可选）
  - [ ] 用户文档：如何启用流式响应
  - [ ] 开发文档：如何扩展流式功能

---

## 6. TypeScript vs Rust 实现差异 / Implementation Differences

基于详细的对比分析（详见 `implementation-comparison.md`），以下是需要补全的功能差异：

### 6.1 缺失的事件类型 / Missing Event Types

#### 6.1.1 MessageStart 事件 ❌
- **TypeScript 实现**:
  ```typescript
  case 'message_start':
    messageStartEvent = event
    finalResponse = {
      ...event.message,
      content: []
    }
  ```
  - 位置: `claude.ts:1527-1531`
  - 作用: 初始化响应对象，包含响应元数据（id、type、role、model）

- **Rust 实现现状**:
  - ❌ 无对应事件
  - ⚠️ 直接开始处理 content_block

- **实施任务**:
  - [ ] 添加 `MessageStart` 变体到 `StreamChunk` 枚举
    ```rust
    MessageStart {
        response_id: String,
        model: String,
        role: String,
    }
    ```
  - [ ] 在 SSE 解析中检测 `message_start` 事件类型
  - [ ] 发送 `MessageStart` 事件到流
  - [ ] 参考位置: `anthropic.rs:770-800`

### 6.2 性能监控差异 / Performance Monitoring Differences

#### 6.2.1 TTFT 统计 ❌
- **TypeScript 实现**:
  ```typescript
  let start = Date.now()
  // ... 流式处理
  const ttftMs = start - Date.now()  // 错误的公式，应该是 firstToken - start
  ```
  - 位置: `claude.ts:1482, 1565`

- **Rust 实现现状**:
  - ❌ 完全缺失

- **实施任务**: 见章节 3.3.1

#### 6.2.2 流事件计数 ❌
- **TypeScript 实现**:
  ```typescript
  const contentBlocks: any[] = []
  const inputJSONBuffers = new Map<number, string>()
  // 隐式计数通过数组长度
  ```
  - 位置: `claude.ts:1512-1513`

- **Rust 实现现状**:
  - ❌ 无显式计数

- **实施任务**: 见章节 3.3.2

### 6.3 日志系统差异 / Logging System Differences

#### 6.3.1 结构化日志 ❌
- **TypeScript 实现**:
  ```typescript
  debugLogger.api('ANTHROPIC_API_CALL_START_STREAMING', {
    endpoint: modelProfile?.baseURL,
    model,
    streamMode: true,
    toolsCount: toolSchemas.length,
    timestamp: new Date().toISOString()
  })
  ```
  - 事件类型: `api`, `flow`, `error`
  - 位置: `claude.ts:1479-1523`

- **Rust 实现现状**:
  - ⚠️ 只有基础 `tracing::debug!` 日志
  - ❌ 无事件分类
  - ❌ 无结构化上下文

- **实施任务**: 见章节 3.4

### 6.4 错误处理差异 / Error Handling Differences

#### 6.4.1 重试机制 ❌
- **TypeScript 实现**:
  ```typescript
  response = await withRetry(async attempt => {
    // ...
  })
  ```
  - 位置: `claude.ts:1458`
  - 实现: 指数退避重试

- **Rust 实现现状**:
  - ✅ 非流式请求有重试（`send_message_with_retry`）
  - ❌ 流式请求无重试

- **实施任务**: 见章节 3.1.2

### 6.5 工具调用处理差异 / Tool Call Handling Differences

#### 6.5.1 JSON 解析时机 ⚠️
- **TypeScript 实现**:
  ```typescript
  case 'content_block_stop':
    if (block?.type === 'tool_use') {
      const jsonStr = inputJSONBuffers.get(stopIndex)
      block.input = JSON.parse(jsonStr)  // 立即解析
      inputJSONBuffers.delete(stopIndex)
    }
  ```
  - 位置: `claude.ts:1557-1568`
  - 特点: 在 `content_block_stop` 时立即解析为对象

- **Rust 实现现状**:
  ```rust
  "content_block_stop" => {
      if let Some(json_str) = json_buffers.remove(&index) {
          tx.send(Ok(StreamChunk::tool_use_complete(index, json_str)));
      }
  }
  ```
  - 位置: `anthropic.rs:860-877`
  - 特点: 发送原始 JSON 字符串

- **评价**:
  - ✅ Rust 设计更灵活
  - ✅ 允许调用方决定何时解析
  - ✅ 避免在流处理中阻塞
  - ⚠️ 与 TypeScript 不对等

- **决策**: 保持 Rust 设计，不修改为对等实现

---

## 7. 验证 / Validation ❌

### 6.1 代码质量检查 ❌
- [ ] 6.1.1 运行 clippy
  - [ ] 修复所有警告
  - [ ] 确保 0 warnings
- [ ] 6.1.2 运行 fmt
  - [ ] 格式化所有代码
  - [ ] 运行 `cargo fmt --check` 验证
- [ ] 6.1.3 运行 doc test
  - [ ] `cargo doc --no-deps` 生成文档
  - [ ] 验证所有文档示例可编译

### 6.2 测试覆盖率 ❌
- [ ] 6.2.1 运行单元测试
  - [ ] `cargo test` 全部通过
  - [ ] 目标覆盖率 > 80%
- [ ] 6.2.2 运行集成测试
  - [ ] 集成测试全部通过
  - [ ] 手动测试真实 API
- [ ] 6.2.3 性能验证
  - [ ] TTFT < 2s
  - [ ] 内存使用稳定

### 6.3 兼容性验证 ❌
- [ ] 6.3.1 对比原版行为
  - [ ] 测试相同输入的输出是否一致
  - [ ] 验证错误处理行为
- [ ] 6.3.2 配置兼容性
  - [ ] 验证原版配置文件可用
  - [ ] 验证 Agent 定义兼容

---

## 7. 技术债务和改进建议 / Technical Debt

### 当前实现的已知问题 / Known Issues

1. **缺少 AbortSignal 支持**
   - **影响**: 用户无法中断正在进行的流式请求
   - **优先级**: P0 (用户体验关键)
   - **参考**: TS 版本 line 1655, 1693

2. **缺少 TTFT 监控**
   - **影响**: 无法诊断流式响应性能问题
   - **优先级**: P1 (调试和优化需要)
   - **参考**: TS 版本 line 1565-1575

3. **缺少 debug 日志**
   - **影响**: 难以诊断流式问题
   - **优先级**: P1 (开发调试需要)
   - **参考**: TS 版本 `debugLogger.api()` 调用

4. **错误处理不完善**
   - **影响**: 网络错误时可能导致卡死
   - **优先级**: P1 (稳定性)
   - **建议**: 实现超时和重连机制

5. **测试覆盖不足**
   - **影响**: 代码质量难以保证
   - **优先级**: P1 (质量保证)
   - **建议**: 补充单元测试和集成测试

---

## 8. 参考实现映射 / Implementation Mapping

### TypeScript → Rust 映射关系

| TypeScript (TS) | Rust | 状态 |
|----------------|------|------|
| `claude.ts:1654-1753` 流式响应循环 | `anthropic.rs:696-895` | ✅ 已实现 |
| `responsesStreaming.ts` 流式处理器 | `streaming.rs` | ✅ 已实现 |
| `base.ts:StreamingEvent` | `types.rs:StreamChunk` | ✅ 已实现 |
| `debugLogger.api()` 调用 | `tracing::debug!` | ⬜ 部分实现 |
| `AbortSignal` 支持 | `tokio::sync::mpsc` 通道 | ❌ 未实现 |
| `ttftMs` 统计 | - | ❌ 未实现 |
| `chunkCount` 统计 | - | ❌ 未实现 |

### 关键差异 / Key Differences

1. **并发模型**: TS 使用 Promise/async-await，Rust 使用 tokio 异步任务
2. **错误处理**: TS 使用 try-catch，Rust 使用 Result 类型
3. **流式 API**: TS 使用 AsyncGenerator，Rust 使用 Stream trait
4. **中断机制**: TS 使用 AbortSignal，Rust 需要自己实现

---

## 9. 完成标准 / Completion Criteria

变更可以归档的前提条件：

- [ ] **核心功能完整**: 所有 2.x 任务完成
- [ ] **测试覆盖充分**: 4.1, 4.2 测试完成并通过
- [ ] **文档完善**: 5.1, 5.2 文档完成
- [ ] **质量达标**: 6.1 clippy/fmt 通过，6.2 测试通过
- [ ] **无 P0/P1 技术债务**: AbortSignal 支持至少有基础实现

---

## 10. 下一步行动 / Next Actions

### 立即执行 (本次变更)
1. 实现 AbortSignal 支持 (P0)
2. 补充错误处理和超时 (P1)
3. 添加单元测试 (P1)
4. 添加 debug 日志 (P1)

### 后续优化 (新变更)
1. 性能监控和 TTFT 统计
2. 自动重连机制
3. 集成测试补充
4. 性能基准测试
