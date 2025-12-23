//! 配置管理模块
//!
//! 提供配置文件的加载、合并和管理功能。

/// 配置类型定义
pub mod types;

/// 配置加载器
pub mod loader;

/// 环境变量处理
pub mod env;

/// 公开 API 函数
pub mod api;

/// 模型系统 API
pub mod model;

/// CLI 工具函数
pub mod cli;

/// 工具函数
pub mod utils;

// 重新导出主要类型
pub use loader::ConfigLoader;
pub use types::GlobalConfig;
pub use types::ProjectConfig;

// 重新导出环境变量函数
pub use env::{get_config_file_path, get_openai_api_key, get_anthropic_api_key};

// 重新导出核心 API 函数
pub use api::{get_global_config, get_current_project_config, save_global_config, save_current_project_config};

// 重新导出模型系统函数
pub use model::{set_all_pointers_to_model, set_model_pointer};

// 重新导出 CLI 工具函数
pub use cli::{get_config_for_cli, set_config_for_cli, delete_config_for_cli, list_config_for_cli};

// 重新导出工具函数
pub use utils::{normalize_api_key, get_custom_api_key_status, is_auto_updater_disabled, get_or_create_user_id};
