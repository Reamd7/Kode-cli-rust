//! Agent 存储模块
//!
//! 提供基于文件的 Agent 状态隔离存储。
//!
//! # 存储模式
//!
//! 文件命名模式: `${sessionId}-agent-${agentId}.json`
//! 存储位置: `~/.kode/`
//!
//! # 示例
//!
//! ```no_run
//! use kode_core::agent::storage;
//!
//! # async fn example() -> kode_core::error::Result<()> {
//! // 写入数据
//! storage::write_agent_data("my-agent", &serde_json::json!({
//!     "key": "value"
//! })).await?;
//!
//! // 读取数据
//! let data: serde_json::Value = storage::read_agent_data("my-agent")
//!     .await?
//!     .unwrap_or(serde_json::json!({}));
//! # Ok(())
//! # }
//! ```

use crate::error::{Error, Result};
use serde::{de::DeserializeOwned, Serialize};
use std::path::PathBuf;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

#[cfg(test)]
use serial_test::serial;

/// 获取 Kode 配置目录
///
/// 优先级:
/// 1. `KODE_CONFIG_DIR` 环境变量
/// 2. `ANYKODE_CONFIG_DIR` 环境变量
/// 3. `~/.kode/` (默认)
fn get_config_directory() -> Result<PathBuf> {
    if let Ok(dir) = std::env::var("KODE_CONFIG_DIR") {
        return Ok(PathBuf::from(dir));
    }

    if let Ok(dir) = std::env::var("ANYKODE_CONFIG_DIR") {
        return Ok(PathBuf::from(dir));
    }

    dirs::home_dir()
        .map(|h| h.join(".kode"))
        .ok_or_else(|| Error::ConfigError("Failed to determine home directory".to_string()))
}

/// 获取当前会话 ID
///
/// 从环境变量 `ANYKODE_SESSION_ID` 读取，如果不存在则返回 `"default-session"`。
fn get_session_id() -> String {
    std::env::var("ANYKODE_SESSION_ID").unwrap_or_else(|_| "default-session".to_string())
}

/// 生成 agent 文件路径
///
/// 文件命名模式: `${sessionId}-agent-${agentId}.json`
/// 存储位置: `~/.kode/`
///
/// # Arguments
///
/// * `agent_id` - Agent 标识符
///
/// # Returns
///
/// 返回完整的文件路径。
///
/// # Errors
///
/// 如果无法确定配置目录，返回错误。
///
/// # Examples
///
/// ```
/// use kode_core::agent::storage;
///
/// let path = storage::get_agent_file_path("my-agent").unwrap();
/// // 路径类似: ~/.kode/default-session-agent-my-agent.json
/// ```
pub fn get_agent_file_path(agent_id: &str) -> Result<PathBuf> {
    let session_id = get_session_id();
    let filename = format!("{}-agent-{}.json", session_id, agent_id);
    let config_dir = get_config_directory()?;

    Ok(config_dir.join(filename))
}

/// 读取 agent 特定数据
///
/// 从存储文件中读取并反序列化 JSON 数据。
///
/// # Type Parameters
///
/// * `T` - 目标类型，必须实现 `DeserializeOwned`
///
/// # Arguments
///
/// * `agent_id` - Agent 标识符
///
/// # Returns
///
/// - `Ok(Some(T))` - 文件存在且解析成功
/// - `Ok(None)` - 文件不存在
/// - `Err(Error)` - 读取或解析失败
///
/// # Examples
///
/// ```no_run
/// use kode_core::agent::storage;
/// use serde_json::Value;
///
/// # async fn example() -> kode_core::error::Result<()> {
/// let data: Option<Value> = storage::read_agent_data("my-agent").await?;
/// # Ok(())
/// # }
/// ```
pub async fn read_agent_data<T: DeserializeOwned>(agent_id: &str) -> Result<Option<T>> {
    let file_path = get_agent_file_path(agent_id)?;

    // 如果文件不存在，返回 None（不是错误）
    if !file_path.exists() {
        return Ok(None);
    }

    // 读取文件内容
    let content = fs::read_to_string(&file_path)
        .await
        .map_err(|e| Error::ConfigLoadError {
            path: file_path.clone(),
            message: format!("Failed to read agent data: {}", e),
        })?;

    // 解析 JSON
    let data = serde_json::from_str(&content).map_err(|e| Error::ConfigParseError {
        path: file_path,
        message: format!("Failed to parse agent data: {}", e),
    })?;

    Ok(Some(data))
}

