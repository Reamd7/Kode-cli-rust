//! MCP (.mcprc) 文件支持
//!
//! 提供项目级 MCP 服务器配置功能。

use crate::config::types::McpServerConfig;
use crate::error::Error;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::fs;

/// 获取 .mcprc 配置
///
/// 读取当前目录的 .mcprc 文件，返回 MCP 服务器配置
/// 使用 memoize 优化（基于文件内容）
///
/// # Returns
///
/// 返回 MCP 服务器配置的 JSON 字符串
pub async fn get_mcprc_config() -> Result<String, Error> {
    let mcprc_path = get_mcprc_path()?;

    // 检查文件是否存在
    if !mcprc_path.exists() {
        let empty = serde_json::json!({});
        return Ok(serde_json::to_string(&empty)
            .map_err(|e| Error::ConfigError(format!("Failed to serialize: {}", e)))?);
    }

    // 读取文件内容（容忍读取错误）
    let content = fs::read_to_string(&mcprc_path).await
        .unwrap_or_else(|_| String::new());

    // 如果文件为空或读取失败，返回空配置
    if content.trim().is_empty() {
        let empty = serde_json::json!({});
        return Ok(serde_json::to_string(&empty)
            .map_err(|e| Error::ConfigError(format!("Failed to serialize: {}", e)))?);
    }

    // 解析 JSON（容忍解析错误）
    let config: serde_json::Value = serde_json::from_str(&content)
        .unwrap_or(serde_json::json!({}));

    Ok(serde_json::to_string(&config)
        .map_err(|e| Error::ConfigError(format!("Failed to serialize config: {}", e)))?)
}

/// 清空 .mcprc 配置（仅用于测试）
///
/// # Returns
///
/// 返回操作结果
#[cfg(test)]
pub async fn clear_mcprc_config_for_testing() -> Result<(), Error> {
    let mcprc_path = get_mcprc_path()?;

    // 如果文件存在，删除它
    if mcprc_path.exists() {
        fs::remove_file(&mcprc_path).await
            .map_err(|e| Error::ConfigError(format!("Failed to remove .mcprc file: {}", e)))?;
    }

    Ok(())
}

/// 添加 MCP 服务器配置（仅用于测试）
///
/// # Arguments
///
/// * `name` - 服务器名称
/// * `server_config` - 服务器配置
///
/// # Returns
///
/// 返回操作结果
#[cfg(test)]
pub async fn add_mcprc_server_for_testing(
    name: String,
    server_config: McpServerConfig,
) -> Result<(), Error> {
    let mut config: HashMap<String, McpServerConfig> = get_mcprc_config_hashmap().await?;

    config.insert(name, server_config);

    save_mcprc_config(&config).await?;

    Ok(())
}

/// 移除 MCP 服务器配置（仅用于测试）
///
/// # Arguments
///
/// * `name` - 服务器名称
///
/// # Returns
///
/// 返回操作结果
#[cfg(test)]
pub async fn remove_mcprc_server_for_testing(name: String) -> Result<(), Error> {
    let mut config: HashMap<String, McpServerConfig> = get_mcprc_config_hashmap().await?;

    if !config.contains_key(&name) {
        return Err(Error::ConfigError(format!(
            "MCP server '{}' not found in .mcprc config",
            name
        )));
    }

    config.remove(&name);

    save_mcprc_config(&config).await?;

    Ok(())
}

/// 获取 .mcprc 文件路径
fn get_mcprc_path() -> Result<PathBuf, Error> {
    let current_dir = std::env::current_dir()
        .map_err(|e| Error::ConfigError(format!("Cannot get current directory: {}", e)))?;

    Ok(current_dir.join(".mcprc"))
}

/// 获取 .mcprc 配置为 HashMap
async fn get_mcprc_config_hashmap() -> Result<HashMap<String, McpServerConfig>, Error> {
    let config_str = get_mcprc_config().await?;
    let config: serde_json::Value = serde_json::from_str(&config_str)
        .map_err(|e| Error::ConfigError(format!("Failed to parse config: {}", e)))?;

    if config.is_null() || !config.is_object() {
        return Ok(HashMap::new());
    }

    let config_map: HashMap<String, McpServerConfig> = serde_json::from_value(config)
        .map_err(|e| Error::ConfigError(format!("Failed to parse .mcprc config: {}", e)))?;

    Ok(config_map)
}

/// 保存 .mcprc 配置
async fn save_mcprc_config(config: &HashMap<String, McpServerConfig>) -> Result<(), Error> {
    let mcprc_path = get_mcprc_path()?;

    // 序列化为 JSON（2 空格缩进）
    let content = serde_json::to_string_pretty(config)
        .map_err(|e| Error::ConfigError(format!("Failed to serialize .mcprc config: {}", e)))?;

    // 写入文件
    fs::write(&mcprc_path, content).await
        .map_err(|e| Error::ConfigSaveError {
            path: mcprc_path,
            message: e.to_string(),
        })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_mcprc_config_empty() {
        // 只测试不修改文件系统的功能
        let result = get_mcprc_config().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_mcprc_path() {
        let path = get_mcprc_path();
        assert!(path.is_ok());
        let path = path.unwrap();
        assert!(path.ends_with(".mcprc"));
    }
}
