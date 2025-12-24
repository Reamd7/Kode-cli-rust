//! 消息管理模块
//!
//! 提供消息定义和上下文管理功能。

pub mod types;

pub use types::{
    ContentBlock, ImageBlock, Message, MessageContent, Role, TextBlock, ToolResultBlock,
    ToolUseBlock,
};
