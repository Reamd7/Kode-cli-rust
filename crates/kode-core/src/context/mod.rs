//! 上下文管理模块
//!
//! 提供消息上下文窗口管理功能。

pub mod freshness;
pub mod manager;

pub use freshness::{FileFreshnessService, FileTimestamp, FreshnessStatus};
pub use manager::{
    MessageContextManager, MessagePriority, RecoveredFile, RetentionPreference, TokenCounter,
    TrimmingStrategy,
};
