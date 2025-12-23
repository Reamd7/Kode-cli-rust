//! CLI 工具函数
//!
//! 提供命令行界面使用的配置管理函数。

use crate::config::api::{get_global_config, get_current_project_config, save_global_config, save_current_project_config};
use crate::config::types::{GlobalConfig, ProjectConfig};
use crate::error::Error;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

/// 全局配置键列表
///
/// 这些键可以在全局配置中通过 CLI 修改
const GLOBAL_CONFIG_KEYS: &[&str] = &[
    "theme",
    "verbose",
    "autoUpdaterStatus",
    "primaryProvider",
    "maxTokens",
    "stream",
    "proxy",
    "preferredNotifChannel",
];

/// 项目配置键列表
///
/// 这些键可以在项目配置中通过 CLI 修改
const PROJECT_CONFIG_KEYS: &[&str] = &[
    "allowedTools",
    "context",
    "history",
    "mcpContextUris",
    "enableArchitectTool",
];

/// 检查是否为全局配置键
///
/// # Arguments
///
/// * `key` - 配置键
///
/// # Returns
///
/// 返回是否为有效的全局配置键
pub fn is_global_config_key(key: &str) -> bool {
    GLOBAL_CONFIG_KEYS.contains(&key)
}

/// 检查是否为项目配置键
///
/// # Arguments
///
/// * `key` - 配置键
///
/// # Returns
///
/// 返回是否为有效的项目配置键
pub fn is_project_config_key(key: &str) -> bool {
    PROJECT_CONFIG_KEYS.contains(&key)
}

/// 获取配置值（CLI 使用）
///
/// 根据键获取配置值，支持全局和项目配置
///
/// # Arguments
///
/// * `key` - 配置键
/// * `global` - 是否读取全局配置
///
/// # Returns
///
/// 返回配置值的 JSON 字符串，或错误
///
/// # Examples
///
/// ```no_run
/// use kode_core::config::cli::get_config_for_cli;
///
/// #[tokio::main]
/// async fn main() {
///     let value = get_config_for_cli("theme", false).await.unwrap();
///     println!("Theme: {}", value);
/// }
/// ```
pub async fn get_config_for_cli(key: &str, global: bool) -> Result<String, Error> {
    if global {
        let config = get_global_config().await?;
        get_nested_value(&config, key)
    } else {
        let config = get_current_project_config().await?;
        get_nested_value(&config, key)
    }
}

/// 设置配置值（CLI 使用）
///
/// 根据键设置配置值，支持全局和项目配置
///
/// # Arguments
///
/// * `key` - 配置键
/// * `value` - 配置值（JSON 字符串）
/// * `global` - 是否设置全局配置
///
/// # Returns
///
/// 返回操作结果
pub async fn set_config_for_cli(key: &str, value: &str, global: bool) -> Result<(), Error> {
    if global {
        // 验证配置键
        if !is_global_config_key(key) {
            return Err(Error::ConfigError(format!(
                "Invalid global config key: {}. Valid keys are: {}",
                key,
                GLOBAL_CONFIG_KEYS.join(", ")
            )));
        }

        let mut config = get_global_config().await?;

        // 特殊处理 autoUpdaterStatus
        if key == "autoUpdaterStatus" {
            if !matches!(value, "disabled" | "enabled" | "noPermissions" | "notConfigured") {
                return Err(Error::ConfigError(format!(
                    "Invalid autoUpdaterStatus: {}. Must be one of: disabled, enabled, noPermissions, notConfigured",
                    value
                )));
            }
        }

        set_nested_value(&mut config, key, value)?;
        save_global_config(&config).await?;
    } else {
        // 验证配置键
        if !is_project_config_key(key) {
            return Err(Error::ConfigError(format!(
                "Invalid project config key: {}. Valid keys are: {}",
                key,
                PROJECT_CONFIG_KEYS.join(", ")
            )));
        }

        let mut config = get_current_project_config().await?;
        set_nested_value(&mut config, key, value)?;
        save_current_project_config(&config).await?;
    }

    Ok(())
}

