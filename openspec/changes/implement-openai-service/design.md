# Design: OpenAI-compatible 服务实现 / OpenAI-compatible Service Implementation

## 设计概述 / Design Overview

本文档详细描述 OpenAI-compatible 服务的实现设计，基于对原版 TypeScript 实现的深入分析。

This document details the implementation design of the OpenAI-compatible service, based on an in-depth analysis of the original TypeScript implementation.

## 架构设计 / Architecture Design

### 整体架构 / Overall Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    CLI Application Layer                     │
│                   (kode-cli / kode-ui)                       │
└───────────────────────────┬─────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                   Model Adapter Trait                        │
│              (kode-core/src/model/adapter.rs)                │
│  - send_message()                                            │
│  - send_message_with_tools()                                 │
│  - stream_message()                                          │
│  - stream_message_with_tools()                               │
└───────────┬─────────────────────────────┬───────────────────┘
            │                             │
            ▼                             ▼
┌──────────────────────┐      ┌──────────────────────────────┐
│  AnthropicService    │      │    OpenAIService             │
│  (anthropic.rs)      │      │    (openai/service.rs)       │
└──────────────────────┘      └──────────────────────────────┘
            │                             │
            ▼                             ▼
┌──────────────────────┐      ┌──────────────────────────────┐
│  Anthropic API       │      │    OpenAI-compatible APIs    │
│  - Messages API      │      │    - Chat Completions API    │
│  - Streaming API     │      │    - Multiple Providers      │
└──────────────────────┘      └──────────────────────────────┘
```

### 模块划分 / Module Organization

OpenAI 服务模块采用与 AnthropicService 相似的模块化设计：

The OpenAI service module adopts a modular design similar to AnthropicService:

```
crates/kode-services/src/openai/
├── mod.rs           # 模块导出 / Module exports
├── service.rs       # OpenAIService 核心实现 / OpenAIService core implementation
├── types.rs         # OpenAI API 类型定义 / OpenAI API type definitions
├── error.rs         # OpenAI 特定错误 / OpenAI-specific errors
├── streaming.rs     # SSE 流处理 / SSE stream processing
├── adapter.rs       # 参数适配逻辑 / Parameter adaptation logic
└── tests.rs         # 集成测试 / Integration tests
```

## 核心组件设计 / Core Components Design

### 1. OpenAIService 结构体 / OpenAIService Struct

**文件位置：** `crates/kode-services/src/openai/service.rs`

**File location:** `crates/kode-services/src/openai/service.rs`

```rust
use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use std::time::Duration;

use crate::openai::types::OpenAIConfig;
use crate::openai::error::{OpenAIError, OpenAIErrorKind};
use kode_core::model::adapter::{ModelAdapter, ModelConfig, ModelResponse};
use kode_core::message::Message;

/// OpenAI API 客户端
///
/// 实现了 ModelAdapter trait，用于与 OpenAI-compatible API 交互。
/// 支持多种提供商（OpenAI、DeepSeek、MiniMax 等）。
#[derive(Debug, Clone)]
pub struct OpenAIService {
    /// HTTP 客户端
    client: Client,
    /// API 配置
    config: OpenAIConfig,
}

impl OpenAIService {
    /// 创建新的 OpenAIService
    ///
    /// # Arguments
    ///
    /// * `config` - OpenAI 配置
    ///
    /// # Returns
    ///
    /// 新的 OpenAIService 实例
    pub fn new(config: OpenAIConfig) -> Self {
        // 实现细节...
    }

    /// 从 ModelConfig 创建 OpenAIService
    pub fn from_model_config(config: ModelConfig) -> Self {
        // 实现细节...
    }

    /// 获取基础 URL
    fn base_url(&self) -> &str {
        &self.config.base_url
    }

    /// 获取模型名称
    fn model_name(&self) -> &str {
        &self.config.model_name
    }
}

