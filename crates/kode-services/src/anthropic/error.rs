//! Anthropic API 错误类型
//!
//! 定义 Anthropic API 相关的错误类型。

use thiserror::Error;

/// Anthropic API 错误
///
/// 表示与 Anthropic API 交互时可能发生的错误。
#[derive(Error, Debug)]
pub enum AnthropicError {
    /// 请求失败
    #[error("Request failed: {0}")]
    RequestError(#[from] anyhow::Error),

    /// API 返回错误
    #[error("API error: {message} (type: {error_type})")]
    ApiError {
        /// 错误消息
        message: String,
        /// 错误类型
        error_type: String,
    },

    /// 认证失败
    #[error("Authentication failed: invalid API key")]
    AuthenticationError,

    /// 速率限制
    #[error("Rate limited: please try again later")]
    RateLimitError,

    /// 请求超时
    #[error("Request timed out")]
    TimeoutError,

    /// 响应解析失败
    #[error("Failed to parse response: {0}")]
    ParseError(String),

    /// 流式响应错误
    #[error("Stream error: {0}")]
    StreamError(String),

    /// 配置错误
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

impl From<AnthropicError> for kode_core::error::Error {
    fn from(err: AnthropicError) -> Self {
        match err {
            AnthropicError::RequestError(e) => {
                kode_core::error::Error::ModelRequestError(e.to_string())
            }
            AnthropicError::ApiError { message, .. } => {
                kode_core::error::Error::ModelRequestError(message)
            }
            AnthropicError::AuthenticationError => {
                kode_core::error::Error::ModelRequestError("Authentication failed".to_string())
            }
            AnthropicError::RateLimitError => {
                kode_core::error::Error::ModelRequestError("Rate limited".to_string())
            }
            AnthropicError::TimeoutError => {
                kode_core::error::Error::ModelRequestError("Request timed out".to_string())
            }
            AnthropicError::ParseError(msg) => kode_core::error::Error::ModelResponseError(msg),
            AnthropicError::StreamError(msg) => kode_core::error::Error::ModelStreamError(msg),
            AnthropicError::ConfigError(msg) => kode_core::error::Error::ConfigError(msg),
        }
    }
}

/// 检查错误是否应该重试
pub fn should_retry(error: &AnthropicError) -> bool {
    match error {
        AnthropicError::RateLimitError => true,
        AnthropicError::AuthenticationError => false,
        _ => true,
    }
}

/// 获取重试延迟时间
pub fn get_retry_delay(attempt: u32) -> u64 {
    // 指数退避：500ms * 2^attempt，最大 32 秒
    let delay = 500u64.saturating_mul(2_u64.pow(attempt));
    delay.min(32000)
}

impl From<reqwest::Response> for AnthropicError {
    fn from(response: reqwest::Response) -> Self {
        let status = response.status();
        Self::RequestError(anyhow::anyhow!("HTTP {}", status))
    }
}

impl From<reqwest::Error> for AnthropicError {
    fn from(error: reqwest::Error) -> Self {
        Self::RequestError(anyhow::anyhow!("{}", error))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_retry_connect_error() {
        // Test that the should_retry function handles errors correctly
        // Note: testing with actual reqwest errors is complex, so we test the logic
        // by checking that rate limit errors are retried
        assert!(should_retry(&AnthropicError::RateLimitError));
        assert!(!should_retry(&AnthropicError::AuthenticationError));
    }

    #[test]
    fn test_get_retry_delay() {
        // Test exponential backoff
        assert_eq!(get_retry_delay(0), 500);
        assert_eq!(get_retry_delay(1), 1000);
        assert_eq!(get_retry_delay(2), 2000);
        assert_eq!(get_retry_delay(3), 4000);
        assert_eq!(get_retry_delay(10), 32000); // Max cap
    }
}