/// 写入 agent 特定数据
///
/// 将数据序列化为 JSON 并写入存储文件。
/// 如果配置目录不存在，会自动创建。
///
/// # Type Parameters
///
/// * `T` - 数据类型，必须实现 `Serialize`
///
/// # Arguments
///
/// * `agent_id` - Agent 标识符
/// * `data` - 要写入的数据
///
/// # Errors
///
/// - 如果无法创建配置目录
/// - 如果写入文件失败
///
/// # Examples
///
/// ```no_run
/// use kode_core::agent::storage;
///
/// # async fn example() -> kode_core::error::Result<()> {
/// storage::write_agent_data("my-agent", &serde_json::json!({
///     "counter": 42
/// })).await?;
/// # Ok(())
/// # }
/// ```
pub async fn write_agent_data<T: Serialize>(agent_id: &str, data: &T) -> Result<()> {
    let file_path = get_agent_file_path(agent_id)?;
    let config_dir = get_config_directory()?;

    // 确保配置目录存在
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)
            .await
            .map_err(|e| Error::ConfigError(format!("Failed to create config directory: {}", e)))?;
    }

    // 序列化为 JSON
    let json = serde_json::to_string_pretty(data)
        .map_err(|e| Error::ConfigError(format!("Failed to serialize agent data: {}", e)))?;

    // 写入文件并 sync 确保数据落盘
    let mut file = fs::File::create(&file_path)
        .await
        .map_err(|e| Error::ConfigLoadError {
            path: file_path.clone(),
            message: format!("Failed to create agent data file: {}", e),
        })?;

    file.write_all(json.as_bytes())
        .await
        .map_err(|e| Error::ConfigLoadError {
            path: file_path.clone(),
            message: format!("Failed to write agent data: {}", e),
        })?;

    file.sync_all().await.map_err(|e| Error::ConfigLoadError {
        path: file_path,
        message: format!("Failed to sync agent data file: {}", e),
    })?;

    Ok(())
}

/// 获取默认 Agent ID
///
/// 返回 `"default"` 作为默认的 Agent 标识符。
///
/// # Examples
///
/// ```
/// use kode_core::agent::storage;
///
/// assert_eq!(storage::get_default_agent_id(), "default");
/// ```
pub fn get_default_agent_id() -> &'static str {
    "default"
}

/// 解析 Agent ID
///
/// 如果提供的 `agent_id` 为 `None` 或空字符串，则返回默认 Agent ID。
///
/// # Arguments
///
/// * `agent_id` - 可选的 Agent 标识符
///
/// # Returns
///
/// 返回有效的 Agent ID（如果输入为空，则返回默认值）。
///
/// # Examples
///
/// ```
/// use kode_core::agent::storage;
///
/// assert_eq!(storage::resolve_agent_id(Some("my-agent")), "my-agent");
/// assert_eq!(storage::resolve_agent_id(None), "default");
/// assert_eq!(storage::resolve_agent_id(Some("")), "default");
/// ```
pub fn resolve_agent_id(agent_id: Option<&str>) -> String {
    match agent_id {
        Some(id) if !id.is_empty() => id.to_string(),
        _ => get_default_agent_id().to_string(),
    }
}