#[async_trait]
impl ModelAdapter for OpenAIService {
    // 实现 ModelAdapter trait 的所有方法...
}
```

**设计要点：**

**Design points:**
1. **结构简单**：只包含 HTTP 客户端和配置，保持轻量级
   **Simple structure**: Only contains HTTP client and configuration, keeping it lightweight

2. **Clone 安全**：HTTP 客户端可以安全克隆（reqwest::Client 支持 Arc）
   **Clone-safe**: HTTP client can be safely cloned (reqwest::Client supports Arc)

3. **配置不可变**：创建后配置不可修改，保证线程安全
   **Immutable configuration**: Configuration cannot be modified after creation, ensuring thread safety

### 2. 类型定义 / Type Definitions

**文件位置：** `crates/kode-services/src/openai/types.rs`

**File location:** `crates/kode-services/src/openai/types.rs`

```rust
use serde::{Deserialize, Serialize};

/// OpenAI API 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIConfig {
    /// API 密钥
    #[serde(rename = "apiKey")]
    pub api_key: String,

    /// 基础 URL
    #[serde(rename = "baseURL")]
    pub base_url: String,

    /// 模型名称
    #[serde(rename = "modelName")]
    pub model_name: String,

    /// 最大 token 数
    #[serde(rename = "maxTokens")]
    pub max_tokens: usize,

    /// 提供商（可选，用于确定特殊逻辑）
    pub provider: Option<String>,
}

/// OpenAI Chat Completions API 请求
#[derive(Debug, Serialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ToolDefinition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
}

/// OpenAI Chat 消息
#[derive(Debug, Serialize, Clone)]
#[serde(tag = "role")]
pub enum ChatMessage {
    #[serde(rename = "system")]
    System { content: String },
    #[serde(rename = "user")]
    User { content: String },
    #[serde(rename = "assistant")]
    Assistant { content: String },
    #[serde(rename = "tool")]
    Tool {
        tool_call_id: String,
        content: String,
    },
}

/// OpenAI API 响应（非流式）
#[derive(Debug, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

/// 选择项
#[derive(Debug, Deserialize)]
pub struct Choice {
    pub index: usize,
    pub message: ChatMessageResponse,
    pub finish_reason: String,
}

/// 聊天消息响应
#[derive(Debug, Deserialize)]
pub struct ChatMessageResponse {
    pub role: String,
    pub content: Option<String>,
    #[serde(default)]
    pub tool_calls: Vec<ToolCall>,
}

/// 工具调用
#[derive(Debug, Deserialize, Clone)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub call_type: String,
    pub function: FunctionCall,
}

/// 函数调用
#[derive(Debug, Deserialize, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

/// Token 使用统计
#[derive(Debug, Deserialize, Clone)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// 流式响应块
#[derive(Debug, Deserialize)]
pub struct ChatCompletionChunk {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<StreamChoice>,
}

/// 流式选择项
#[derive(Debug, Deserialize)]
pub struct StreamChoice {
    pub index: usize,
    pub delta: StreamDelta,
    #[serde(default)]
    pub finish_reason: Option<String>,
}

/// 流式增量
#[derive(Debug, Deserialize)]
pub struct StreamDelta {
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub tool_calls: Option<Vec<StreamToolCall>>,
}
```

**设计要点：**

**Design points:**
1. **序列化友好**：使用 serde 的 `rename` 属性确保 JSON 字段名与 API 一致（camelCase）
   **Serialization-friendly**: Use serde's `rename` attribute to ensure JSON field names match the API (camelCase)

2. **可选字段**：使用 `Option` 和 `skip_serializing_if = "Option::is_none"` 灵活处理可选参数
   **Optional fields**: Use `Option` and `skip_serializing_if = "Option::is_none"` for flexible optional parameter handling

3. **流式和非流式**：分别定义流式和非流式响应类型
   **Streaming and non-streaming**: Define streaming and non-streaming response types separately

### 3. 错误处理 / Error Handling

**文件位置：** `crates/kode-services/src/openai/error.rs`

**File location:** `crates/kode-services/src/openai/error.rs`

```rust
use thiserror::Error;

