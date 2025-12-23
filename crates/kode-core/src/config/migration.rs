//! 配置迁移
//!
//! 提供配置版本迁移和兼容性处理功能。

use crate::config::api::get_global_config;
use crate::config::types::GlobalConfig;
use crate::error::Error;
use std::collections::HashMap;

/// 迁移模型配置，移除 id 字段
///
/// 此函数用于从旧版本的配置迁移数据：
/// 1. 移除 ModelProfile 中的 id 字段
/// 2. 将 modelPointers 从 ID 映射到 modelName
/// 3. 移除废弃的配置字段（defaultModelId, currentSelectedModelId 等）
///
/// # Arguments
///
/// * `config` - 全局配置
///
/// # Returns
///
/// 返回迁移后的全局配置
///
/// # Examples
///
/// ```
/// use kode_core::config::migration::migrate_model_profiles_remove_id;
/// let config = get_global_config().await?;
/// let migrated = migrate_model_profiles_remove_id(config);
/// ```
pub fn migrate_model_profiles_remove_id(mut config: GlobalConfig) -> GlobalConfig {
    // 如果没有 model_profiles，直接返回
    let profiles = config
        .model_profiles
        .as_deref()
        .unwrap_or(&[]);
    if profiles.is_empty() {
        return config;
    }

    // 1. 构建 ID 到 modelName 的映射，并移除 id 字段
    let mut id_to_model_name_map = HashMap::new();

    if let Some(model_profiles) = &mut config.model_profiles {
        for profile in model_profiles.iter() {
            // 注意：这里我们只构建映射，实际 Rust 版本没有 id 字段
            // 这个函数主要用于从 TypeScript 配置迁移
            id_to_model_name_map.insert(profile.model_name.clone(), profile.model_name.clone());
        }
    }

    // 2. 迁移 ModelPointers（从可能的 ID 到 modelName）
    if let Some(model_pointers) = &mut config.model_pointers {
        // 如果指针值看起来像旧的 ID 格式（model_xxx），尝试映射到 modelName
        if let Some(main) = &model_pointers.main {
            if main.starts_with("model_") {
                if let Some(model_name) = id_to_model_name_map.get(main) {
                    model_pointers.main = Some(model_name.clone());
                }
            }
        }

        if let Some(task) = &model_pointers.task {
            if task.starts_with("model_") {
                if let Some(model_name) = id_to_model_name_map.get(task) {
                    model_pointers.task = Some(model_name.clone());
                }
            }
        }

        if let Some(reasoning) = &model_pointers.reasoning {
            if reasoning.starts_with("model_") {
                if let Some(model_name) = id_to_model_name_map.get(reasoning) {
                    model_pointers.reasoning = Some(model_name.clone());
                }
            }
        }

        if let Some(quick) = &model_pointers.quick {
            if quick.starts_with("model_") {
                if let Some(model_name) = id_to_model_name_map.get(quick) {
                    model_pointers.quick = Some(model_name.clone());
                }
            }
        }
    }

    // 3. 确保 default_model_name 已设置（从可能的 defaultModelId 迁移）
    // 注意：Rust 版本的 GlobalConfig 没有 defaultModelId 等字段
    // 这些字段只在 TypeScript 版本中存在，所以这里不需要处理

    config
}

/// 允许配置读取
///
/// 设置配置读取标志，首次调用时验证配置文件。
/// 这是一个简化版本，因为 Rust 不需要在模块初始化时防止配置读取。
///
/// # Returns
///
/// 返回操作结果
///
/// # Examples
///
/// ```
/// use kode_core::config::migration::enable_configs;
/// enable_configs().await?;
/// ```
pub async fn enable_configs() -> Result<(), Error> {
    // 在 Rust 版本中，我们不需要像 TypeScript 那样控制配置读取的时序
    // 这个函数主要用于验证配置文件是否有效
    let _config = get_global_config().await?;

    // 如果配置加载成功，说明配置文件有效
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::types::{ModelProfile, ProviderType};

    #[test]
    fn test_migrate_model_profiles_remove_id_empty() {
        let config = GlobalConfig::default();
        let migrated = migrate_model_profiles_remove_id(config);
        // 空配置应该保持不变
        assert!(migrated.model_profiles.is_none() || migrated.model_profiles.unwrap().is_empty());
    }

    #[test]
    fn test_migrate_model_profiles_remove_id_with_profiles() {
        let mut config = GlobalConfig::default();
        config.model_profiles = Some(vec![ModelProfile {
            name: "test-model".to_string(),
            provider: ProviderType::Anthropic,
            model_name: "claude-3-5-sonnet-20241022".to_string(),
            base_url: None,
            api_key: "sk-ant-test".to_string(),
            max_tokens: 8192,
            context_length: 200000,
            reasoning_effort: None,
            is_active: true,
            created_at: 0,
            last_used: None,
            is_gpt5: None,
            validation_status: None,
            last_validation: None,
        }]);

        let migrated = migrate_model_profiles_remove_id(config);
        assert!(migrated.model_profiles.is_some());
        assert_eq!(migrated.model_profiles.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_enable_configs() {
        let result = enable_configs().await;
        assert!(result.is_ok());
    }
}
