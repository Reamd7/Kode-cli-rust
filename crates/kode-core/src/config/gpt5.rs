//! GPT-5 模型支持
//!
//! 提供 GPT-5 模型的特殊处理和验证功能。

use crate::config::api::{get_global_config, save_global_config};
use crate::config::types::{ModelProfile, ProviderType};
use crate::error::Error;
use serde_json;

/// 判断是否为 GPT-5 模型名称
///
/// 检查模型名称（小写转换）是否以 "gpt-5" 开头或包含 "gpt-5"
///
/// # Arguments
///
/// * `model_name` - 模型名称
///
/// # Returns
///
/// 返回是否为 GPT-5 模型
///
/// # Examples
///
/// ```
/// use kode_core::config::gpt5::is_gpt5_model_name;
///
/// assert!(is_gpt5_model_name("gpt-5"));
/// assert!(is_gpt5_model_name("gpt-5-turbo"));
/// assert!(is_gpt5_model_name("GPT-5-Mini"));
/// assert!(!is_gpt5_model_name("gpt-4"));
/// ```
pub fn is_gpt5_model_name(model_name: &str) -> bool {
    let lower = model_name.to_lowercase();
    lower.starts_with("gpt-5") || lower.contains("gpt-5")
}

/// 获取 GPT-5 配置推荐
///
/// 根据模型名称返回推荐的 GPT-5 配置
///
/// # Arguments
///
/// * `model_name` - 模型名称
///
/// # Returns
///
/// 返回推荐的模型配置（作为 JSON 字符串）
pub fn get_gpt5_config_recommendations(model_name: &str) -> Result<String, Error> {
    let lower = model_name.to_lowercase();

    // 根据不同的 GPT-5 版本返回不同的推荐配置
    let (context_length, max_tokens, reasoning_effort) = if lower.contains("nano") {
        (128000, 2048, "minimal")
    } else if lower.contains("mini") {
        (128000, 4096, "low")
    } else {
        (128000, 8192, "medium")
    };

    let recommendations = serde_json::json!({
        "context_length": context_length,
        "max_tokens": max_tokens,
        "reasoning_effort": reasoning_effort,
        "is_gpt5": true
    });

    Ok(serde_json::to_string(&recommendations)
        .map_err(|e| Error::ConfigError(format!("Failed to serialize recommendations: {}", e)))?)
}

/// 验证和修复 GPT-5 模型配置
///
/// 检查并修复 GPT-5 模型的配置参数
///
/// # Arguments
///
/// * `profile` - 模型配置
///
/// # Returns
///
/// 返回修复后的模型配置
pub fn validate_and_repair_gpt5_profile(mut profile: ModelProfile) -> Result<ModelProfile, Error> {
    if !is_gpt5_model_name(&profile.model_name) {
        return Ok(profile);
    }

    // 设置 is_gpt5 标志
    profile.is_gpt5 = Some(true);

    // 验证并修复 reasoning_effort
    if profile.reasoning_effort.is_none() {
        let lower = profile.model_name.to_lowercase();
        let effort = if lower.contains("nano") {
            Some("minimal".to_string())
        } else if lower.contains("mini") {
            Some("low".to_string())
        } else {
            Some("medium".to_string())
        };
        profile.reasoning_effort = effort;
    }

    // 验证 context_length（最小 128000）
    if profile.context_length < 128000 {
        profile.context_length = 128000;
    }

    // 验证 max_tokens
    let lower = profile.model_name.to_lowercase();
    let min_tokens = if lower.contains("nano") {
        2048
    } else if lower.contains("mini") {
        4096
    } else {
        4000
    };

    if profile.max_tokens < min_tokens {
        profile.max_tokens = min_tokens;
    }

    // 验证 provider（应为 openai/custom-openai/azure）
    match profile.provider {
        ProviderType::Openai | ProviderType::CustomOpenai | ProviderType::Azure => {
            // 正确的 provider
        }
        _ => {
            // 其他 provider 不支持 GPT-5，但不强制修改
        }
    }

    // 设置默认 base_url（如需要）
    if profile.base_url.is_none() && matches!(profile.provider, ProviderType::Custom | ProviderType::CustomOpenai) {
        profile.base_url = Some("https://api.openai.com/v1".to_string());
    }

    // 更新验证状态
    profile.validation_status = Some("valid".to_string());
    profile.last_validation = Some(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );

    Ok(profile)
}

