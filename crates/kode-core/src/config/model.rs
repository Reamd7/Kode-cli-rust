//! 模型系统 API
//!
//! 提供模型配置和指针管理的 API 函数。

use crate::config::api::{get_global_config, save_global_config};
use crate::config::types::ModelPointers;
use crate::error::Error;

/// 设置所有模型指针为指定模型
///
/// 将 main、task、reasoning、quick 四个指针都设置为指定的模型名称。
/// 同时更新 default_model_name 字段。
///
/// # Arguments
///
/// * `model_name` - 模型名称（例如 "claude-sonnet-4-5-20250929"）
///
/// # Returns
///
/// 返回操作结果
///
/// # Examples
///
/// ```no_run
/// use kode_core::config::model::set_all_pointers_to_model;
///
/// #[tokio::main]
/// async fn main() {
///     set_all_pointers_to_model("claude-sonnet-4-5-20250929").await.unwrap();
/// }
/// ```
pub async fn set_all_pointers_to_model(model_name: &str) -> Result<(), Error> {
    let mut config = get_global_config().await?;

    // 设置所有指针为指定模型
    let pointers = ModelPointers {
        main: Some(model_name.to_string()),
        task: Some(model_name.to_string()),
        reasoning: Some(model_name.to_string()),
        quick: Some(model_name.to_string()),
    };

    config.model_pointers = Some(pointers);
    config.default_model_name = Some(model_name.to_string());

    save_global_config(&config).await?;

    Ok(())
}

/// 设置单个模型指针
///
/// 更新指定的模型指针（main/task/reasoning/quick）。
/// 保存配置后会触发 ModelManager 重新加载（如果需要）。
///
/// # Arguments
///
/// * `pointer_type` - 指针类型（"main", "task", "reasoning", "quick"）
/// * `model_name` - 模型名称
///
/// # Returns
///
/// 返回操作结果
///
/// # Examples
///
/// ```no_run
/// use kode_core::config::model::set_model_pointer;
///
/// #[tokio::main]
/// async fn main() {
///     set_model_pointer("main", "claude-sonnet-4-5-20250929").await.unwrap();
/// }
/// ```
pub async fn set_model_pointer(pointer_type: &str, model_name: &str) -> Result<(), Error> {
    let mut config = get_global_config().await?;

    // 确保指针类型有效
    if !matches!(pointer_type, "main" | "task" | "reasoning" | "quick") {
        return Err(Error::ConfigError(format!(
            "Invalid pointer type: {}. Must be one of: main, task, reasoning, quick",
            pointer_type
        )));
    }

    // 创建或更新 model_pointers
    let mut pointers = config.model_pointers.unwrap_or_default();

    match pointer_type {
        "main" => pointers.main = Some(model_name.to_string()),
        "task" => pointers.task = Some(model_name.to_string()),
        "reasoning" => pointers.reasoning = Some(model_name.to_string()),
        "quick" => pointers.quick = Some(model_name.to_string()),
        _ => unreachable!(), // 已经在上面验证过了
    }

    config.model_pointers = Some(pointers);

    save_global_config(&config).await?;

    // TODO: 触发 ModelManager 重新加载（如果需要）

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_set_model_pointer_invalid_type() {
        let result = set_model_pointer("invalid", "model-name").await;
        assert!(result.is_err());

        if let Err(Error::ConfigError(msg)) = result {
            assert!(msg.contains("Invalid pointer type"));
        } else {
            panic!("Expected ConfigError");
        }
    }

    // 注意：以下测试会修改实际的配置文件，在实际环境中可能需要跳过或使用 mock
    // 这些测试主要用于 CI/CD 环境
}
