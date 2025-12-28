//! OpenAI 服务核心实现
//!
//! 实现 OpenAI API 客户端和 ModelAdapter trait。

use async_trait::async_trait;
use futures::StreamExt;
use kode_core::{
    error::Result,
    message::{
        ContentBlock, Message, MessageContent, Role, TextBlock, ToolResultBlock, ToolUseBlock,
    },
    model::adapter::{ModelAdapter, ModelConfig, ModelResponse},
    model::streaming::StreamingResponse,
};
use reqwest::{Client, ClientBuilder};
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};

use crate::openai::{
    adapter::{
        apply_cached_fixes, apply_model_transformations, get_model_features, try_fix_request,
    },
    cache::SessionCache,
    pricing::calculate_cost,
    streaming::SseStreamProcessor,
    types::{
        ChatCompletionRequest, ChatMessage, FunctionCall, FunctionDefinition, StreamOptions,
        ToolCall, ToolChoice, ToolDefinition, ToolType, Usage,
    },
    OpenAIError, OpenAIModelFeatures,
};

/// OpenAI 服务配置
#[derive(Debug, Clone)]
struct Config {
    /// API 密钥
    api_key: String,
    /// API 基础 URL
    base_url: String,
    /// 模型名称
    model_name: String,
    /// 最大输出 token 数
    #[allow(dead_code)]
    max_tokens: usize,
}

/// OpenAI 服务
///
/// 实现与 OpenAI API 及其兼容服务的交互。
#[derive(Debug, Clone)]
pub struct OpenAIService {
    /// HTTP 客户端
    client: Client,
    /// 服务配置
    config: Config,
    /// 模型特性
    model_features: OpenAIModelFeatures,
    /// Session 缓存（用于缓存已知错误和应用的修复）
    session_cache: SessionCache,
    /// 取消令牌（用于取消长时间运行的请求）
    cancellation_token: Option<CancellationToken>,
}

impl OpenAIService {
    /// 创建新的 OpenAI 服务实例
    ///
    /// # Arguments
    ///
    /// * `config` - OpenAI 配置
    pub fn new(config: super::types::OpenAIConfig) -> Self {
        // 检测模型特性
        let model_features = get_model_features(&config.model_name);

        // 构建带有默认 headers 的 HTTP 客户端
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", config.api_key)
                .parse()
                .expect("Invalid auth header"),
        );
        headers.insert(
            "Content-Type",
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        // 构建客户端构建器
        let mut client_builder = ClientBuilder::new()
            .timeout(Duration::from_secs(60))
            .default_headers(headers);

        // 配置代理
        if let Some(proxy_url) = &config.proxy {
            info!("Configuring proxy: {}", proxy_url);
            match reqwest::Proxy::all(proxy_url) {
                Ok(proxy) => {
                    client_builder = client_builder.proxy(proxy);
                    info!("Successfully configured proxy: {}", proxy_url);
                }
                Err(e) => {
                    warn!("Failed to configure proxy '{}': {}", proxy_url, e);
                    // 如果配置失败，尝试使用环境变量
                    let env_proxy_url = std::env::var("HTTP_PROXY")
                        .or_else(|_| std::env::var("HTTPS_PROXY"))
                        .unwrap_or_default();
                    if let Ok(env_proxy) = reqwest::Proxy::all(env_proxy_url.as_str()) {
                        client_builder = client_builder.proxy(env_proxy);
                        info!("Using proxy from environment variable");
                    }
                }
            }
        } else {
            // 如果没有显式配置代理，尝试使用环境变量
            if let Ok(proxy_url) =
                std::env::var("HTTP_PROXY").or_else(|_| std::env::var("HTTPS_PROXY"))
            {
                info!("Detected proxy from environment: {}", proxy_url);
                if let Ok(env_proxy) = reqwest::Proxy::all(proxy_url.as_str()) {
                    client_builder = client_builder.proxy(env_proxy);
                    info!("Using proxy from environment variable");
                }
            }
        }

        let client = client_builder.build().expect("Failed to build HTTP client");

        info!(
            "Created OpenAI service for model {} (base_url: {})",
            config.model_name, config.base_url
        );

        Self {
            client,
            config: Config {
                api_key: config.api_key,
                base_url: config.base_url,
                model_name: config.model_name,
                max_tokens: config.max_tokens,
            },
            model_features,
            session_cache: SessionCache::new(),
            cancellation_token: None,
        }
    }

