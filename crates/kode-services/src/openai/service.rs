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
use tracing::{debug, info, warn};

use crate::openai::{
    adapter::{apply_model_transformations, get_model_features, try_fix_request},
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

        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(60))
            .default_headers(headers)
            .build()
            .expect("Failed to build HTTP client");

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

    /// 获取 API 端点
    fn endpoint(&self) -> String {
        format!(
            "{}/chat/completions",
            self.config.base_url.trim_end_matches('/')
        )
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
        // 转换消息为 OpenAI 格式
        let mut openai_messages = Vec::new();

        // 添加系统提示词（如果提供）
        if let Some(system) = system_prompt {
            openai_messages.push(ChatMessage::System {
                content: system.to_string(),
            });
        }

        // 转换消息列表
        for msg in messages {
            openai_messages.push(self.message_to_openai(msg)?);
        }

        // 构建工具定义
        let openai_tools = tools.map(|tools| {
            tools
                .iter()
                .map(|tool| {
                    let tool_obj = tool.as_object().unwrap();
                    ToolDefinition {
                        tool_type: ToolType::Function,
                        function: FunctionDefinition {
                            name: tool_obj.get("name").unwrap().as_str().unwrap().to_string(),
                            description: tool_obj
                                .get("description")
                                .and_then(|d| d.as_str().map(String::from)),
                            parameters: tool_obj.get("input_schema").unwrap().clone(),
                        },
                    }
                })
                .collect()
        });

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

    /// 发送 HTTP 请求（带重试）
    async fn send_request_with_retry(
        &self,
        mut request_body: ChatCompletionRequest,
    ) -> std::result::Result<reqwest::Response, OpenAIError> {
        let max_retries = 10;
        let mut attempt = 0;

        loop {
            attempt += 1;

            debug!(
                "Sending request to {} (attempt {}/{})",
                self.endpoint(),
                attempt,
                max_retries
            );

            // 发送请求
            let response = self
                .client
                .post(self.endpoint())
                .json(&request_body)
                .send()
                .await
                .map_err(OpenAIError::RequestError)?;

            let status = response.status();

            // 检查是否成功
            if status.is_success() {
                return Ok(response);
            }

            // 读取错误消息
            let error_text = response.text().await.unwrap_or_default();

            // 检查是否为可修复的错误
            if attempt == 1 && crate::openai::adapter::detect_fixable_error(&error_text) {
                debug!("Detected fixable error: {}", error_text);
                if try_fix_request(&mut request_body, &error_text) {
                    debug!("Applied fix, retrying...");
                    continue;
                }
            }

            // 检查是否需要重试
            if attempt < max_retries
                && OpenAIError::from_response(status, &error_text).is_retryable()
            {
                let delay = calculate_retry_delay(attempt);
                debug!("Retryable error, waiting {}ms...", delay);
                tokio::time::sleep(Duration::from_millis(delay)).await;
                continue;
            }

            // 无法恢复的错误
            return Err(OpenAIError::from_response(status, &error_text));
        }
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

        Ok(ModelResponse {
            content: response_text,
            usage: token_usage,
            model: self.config.model_name.clone(),
            cost_usd: None, // TODO: 计算成本
        })
    }
}

/// 计算重试延迟（指数退避）
fn calculate_retry_delay(attempt: usize) -> u64 {
    let base_delay = 1000u64;
    let max_delay = 32000u64;
    let delay = base_delay * 2u64.pow(attempt as u32 - 1);
    std::cmp::min(delay, max_delay)
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
    fn test_calculate_retry_delay() {
        let delay1 = calculate_retry_delay(1);
        assert_eq!(delay1, 1000);

        let delay2 = calculate_retry_delay(2);
        assert_eq!(delay2, 2000);

        let delay5 = calculate_retry_delay(5);
        assert_eq!(delay5, 16000);
    }

    #[test]
    fn test_openai_service_creation() {
        use crate::openai::types;

        let config = types::OpenAIConfig {
            api_key: "test-key".to_string(),
            base_url: "https://api.openai.com/v1".to_string(),
            model_name: "gpt-4".to_string(),
            max_tokens: 4096,
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
}
