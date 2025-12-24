//! 模型抽象模块
//!
//! 提供模型适配器接口和模型管理功能。

pub mod adapter;
pub mod streaming;
pub mod types;

pub use adapter::{ModelAdapter, ModelConfig, ModelResponse};
pub use streaming::StreamingResponse;
pub use types::{StreamChunk, TokenUsage};