    /// 从 ModelConfig 创建 OpenAIService
    ///
    /// # Arguments
    ///
    /// * `config` - 模型配置
    pub fn from_model_config(config: ModelConfig) -> Self {
        Self::new(super::types::OpenAIConfig {
            api_key: config.api_key,
            base_url: config
                .base_url
                .unwrap_or_else(|| "https://api.openai.com/v1".to_string()),
            model_name: config.model_name,
            max_tokens: config.max_tokens,
            proxy: None,
        })
    }

    /// 获取基础 URL
    pub fn base_url(&self) -> &str {
        &self.config.base_url
    }

    /// 获取模型名称
    pub fn model_name(&self) -> &str {
        &self.config.model_name
    }

    /// 设置取消令牌
    ///
    /// # Arguments
    ///
    /// * `token` - CancellationToken 实例
    pub fn set_cancellation_token(&mut self, token: CancellationToken) {
        self.cancellation_token = Some(token);
    }

    /// 检查是否已取消
    ///
    /// 如果操作已被取消，返回错误
    #[allow(dead_code)]
    fn check_cancelled(&self) -> std::result::Result<(), OpenAIError> {
        if let Some(token) = &self.cancellation_token {
            if token.is_cancelled() {
                warn!("Request was cancelled");
                return Err(OpenAIError::Cancelled);
            }
        }
        Ok(())
    }

    /// 获取 API 端点
    fn endpoint(&self) -> String {
        format!(
            "{}/chat/completions",
            self.config.base_url.trim_end_matches('/')
        )
    }

    /// 获取备用端点列表（用于端点回退）
    ///
    /// 某些提供商（如 MiniMax）可能需要尝试多个端点
    fn fallback_endpoints(&self) -> Vec<String> {
        let base = self.config.base_url.trim_end_matches('/');

        // MiniMax 特殊处理
        if base.contains("minimax") {
            vec![
                format!("{}/chat/completions", base),       // 标准 OpenAI 格式
                format!("{}/text/chatcompletion_v2", base), // MiniMax 旧格式
            ]
        } else {
            vec![] // 默认无备用端点
        }
    }