/// 删除配置值（CLI 使用）
///
/// 根据键删除配置值，恢复为默认值
///
/// # Arguments
///
/// * `key` - 配置键
/// * `global` - 是否删除全局配置
///
/// # Returns
///
/// 返回操作结果
pub async fn delete_config_for_cli(key: &str, global: bool) -> Result<(), Error> {
    if global {
        let mut config = get_global_config().await?;
        delete_nested_value(&mut config, key)?;
        save_global_config(&config).await?;
    } else {
        let mut config = get_current_project_config().await?;
        delete_nested_value(&mut config, key)?;
        save_current_project_config(&config).await?;
    }

    Ok(())
}

/// 列出配置（CLI 使用）
///
/// 列出全局或项目配置的所有键值对
///
/// # Arguments
///
/// * `global` - 是否列出全局配置
///
/// # Returns
///
/// 返回配置的 JSON 字符串
pub async fn list_config_for_cli(global: bool) -> Result<String, Error> {
    if global {
        let config = get_global_config().await?;
        // 过滤出非默认值的字段
        let filtered = filter_default_fields_global(&config);
        Ok(serde_json::to_string_pretty(&filtered)
            .map_err(|e| Error::ConfigError(format!("Failed to serialize config: {}", e)))?)
    } else {
        let config = get_current_project_config().await?;
        let filtered = filter_default_fields_project(&config);
        Ok(serde_json::to_string_pretty(&filtered)
            .map_err(|e| Error::ConfigError(format!("Failed to serialize config: {}", e)))?)
    }
}

/// 从嵌套结构中获取值
fn get_nested_value<T: Serialize>(obj: &T, key: &str) -> Result<String, Error> {
    let value = serde_json::to_value(obj)
        .map_err(|e| Error::ConfigError(format!("Failed to serialize config: {}", e)))?;
    let keys: Vec<&str> = key.split('.').collect();

    let mut current = &value;
    for k in keys {
        match current.get(k) {
            Some(v) => current = v,
            None => return Ok("null".to_string()),
        }
    }

    Ok(serde_json::to_string(current)
        .map_err(|e| Error::ConfigError(format!("Failed to serialize value: {}", e)))?)
}

/// 在嵌套结构中设置值
fn set_nested_value<T: Serialize + DeserializeOwned>(obj: &mut T, key: &str, value_str: &str) -> Result<(), Error> {
    let mut value_map = serde_json::to_value(&*obj)
        .map_err(|e| Error::ConfigError(format!("Failed to serialize config: {}", e)))?;

    // 尝试解析为 JSON
    let value: Value = if let Ok(v) = serde_json::from_str(value_str) {
        v
    } else {
        // 如果不是 JSON，当作字符串处理
        Value::String(value_str.to_string())
    };

    let keys: Vec<&str> = key.split('.').collect();

    if keys.len() == 1 {
        value_map[keys[0]] = value;
    } else {
        // 处理嵌套键（如 "modelPointers.main"）
        let mut current = &mut value_map;
        for (i, k) in keys.iter().enumerate() {
            if i == keys.len() - 1 {
                current[*k] = value.clone();
            } else {
                if !current.as_object().map(|o| o.contains_key(*k)).unwrap_or(false) {
                    current[*k] = Value::Object(serde_json::Map::new());
                }
                current = &mut current[*k];
            }
        }
    }

    // 反序列化回原类型
    *obj = serde_json::from_value(value_map)
        .map_err(|e| Error::ConfigError(format!("Failed to deserialize config: {}", e)))?;
    Ok(())
}

