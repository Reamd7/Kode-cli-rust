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
}