/// OpenAI 服务错误类型
#[derive(Error, Debug)]
pub enum OpenAIError {
    /// API 请求错误
    #[error("API request failed: {0}")]
    ApiRequestFailed(String),

    /// API 响应错误
    #[error("API response error (status {status}): {message}")]
    ApiResponseError { status: u16, message: String },

    /// 网络错误
    #[error("Network error: {0}")]
    NetworkError(String),

    /// 认证错误
    #[error("Authentication failed: invalid API key")]
    AuthenticationError,

    /// 速率限制错误
    #[error("Rate limit exceeded: retry after {retry_after}s")]
    RateLimitError { retry_after: u32 },

    /// 参数错误
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// 流处理错误
    #[error("Stream processing error: {0}")]
    StreamError(String),

    /// JSON 解析错误
    #[error("JSON parse error: {0}")]
    JsonParseError(String),

    /// 可修复的参数错误（需要重试）
    #[error("Fixable parameter error ({kind}): {message}")]
    FixableParameterError {
        kind: OpenAIErrorKind,
        message: String,
    },
}

/// 可修复的错误类型
#[derive(Debug, Clone, PartialEq)]
pub enum OpenAIErrorKind {
    /// 需要 max_completion_tokens 而非 max_tokens
    MaxCompletionTokensRequired,
    /// temperature 必须为 1
    TemperatureMustBeOne,
    /// 工具描述过长（>1024 字符）
    ToolDescriptionTooLong,
    /// 不支持 stream_options
    StreamOptionsNotSupported,
    /// 不支持 citations 参数
    CitationsNotSupported,
}
```

**设计要点：**

**Design points:**
1. **详细错误类型**：区分不同类型的错误，便于精确处理
   **Detailed error types**: Distinguish between different types of errors for precise handling

2. **可修复错误**：特别标记可修复的错误，支持自动重试（参考原版 TS 实现）
   **Fixable errors**: Specifically mark fixable errors to support automatic retry (referencing original TS implementation)

3. **thiserror**：使用 thiserror 简化错误定义和转换
   **thiserror**: Use thiserror to simplify error definition and conversion

### 4. 流式响应处理 / Streaming Response Processing

**文件位置：** `crates/kode-services/src/openai/streaming.rs`

**File location:** `crates/kode-services/src/openai/streaming.rs`

```rust
use anyhow::Result;
use futures::stream::Stream;
use kode_core::model::types::StreamChunk;
use kode_core::model::streaming::StreamingResponse;
use crate::openai::types::ChatCompletionChunk;

/// SSE 流处理器
///
/// 解析 OpenAI SSE 格式的流式响应。
pub struct SseStreamProcessor {
    /// 响应 ID
    response_id: String,
    /// 累积的内容
    accumulated_content: String,
    /// 待处理的工具调用
    pending_tool_calls: Vec<PendingToolCall>,
}

#[derive(Debug, Clone)]
struct PendingToolCall {
    id: String,
    name: String,
    arguments: String,
}

impl SseStreamProcessor {
    /// 创建新的处理器
    pub fn new() -> Self {
        Self {
            response_id: String::new(),
            accumulated_content: String::new(),
            pending_tool_calls: Vec::new(),
        }
    }

    /// 处理 SSE 行
    pub fn process_line(&mut self, line: &str) -> Result<Option<StreamChunk>> {
        // 跳过空行
        let line = line.trim();
        if line.is_empty() {
            return Ok(None);
        }

        // 检查 data: [DONE]
        if line == "data: [DONE]" {
            return Ok(None);
        }

        // 解析 data: JSON
        if let Some(json_str) = line.strip_prefix("data: ") {
            let chunk: ChatCompletionChunk = serde_json::from_str(json_str)
                .map_err(|e| anyhow::anyhow!("Failed to parse SSE chunk: {}", e))?;

            return self.process_chunk(chunk);
        }

        Ok(None)
    }

