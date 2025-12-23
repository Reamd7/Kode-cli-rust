//! 工具函数
//!
//! 提供配置管理的辅助函数。

use crate::config::api::{get_global_config, save_global_config};
use crate::error::Error;

/// 规范化 API Key
///
/// 截取 API Key 的最后 20 个字符，用于安全显示
///
/// # Arguments
///
/// * `api_key` - API Key 字符串
///
/// # Returns
///
/// 返回规范化后的 API Key（最后 20 个字符）
///
/// # Examples
///
/// ```
/// use kode_core::config::utils::normalize_api_key;
///
/// let key = "sk-ant-api03-very-long-key-here";
/// let normalized = normalize_api_key(key);
/// assert_eq!(normalized.len(), 20);
/// ```
pub fn normalize_api_key(api_key: &str) -> String {
    if api_key.len() <= 20 {
        api_key.to_string()
    } else {
        api_key[api_key.len() - 20..].to_string()
    }
}

/// 获取自定义 API Key 状态
///
/// 检查 API key 是否在 approved 或 rejected 列表中
///
/// # Returns
///
/// 返回 'approved', 'rejected', 或 'new'
pub async fn get_custom_api_key_status() -> Result<String, Error> {
    let config = get_global_config().await?;

    if let Some(responses) = &config.custom_api_key_responses {
        // 这里需要一个实际的 API key 来检查，暂时返回 "new"
        return Ok("new".to_string());
    }

    Ok("new".to_string())
}

/// 检查自动更新是否禁用
///
/// # Returns
///
/// 返回自动更新是否被禁用
pub async fn is_auto_updater_disabled() -> Result<bool, Error> {
    let config = get_global_config().await?;

    if let Some(status) = &config.auto_updater_status {
        Ok(matches!(status, crate::config::types::AutoUpdaterStatus::Disabled))
    } else {
        Ok(false)
    }
}

/// 获取或创建用户 ID
///
/// 如果用户 ID 不存在，则生成一个新的并保存
///
/// # Returns
///
/// 返回用户 ID
pub async fn get_or_create_user_id() -> Result<String, Error> {
    let mut config = get_global_config().await?;

    if let Some(user_id) = &config.user_id {
        Ok(user_id.clone())
    } else {
        // 生成随机的 user_id (32 字节 hex)
        use rand::Rng;
        const USER_ID_LEN: usize = 32;
        let mut rng = rand::thread_rng();
        let user_id: String = (0..USER_ID_LEN)
            .map(|_| format!("{:02x}", rng.gen::<u8>()))
            .collect();

        config.user_id = Some(user_id.clone());
        save_global_config(&config).await?;

        Ok(user_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_api_key_short() {
        let key = "sk-short";
        assert_eq!(normalize_api_key(key), "sk-short");
    }

    #[test]
    fn test_normalize_api_key_long() {
        let key = "sk-ant-api03-very-long-api-key-here-12345678901234567890";
        let normalized = normalize_api_key(key);
        assert_eq!(normalized.len(), 20);
        // 应该返回最后 20 个字符
        assert_eq!(normalized, "12345678901234567890");
    }

    #[tokio::test]
    async fn test_is_auto_updater_disabled() {
        let result = is_auto_updater_disabled().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_or_create_user_id() {
        let user_id = get_or_create_user_id().await.unwrap();
        // 第二次调用应该返回相同的 ID
        let user_id2 = get_or_create_user_id().await.unwrap();
        assert_eq!(user_id, user_id2);
        assert!(!user_id.is_empty());
    }
}
