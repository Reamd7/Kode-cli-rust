//! Kode Core Library
//!
//! 提供核心功能：配置管理、Agent 系统、模型抽象、消息管理等。

#![warn(missing_docs)]
#![warn(clippy::all)]

/// 错误类型定义
pub mod error;

/// 配置管理模块
pub mod config;

/// Agent 系统模块
pub mod agent;

/// 模型抽象模块
pub mod model;

/// 消息管理模块
pub mod message;

/// 上下文管理模块
pub mod context;

// 重新导出常用类型
pub use error::Result;
