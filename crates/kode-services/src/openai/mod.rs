//! OpenAI-compatible API 服务实现
//!
//! 提供与 OpenAI API 及其兼容服务（DeepSeek、MiniMax 等）的客户端实现。

pub mod adapter;
pub mod error;
pub mod service;
pub mod streaming;
pub mod types;

pub use error::OpenAIError;
pub use service::OpenAIService;
pub use types::{OpenAIConfig, OpenAIModelFeatures};
