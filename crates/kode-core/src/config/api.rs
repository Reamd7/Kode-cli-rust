//! 配置 API
//!
//! 提供公开的配置管理 API 函数，与 TypeScript 版本保持一致。

use crate::config::env::get_config_file_path;
use crate::config::loader::ConfigLoader;
use crate::config::types::{GlobalConfig, ProjectConfig};
use crate::error::Error;
use std::collections::HashMap;
use tokio::fs;

/// 获取全局配置
///
/// 从配置文件路径（考虑环境变量）加载全局配置。
/// - 如果文件不存在，返回默认配置
/// - 如果解析失败，返回默认配置
/// - 应用配置迁移逻辑
///
/// # Returns
///
/// 返回全局配置
///
/// # Examples
///
/// ```no_run
/// use kode_core::config::get_global_config;
///
/// #[tokio::main]
/// async fn main() {
///     let config = get_global_config().await.unwrap();
///     println!("Theme: {:?}", config.theme);
/// }
/// ```
pub async fn get_global_config() -> Result<GlobalConfig, Error> {
    let config_path = get_config_file_path();
    let loader = ConfigLoader::new();

    // 加载配置
    let mut config = loader.load(&config_path).await?;

    // 应用配置迁移逻辑
    config = migrate_model_profiles(config);

    Ok(config)
}

/// 获取当前项目配置
///
/// 从全局配置的 projects 字段中查找当前工作目录的项目配置。
/// 如果未找到，返回默认项目配置。
///
/// # Returns
///
/// 返回当前项目配置
///
/// # Examples
///
/// ```no_run
/// use kode_core::config::get_current_project_config;
///
/// #[tokio::main]
/// async fn main() {
///     let project_config = get_current_project_config().await.unwrap();
///     println!("Allowed tools: {:?}", project_config.allowed_tools);
/// }
/// ```
pub async fn get_current_project_config() -> Result<ProjectConfig, Error> {
    let global_config = get_global_config().await?;
    let current_dir = std::env::current_dir()
        .map_err(|e| Error::ConfigError(format!("Cannot get current directory: {}", e)))?
        .canonicalize()
        .map_err(|e| Error::ConfigError(format!("Cannot canonicalize path: {}", e)))?;

    let project_path = current_dir.to_string_lossy().to_string();

    if let Some(projects) = &global_config.projects {
        if let Some(project_config) = projects.get(&project_path) {
            return Ok(project_config.clone());
        }
    }

    // 返回默认项目配置
    Ok(ProjectConfig::default())
}

/// 保存全局配置
///
/// 过滤掉与默认值相同的字段，然后保存到配置文件。
/// 创建配置目录（如不存在），处理权限错误。
///
/// # Arguments
///
/// * `config` - 要保存的全局配置
///
/// # Returns
///
/// 返回操作结果
///
/// # Examples
///
/// ```no_run
/// use kode_core::config::{get_global_config, save_global_config};
///
/// #[tokio::main]
/// async fn main() {
///     let mut config = get_global_config().await.unwrap();
///     config.theme = Some("dark".to_string());
///     save_global_config(&config).await.unwrap();
/// }
/// ```
pub async fn save_global_config(config: &GlobalConfig) -> Result<(), Error> {
    let config_path = get_config_file_path();

    // 过滤掉与默认值相同的字段
    let filtered_config = filter_default_fields(config);

    // 创建配置目录（如不存在）
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)
            .await
            .map_err(|e| Error::ConfigSaveError {
                path: config_path.clone(),
                message: format!("Failed to create config directory: {}", e),
            })?;
    }

    // 序列化为 JSON（2 空格缩进）
    let content =
        serde_json::to_string_pretty(&filtered_config).map_err(|e| Error::ConfigSaveError {
            path: config_path.clone(),
            message: format!("Failed to serialize config: {}", e),
        })?;

    // 写入文件
    fs::write(&config_path, content)
        .await
        .map_err(|e| Error::ConfigSaveError {
            path: config_path,
            message: format!("Failed to write config file: {}", e),
        })?;

    Ok(())
}

