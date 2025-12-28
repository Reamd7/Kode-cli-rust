//! Kode Services Library
//!
//! 提供外部服务集成：API 客户端、MCP 客户端等。

#![warn(missing_docs)]
#![warn(clippy::all)]

// Anthropic API 服务
pub mod anthropic;

// VCR 录制/回放支持
pub mod vcr;

// Integration tests
#[cfg(test)]
mod integration_test;

// Re-export commonly used types
pub use anthropic::{AnthropicConfig, AnthropicError, AnthropicService};
pub use vcr::{Vcr, VcrBuilder, VcrConfig, VcrMode};