    /// 构建请求体
    fn build_request_body(
        &self,
        messages: &[Message],
        system_prompt: Option<&str>,
        max_tokens: usize,
        tools: Option<&[serde_json::Value]>,
        stream: bool,
    ) -> std::result::Result<ChatCompletionRequest, OpenAIError> {
        // 构建工具定义（带描述截断），并收集被截断的指令
        const MAX_DESCRIPTION_LENGTH: usize = 1024;
        let mut additional_tool_instructions = Vec::new();

        let openai_tools = tools.map(|tools| {
            tools
                .iter()
                .map(|tool| {
                    let tool_obj = tool.as_object().unwrap();
                    let tool_name = tool_obj.get("name").unwrap().as_str().unwrap().to_string();
                    let original_description = tool_obj
                        .get("description")
                        .and_then(|d| d.as_str().map(String::from));

                    // 检查描述长度并截断
                    let description = if let Some(desc) = &original_description {
                        if desc.len() > MAX_DESCRIPTION_LENGTH {
                            warn!(
                                "Tool '{}' description exceeds {} chars, truncating",
                                tool_name, MAX_DESCRIPTION_LENGTH
                            );

                            // 截断描述
                            let truncated = &desc[..MAX_DESCRIPTION_LENGTH];
                            let remaining = &desc[MAX_DESCRIPTION_LENGTH..];

                            // 将截断的部分收集为额外指令
                            let extra_instructions =
                                format!("Tool '{}':\n{}", tool_name, remaining);
                            additional_tool_instructions.push(extra_instructions);

                            Some(truncated.to_string())
                        } else {
                            original_description.clone()
                        }
                    } else {
                        None
                    };

                    ToolDefinition {
                        tool_type: ToolType::Function,
                        function: FunctionDefinition {
                            name: tool_name,
                            description,
                            parameters: tool_obj.get("input_schema").unwrap().clone(),
                        },
                    }
                })
                .collect()
        });

        // 转换消息为 OpenAI 格式
        let mut openai_messages = Vec::new();

        // 构建系统提示词（包含额外的工具指令）
        let system_content = if let Some(system) = system_prompt {
            if !additional_tool_instructions.is_empty() {
                // 将额外的工具指令添加到系统提示词
                let additional = additional_tool_instructions.join("\n\n");
                format!(
                    "{}\n\n<additional-tool-usage-instructions>\n{}</additional-tool-usage-instructions>",
                    system, additional
                )
            } else {
                system.to_string()
            }
        } else if !additional_tool_instructions.is_empty() {
            // 如果没有系统提示词但有额外指令，创建一个默认的系统提示词
            let additional = additional_tool_instructions.join("\n\n");
            format!(
                "<additional-tool-usage-instructions>\n{}</additional-tool-usage-instructions>",
                additional
            )
        } else {
            // 既没有系统提示词也没有额外指令，不添加系统消息
            String::new()
        };

        // 添加系统提示词（如果有内容）
        if !system_content.is_empty() {
            openai_messages.push(ChatMessage::System {
                content: system_content,
            });
        }

        // 转换消息列表
        for msg in messages {
            openai_messages.push(self.message_to_openai(msg)?);
        }

        // 构建请求
        let has_tools = tools.is_some_and(|t| !t.is_empty());
        let mut request = ChatCompletionRequest {
            model: self.config.model_name.clone(),
            messages: openai_messages,
            max_tokens: if !self.model_features.uses_max_completion_tokens {
                Some(max_tokens)
            } else {
                None
            },
            max_completion_tokens: if self.model_features.uses_max_completion_tokens {
                Some(max_tokens)
            } else {
                None
            },
            temperature: if self.model_features.requires_temperature_one {
                Some(1.0)
            } else {
                None
            },
            tools: openai_tools,
            tool_choice: if has_tools {
                Some(ToolChoice::Auto)
            } else {
                None
            },
            stream: Some(stream),
            stream_options: if stream {
                Some(StreamOptions {
                    include_usage: true,
                })
            } else {
                None
            },
        };

        // 应用模型特定的参数转换
        apply_model_transformations(&mut request, &self.model_features);

        // 预应用已知的修复（基于缓存）
        apply_cached_fixes(&mut request, &self.session_cache);

        Ok(request)
    }

    /// 将 Message 转换为 OpenAI 格式
    fn message_to_openai(
        &self,
        message: &Message,
    ) -> std::result::Result<ChatMessage, OpenAIError> {
        let role = match message.role {
            Role::User => "user",
            Role::Assistant => "assistant",
            Role::System => "system",
        };

        match &message.content {
            MessageContent::Text(text) => {
                if role == "assistant" {
                    Ok(ChatMessage::Assistant {
                        content: Some(text.clone()),
                        tool_calls: None,
                        tool_call_id: None,
                    })
                } else {
                    Ok(ChatMessage::User {
                        content: text.clone().into(),
                    })
                }
            }
            MessageContent::Blocks(blocks) => {
                let mut tool_calls = Vec::new();
                let mut text_content = String::new();

                for block in blocks {
                    match block {
                        ContentBlock::Text(TextBlock { text }) => {
                            text_content.push_str(text);
                        }
                        ContentBlock::ToolUse(ToolUseBlock {
                            tool_use_id,
                            tool_name,
                            parameters,
                        }) => {
                            tool_calls.push(ToolCall {
                                id: tool_use_id.clone(),
                                call_type: ToolType::Function,
                                function: FunctionCall {
                                    name: tool_name.clone(),
                                    arguments: serde_json::to_string(parameters)
                                        .map_err(OpenAIError::JsonError)?,
                                },
                            });
                        }
                        ContentBlock::ToolResult(ToolResultBlock {
                            tool_use_id,
                            content,
                            is_error: _,
                        }) => {
                            // Tool 结果作为单独的消息
                            return Ok(ChatMessage::Tool {
                                tool_call_id: tool_use_id.clone(),
                                content: content.clone(),
                            });
                        }
                        ContentBlock::Image(_) => {
                            // 暂不支持图片
                            warn!("Image blocks not yet supported in OpenAI service");
                        }
                    }
                }

                if role == "assistant" && !tool_calls.is_empty() {
                    Ok(ChatMessage::Assistant {
                        content: if text_content.is_empty() {
                            None
                        } else {
                            Some(text_content)
                        },
                        tool_calls: Some(tool_calls),
                        tool_call_id: None,
                    })
                } else {
                    Ok(ChatMessage::User {
                        content: if text_content.is_empty() {
                            "".into()
                        } else {
                            text_content.into()
                        },
                    })
                }
            }
        }
    }

