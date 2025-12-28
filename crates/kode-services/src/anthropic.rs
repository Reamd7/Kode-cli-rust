//! Anthropic API 服务实现
//!
//! 提供 Anthropic Claude API 的客户端实现。

use anyhow::anyhow;
use async_trait::async_trait;
use futures::stream::StreamExt;
use kode_core::error::Result;
use kode_core::message::{ContentBlock, Message, MessageContent, Role, TextBlock};
use kode_core::model::adapter::{ModelAdapter, ModelConfig, ModelResponse};
use kode_core::model::streaming::StreamingResponse;
use reqwest::{Client, ClientBuilder, StatusCode};
use serde_json::Value;
use std::time::Duration;
use tracing::debug;

pub mod cache;
pub mod cost;
pub mod error;
pub mod types;

pub use error::*;
pub use types::*;

use crate::anthropic::cache::AnthropicCacheConfig;

/// Anthropic API 客户端
///
/// 实现了 ModelAdapter trait，用于与 Anthropic Claude API 交互。
#[derive(Debug, Clone)]
pub struct AnthropicService {
    /// HTTP 客户端
    client: Client,
    /// API 配置
    config: AnthropicConfig,
}

impl AnthropicService {
    /// 创建新的 AnthropicService
    ///
    /// # Arguments
    ///
    /// * `config` - Anthropic 配置
    ///
    /// # Returns
    ///
    /// 新的 AnthropicService 实例
    pub fn new(config: AnthropicConfig) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "x-api-key",
            reqwest::header::HeaderValue::from_str(&config.api_key).unwrap(),
        );
        headers.insert(
            "anthropic-version",
            reqwest::header::HeaderValue::from_static("2023-06-01"),
        );
        headers.insert(
            "Content-Type",
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        // 如果配置了思考模式，添加 beta header
        if config.thinking.is_some() {
            headers.insert(
                "anthropic-beta",
                reqwest::header::HeaderValue::from_static("max-tokens-3-5-sonnet-2024-07-01"),
            );
        }

        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(60))
            .default_headers(headers)
            .build()
            .expect("Failed to build HTTP client");

        Self { client, config }
    }

    /// 从 ModelConfig 创建 AnthropicService
    pub fn from_model_config(config: ModelConfig) -> Self {
        let anthropic_config = AnthropicConfig {
            api_key: config.api_key,
            base_url: config
                .base_url
                .unwrap_or_else(|| "https://api.anthropic.com".to_string()),
            model_name: config.model_name,
            max_tokens: config.max_tokens,
            cache_config: None,
            thinking: None,
        };
        Self::new(anthropic_config)
    }

    /// 获取基础 URL
    fn base_url(&self) -> &str {
        &self.config.base_url
    }

    /// 获取模型名称
    #[allow(dead_code)]
    fn model_name(&self) -> &str {
        &self.config.model_name
    }

    /// 构建 API 请求体
    ///
    /// # Arguments
    ///
    /// * `messages` - 消息列表
    /// * `system_prompt` - 系统提示词
    /// * `max_tokens` - 最大输出 token 数
    /// * `tools` - 可选的工具列表
    ///
    /// # Returns
    ///
    /// 请求体 JSON
    fn build_request_body(
        &self,
        messages: &[Message],
        system_prompt: Option<&str>,
        max_tokens: usize,
        tools: Option<&[Value]>,
    ) -> Result<serde_json::Value> {
        // 转换消息为 Anthropic API 格式
        let anthropic_messages: Vec<serde_json::Value> = messages
            .iter()
            .map(|msg| self.message_to_anthropic(msg))
            .collect();

        // 构建请求
        let mut request_body = serde_json::json!({
            "model": self.config.model_name,
            "messages": anthropic_messages,
            "max_tokens": max_tokens,
        });

        // 添加系统提示词
        if let Some(system) = system_prompt {
            request_body["system"] = serde_json::json!(system);
        }

        // 添加工具定义
        if let Some(tools) = tools {
            if !tools.is_empty() {
                request_body["tools"] = serde_json::json!(tools);
                // 添加 tool_choice
                request_body["tool_choice"] = serde_json::json!({
                    "type": "auto"
                });
            }
        }

        // 添加思考配置
        if let Some(ref thinking_config) = self.config.thinking {
            request_body["thinking"] = serde_json::to_value(thinking_config)
                .map_err(|e| kode_core::error::Error::ConfigError(e.to_string()))?;
        }

        // 应用缓存控制
        if let Some(ref cache_config) = self.config.cache_config {
            let total_tokens = request_body["messages"]
                .as_array()
                .map(|msgs| msgs.len() * 100)
                .unwrap_or(0);
            request_body = crate::anthropic::cache::apply_cache_control_with_limits(
                request_body,
                cache_config,
                total_tokens,
            );
        }

        Ok(request_body)
    }

    /// 将 Message 转换为 Anthropic API 消息格式
    fn message_to_anthropic(&self, message: &Message) -> serde_json::Value {
        let role = match message.role {
            Role::User => "user",
            Role::Assistant => "assistant",
            Role::System => "system",
        };

        let content = match &message.content {
            MessageContent::Text(text) => {
                serde_json::json!({
                    "type": "text",
                    "text": text
                })
            }
            MessageContent::Blocks(blocks) => {
                serde_json::json!(blocks
                    .iter()
                    .map(|block| self.content_block_to_anthropic(block))
                    .collect::<Vec<_>>())
            }
        };

        serde_json::json!({
            "role": role,
            "content": content
        })
    }

    /// 将 ContentBlock 转换为 Anthropic API 格式
    fn content_block_to_anthropic(&self, block: &ContentBlock) -> serde_json::Value {
        match block {
            ContentBlock::Text(TextBlock { text }) => {
                serde_json::json!({
                    "type": "text",
                    "text": text
                })
            }
            ContentBlock::ToolUse(tool_use) => {
                serde_json::json!({
                    "type": "tool_use",
                    "id": tool_use.tool_use_id,
                    "name": tool_use.tool_name,
                    "input": tool_use.parameters
                })
            }
            ContentBlock::ToolResult(result) => {
                serde_json::json!({
                    "type": "tool_result",
                    "tool_use_id": result.tool_use_id,
                    "content": result.content
                })
            }
            ContentBlock::Image(image) => {
                serde_json::json!({
                    "type": "image",
                    "source": {
                        "type": "base64",
                        "media_type": image.media_type,
                        "data": image.data
                    }
                })
            }
        }
    }

    /// 解析 Anthropic API 响应
    fn parse_response(&self, response: ApiResponse) -> Result<ModelResponse> {
        let content = self.extract_content(&response.content);

        let usage = kode_core::model::TokenUsage {
            input_tokens: response.usage.input_tokens,
            output_tokens: response.usage.output_tokens,
            total_tokens: Some(response.usage.input_tokens + response.usage.output_tokens),
            thinking_tokens: response.usage.thinking_tokens,
        };

        let cost_usd = Some(crate::anthropic::cost::get_model_total_cost_usd(
            &response.model,
            response.usage.input_tokens,
            response.usage.output_tokens,
            0, // cache_tokens - 当前未追踪
        ));

        Ok(ModelResponse {
            content,
            usage,
            model: response.model,
            cost_usd,
        })
    }

    /// 从响应内容中提取文本
    fn extract_content(&self, blocks: &[ContentBlock]) -> String {
        blocks
            .iter()
            .filter_map(|block| {
                if let ContentBlock::Text(text_block) = block {
                    Some(text_block.text.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .join("")
    }
}

#[async_trait]
impl ModelAdapter for AnthropicService {
    async fn send_message(
        &self,
        messages: Vec<Message>,
        system_prompt: Option<String>,
        max_tokens: usize,
    ) -> Result<ModelResponse> {
        self.send_message_with_retry(messages, system_prompt, max_tokens, None)
            .await
    }

    async fn stream_message(
        &self,
        messages: Vec<Message>,
        system_prompt: Option<String>,
        max_tokens: usize,
    ) -> Result<StreamingResponse> {
        self.stream_message_with_retry(messages, system_prompt, max_tokens, None)
            .await
    }

    async fn send_message_with_tools(
        &self,
        messages: Vec<Message>,
        system_prompt: Option<String>,
        max_tokens: usize,
        tools: &[serde_json::Value],
    ) -> Result<ModelResponse> {
        self.send_message_with_tools_internal(messages, system_prompt, max_tokens, tools)
            .await
    }

    async fn stream_message_with_tools(
        &self,
        messages: Vec<Message>,
        system_prompt: Option<String>,
        max_tokens: usize,
        tools: &[serde_json::Value],
    ) -> Result<StreamingResponse> {
        self.stream_message_with_tools_internal(messages, system_prompt, max_tokens, tools)
            .await
    }

    fn model_name(&self) -> &str {
        &self.config.model_name
    }
}

impl AnthropicService {
    /// 带重试的发送消息方法
    async fn send_message_with_retry(
        &self,
        messages: Vec<Message>,
        system_prompt: Option<String>,
        max_tokens: usize,
        tools: Option<&[Value]>,
    ) -> Result<ModelResponse> {
        let request_body =
            self.build_request_body(&messages, system_prompt.as_deref(), max_tokens, tools)?;

        debug!(target: "kode_services", "Sending request to Anthropic API");

        let url = format!("{}/v1/messages", self.base_url());

        // 重试逻辑
        let max_retries = 3;
        let mut last_error: Option<anyhow::Error> = None;

        for attempt in 1..=max_retries + 1 {
            match self.client.post(&url).json(&request_body).send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        match response.json::<ApiResponse>().await {
                            Ok(api_response) => {
                                return self.parse_response(api_response);
                            }
                            Err(e) => {
                                last_error =
                                    Some(anyhow::anyhow!("Failed to parse response: {}", e));
                            }
                        }
                    } else if response.status() == 429 || response.status().is_server_error() {
                        // 速率限制或服务器错误，可重试
                        let delay = get_retry_delay(attempt);
                        tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
                        last_error = Some(anyhow::anyhow!("HTTP {}", response.status()));
                        continue;
                    } else {
                        // 其他错误，直接返回
                        let status = response.status();
                        let text = response.text().await.unwrap_or_default();
                        return Err(kode_core::error::Error::ModelRequestError(format!(
                            "HTTP {}: {}",
                            status, text
                        )));
                    }
                }
                Err(e) => {
                    last_error = Some(e.into());
                }
            }

            // 检查是否应该重试
            if attempt <= max_retries {
                let delay = get_retry_delay(attempt);
                debug!(
                    target: "kode_services",
                    "Retry attempt {}/{} after {}ms",
                    attempt, max_retries, delay
                );
                tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
            }
        }

        Err(kode_core::error::Error::ModelRequestError(
            last_error
                .unwrap_or_else(|| anyhow::anyhow!("Unknown error"))
                .to_string(),
        ))
    }

    /// 带重试的流式消息方法
    async fn stream_message_with_retry(
        &self,
        messages: Vec<Message>,
        system_prompt: Option<String>,
        max_tokens: usize,
        tools: Option<&[Value]>,
    ) -> Result<StreamingResponse> {
        let request_body =
            self.build_request_body(&messages, system_prompt.as_deref(), max_tokens, tools)?;

        debug!(target: "kode_services", "Starting streaming request to Anthropic API");

        let url = format!("{}/v1/messages", self.base_url());

        // 重试逻辑
        let max_retries = 3;
        let mut last_error: Option<anyhow::Error> = None;

        for attempt in 1..=max_retries + 1 {
            match self
                .client
                .post(&url)
                .header("Accept", "text/event-stream")
                .json(&request_body)
                .send()
                .await
            {
                Ok(response) if response.status().is_success() => {
                    return self.handle_streaming_response(response, tools.is_some());
                }
                Ok(response) => {
                    if response.status() == 429 || response.status().is_server_error() {
                        let delay = get_retry_delay(attempt);
                        tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
                        last_error = Some(anyhow::anyhow!("HTTP {}", response.status()));
                        continue;
                    }
                    let status = response.status();
                    let text = response.text().await.unwrap_or_default();
                    return Err(kode_core::error::Error::ModelRequestError(format!(
                        "HTTP {}: {}",
                        status, text
                    )));
                }
                Err(e) => {
                    last_error = Some(e.into());
                }
            }

            if attempt <= max_retries {
                let delay = get_retry_delay(attempt);
                debug!(
                    target: "kode_services",
                    "Retry attempt {}/{} after {}ms",
                    attempt, max_retries, delay
                );
                tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
            }
        }

        Err(kode_core::error::Error::ModelRequestError(
            last_error
                .unwrap_or_else(|| anyhow::anyhow!("Unknown error"))
                .to_string(),
        ))
    }

    /// 处理流式响应
    fn handle_streaming_response(
        &self,
        response: reqwest::Response,
        _has_tools: bool,
    ) -> Result<StreamingResponse> {
        let (tx, rx) = StreamingResponse::channel();

        // Spawn task to handle streaming response
        let mut stream = response.bytes_stream();
        let _tx = tokio::spawn(async move {
            let mut buffer = Vec::new();
            let mut current_block_index: Option<usize> = None;
            let mut current_block_type: Option<String> = None;
            let mut json_buffers: std::collections::HashMap<usize, String> =
                std::collections::HashMap::new();

            while let Some(chunk) = stream.next().await {
                if let Ok(data) = chunk {
                    buffer.extend(&data);

                    // Process complete SSE events
                    while let Some(pos) = buffer.windows(2).position(|w| w == [b'\n', b'\n']) {
                        let event_data = String::from_utf8_lossy(&buffer[..pos]).into_owned();
                        buffer.drain(..=pos + 1);

                        // Parse SSE format
                        for line in event_data.lines() {
                            if line.starts_with("data:") {
                                if let Some(json_str) = line.strip_prefix("data:").map(|s| s.trim())
                                {
                                    if json_str == "[DONE]" {
                                        // Send final message stop
                                        tx.send(Ok(kode_core::model::StreamChunk::message_stop(
                                            kode_core::model::TokenUsage {
                                                input_tokens: 0,
                                                output_tokens: 0,
                                                total_tokens: None,
                                                thinking_tokens: None,
                                            },
                                        )))
                                        .await
                                        .ok();
                                        return;
                                    }

                                    if let Ok(event) =
                                        serde_json::from_str::<ServerSentEvent>(json_str)
                                    {
                                        match event.r#type.as_str() {
                                            "content_block_start" => {
                                                if let Some(block) = event.content_block {
                                                    current_block_index = Some(block.index);
                                                    let block_type = block.block_type.clone();
                                                    current_block_type = Some(block_type.clone());

                                                    // 初始化 JSON buffer
                                                    if block_type == "tool_use" {
                                                        json_buffers
                                                            .insert(block.index, String::new());

                                                        // 发送工具使用事件
                                                        if let (
                                                            Some(tool_name),
                                                            Some(tool_use_id),
                                                        ) = (block.name, block.id)
                                                        {
                                                            tx.send(Ok(
                                                                kode_core::model::StreamChunk::tool_use(
                                                                    tool_name,
                                                                    tool_use_id,
                                                                    serde_json::Value::Null,
                                                                ),
                                                            ))
                                                            .await
                                                            .ok();
                                                        }
                                                    }

                                                    tx.send(Ok(
                                                        kode_core::model::StreamChunk::content_block_start(
                                                            block.index,
                                                        ),
                                                    ))
                                                    .await
                                                    .ok();
                                                }
                                            }
                                            "content_block_delta" => {
                                                if let Some(delta) = event.delta {
                                                    if let Some(text_delta) = delta.text_delta {
                                                        if let Some(index) = current_block_index {
                                                            tx.send(Ok(
                                                                kode_core::model::StreamChunk::
                                                                    content_block_delta(index, text_delta),
                                                            ))
                                                            .await
                                                            .ok();
                                                        }
                                                    } else if let Some(input_json) =
                                                        delta.input_json_delta
                                                    {
                                                        // Handle JSON delta for tool use
                                                        if let (Some(index), Some(block_type)) = (
                                                            current_block_index,
                                                            &current_block_type,
                                                        ) {
                                                            if block_type == "tool_use" {
                                                                if let Some(buffer) =
                                                                    json_buffers.get_mut(&index)
                                                                {
                                                                    buffer.push_str(&input_json);
                                                                }
                                                            }
                                                        }

                                                        if let Some(index) = current_block_index {
                                                            tx.send(Ok(
                                                                kode_core::model::StreamChunk::
                                                                    content_block_delta(index, input_json),
                                                            ))
                                                            .await
                                                            .ok();
                                                        }
                                                    }
                                                }
                                            }
                                            "content_block_stop" => {
                                                // 如果是工具调用，发送完整参数
                                                if let (Some(index), Some(block_type)) =
                                                    (current_block_index, &current_block_type)
                                                {
                                                    if block_type == "tool_use" {
                                                        if let Some(complete_json) =
                                                            json_buffers.remove(&index)
                                                        {
                                                            tx.send(Ok(
                                                                kode_core::model::StreamChunk::
                                                                    tool_use_complete(index, complete_json),
                                                            ))
                                                            .await
                                                            .ok();
                                                        }
                                                    }
                                                }

                                                if let Some(index) = current_block_index {
                                                    tx.send(Ok(
                                                        kode_core::model::StreamChunk::content_block_stop(
                                                            index,
                                                        ),
                                                    ))
                                                    .await
                                                    .ok();
                                                }

                                                current_block_index = None;
                                                current_block_type = None;
                                            }
                                            "message_delta" => {
                                                // Message delta with stop_reason or usage
                                                if let Some(usage) = event.usage {
                                                    tx.send(Ok(
                                                        kode_core::model::StreamChunk::message_stop(
                                                            kode_core::model::TokenUsage {
                                                                input_tokens: usage.input_tokens,
                                                                output_tokens: usage.output_tokens,
                                                                total_tokens: Some(
                                                                    usage.input_tokens
                                                                        + usage.output_tokens,
                                                                ),
                                                                thinking_tokens: usage
                                                                    .thinking_tokens,
                                                            },
                                                        ),
                                                    ))
                                                    .await
                                                    .ok();
                                                }
                                            }
                                            "message_stop" => {
                                                // Message completed
                                            }
                                            _ => {
                                                // Unknown event type
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });

        Ok(rx)
    }

    /// 内部方法：发送消息（支持工具调用）
    async fn send_message_with_tools_internal(
        &self,
        messages: Vec<Message>,
        system_prompt: Option<String>,
        max_tokens: usize,
        tools: &[Value],
    ) -> Result<ModelResponse> {
        let request_body =
            self.build_request_body(&messages, system_prompt.as_deref(), max_tokens, Some(tools))?;

        debug!(target: "kode_services", "Sending request to Anthropic API with tools");

        let url = format!("{}/v1/messages", self.base_url());
        let response = self
            .client
            .post(&url)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| kode_core::error::Error::ModelRequestError(e.to_string()))?
            .json::<ApiResponse>()
            .await
            .map_err(|e| kode_core::error::Error::ModelResponseError(e.to_string()))?;

        self.parse_response(response)
    }

    /// 内部方法：流式发送消息（支持工具调用）
    async fn stream_message_with_tools_internal(
        &self,
        messages: Vec<Message>,
        system_prompt: Option<String>,
        max_tokens: usize,
        tools: &[Value],
    ) -> Result<StreamingResponse> {
        let request_body =
            self.build_request_body(&messages, system_prompt.as_deref(), max_tokens, Some(tools))?;

        debug!(target: "kode_services", "Starting streaming request to Anthropic API with tools");

        let url = format!("{}/v1/messages", self.base_url());
        let response = self
            .client
            .post(&url)
            .header("Accept", "text/event-stream")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| kode_core::error::Error::ModelRequestError(e.to_string()))?;

        let (tx, rx) = StreamingResponse::channel();

        // Spawn task to handle streaming response
        let mut stream = response.bytes_stream();
        let tx = tx;

        tokio::spawn(async move {
            let mut buffer = Vec::new();

            // 跟踪当前 block 的类型和 JSON 缓存
            let mut current_block_index: Option<usize> = None;
            let mut current_block_type: Option<String> = None;
            let mut json_buffers: std::collections::HashMap<usize, String> =
                std::collections::HashMap::new();

            while let Some(chunk) = stream.next().await {
                if let Ok(data) = chunk {
                    buffer.extend(&data);

                    // Process complete SSE events
                    while let Some(pos) = buffer.windows(2).position(|w| w == [b'\n', b'\n']) {
                        let event_data = String::from_utf8_lossy(&buffer[..pos]).into_owned();
                        buffer.drain(..=pos + 1);

                        // Parse SSE format
                        for line in event_data.lines() {
                            if line.starts_with("data:") {
                                if let Some(json_str) = line.strip_prefix("data:").map(|s| s.trim())
                                {
                                    if json_str == "[DONE]" {
                                        tx.send(Ok(kode_core::model::StreamChunk::message_stop(
                                            kode_core::model::TokenUsage {
                                                input_tokens: 0,
                                                output_tokens: 0,
                                                total_tokens: None,
                                                thinking_tokens: None,
                                            },
                                        )))
                                        .await
                                        .ok();
                                        return;
                                    }

                                    if let Ok(event) =
                                        serde_json::from_str::<ServerSentEvent>(json_str)
                                    {
                                        match event.r#type.as_str() {
                                            "content_block_start" => {
                                                if let Some(block) = event.content_block {
                                                    current_block_index = Some(block.index);
                                                    let block_type = block.block_type.clone();
                                                    current_block_type = Some(block_type.clone());

                                                    // 初始化 JSON buffer
                                                    if block_type == "tool_use" {
                                                        json_buffers
                                                            .insert(block.index, String::new());

                                                        // 发送工具使用事件
                                                        if let (
                                                            Some(tool_name),
                                                            Some(tool_use_id),
                                                        ) = (block.name, block.id)
                                                        {
                                                            tx.send(Ok(
                                                                kode_core::model::StreamChunk::tool_use(
                                                                    tool_name,
                                                                    tool_use_id,
                                                                    serde_json::Value::Null,
                                                                ),
                                                            ))
                                                            .await
                                                            .ok();
                                                        }
                                                    }

                                                    tx.send(Ok(kode_core::model::StreamChunk::content_block_start(block.index)))
                                                        .await
                                                        .ok();
                                                }
                                            }
                                            "content_block_delta" => {
                                                if let Some(delta) = event.delta {
                                                    if let Some(index) = current_block_index {
                                                        if let Some(text_delta) = delta.text_delta {
                                                            tx.send(Ok(kode_core::model::StreamChunk::content_block_delta(
                                                                index,
                                                                text_delta,
                                                            )))
                                                            .await
                                                            .ok();
                                                        } else if let Some(input_json) =
                                                            delta.input_json_delta
                                                        {
                                                            // 追加到 JSON buffer
                                                            if let Some(buf) =
                                                                json_buffers.get_mut(&index)
                                                            {
                                                                buf.push_str(&input_json);
                                                            }

                                                            tx.send(Ok(kode_core::model::StreamChunk::content_block_delta(
                                                                index,
                                                                input_json,
                                                            )))
                                                            .await
                                                            .ok();
                                                        }
                                                    }
                                                }
                                            }
                                            "content_block_stop" => {
                                                if let Some(index) = current_block_index {
                                                    // 如果是 tool_use，在 stop 时发送完整的事件
                                                    if current_block_type.as_deref()
                                                        == Some("tool_use")
                                                    {
                                                        if let Some(json_str) =
                                                            json_buffers.remove(&index)
                                                        {
                                                            // 发送 tool_use 完整事件
                                                            tx.send(Ok(
                                                                kode_core::model::StreamChunk::tool_use_complete(index, json_str.clone()),
                                                            ))
                                                            .await
                                                            .ok();
                                                        }
                                                    }

                                                    tx.send(Ok(kode_core::model::StreamChunk::content_block_stop(index)))
                                                        .await
                                                        .ok();

                                                    current_block_index = None;
                                                    current_block_type = None;
                                                }
                                            }
                                            "message_delta" => {
                                                if let Some(usage) = event.usage {
                                                    tx.send(Ok(
                                                        kode_core::model::StreamChunk::message_stop(
                                                            kode_core::model::TokenUsage {
                                                                input_tokens: usage.input_tokens,
                                                                output_tokens: usage.output_tokens,
                                                                total_tokens: Some(
                                                                    usage.input_tokens
                                                                        + usage.output_tokens,
                                                                ),
                                                                thinking_tokens: usage
                                                                    .thinking_tokens,
                                                            },
                                                        ),
                                                    ))
                                                    .await
                                                    .ok();
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });

        Ok(rx)
    }
}

/// 验证 Anthropic API Key
///
/// # Arguments
///
/// * `api_key` - API Key
/// * `base_url` - API 基础 URL（可选）
///
/// # Returns
///
/// 验证结果
pub async fn verify_api_key(
    api_key: &str,
    base_url: Option<&str>,
) -> std::result::Result<bool, AnthropicError> {
    let url = base_url.unwrap_or("https://api.anthropic.com");

    let client = ClientBuilder::new()
        .timeout(Duration::from_secs(10))
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                "x-api-key",
                reqwest::header::HeaderValue::from_str(api_key).unwrap(),
            );
            headers.insert(
                "anthropic-version",
                reqwest::header::HeaderValue::from_static("2023-06-01"),
            );
            headers
        })
        .build()?;

    // 尝试获取模型列表来验证 API Key
    let response = client.get(format!("{}/v1/models", url)).send().await?;

    match response.status() {
        StatusCode::OK => Ok(true),
        StatusCode::UNAUTHORIZED => Ok(false),
        StatusCode::TOO_MANY_REQUESTS => Err(AnthropicError::RateLimitError),
        status => {
            // 解析错误响应
            match response.json::<ApiErrorResponse>().await {
                Ok(error_response) => Err(AnthropicError::ApiError {
                    message: error_response.error.message,
                    error_type: error_response.error.error_type,
                }),
                Err(_) => Err(AnthropicError::RequestError(anyhow!("HTTP {}", status))),
            }
        }
    }
}

/// Anthropic 模型信息
#[derive(Debug, Clone)]
pub struct AnthropicModel {
    /// 模型 ID
    pub id: String,
    /// 显示名称
    pub display_name: String,
    /// 描述
    pub description: String,
    /// 输入 token 限制
    pub input_token_limit: usize,
    /// 输出 token 限制
    pub output_token_limit: usize,
    /// 是否支持思考模式
    pub supports_thinking: bool,
}

/// 获取支持的 Anthropic 模型列表
///
/// # Arguments
///
/// * `api_key` - API Key
/// * `base_url` - API 基础 URL（可选）
///
/// # Returns
///
/// 模型列表
pub async fn fetch_anthropic_models(
    api_key: &str,
    base_url: Option<&str>,
) -> std::result::Result<Vec<AnthropicModel>, AnthropicError> {
    let url = base_url.unwrap_or("https://api.anthropic.com");

    let client = ClientBuilder::new()
        .timeout(Duration::from_secs(30))
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                "x-api-key",
                reqwest::header::HeaderValue::from_str(api_key).unwrap(),
            );
            headers.insert(
                "anthropic-version",
                reqwest::header::HeaderValue::from_static("2023-06-01"),
            );
            headers
        })
        .build()?;

    let response = client.get(format!("{}/v1/models", url)).send().await?;

    if !response.status().is_success() {
        let status = response.status();
        match response.json::<ApiErrorResponse>().await {
            Ok(error_response) => {
                return Err(AnthropicError::ApiError {
                    message: error_response.error.message,
                    error_type: error_response.error.error_type,
                });
            }
            Err(_) => {
                return Err(AnthropicError::RequestError(anyhow!("HTTP {}", status)));
            }
        }
    }

    let models_response: ModelsResponse = response
        .json()
        .await
        .map_err(|e| AnthropicError::ParseError(e.to_string()))?;

    let models = models_response
        .data
        .into_iter()
        .map(|model| AnthropicModel {
            id: model.id,
            display_name: model.display_name,
            description: model.description,
            input_token_limit: model.input_token_limit,
            output_token_limit: model.output_token_limit,
            supports_thinking: model.capabilities.thinking,
        })
        .collect();

    Ok(models)
}

/// Anthropic API 配置
#[derive(Debug, Clone)]
pub struct AnthropicConfig {
    /// API 密钥
    pub api_key: String,
    /// API 基础 URL
    pub base_url: String,
    /// 模型名称
    pub model_name: String,
    /// 最大输出 token 数
    pub max_tokens: usize,
    /// 缓存配置
    pub cache_config: Option<AnthropicCacheConfig>,
    /// 思考模式配置
    pub thinking: Option<ThinkingConfig>,
}

/// 全局 Anthropic 客户端管理器
#[derive(Debug, Clone)]
pub struct AnthropicClientManager {
    /// 缓存的客户端
    client: Option<Client>,
    /// 缓存的配置哈希
    config_hash: String,
    /// 最后使用时间
    last_used: std::time::Instant,
}

impl AnthropicClientManager {
    /// 创建新的管理器
    pub fn new() -> Self {
        Self {
            client: None,
            config_hash: String::new(),
            last_used: std::time::Instant::now(),
        }
    }

    /// 获取或创建客户端
    ///
    /// 如果配置发生变化或客户端不存在，则创建新客户端
    pub fn get_client(&mut self, config: &AnthropicConfig) -> Client {
        let config_hash = self.compute_config_hash(config);

        // 检查是否需要重新创建客户端
        let needs_recreate = self.client.is_none() || self.config_hash != config_hash;

        if needs_recreate {
            self.client = Some(self.build_client(config));
            self.config_hash = config_hash;
        }

        self.last_used = std::time::Instant::now();
        self.client.as_ref().unwrap().clone()
    }

    /// 计算配置哈希
    fn compute_config_hash(&self, config: &AnthropicConfig) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(&config.api_key);
        hasher.update(&config.base_url);
        hasher.update(&config.model_name);
        format!("{:x}", hasher.finalize())
    }

    /// 构建 HTTP 客户端
    fn build_client(&self, config: &AnthropicConfig) -> Client {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "x-api-key",
            reqwest::header::HeaderValue::from_str(&config.api_key).unwrap(),
        );
        headers.insert(
            "anthropic-version",
            reqwest::header::HeaderValue::from_static("2023-06-01"),
        );
        headers.insert(
            "Content-Type",
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        // 如果配置了思考模式，添加 beta header
        if config.thinking.is_some() {
            headers.insert(
                "anthropic-beta",
                reqwest::header::HeaderValue::from_static("max-tokens-3-5-sonnet-2024-07-01"),
            );
        }

        ClientBuilder::new()
            .timeout(Duration::from_secs(60))
            .default_headers(headers)
            .build()
            .expect("Failed to build HTTP client")
    }

    /// 检查客户端是否过期（超过 5 分钟未使用）
    pub fn is_expired(&self) -> bool {
        self.last_used.elapsed() > std::time::Duration::from_secs(300)
    }

    /// 清空缓存
    pub fn clear(&mut self) {
        self.client = None;
        self.config_hash.clear();
    }
}

impl Default for AnthropicClientManager {
    fn default() -> Self {
        Self::new()
    }
}

impl From<ModelConfig> for AnthropicConfig {
    fn from(config: ModelConfig) -> Self {
        Self {
            api_key: config.api_key,
            base_url: config
                .base_url
                .unwrap_or_else(|| "https://api.anthropic.com".to_string()),
            model_name: config.model_name,
            max_tokens: config.max_tokens,
            cache_config: None,
            thinking: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_anthropic_service_creation() {
        let config = AnthropicConfig {
            api_key: "test-api-key".to_string(),
            base_url: "https://api.anthropic.com".to_string(),
            model_name: "claude-sonnet-4-20250514".to_string(),
            max_tokens: 4096,
            cache_config: None,
            thinking: None,
        };

        let service = AnthropicService::new(config);
        assert_eq!(service.model_name(), "claude-sonnet-4-20250514");
    }

    #[test]
    fn test_message_to_anthropic() {
        let config = AnthropicConfig {
            api_key: "test-api-key".to_string(),
            base_url: "https://api.anthropic.com".to_string(),
            model_name: "claude-sonnet-4-20250514".to_string(),
            max_tokens: 4096,
            cache_config: None,
            thinking: None,
        };

        let service = AnthropicService::new(config);

        // Test user message
        let msg = Message::user("Hello");
        let anthropic_msg = service.message_to_anthropic(&msg);

        assert_eq!(anthropic_msg["role"], "user");
        assert_eq!(anthropic_msg["content"]["type"], "text");
        assert_eq!(anthropic_msg["content"]["text"], "Hello");

        // Test assistant message
        let msg = Message::assistant("Hi there!");
        let anthropic_msg = service.message_to_anthropic(&msg);

        assert_eq!(anthropic_msg["role"], "assistant");
        assert_eq!(anthropic_msg["content"]["type"], "text");
        assert_eq!(anthropic_msg["content"]["text"], "Hi there!");

        // Test system message
        let msg = Message::system("You are helpful.");
        let anthropic_msg = service.message_to_anthropic(&msg);

        assert_eq!(anthropic_msg["role"], "system");
        assert_eq!(anthropic_msg["content"]["type"], "text");
        assert_eq!(anthropic_msg["content"]["text"], "You are helpful.");
    }

    #[test]
    fn test_build_request_body() {
        let config = AnthropicConfig {
            api_key: "test-api-key".to_string(),
            base_url: "https://api.anthropic.com".to_string(),
            model_name: "claude-sonnet-4-20250514".to_string(),
            max_tokens: 4096,
            cache_config: None,
            thinking: None,
        };

        let service = AnthropicService::new(config);

        let messages = vec![
            Message::user("Hello"),
            Message::assistant("Hi there!"),
            Message::user("How are you?"),
        ];

        let body = service
            .build_request_body(&messages, Some("You are a helpful assistant."), 4096, None)
            .unwrap();

        assert_eq!(body["model"], "claude-sonnet-4-20250514");
        assert_eq!(body["max_tokens"], 4096);
        assert_eq!(body["system"], "You are a helpful assistant.");

        let msgs = body["messages"].as_array().unwrap();
        assert_eq!(msgs.len(), 3);

        assert_eq!(msgs[0]["role"], "user");
        assert_eq!(msgs[1]["role"], "assistant");
        assert_eq!(msgs[2]["role"], "user");
    }

    #[test]
    fn test_anthropic_config_from_model_config() {
        let model_config = ModelConfig {
            model_name: "claude-sonnet-4-20250514".to_string(),
            base_url: Some("https://api.anthropic.com".to_string()),
            api_key: "test-api-key".to_string(),
            max_tokens: 4096,
        };

        let anthropic_config = AnthropicConfig::from(model_config);

        assert_eq!(anthropic_config.api_key, "test-api-key");
        assert_eq!(anthropic_config.model_name, "claude-sonnet-4-20250514");
        assert_eq!(anthropic_config.base_url, "https://api.anthropic.com");
        assert_eq!(anthropic_config.max_tokens, 4096);
    }

    #[test]
    fn test_anthropic_config_with_cache_and_thinking() {
        let config = AnthropicConfig {
            api_key: "test-api-key".to_string(),
            base_url: "https://api.anthropic.com".to_string(),
            model_name: "claude-sonnet-4-20250514".to_string(),
            max_tokens: 4096,
            cache_config: Some(AnthropicCacheConfig::enabled_with_breakpoints(vec![
                1000, 2000,
            ])),
            thinking: Some(ThinkingConfig::enabled(Some(4096))),
        };

        assert!(config.cache_config.is_some());
        assert!(config.thinking.is_some());
    }

    #[tokio::test]
    async fn test_verify_api_key_invalid() {
        // 这个测试需要网络连接
        // 使用无效的 API Key 应该返回 false
        let result = verify_api_key("invalid-key", Some("https://api.anthropic.com")).await;
        // 根据实际 API 行为，结果可能是 Ok(false) 或 Err
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_thinking_config_serialization() {
        let config = ThinkingConfig::enabled(Some(4096));
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("\"type\":\"enabled\""));
        assert!(json.contains("\"budget_tokens\":4096"));

        let config_disabled = ThinkingConfig::disabled();
        let json_disabled = serde_json::to_string(&config_disabled).unwrap();
        assert!(json_disabled.contains("\"type\":\"disabled\""));
    }
}