    /// 处理数据块
    fn process_chunk(&mut self, chunk: ChatCompletionChunk) -> Result<Option<StreamChunk>> {
        // 更新响应 ID
        if !chunk.id.is_empty() {
            self.response_id = chunk.id.clone();
        }

        // 处理选择项
        if let Some(choice) = chunk.choices.first() {
            // 处理内容增量
            if let Some(content) = &choice.delta.content {
                if !content.is_empty() {
                    self.accumulated_content.push_str(content);
                    return Ok(Some(StreamChunk::content_block_delta(
                        choice.index,
                        content,
                    )));
                }
            }

            // 处理工具调用
            if let Some(tool_calls) = &choice.delta.tool_calls {
                for tool_call in tool_calls {
                    self.process_tool_call_delta(tool_call);
                }
            }

            // 检查是否完成
            if let Some(finish_reason) = &choice.finish_reason {
                return Ok(Some(StreamChunk::message_stop(
                    self.build_usage(),
                )));
            }
        }

        Ok(None)
    }

    /// 处理工具调用增量
    fn process_tool_call_delta(&mut self, tool_call: &StreamToolCall) {
        // 查找或创建待处理的工具调用
        let pending = self.pending_tool_calls
            .iter_mut()
            .find(|t| t.id == tool_call.id);

        if let Some(pending) = pending {
            if let Some(name) = &tool_call.function.name {
                pending.name.clone_from(name);
            }
            if let Some(args) = &tool_call.function.arguments {
                pending.arguments.push_str(args);
            }
        } else {
            self.pending_tool_calls.push(PendingToolCall {
                id: tool_call.id.clone(),
                name: tool_call.function.name.clone().unwrap_or_default(),
                arguments: tool_call.function.arguments.clone().unwrap_or_default(),
            });
        }
    }

    /// 构建使用统计
    fn build_usage(&self) -> TokenUsage {
        // 简化的使用统计，实际应该从累积数据中计算
        TokenUsage {
            input_tokens: 0,
            output_tokens: self.accumulated_content.len() as u32 / 4, // 粗略估计
            total_tokens: None,
            thinking_tokens: None,
        }
    }
}

/// 将 reqwest 响应转换为流式响应
pub async fn response_to_streaming(
    response: reqwest::Response,
) -> Result<StreamingResponse> {
    let (tx, rx) = StreamingResponse::channel();

    tokio::spawn(async move {
        let mut processor = SseStreamProcessor::new();
        let byte_stream = response.bytes_stream();

        use futures::stream::StreamExt;
        let mut lines = byte_stream.map(|chunk| {
            chunk
                .map_err(|e| anyhow::anyhow!("Stream error: {}", e))
                .and_then(|bytes| {
                    String::from_utf8(bytes.to_vec())
                        .map_err(|e| anyhow::anyhow!("UTF-8 decode error: {}", e))
                })
        });

        while let Some(line_result) = lines.next().await {
            match line_result {
                Ok(line) => {
                    match processor.process_line(&line) {
                        Ok(Some(chunk)) => {
                            if let Err(e) = tx.send(Ok(chunk)).await {
                                tracing::error!("Failed to send chunk: {}", e);
                                break;
                            }
                        }
                        Ok(None) => {}
                        Err(e) => {
                            let _ = tx.send(Err(e.into())).await;
                            break;
                        }
                    }
                }
                Err(e) => {
                    let _ = tx.send(Err(e)).await;
                    break;
                }
            }
        }
    });

    Ok(rx)
}
```

**设计要点：**

**Design points:**
1. **状态累积**：处理器维护累积状态（内容、工具调用等）
   **State accumulation**: Processor maintains accumulated state (content, tool calls, etc.)

2. **SSE 格式解析**：正确处理 OpenAI SSE 格式（`data: JSON`）
   **SSE format parsing**: Correctly handle OpenAI SSE format (`data: JSON`)

3. **异步处理**：使用 tokio task 在后台处理流
   **Async processing**: Use tokio task to handle stream in background

### 5. 参数适配逻辑 / Parameter Adaptation Logic

**文件位置：** `crates/kode-services/src/openai/adapter.rs`

**File location:** `crates/kode-services/src/openai/adapter.rs`

```rust
use crate::openai::types::{ChatCompletionRequest, ChatMessage};
use crate::openai::error::{OpenAIError, OpenAIErrorKind};
use kode_core::message::Message;