    /// 发送 HTTP 请求（带重试和端点回退）
    async fn send_request_with_retry(
        &self,
        request_body: ChatCompletionRequest,
    ) -> std::result::Result<reqwest::Response, OpenAIError> {
        // 获取主端点和备用端点列表
        let primary_endpoint = self.endpoint();
        let fallback_endpoints = self.fallback_endpoints();

        // 如果有备用端点，尝试使用端点回退
        if !fallback_endpoints.is_empty() {
            return self
                .try_with_endpoint_fallback(request_body, &primary_endpoint, &fallback_endpoints)
                .await;
        }

        // 否则使用标准重试逻辑
        self.send_request_with_standard_retry(request_body, &primary_endpoint)
            .await
    }

    /// 尝试使用端点回退发送请求
    ///
    /// 依次尝试主端点和备用端点，直到成功或全部失败
    async fn try_with_endpoint_fallback(
        &self,
        request_body: ChatCompletionRequest,
        primary_endpoint: &str,
        fallback_endpoints: &[String],
    ) -> std::result::Result<reqwest::Response, OpenAIError> {
        let all_endpoints = std::iter::once(primary_endpoint)
            .chain(fallback_endpoints.iter().map(|s| s.as_str()))
            .collect::<Vec<_>>();

        for (index, endpoint) in all_endpoints.iter().enumerate() {
            if index > 0 {
                info!(
                    "Trying fallback endpoint {}/{}: {}",
                    index,
                    all_endpoints.len() - 1,
                    endpoint
                );
            }

            match self
                .send_request_with_standard_retry(request_body.clone(), endpoint)
                .await
            {
                Ok(response) => {
                    if index > 0 {
                        info!("Successfully connected to fallback endpoint: {}", endpoint);
                    }
                    return Ok(response);
                }
                Err(e) => {
                    // 如果是 404 错误，尝试下一个端点
                    if e.to_string().contains("404") || e.to_string().contains("Not Found") {
                        warn!(
                            "Endpoint {} returned 404, trying next endpoint...",
                            endpoint
                        );
                        continue;
                    }
                    // 其他错误直接返回
                    return Err(e);
                }
            }
        }

        // 所有端点都失败
        Err(OpenAIError::ConfigError(format!(
            "All endpoints failed: {}",
            all_endpoints.join(", ")
        )))
    }

    /// 标准重试逻辑（不带端点回退）
    async fn send_request_with_standard_retry(
        &self,
        mut request_body: ChatCompletionRequest,
        endpoint: &str,
    ) -> std::result::Result<reqwest::Response, OpenAIError> {
        let max_retries = 10;
        let mut attempt = 0;

        loop {
            attempt += 1;

            debug!(
                "Sending request to {} (attempt {}/{})",
                endpoint, attempt, max_retries
            );

            // 记录 API 调用详情
            self.debug_log_api_call(&request_body, endpoint);

            // 发送请求
            let response = self
                .client
                .post(endpoint)
                .json(&request_body)
                .send()
                .await
                .map_err(OpenAIError::RequestError)?;

            let status = response.status();

            // 检查是否成功
            if status.is_success() {
                return Ok(response);
            }

            // 在读取响应体之前，先提取 retry-after header
            let retry_after = Self::extract_retry_after(&response);

            // 读取错误消息
            let error_text = response.text().await.unwrap_or_default();

            // 记录 API 错误
            self.log_api_error(status, &error_text, endpoint);

            // 检查是否为可修复的错误
            if attempt == 1 && crate::openai::adapter::detect_fixable_error(&error_text) {
                debug!("Detected fixable error: {}", error_text);
                if try_fix_request(&mut request_body, &error_text, Some(&self.session_cache)) {
                    debug!("Applied fix, retrying...");
                    continue;
                }
            }

            // 检查是否需要重试
            if attempt < max_retries
                && OpenAIError::from_response(status, &error_text).is_retryable()
            {
                let delay = calculate_retry_delay_with_jitter(attempt, retry_after);
                debug!("Retryable error, waiting {}ms...", delay);
                abortable_delay(delay, self.cancellation_token.as_ref())
                    .await
                    .map_err(|e| match e {
                        OpenAIError::Cancelled => {
                            info!("Request cancelled during retry delay");
                            e
                        }
                        _ => e,
                    })?;
                continue;
            }

            // 无法恢复的错误
            return Err(OpenAIError::from_response(status, &error_text));
        }
    }

