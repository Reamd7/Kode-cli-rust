//! 配置管理模块
//!
//! 提供配置文件的加载、合并和管理功能。

/// 配置类型定义
pub mod types;

/// 配置加载器
pub mod loader;

// 重新导出主要类型
pub use loader::ConfigLoader;
pub use types::Config;
