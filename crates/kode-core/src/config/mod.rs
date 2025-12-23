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

/// GPT-5 支持
pub mod gpt5;

/// MCP 支持
pub mod mcp;

/// 配置迁移
pub mod migration;

// 重新导出主要类型
pub use loader::ConfigLoader;
pub use types::GlobalConfig;
pub use types::ProjectConfig;

// 重新导出环境变量函数
pub use env::{get_anthropic_api_key, get_config_file_path, get_openai_api_key};

// 重新导出核心 API 函数
pub use api::{
    get_current_project_config, get_global_config, save_current_project_config, save_global_config,
};

// 重新导出模型系统函数
pub use model::{set_all_pointers_to_model, set_model_pointer};

// 重新导出 CLI 工具函数
pub use cli::{delete_config_for_cli, get_config_for_cli, list_config_for_cli, set_config_for_cli};

// 重新导出工具函数
pub use utils::{
    check_has_trust_dialog_accepted, get_custom_api_key_status, get_or_create_user_id,
    is_auto_updater_disabled, normalize_api_key,
};

// 重新导出 GPT-5 支持函数
pub use gpt5::{
    create_gpt5_model_profile, get_gpt5_config_recommendations, is_gpt5_model_name,
    validate_and_repair_all_gpt5_profiles, validate_and_repair_gpt5_profile,
};

// 重新导出 MCP 支持函数
pub use mcp::get_mcprc_config;

// 测试专用函数（仅在测试时可用）
#[cfg(test)]
pub use mcp::{
    add_mcprc_server_for_testing, clear_mcprc_config_for_testing, remove_mcprc_server_for_testing,
};

// 重新导出配置迁移函数
pub use migration::{enable_configs, migrate_model_profiles_remove_id};