    /// 从响应中提取 retry-after header
    ///
    /// 返回秒数，如果 header 不存在或无效则返回 None
    fn extract_retry_after(response: &reqwest::Response) -> Option<u64> {
        response
            .headers()
            .get("retry-after")
            .and_then(|value| value.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok())
            .map(|seconds| seconds * 1000) // 转换为毫秒
    }

    /// 将 OpenAI 响应转换为 ModelResponse
    fn convert_response(
        &self,
        response_text: String,
        usage: Usage,
        _tool_calls: Vec<ToolCall>,
    ) -> std::result::Result<ModelResponse, OpenAIError> {
        let token_usage = kode_core::model::types::TokenUsage {
            input_tokens: usage.prompt_tokens,
            output_tokens: usage.completion_tokens,
            total_tokens: Some(usage.total_tokens),
            thinking_tokens: None,
        };

        // 计算成本
        let cost_usd = calculate_cost(
            &self.config.model_name,
            usage.prompt_tokens as u32,
            usage.completion_tokens as u32,
        );

        Ok(ModelResponse {
            content: response_text,
            usage: token_usage,
            model: self.config.model_name.clone(),
            cost_usd,
        })
    }

    /// 记录 API 调用详情（调试日志）
    ///
    /// # Arguments
    ///
    /// * `request` - 请求体
    /// * `endpoint` - API 端点
    fn debug_log_api_call(&self, request: &ChatCompletionRequest, endpoint: &str) {
        debug!(
            "API Call: {} - model={}, max_tokens={:?}, max_completion_tokens={:?}, temperature={:?}, tools_count={}",
            endpoint,
            request.model,
            request.max_tokens,
            request.max_completion_tokens,
            request.temperature,
            request.tools.as_ref().map(|t| t.len()).unwrap_or(0)
        );
    }

    /// 记录 API 错误（调试日志）
    ///
    /// # Arguments
    ///
    /// * `status` - HTTP 状态码
    /// * `error_text` - 错误消息
    /// * `endpoint` - API 端点
    fn log_api_error(&self, status: reqwest::StatusCode, error_text: &str, endpoint: &str) {
        error!(
            "API Error: {} - HTTP {}: {}",
            endpoint,
            status.as_u16(),
            error_text
        );
    }
}

/// 计算重试延迟（指数退避 + 随机抖动）
///
/// # Arguments
///
/// * `attempt` - 当前重试次数（从 1 开始）
/// * `retry_after_ms` - 可选的服务器建议延迟（毫秒）
///
/// # Returns
///
/// 延迟时间（毫秒）
async fn abortable_delay(
    delay_ms: u64,
    cancellation_token: Option<&CancellationToken>,
) -> std::result::Result<(), OpenAIError> {
    use tokio::time::{sleep, Duration};

    // 检查是否已取消
    if let Some(token) = cancellation_token {
        if token.is_cancelled() {
            return Err(OpenAIError::Cancelled);
        }
    }

    // 创建睡眠任务
    let sleep_future = sleep(Duration::from_millis(delay_ms));

    // 如果有取消令牌，使用 tokio::select! 宏等待取消或睡眠完成
    if let Some(token) = cancellation_token {
        let cancel_future = token.cancelled();

        tokio::select! {
            _ = sleep_future => Ok(()),
            _ = cancel_future => {
                warn!("Delay was cancelled after {}ms", delay_ms);
                Err(OpenAIError::Cancelled)
            }
        }
    } else {
        // 没有取消令牌，直接睡眠
        sleep_future.await;
        Ok(())
    }
}

