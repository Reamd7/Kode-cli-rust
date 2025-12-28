//! 模型参数适配逻辑
//!
//! 处理不同模型的参数差异，实现参数自适应转换。

use crate::openai::types::{ChatCompletionRequest, OpenAIModelFeatures};
use tracing::debug;

/// GPT-5/o1/o3 模型标识符模式
const GPT5_MODELS: &[&str] = &[
    "gpt-5",
    "gpt-5-mini",
    "gpt-5-nano",
    "gpt-5-chat-latest",
    "o1",
    "o1-preview",
    "o1-mini",
    "o1-pro",
    "o3-mini",
];

/// DeepSeek 模型标识符模式
const DEEPSEEK_MODELS: &[&str] = &["deepseek", "deepseek-coder", "deepseek-chat"];

/// 获取模型特性
///
/// 根据模型名称检测其支持的功能特性。
///
/// # Arguments
///
/// * `model_name` - 模型名称
///
/// # Returns
///
/// 模型特性标志
pub fn get_model_features(model_name: &str) -> OpenAIModelFeatures {
    let model_lower = model_name.to_lowercase();

    // GPT-5/o1/o3 系列特性
    if GPT5_MODELS.iter().any(|m| model_lower.contains(m)) {
        debug!("Detected GPT-5/o1 series model: {}", model_name);
        return OpenAIModelFeatures {
            uses_max_completion_tokens: true,
            requires_temperature_one: true,
            supports_responses_api: true,
            supports_custom_tools: true,
            supports_allowed_tools: true,
        };
    }

    // DeepSeek 特性
    if DEEPSEEK_MODELS.iter().any(|m| model_lower.contains(m)) {
        debug!("Detected DeepSeek model: {}", model_name);
        return OpenAIModelFeatures {
            uses_max_completion_tokens: false,
            requires_temperature_one: false,
            supports_responses_api: false,
            supports_custom_tools: false,
            supports_allowed_tools: false,
        };
    }

    // 默认特性（标准 OpenAI 模型）
    debug!("Using default features for model: {}", model_name);
    OpenAIModelFeatures::default()
}

/// 应用模型参数转换
///
/// 根据模型特性调整请求参数。
///
/// # Arguments
///
/// * `request` - 可变引用的聊天完成请求
/// * `features` - 模型特性
pub fn apply_model_transformations(
    request: &mut ChatCompletionRequest,
    features: &OpenAIModelFeatures,
) {
    // GPT-5/o1 系列：使用 max_completion_tokens 而非 max_tokens
    if features.uses_max_completion_tokens {
        if let Some(max_tokens) = request.max_tokens {
            debug!(
                "Converting max_tokens ({}) to max_completion_tokens",
                max_tokens
            );
            request.max_completion_tokens = Some(max_tokens);
            request.max_tokens = None;
        }
    }

    // GPT-5/o1 系列：temperature 必须为 1
    if features.requires_temperature_one {
        if let Some(temp) = request.temperature {
            if temp != 1.0 {
                debug!("Adjusting temperature from {} to 1", temp);
                request.temperature = Some(1.0);
            }
        }
    }

    // 移除不支持的参数（某些提供商不支持 stream_options）
    if !features.supports_responses_api && request.stream_options.is_some() {
        debug!("Removing stream_options (not supported by this model)");
        request.stream_options = None;
    }
}

/// 检测可修复错误
///
/// 检查错误消息是否包含可自动修复的问题。
///
/// # Arguments
///
/// * `error_message` - 错误消息
///
/// # Returns
///
/// 是否为可修复错误
pub fn detect_fixable_error(error_message: &str) -> bool {
    let lower = error_message.to_lowercase();

    // max_tokens vs max_completion_tokens 错误
    if lower.contains("max_tokens")
        && (lower.contains("max_completion_tokens") || lower.contains("not supported"))
    {
        return true;
    }

    // temperature 限制错误
    if lower.contains("temperature") && lower.contains("must be 1") {
        return true;
    }

    // stream_options 不支持错误
    if lower.contains("stream_options") && lower.contains("not supported") {
        return true;
    }

    // 工具描述过长错误
    if lower.contains("1024") && lower.contains("maximum length") {
        return true;
    }

    false
}