/// 生成新的唯一 Agent ID
///
/// 使用 UUID v4 生成唯一的标识符。
///
/// # Returns
///
/// 返回 UUID 字符串。
///
/// # Examples
///
/// ```
/// use kode_core::agent::storage;
///
/// let id = storage::generate_agent_id();
/// // 每次调用生成不同的 UUID
/// assert!(id.len() > 0);
/// ```
pub fn generate_agent_id() -> String {
    Uuid::new_v4().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tempfile::TempDir;

    #[test]
    fn test_get_default_agent_id() {
        assert_eq!(get_default_agent_id(), "default");
    }

    #[test]
    fn test_resolve_agent_id() {
        assert_eq!(resolve_agent_id(Some("my-agent")), "my-agent");
        assert_eq!(resolve_agent_id(None), "default");
        assert_eq!(resolve_agent_id(Some("")), "default");
    }

    #[test]
    fn test_generate_agent_id() {
        let id1 = generate_agent_id();
        let id2 = generate_agent_id();
        assert_ne!(id1, id2);
        assert!(id1.len() > 0);
    }

    #[test]
    fn test_get_agent_file_path() {
        let path = get_agent_file_path("test-agent").unwrap();
        // 路径应包含 sessionId 和 agentId
        let path_str = path.to_string_lossy();
        assert!(path_str.contains("test-agent"));
        assert!(path_str.contains(".json"));
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_write_and_read_agent_data() {
        // 使用临时目录进行测试
        let temp_dir = TempDir::new().unwrap();
        // 使用唯一的 agent ID 避免测试间冲突
        let agent_id = "test-storage-agent-write-read";

        // 设置临时目录作为配置目录
        std::env::set_var("KODE_CONFIG_DIR", temp_dir.path());

        // 写入数据
        let data = json!({
            "counter": 42,
            "name": "test"
        });
        write_agent_data(agent_id, &data).await.unwrap();

        // 读取数据
        let read_data: Option<serde_json::Value> = read_agent_data(agent_id).await.unwrap();
        assert!(read_data.is_some());
        assert_eq!(read_data.unwrap(), data);

        // 清理
        std::env::remove_var("KODE_CONFIG_DIR");
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_read_nonexistent_agent_data() {
        let temp_dir = TempDir::new().unwrap();
        std::env::set_var("KODE_CONFIG_DIR", temp_dir.path());

        let result: Option<serde_json::Value> = read_agent_data("nonexistent-agent").await.unwrap();
        assert!(result.is_none());

        std::env::remove_var("KODE_CONFIG_DIR");
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_write_creates_config_directory() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().join("nested/config");
        std::env::set_var("KODE_CONFIG_DIR", &config_dir);

        // 目录不存在
        assert!(!config_dir.exists());

        // 写入数据应该自动创建目录
        write_agent_data("test-agent", &json!({"key": "value"}))
            .await
            .unwrap();

        // 目录应该被创建
        assert!(config_dir.exists());

        // 文件应该存在
        let file_path = config_dir.join(format!("{}-agent-test-agent.json", get_session_id()));
        assert!(file_path.exists());

        std::env::remove_var("KODE_CONFIG_DIR");
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_write_overwrites_existing_data() {
        let temp_dir = TempDir::new().unwrap();
        std::env::set_var("KODE_CONFIG_DIR", temp_dir.path());

        let agent_id = "overwrite-agent";

        // 写入初始数据
        write_agent_data(agent_id, &json!({"version": 1}))
            .await
            .unwrap();

        // 覆盖写入
        write_agent_data(agent_id, &json!({"version": 2, "updated": true}))
            .await
            .unwrap();

        // 验证最终数据
        let data: Option<serde_json::Value> = read_agent_data(agent_id).await.unwrap();
        assert!(data.is_some());
        assert_eq!(data.unwrap()["version"], 2);

        std::env::remove_var("KODE_CONFIG_DIR");
    }

    #[test]
    #[serial_test::serial]
    fn test_session_id_from_env() {
        // 默认 session
        std::env::remove_var("ANYKODE_SESSION_ID");
        assert_eq!(get_session_id(), "default-session");

        // 自定义 session
        std::env::set_var("ANYKODE_SESSION_ID", "my-session-123");
        assert_eq!(get_session_id(), "my-session-123");

        // 清理
        std::env::remove_var("ANYKODE_SESSION_ID");
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_structured_data_serialization() {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct TestStruct {
            count: i32,
            label: String,
        }

        let temp_dir = TempDir::new().unwrap();
        std::env::set_var("KODE_CONFIG_DIR", temp_dir.path());

        let data = TestStruct {
            count: 99,
            label: "test-label".to_string(),
        };

        write_agent_data("struct-agent-test", &data).await.unwrap();

        let read_data: Option<TestStruct> = read_agent_data("struct-agent-test").await.unwrap();
        assert!(read_data.is_some());
        assert_eq!(read_data.unwrap(), data);

        std::env::remove_var("KODE_CONFIG_DIR");
    }
}