/// 模型特性
#[derive(Debug, Clone)]
pub struct ModelFeatures {
    /// 是否使用 max_completion_tokens 而非 max_tokens
    pub uses_max_completion_tokens: bool,
    /// temperature 是否必须为 1
    pub requires_temperature_one: bool,
    /// 是否支持 reasoning_effort 参数
    pub supports_reasoning_effort: bool,
}

/// 获取模型特性
pub fn get_model_features(model_name: &str) -> ModelFeatures {
    // GPT-5/o1/o3 系列
    if model_name.contains("gpt-5")
        || model_name.starts_with("o1-")
        || model_name.starts_with("o3-")
    {
        return ModelFeatures {
            uses_max_completion_tokens: true,
            requires_temperature_one: true,
            supports_reasoning_effort: true,
        };
    }

    // 默认特性
    ModelFeatures {
        uses_max_completion_tokens: false,
        requires_temperature_one: false,
        supports_reasoning_effort: false,
    }
}

/// 应用模型特定的参数转换
pub fn apply_model_transformations(
    mut request: ChatCompletionRequest,
    model_name: &str,
) -> Result<ChatCompletionRequest, OpenAIError> {
    let features = get_model_features(model_name);

    // 转换 max_tokens → max_completion_tokens
    if features.uses_max_completion_tokens {
        if let Some(max_tokens) = request.max_tokens {
            request.max_completion_tokens = Some(max_tokens);
            request.max_tokens = None;
        }
    }

    // 固定 temperature 为 1
    if features.requires_temperature_one {
        request.temperature = Some(1.0);
    }

    // 移除不支持的参数
    if !features.supports_reasoning_effort {
        // 移除 reasoning_effort（如果有）
        // request.reasoning_effort = None;
    }

    Ok(request)
}

/// 检测并标记可修复的错误
pub fn detect_fixable_error(error_message: &str) -> Option<OpenAIErrorKind> {
    let lower = error_message.to_lowercase();

    // max_tokens vs max_completion_tokens
    if lower.contains("max_tokens")
        && (lower.contains("max_completion_tokens")
            || lower.contains("use max_completion_tokens"))
    {
        return Some(OpenAIErrorKind::MaxCompletionTokensRequired);
    }

    // temperature must be 1
    if lower.contains("temperature")
        && (lower.contains("must be 1")
            || lower.contains("only supports")
            || lower.contains("invalid temperature"))
    {
        return Some(OpenAIErrorKind::TemperatureMustBeOne);
    }

    // stream_options not supported
    if lower.contains("stream_options") {
        return Some(OpenAIErrorKind::StreamOptionsNotSupported);
    }

    None
}
```

**设计要点：**

**Design points:**
1. **模型特性检测**：根据模型名称确定特性
   **Model feature detection**: Determine features based on model name

2. **参数自动转换**：自动转换不兼容的参数
   **Automatic parameter transformation**: Automatically transform incompatible parameters

3. **可修复错误检测**：检测可以自动修复的错误类型
   **Fixable error detection**: Detect error types that can be automatically fixed

## 实现策略 / Implementation Strategy

### 1. 重试机制 / Retry Mechanism

参考原版 TypeScript 实现，实现指数退避重试：

Referencing the original TypeScript implementation, implement exponential backoff retry:

```rust
const BASE_DELAY_MS: u64 = 1000;
const MAX_DELAY_MS: u64 = 32000;
const MAX_ATTEMPTS: usize = 10;