/// 尝试自动修复请求参数
///
/// 根据错误消息自动调整请求参数。
///
/// # Arguments
///
/// * `request` - 可变引用的聊天完成请求
/// * `error_message` - 错误消息
///
/// # Returns
///
/// 是否成功应用修复
pub fn try_fix_request(request: &mut ChatCompletionRequest, error_message: &str) -> bool {
    let lower = error_message.to_lowercase();
    let mut fixed = false;

    // 修复 max_tokens → max_completion_tokens
    if lower.contains("max_tokens")
        && (lower.contains("max_completion_tokens") || lower.contains("not supported"))
    {
        if let Some(max_tokens) = request.max_tokens {
            debug!("Auto-fix: Converting max_tokens to max_completion_tokens");
            request.max_completion_tokens = Some(max_tokens);
            request.max_tokens = None;
            fixed = true;
        }
    }

    // 修复 temperature
    if lower.contains("temperature") && lower.contains("must be 1") {
        debug!("Auto-fix: Setting temperature to 1");
        request.temperature = Some(1.0);
        fixed = true;
    }

    // 移除 stream_options
    if lower.contains("stream_options") && lower.contains("not supported") {
        debug!("Auto-fix: Removing stream_options");
        request.stream_options = None;
        fixed = true;
    }

    fixed
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::openai::types::StreamOptions;

    #[test]
    fn test_get_model_features_gpt5() {
        let features = get_model_features("gpt-5");
        assert!(features.uses_max_completion_tokens);
        assert!(features.requires_temperature_one);
        assert!(features.supports_responses_api);
    }

    #[test]
    fn test_get_model_features_o1() {
        let features = get_model_features("o1-preview");
        assert!(features.uses_max_completion_tokens);
        assert!(features.requires_temperature_one);
    }

    #[test]
    fn test_get_model_features_deepseek() {
        let features = get_model_features("deepseek-chat");
        assert!(!features.uses_max_completion_tokens);
        assert!(!features.requires_temperature_one);
    }

    #[test]
    fn test_detect_fixable_error() {
        assert!(detect_fixable_error(
            "unsupported parameter: 'max_tokens'. Use 'max_completion_tokens'"
        ));
        assert!(detect_fixable_error("temperature must be 1"));
        assert!(detect_fixable_error("stream_options not supported"));
        assert!(!detect_fixable_error("invalid API key"));
    }

    #[test]
    fn test_try_fix_request() {
        let mut request = ChatCompletionRequest {
            model: "gpt-5".to_string(),
            messages: vec![],
            max_tokens: Some(1000),
            max_completion_tokens: None,
            temperature: Some(0.7),
            tools: None,
            tool_choice: None,
            stream: None,
            stream_options: Some(StreamOptions {
                include_usage: true,
            }),
        };

        let fixed = try_fix_request(
            &mut request,
            "Use 'max_completion_tokens' instead of 'max_tokens'",
        );
        assert!(fixed);
        assert!(request.max_completion_tokens.is_some());
        assert!(request.max_tokens.is_none());
    }

    #[test]
    fn test_apply_model_transformations() {
        let mut request = ChatCompletionRequest {
            model: "gpt-5".to_string(),
            messages: vec![],
            max_tokens: Some(1000),
            max_completion_tokens: None,
            temperature: Some(0.7),
            tools: None,
            tool_choice: None,
            stream: None,
            stream_options: Some(StreamOptions {
                include_usage: true,
            }),
        };

        let features = get_model_features("gpt-5");
        apply_model_transformations(&mut request, &features);

        assert!(request.max_completion_tokens.is_some());
        assert!(request.max_tokens.is_none());
        assert_eq!(request.temperature, Some(1.0));
        // GPT-5 supports Responses API, so stream_options should NOT be removed
        assert!(request.stream_options.is_some()); // GPT-5 supports it, but let's verify logic
    }
}