/// 计算重试延迟（带指数退避和抖动）
///
/// # Arguments
///
/// * `attempt` - 当前尝试次数
/// * `retry_after_ms` - 服务器建议的等待时间（毫秒）
///
/// # Returns
///
/// 延迟时间（毫秒）
fn calculate_retry_delay_with_jitter(attempt: usize, retry_after_ms: Option<u64>) -> u64 {
    // 如果服务器提供了 retry-after，优先使用
    if let Some(server_delay) = retry_after_ms {
        debug!("Using server-provided retry-after: {}ms", server_delay);
        // 添加少量随机抖动（±10%）以避免惊群效应
        return add_jitter(server_delay, 0.1);
    }

    // 否则使用指数退避
    let base_delay = 1000u64;
    let max_delay = 32000u64;
    let delay = base_delay * 2u64.pow(attempt as u32 - 1);
    let delay = std::cmp::min(delay, max_delay);

    // 添加随机抖动（±10%）
    add_jitter(delay, 0.1)
}

/// 添加随机抖动
///
/// # Arguments
///
/// * `delay` - 基础延迟时间（毫秒）
/// * `jitter_ratio` - 抖动比例（例如 0.1 表示 ±10%）
///
/// # Returns
///
/// 带抖动的延迟时间
fn add_jitter(delay: u64, jitter_ratio: f64) -> u64 {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    // 计算抖动范围
    let jitter_range = (delay as f64 * jitter_ratio) as i64;
    let jitter = rng.gen_range(-jitter_range..=jitter_range);

    // 应用抖动并确保非负
    let new_delay = (delay as i64 + jitter).max(0) as u64;

    debug!(
        "Delay with jitter: base={}ms, jitter={}ms, final={}ms",
        delay, jitter, new_delay
    );

    new_delay
}

#[async_trait]
impl ModelAdapter for OpenAIService {
    async fn send_message(
        &self,
        messages: Vec<Message>,
        system_prompt: Option<String>,
        max_tokens: usize,
    ) -> Result<ModelResponse> {
        let request_body = self
            .build_request_body(&messages, system_prompt.as_deref(), max_tokens, None, false)
            .map_err(|e| kode_core::error::Error::ModelRequestError(e.to_string()))?;

        let response = self
            .send_request_with_retry(request_body)
            .await
            .map_err(|e| kode_core::error::Error::ModelRequestError(e.to_string()))?;

        let response_text = response
            .text()
            .await
            .map_err(OpenAIError::RequestError)
            .map_err(|e| kode_core::error::Error::ModelRequestError(e.to_string()))?;
        let completion: crate::openai::types::ChatCompletionResponse =
            serde_json::from_str(&response_text)
                .map_err(OpenAIError::JsonError)
                .map_err(|e| kode_core::error::Error::ModelRequestError(e.to_string()))?;

        let choice = completion.choices.first().unwrap();
        let content = match &choice.message {
            ChatMessage::Assistant { content: c, .. } => c.clone().unwrap_or_default(),
            _ => String::new(),
        };

        self.convert_response(content, completion.usage, vec![])
            .map_err(|e| kode_core::error::Error::ModelRequestError(e.to_string()))
    }

    async fn send_message_with_tools(
        &self,
        messages: Vec<Message>,
        system_prompt: Option<String>,
        max_tokens: usize,
        tools: &[serde_json::Value],
    ) -> Result<ModelResponse> {
        let request_body = self
            .build_request_body(
                &messages,
                system_prompt.as_deref(),
                max_tokens,
                Some(tools),
                false,
            )
            .map_err(|e| kode_core::error::Error::ModelRequestError(e.to_string()))?;

        let response = self
            .send_request_with_retry(request_body)
            .await
            .map_err(|e| kode_core::error::Error::ModelRequestError(e.to_string()))?;

        let response_text = response
            .text()
            .await
            .map_err(OpenAIError::RequestError)
            .map_err(|e| kode_core::error::Error::ModelRequestError(e.to_string()))?;
        let completion: crate::openai::types::ChatCompletionResponse =
            serde_json::from_str(&response_text)
                .map_err(OpenAIError::JsonError)
                .map_err(|e| kode_core::error::Error::ModelRequestError(e.to_string()))?;

        let choice = completion.choices.first().unwrap();
        let content = match &choice.message {
            ChatMessage::Assistant { content: c, .. } => c.clone().unwrap_or_default(),
            _ => String::new(),
        };
        let tool_calls = choice.tool_calls.clone();

        self.convert_response(content, completion.usage, tool_calls)
            .map_err(|e| kode_core::error::Error::ModelRequestError(e.to_string()))
    }