/// 验证和修复所有 GPT-5 模型配置
///
/// 遍历所有模型配置，对 GPT-5 模型进行验证和修复
///
/// # Returns
///
/// 返回修复统计（repaired: 修复数量, total: 总数量）
pub async fn validate_and_repair_all_gpt5_profiles() -> Result<String, Error> {
    let mut config = get_global_config().await?;

    let profiles = config.model_profiles.as_mut().map(|p| p.as_mut_slice()).unwrap_or(&mut []);

    let mut repaired = 0;
    let total = profiles.len();

    for profile in profiles.iter_mut() {
        if is_gpt5_model_name(&profile.model_name) {
            let original = profile.clone();
            let repaired_profile = validate_and_repair_gpt5_profile(profile.clone())?;
            *profile = repaired_profile;

            // 检查是否有修改
            if profile.clone() != original {
                repaired += 1;
            }
        }
    }

    if repaired > 0 {
        save_global_config(&config).await?;
    }

    let result = serde_json::json!({
        "repaired": repaired,
        "total": total
    });

    Ok(serde_json::to_string(&result)
        .map_err(|e| Error::ConfigError(format!("Failed to serialize result: {}", e)))?)
}

/// 创建 GPT-5 模型配置
///
/// 根据提供的参数创建一个 GPT-5 模型配置
///
/// # Arguments
///
/// * `name` - 配置名称
/// * `model_name` - 模型名称（如 gpt-5, gpt-5-turbo）
/// * `api_key` - API 密钥
/// * `base_url` - API 基础 URL（可选）
/// * `provider` - 提供商类型
///
/// # Returns
///
/// 返回创建的模型配置
pub async fn create_gpt5_model_profile(
    name: String,
    model_name: String,
    api_key: String,
    base_url: Option<String>,
    provider: ProviderType,
) -> Result<ModelProfile, Error> {
    let mut profile = ModelProfile {
        name,
        model_name: model_name.clone(),
        provider,
        base_url,
        api_key,
        max_tokens: 8192,
        context_length: 128000,
        reasoning_effort: Some("medium".to_string()),
        is_active: true,
        created_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        last_used: None,
        is_gpt5: Some(true),
        validation_status: Some("created".to_string()),
        last_validation: Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        ),
    };

    // 应用 GPT-5 验证和修复
    profile = validate_and_repair_gpt5_profile(profile)?;

    Ok(profile)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::types::ProviderType;

    #[test]
    fn test_is_gpt5_model_name() {
        assert!(is_gpt5_model_name("gpt-5"));
        assert!(is_gpt5_model_name("gpt-5-turbo"));
        assert!(is_gpt5_model_name("GPT-5-Mini"));
        assert!(is_gpt5_model_name("gpt-5-pro"));
        assert!(!is_gpt5_model_name("gpt-4"));
        assert!(!is_gpt5_model_name("claude-3"));
    }

    #[test]
    fn test_get_gpt5_config_recommendations() {
        let result = get_gpt5_config_recommendations("gpt-5");
        assert!(result.is_ok());

        let config: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(config["context_length"], 128000);
        assert_eq!(config["max_tokens"], 8192);
        assert_eq!(config["reasoning_effort"], "medium");
        assert_eq!(config["is_gpt5"], true);
    }

    #[test]
    fn test_get_gpt5_mini_recommendations() {
        let result = get_gpt5_config_recommendations("gpt-5-mini");
        assert!(result.is_ok());

        let config: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(config["max_tokens"], 4096);
        assert_eq!(config["reasoning_effort"], "low");
    }

    #[test]
    fn test_validate_and_repair_gpt5_profile() {
        let mut profile = ModelProfile {
            name: "test-gpt5".to_string(),
            provider: ProviderType::Openai,
            model_name: "gpt-5".to_string(),
            base_url: None,
            api_key: "sk-test".to_string(),
            max_tokens: 1000, // 故意设置过小
            context_length: 1000, // 故意设置过小
            reasoning_effort: None,
            is_active: true,
            created_at: 0,
            last_used: None,
            is_gpt5: None,
            validation_status: None,
            last_validation: None,
        };

        let repaired = validate_and_repair_gpt5_profile(profile).unwrap();
        assert_eq!(repaired.is_gpt5, Some(true));
        assert_eq!(repaired.context_length, 128000); // 应该被修复
        assert_eq!(repaired.max_tokens, 4000); // 应该被修复到最小值
        assert_eq!(repaired.reasoning_effort, Some("medium".to_string())); // 应该被设置
        assert_eq!(repaired.validation_status, Some("valid".to_string()));
    }

    #[test]
    fn test_validate_non_gpt5_profile() {
        let profile = ModelProfile {
            name: "test-gpt4".to_string(),
            provider: ProviderType::Openai,
            model_name: "gpt-4".to_string(),
            base_url: None,
            api_key: "sk-test".to_string(),
            max_tokens: 1000,
            context_length: 8000,
            reasoning_effort: None,
            is_active: true,
            created_at: 0,
            last_used: None,
            is_gpt5: None,
            validation_status: None,
            last_validation: None,
        };

        let repaired = validate_and_repair_gpt5_profile(profile).unwrap();
        // GPT-4 模型不应该被修改
        assert_eq!(repaired.model_name, "gpt-4");
        assert_eq!(repaired.context_length, 8000);
    }
}
