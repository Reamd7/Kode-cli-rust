//! 错误类型定义

use anyhow::Error;

/// Result type alias
pub type Result<T> = std::result::Result<T, Error>;
