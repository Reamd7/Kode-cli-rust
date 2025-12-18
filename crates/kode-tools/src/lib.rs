//! Kode Tools Library
//!
//! 提供工具系统：Tool trait、工具注册表、各种工具实现。

#![warn(missing_docs)]
#![warn(clippy::all)]

/// Tool trait 定义
pub mod tool;

/// 工具注册表
pub mod registry;

// 重新导出主要类型
pub use registry::ToolRegistry;
pub use tool::{Tool, ToolContext, ToolResult, ToolSchema};