fn get_retry_delay(attempt: usize, retry_after: Option<u32>) -> Duration {
    // 如果服务器指定了 retry-after，使用它
    if let Some(seconds) = retry_after {
        let ms = (seconds as u64).min(60) * 1000;
        return Duration::from_millis(ms);
    }

    // 指数退避 + 抖动
    let delay_ms = BASE_DELAY_MS * 2u64.pow(attempt as u32);
    let jitter = (delay_ms as f64 * 0.1) as u64;
    let total_delay = (delay_ms + jitter).min(MAX_DELAY_MS);

    Duration::from_millis(total_delay)
}
```

### 2. 端点回退 / Endpoint Fallback

```rust
async fn try_with_fallback(
    client: &Client,
    base_url: &str,
    request: &ChatCompletionRequest,
) -> Result<reqwest::Response> {
    let endpoints = if is_minimax_provider(base_url) {
        vec!["/text/chatcompletion_v2", "/chat/completions"]
    } else {
        vec!["/chat/completions"]
    };

    let mut last_error = None;

    for endpoint in endpoints {
        let url = format!("{}{}", base_url, endpoint);
        match client.post(&url).json(request).send().await {
            Ok(response) => {
                if response.status().is_success() || response.status() != 404 {
                    return Ok(response);
                }
            }
            Err(e) => {
                last_error = Some(e);
            }
        }
    }

    Err(last_error.unwrap_or_else(|| {
        anyhow::anyhow!("All endpoints failed")
    }))
}
```

### 3. 工具调用支持 / Tool Call Support

工具调用需要特殊处理：

Tool calls require special handling:

1. **工具定义转换**：将 Rust 工具定义转换为 OpenAI 格式
   **Tool definition transformation**: Convert Rust tool definitions to OpenAI format

2. **工具调用解析**：从响应中提取工具调用
   **Tool call parsing**: Extract tool calls from responses

3. **工具结果返回**：将工具执行结果返回给模型
   **Tool result return**: Return tool execution results to the model

```rust
impl OpenAIService {
    /// 构建工具定义
    fn build_tools(&self, tools: &[serde_json::Value]) -> Vec<ToolDefinition> {
        tools
            .iter()
            .map(|tool| ToolDefinition {
                r#type: "function".to_string(),
                function: FunctionDefinition {
                    name: tool["name"].as_str().unwrap().to_string(),
                    description: tool["description"].as_str().unwrap().to_string(),
                    parameters: tool["inputSchema"].clone(),
                },
            })
            .collect()
    }
}
```

## 测试策略 / Testing Strategy

### 单元测试 / Unit Tests

每个模块都需要单元测试：

Each module needs unit tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_features_detection() {
        assert!(get_model_features("gpt-5").uses_max_completion_tokens);
        assert!(get_model_features("o1-preview").requires_temperature_one);
        assert!(!get_model_features("gpt-4").uses_max_completion_tokens);
    }

    #[test]
    fn test_parameter_transformation() {
        let request = ChatCompletionRequest {
            model: "gpt-5".to_string(),
            max_tokens: Some(1000),
            temperature: Some(0.7),
            ..Default::default()
        };

        let transformed = apply_model_transformations(request, "gpt-5")
            .unwrap();

        assert_eq!(transformed.max_completion_tokens, Some(1000));
        assert_eq!(transformed.max_tokens, None);
        assert_eq!(transformed.temperature, Some(1.0));
    }
}
```

### 集成测试 / Integration Tests

使用环境变量配置凭证进行真实 API 测试：

Use environment variables to configure credentials for real API testing:

