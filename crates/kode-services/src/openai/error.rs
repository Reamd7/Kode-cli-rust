//! OpenAI 错误类型定义
//!
//! 定义 OpenAI 服务特定的错误类型。

use thiserror::Error;

/// OpenAI 错误类型
///
/// 表示 OpenAI API 调用过程中可能发生的错误。
#[derive(Debug, Error)]
pub enum OpenAIError {
    /// HTTP 请求错误
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    /// JSON 序列化/反序列化错误
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// API 返回错误
    #[error("API error ({code:?}): {message}")]
    ApiError {
        /// 错误代码
        code: Option<String>,
        /// 错误消息
        message: String,
    },

    /// 流处理错误
    #[error("Stream processing error: {0}")]
    StreamError(String),

    /// 工具调用错误
    #[error("Tool call error: {0}")]
    ToolError(String),

    /// 配置错误
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// 认证错误
    #[error("Authentication failed: {0}")]
    AuthError(String),

    /// 速率限制错误
    #[error("Rate limit exceeded: {0}")]
    RateLimitError(String),

    /// 可修复的模型错误
    #[error("Fixable model error: {0}")]
    FixableModelError(String),
}

impl OpenAIError {
    /// 判断是否为可重试的错误
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::RequestError(_) | Self::RateLimitError(_) => true,
            Self::ApiError { code, .. } => {
                matches!(
                    code.as_deref(),
                    Some("429") | Some("500") | Some("502") | Some("503") | Some("504")
                )
            }
            _ => false,
        }
    }

    /// 判断是否为可修复的错误
    pub fn is_fixable(&self) -> bool {
        matches!(self, Self::FixableModelError(_))
    }

    /// 从 HTTP 响应创建 API 错误
    pub fn from_response(status: reqwest::StatusCode, text: &str) -> Self {
        if let Ok(err_resp) = serde_json::from_str::<super::types::OpenAIErrorResponse>(text) {
            Self::ApiError {
                code: Some(status.to_string()),
                message: err_resp.error.message,
            }
        } else {
            Self::ApiError {
                code: Some(status.to_string()),
                message: text.to_string(),
            }
        }
    }

    /// 创建速率限制错误
    pub fn rate_limit(message: impl Into<String>) -> Self {
        Self::RateLimitError(message.into())
    }

    /// 创建可修复错误
    pub fn fixable(message: impl Into<String>) -> Self {
        Self::FixableModelError(message.into())
    }

    /// 创建认证错误
    pub fn auth(message: impl Into<String>) -> Self {
        Self::AuthError(message.into())
    }

    /// 创建流处理错误
    pub fn stream(message: impl Into<String>) -> Self {
        Self::StreamError(message.into())
    }

    /// 创建工具调用错误
    pub fn tool(message: impl Into<String>) -> Self {
        Self::ToolError(message.into())
    }

    /// 创建配置错误
    pub fn config(message: impl Into<String>) -> Self {
        Self::ConfigError(message.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = OpenAIError::config("Invalid base URL");
        assert_eq!(err.to_string(), "Configuration error: Invalid base URL");
    }

    #[test]
    fn test_retryable_error() {
        let err = OpenAIError::RateLimitError("Too many requests".to_string());
        assert!(err.is_retryable());
    }

    #[test]
    fn test_fixable_error() {
        let err = OpenAIError::fixable("max_tokens not supported");
        assert!(err.is_fixable());
    }
}
