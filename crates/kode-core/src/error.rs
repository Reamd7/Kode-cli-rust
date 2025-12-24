//! 错误类型定义

use std::path::PathBuf;
use thiserror::Error;

/// Result type alias
pub type Result<T> = std::result::Result<T, Error>;

/// 错误类型
#[derive(Error, Debug)]
pub enum Error {
    /// 通用配置错误
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// 加载配置时出错
    #[error("Failed to load config from {path}: {message}")]
    ConfigLoadError {
        /// 配置文件路径
        path: PathBuf,
        /// 错误消息
        message: String,
    },

    /// 解析配置时出错
    #[error("Failed to parse config from {path}: {message}")]
    ConfigParseError {
        /// 配置文件路径
        path: PathBuf,
        /// 错误消息
        message: String,
    },

    /// 保存配置时出错
    #[error("Failed to save config to {path}: {message}")]
    ConfigSaveError {
        /// 配置文件路径
        path: PathBuf,
        /// 错误消息
        message: String,
    },

    /// Agent 未找到
    #[error("Agent '{0}' not found")]
    AgentNotFound(String),

    /// Agent 解析错误
    #[error("Failed to parse agent: {0}")]
    AgentParseError(String),

    // ========== 模型相关错误 ==========
    /// 模型 API 请求失败
    #[error("Model request failed: {0}")]
    ModelRequestError(String),

    /// 模型响应解析失败
    #[error("Failed to parse model response: {0}")]
    ModelResponseError(String),

    /// 模型流式响应错误
    #[error("Model stream error: {0}")]
    ModelStreamError(String),

    /// 模型未配置
    #[error("Model not configured: {0}")]
    ModelNotConfigured(String),
}