```rust
#[tokio::test]
#[ignore] // 默认忽略，需要手动运行
async fn test_openai_real_api() {
    let api_key = std::env::var("OPENAI_API_KEY")
        .expect("OPENAI_API_KEY not set");

    let config = OpenAIConfig {
        api_key,
        base_url: "https://api.openai.com/v1".to_string(),
        model_name: "gpt-4".to_string(),
        max_tokens: 100,
        provider: Some("openai".to_string()),
    };

    let service = OpenAIService::new(config);
    let messages = vec![Message::user("Hello!")];

    let response = service.send_message(messages, None, 100)
        .await
        .unwrap();

    assert!(!response.content.is_empty());
    assert_eq!(response.model, "gpt-4");
}
```

### 兼容性测试 / Compatibility Tests

使用原版 TypeScript 的配置文件测试：

Test using original TypeScript configuration files:

```rust
#[tokio::test]
#[ignore]
async fn test_typescript_config_compatibility() {
    // 加载原版配置
    let config_content = std::fs::read_to_string(
        "/Users/gemini/Documents/backup/Kode-cli/test_fixtures/config.json"
    ).unwrap();

    let config: OpenAIConfig = serde_json::from_str(&config_content)
        .expect("Failed to parse config");

    // 验证配置正确解析
    assert!(!config.api_key.is_empty());
    assert!(!config.base_url.is_empty());
}
```

## 性能考虑 / Performance Considerations

### 内存优化 / Memory Optimization

1. **流式处理**：流式响应不累积全部内容，立即处理
   **Streaming processing**: Streaming responses don't accumulate all content, process immediately

2. **零拷贝解析**：使用 `bytes::Bytes` 避免不必要的内存拷贝
   **Zero-copy parsing**: Use `bytes::Bytes` to avoid unnecessary memory copying

3. **连接池**：复用 HTTP 连接
   **Connection pooling**: Reuse HTTP connections

### 并发安全 / Concurrency Safety

1. **Send + Sync**：OpenAIService 实现 Send + Sync
   **Send + Sync**: OpenAIService implements Send + Sync

2. **Arc 共享**：HTTP 客户端内部使用 Arc
   **Arc sharing**: HTTP client uses Arc internally

3. **无状态**：每次请求独立，不共享可变状态
   **Stateless**: Each request is independent, no shared mutable state

## 错误恢复 / Error Recovery

### 自动重试场景 / Automatic Retry Scenarios

以下错误自动重试：

The following errors are automatically retried:

1. **网络错误**：连接超时、DNS 失败等
   **Network errors**: Connection timeout, DNS failure, etc.

2. **速率限制**：HTTP 429，使用 retry-after header
   **Rate limit**: HTTP 429, using retry-after header

3. **服务器错误**：HTTP 5xx
   **Server errors**: HTTP 5xx

4. **可修复的参数错误**：自动修正后重试
   **Fixable parameter errors**: Retry after automatic correction

### 不可重试错误 / Non-retryable Errors

以下错误不重试：

The following errors are not retried:

1. **认证错误**：HTTP 401
   **Authentication errors**: HTTP 401

2. **权限错误**：HTTP 403
   **Permission errors**: HTTP 403

3. **参数错误（不可修复）**：HTTP 400
   **Parameter errors (not fixable)**: HTTP 400

## 未来扩展 / Future Extensions

### 支持更多 API / Support More APIs

1. **Embeddings API**：文本嵌入
   **Embeddings API**: Text embedding

2. **Images API**：图像生成
   **Images API**: Image generation

3. **Audio API**：语音输入输出
   **Audio API**: Speech input/output

### 高级功能 / Advanced Features

1. **批处理**：一次处理多个请求
   **Batching**: Process multiple requests at once

2. **请求缓存**：缓存相同请求的响应
   **Request caching**: Cache responses for identical requests

3. **监控指标**：请求延迟、成功率等
   **Monitoring metrics**: Request latency, success rate, etc.