/// 从嵌套结构中删除值
fn delete_nested_value<T: Serialize + DeserializeOwned>(obj: &mut T, key: &str) -> Result<(), Error> {
    let mut value_map = serde_json::to_value(&*obj)
        .map_err(|e| Error::ConfigError(format!("Failed to serialize config: {}", e)))?;
    let keys: Vec<&str> = key.split('.').collect();

    if keys.len() == 1 {
        value_map.as_object_mut()
            .map(|o| o.remove(keys[0]));
    } else {
        // 处理嵌套键
        let mut current = &mut value_map;
        for (i, k) in keys.iter().enumerate() {
            if i == keys.len() - 1 {
                current.as_object_mut()
                    .map(|o| o.remove(*k));
            } else {
                current = current.get_mut(*k).ok_or_else(|| {
                    Error::ConfigError(format!("Key not found: {}", key))
                })?;
            }
        }
    }

    *obj = serde_json::from_value(value_map)
        .map_err(|e| Error::ConfigError(format!("Failed to deserialize config: {}", e)))?;
    Ok(())
}

/// 过滤全局配置的默认字段
fn filter_default_fields_global(config: &GlobalConfig) -> Value {
    let default = GlobalConfig::default();
    let config_value = serde_json::to_value(config).unwrap_or(Value::Null);
    let default_value = serde_json::to_value(&default).unwrap_or(Value::Null);

    if let (Some(config_obj), Some(default_obj)) = (config_value.as_object(), default_value.as_object()) {
        let mut filtered = serde_json::Map::new();
        for (key, value) in config_obj.iter() {
            if default_obj.get(key) != Some(value) {
                filtered.insert(key.clone(), value.clone());
            }
        }
        Value::Object(filtered)
    } else {
        config_value
    }
}

/// 过滤项目配置的默认字段
fn filter_default_fields_project(config: &ProjectConfig) -> Value {
    let default = ProjectConfig::default();
    let config_value = serde_json::to_value(config).unwrap_or(Value::Null);
    let default_value = serde_json::to_value(&default).unwrap_or(Value::Null);

    if let (Some(config_obj), Some(default_obj)) = (config_value.as_object(), default_value.as_object()) {
        let mut filtered = serde_json::Map::new();
        for (key, value) in config_obj.iter() {
            if default_obj.get(key) != Some(value) {
                filtered.insert(key.clone(), value.clone());
            }
        }
        Value::Object(filtered)
    } else {
        config_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_global_config_key() {
        assert!(is_global_config_key("theme"));
        assert!(is_global_config_key("verbose"));
        assert!(!is_global_config_key("invalidKey"));
    }

    #[test]
    fn test_is_project_config_key() {
        assert!(is_project_config_key("allowedTools"));
        assert!(is_project_config_key("context"));
        assert!(!is_project_config_key("theme"));
    }

    #[tokio::test]
    async fn test_get_config_for_cli() {
        // 测试读取全局配置
        let result = get_config_for_cli("verbose", true).await;
        assert!(result.is_ok());

        let value = result.unwrap();
        assert!(value == "true" || value == "false");
    }

    #[tokio::test]
    async fn test_set_and_get_config() {
        // 设置主题
        let result = set_config_for_cli("theme", "\"dark\"", true).await;
        assert!(result.is_ok());

        // 读取主题
        let value = get_config_for_cli("theme", true).await.unwrap();
        assert!(value.contains("dark"));
    }

    #[tokio::test]
    async fn test_delete_config() {
        // 先设置
        let _ = set_config_for_cli("theme", "\"dark\"", true).await;

        // 删除
        let result = delete_config_for_cli("theme", true).await;
        assert!(result.is_ok());

        // 验证已恢复默认值
        let value = get_config_for_cli("theme", true).await.unwrap();
        // 默认值应该是 null
        assert!(value == "null");
    }

    #[tokio::test]
    async fn test_invalid_auto_updater_status() {
        let result = set_config_for_cli("autoUpdaterStatus", "invalid", true).await;
        assert!(result.is_err());
    }
}