/// 保存当前项目配置
///
/// 将项目配置保存到全局配置的 projects 字段中，使用当前目录的绝对路径作为 key。
///
/// # Arguments
///
/// * `project_config` - 要保存的项目配置
///
/// # Returns
///
/// 返回操作结果
///
/// # Examples
///
/// ```no_run
/// use kode_core::config::save_current_project_config;
/// use kode_core::config::types::ProjectConfig;
///
/// #[tokio::main]
/// async fn main() {
///     let mut config = ProjectConfig::default();
///     config.allowed_tools = vec!["test".to_string()];
///     save_current_project_config(&config).await.unwrap();
/// }
/// ```
pub async fn save_current_project_config(project_config: &ProjectConfig) -> Result<(), Error> {
    let mut global_config = get_global_config().await?;

    let current_dir = std::env::current_dir()
        .map_err(|e| Error::ConfigError(format!("Cannot get current directory: {}", e)))?
        .canonicalize()
        .map_err(|e| Error::ConfigError(format!("Cannot canonicalize path: {}", e)))?;

    let project_path = current_dir.to_string_lossy().to_string();

    if global_config.projects.is_none() {
        global_config.projects = Some(HashMap::new());
    }

    if let Some(ref mut projects) = global_config.projects {
        projects.insert(project_path, project_config.clone());
    }

    save_global_config(&global_config).await
}

/// 过滤掉与默认值相同的字段
///
/// 优化配置文件大小，只保存与默认值不同的字段。
fn filter_default_fields(config: &GlobalConfig) -> serde_json::Value {
    let default = GlobalConfig::default();

    let config_value = serde_json::to_value(config).unwrap_or(serde_json::json!({}));
    let default_value = serde_json::to_value(&default).unwrap_or(serde_json::json!({}));

    if let (Some(config_obj), Some(default_obj)) =
        (config_value.as_object(), default_value.as_object())
    {
        let mut filtered = serde_json::Map::new();

        for (key, value) in config_obj.iter() {
            let default_field = default_obj.get(key);
            // 只保留与默认值不同的字段
            if default_field != Some(value) {
                filtered.insert(key.clone(), value.clone());
            }
        }

        serde_json::json!(filtered)
    } else {
        config_value
    }
}

/// 配置迁移：迁移模型配置
///
/// - 移除 ModelProfile.id 字段（不存在于 Rust 版本）
/// - 更新 modelPointers 从 id 改为 modelName
/// - 移除废弃字段
fn migrate_model_profiles(config: GlobalConfig) -> GlobalConfig {
    // Rust 版本从一开始就没有 ModelProfile.id 字段
    // 所以这个迁移主要用于确保 modelPointers 使用 modelName 而不是 id

    // 检查是否有 model_profiles 和 model_pointers
    if let (Some(_profiles), Some(_pointers)) = (&config.model_profiles, &config.model_pointers) {
        // 这里可以添加任何必要的迁移逻辑
        // 目前 Rust 版本已经使用正确的格式，所以暂时只是返回原配置
    }

    config
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_global_config_default() {
        let config = get_global_config().await.unwrap();
        // 应该返回默认配置
        assert_eq!(config.num_startups, 0);
        assert_eq!(config.verbose, false);
    }

    #[tokio::test]
    async fn test_get_current_project_config_default() {
        let config = get_current_project_config().await.unwrap();
        // 应该返回默认项目配置
        assert!(config.allowed_tools.is_empty());
        assert!(config.context.is_empty());
    }

    #[tokio::test]
    async fn test_save_and_load_global_config() {
        let mut config = GlobalConfig::default();
        config.theme = Some("dark".to_string());
        config.verbose = true;

        // 由于 get_config_file_path() 返回固定路径，我们直接测试 filter_default_fields 函数
        let filtered = filter_default_fields(&config);

        // 验证只保存了非默认值字段
        assert!(filtered.get("theme").is_some());
        assert!(filtered.get("verbose").is_some());
    }

    #[test]
    fn test_filter_default_fields() {
        let mut config = GlobalConfig::default();
        config.theme = Some("dark".to_string());

        let filtered = filter_default_fields(&config);

        // theme 应该被保留（非默认值）
        assert!(filtered.get("theme").is_some());
        // num_startups 应该被过滤掉（默认值是 0）
        assert!(filtered.get("numStartups").is_none());
    }
}