    async fn stream_message(
        &self,
        messages: Vec<Message>,
        system_prompt: Option<String>,
        max_tokens: usize,
    ) -> Result<StreamingResponse> {
        let request_body = self
            .build_request_body(&messages, system_prompt.as_deref(), max_tokens, None, true)
            .map_err(|e| kode_core::error::Error::ModelRequestError(e.to_string()))?;

        // 创建请求构建器用于 EventSource
        let request_builder = self
            .client
            .post(self.endpoint())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", self.config.api_key),
            )
            .json(&request_body);

        let event_source = reqwest_eventsource::EventSource::new(request_builder)
            .map_err(|e| OpenAIError::stream(e.to_string()))
            .map_err(|e| kode_core::error::Error::ModelRequestError(e.to_string()))?;

        let processor = SseStreamProcessor::new(event_source);

        // 转换错误类型
        let stream = processor.map(|result| match result {
            Ok(chunk) => Ok(chunk),
            Err(e) => Ok(kode_core::model::types::StreamChunk::error(e.to_string())),
        });

        Ok(StreamingResponse::new(Box::pin(stream)))
    }

    async fn stream_message_with_tools(
        &self,
        messages: Vec<Message>,
        system_prompt: Option<String>,
        max_tokens: usize,
        tools: &[serde_json::Value],
    ) -> Result<StreamingResponse> {
        let request_body = self
            .build_request_body(
                &messages,
                system_prompt.as_deref(),
                max_tokens,
                Some(tools),
                true,
            )
            .map_err(|e| kode_core::error::Error::ModelRequestError(e.to_string()))?;

        // 创建请求构建器用于 EventSource
        let request_builder = self
            .client
            .post(self.endpoint())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", self.config.api_key),
            )
            .json(&request_body);

        let event_source = reqwest_eventsource::EventSource::new(request_builder)
            .map_err(|e| OpenAIError::stream(e.to_string()))
            .map_err(|e| kode_core::error::Error::ModelRequestError(e.to_string()))?;

        let processor = SseStreamProcessor::new(event_source);

        let stream = processor.map(|result| match result {
            Ok(chunk) => Ok(chunk),
            Err(e) => Ok(kode_core::model::types::StreamChunk::error(e.to_string())),
        });

        Ok(StreamingResponse::new(Box::pin(stream)))
    }

    fn model_name(&self) -> &str {
        &self.config.model_name
    }

    fn supports_streaming(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_retry_delay_with_jitter() {
        // 测试基础延迟（不带 jitter，由于随机性，测试范围）
        let delay1 = calculate_retry_delay_with_jitter(1, None);
        // 1000ms ± 10% = 900-1100ms
        assert!(delay1 >= 900 && delay1 <= 1100);

        let delay2 = calculate_retry_delay_with_jitter(2, None);
        // 2000ms ± 10% = 1800-2200ms
        assert!(delay2 >= 1800 && delay2 <= 2200);

        let delay5 = calculate_retry_delay_with_jitter(5, None);
        // 16000ms ± 10% = 14400-17600ms
        assert!(delay5 >= 14400 && delay5 <= 17600);
    }

    #[test]
    fn test_retry_after_priority() {
        // 当服务器提供 retry-after 时，应该优先使用
        let server_suggested = 5000u64;
        let delay = calculate_retry_delay_with_jitter(1, Some(server_suggested));
        // 5000ms ± 10% = 4500-5500ms
        assert!(delay >= 4500 && delay <= 5500);
    }

    #[test]
    fn test_add_jitter() {
        // 测试抖动函数
        let delay = 1000u64;
        let jittered = add_jitter(delay, 0.1);
        // 应该在 900-1100 范围内
        assert!(jittered >= 900 && jittered <= 1100);
    }

    #[test]
    fn test_openai_service_creation() {
        use crate::openai::types;

        let config = types::OpenAIConfig {
            api_key: "test-key".to_string(),
            base_url: "https://api.openai.com/v1".to_string(),
            model_name: "gpt-4".to_string(),
            max_tokens: 4096,
            proxy: None,
        };

        let service = OpenAIService::new(config);
        assert_eq!(service.model_name(), "gpt-4");
    }

    #[test]
    fn test_model_features_detection() {
        let features = get_model_features("gpt-5");
        assert!(features.uses_max_completion_tokens);
        assert!(features.requires_temperature_one);

        let features = get_model_features("gpt-4");
        assert!(!features.uses_max_completion_tokens);
        assert!(!features.requires_temperature_one);
    }

    #[test]
    fn test_cancellation_token_not_set() {
        use crate::openai::types;

        let config = types::OpenAIConfig {
            api_key: "test-key".to_string(),
            base_url: "https://api.openai.com/v1".to_string(),
            model_name: "gpt-4".to_string(),
            max_tokens: 4096,
            proxy: None,
        };

        let service = OpenAIService::new(config);
        // 没有设置取消令牌，check_cancelled 应该成功
        assert!(service.check_cancelled().is_ok());
    }

    #[test]
    fn test_cancellation_token_not_cancelled() {
        use crate::openai::types;

        let config = types::OpenAIConfig {
            api_key: "test-key".to_string(),
            base_url: "https://api.openai.com/v1".to_string(),
            model_name: "gpt-4".to_string(),
            max_tokens: 4096,
            proxy: None,
        };

        let mut service = OpenAIService::new(config);
        let token = tokio_util::sync::CancellationToken::new();
        service.set_cancellation_token(token);

        // 未取消，check_cancelled 应该成功
        assert!(service.check_cancelled().is_ok());
    }

    #[tokio::test]
    async fn test_cancellation_token_cancelled() {
        use crate::openai::types;

        let config = types::OpenAIConfig {
            api_key: "test-key".to_string(),
            base_url: "https://api.openai.com/v1".to_string(),
            model_name: "gpt-4".to_string(),
            max_tokens: 4096,
            proxy: None,
        };

        let mut service = OpenAIService::new(config);
        let token = tokio_util::sync::CancellationToken::new();
        service.set_cancellation_token(token.clone());

        // 取消令牌
        token.cancel();

        // 已取消，check_cancelled 应该返回错误
        assert!(service.check_cancelled().is_err());
        if let Err(OpenAIError::Cancelled) = service.check_cancelled() {
            // 正确的错误类型
        } else {
            panic!("Expected Cancelled error");
        }
    }

    #[tokio::test]
    async fn test_abortable_delay_not_cancelled() {
        // 测试正常延迟（未取消）
        let start = std::time::Instant::now();
        let result = abortable_delay(100, None).await;
        assert!(result.is_ok());
        assert!(start.elapsed().as_millis() >= 100);
    }

    #[tokio::test]
    async fn test_abortable_delay_cancelled() {
        // 测试取消延迟
        let token = tokio_util::sync::CancellationToken::new();
        let token_clone = token.clone();

        // 在 50ms 后取消
        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            token_clone.cancel();
        });

        let start = std::time::Instant::now();
        let result = abortable_delay(200, Some(&token)).await;

        // 应该被取消
        assert!(result.is_err());
        if let Err(OpenAIError::Cancelled) = result {
            // 正确的错误类型
        } else {
            panic!("Expected Cancelled error");
        }

        // 应该在 50ms 左右被取消，而不是完整的 200ms
        let elapsed = start.elapsed().as_millis();
        assert!(elapsed < 150); // 应该远小于 200ms
    }

    #[tokio::test]
    async fn test_abortable_delay_already_cancelled() {
        // 测试在调用 abortable_delay 之前就已经取消的情况
        let token = tokio_util::sync::CancellationToken::new();
        token.cancel();

        let result = abortable_delay(100, Some(&token)).await;
        assert!(result.is_err());
        if let Err(OpenAIError::Cancelled) = result {
            // 正确的错误类型
        } else {
            panic!("Expected Cancelled error");
        }
    }
}
