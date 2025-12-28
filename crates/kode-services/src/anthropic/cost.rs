//! Anthropic 成本计算模块
//!
//! 提供 Claude API 调用的成本估算功能。

/// Anthropic 模型成本信息
#[derive(Debug, Clone, Copy)]
pub struct ModelCostInfo {
    /// 每百万输入 token 的成本（美元）
    pub input_cost_per_million: f64,
    /// 每百万输出 token 的成本（美元）
    pub output_cost_per_million: f64,
    /// 是否支持缓存
    pub supports_cache: bool,
    /// 每百万缓存 token 的成本（美元）
    pub cache_cost_per_million: Option<f64>,
}

impl ModelCostInfo {
    /// 创建新的模型成本信息
    pub const fn new(
        input_cost_per_million: f64,
        output_cost_per_million: f64,
        supports_cache: bool,
        cache_cost_per_million: Option<f64>,
    ) -> Self {
        Self {
            input_cost_per_million,
            output_cost_per_million,
            supports_cache,
            cache_cost_per_million,
        }
    }

    /// 计算输入 token 的成本（美元）
    #[inline]
    pub fn calculate_input_cost(&self, input_tokens: usize) -> f64 {
        (input_tokens as f64 / 1_000_000.0) * self.input_cost_per_million
    }

    /// 计算输出 token 的成本（美元）
    #[inline]
    pub fn calculate_output_cost(&self, output_tokens: usize) -> f64 {
        (output_tokens as f64 / 1_000_000.0) * self.output_cost_per_million
    }

    /// 计算缓存 token 的成本（美元）
    #[inline]
    pub fn calculate_cache_cost(&self, cache_tokens: usize) -> f64 {
        self.cache_cost_per_million
            .map(|cost| (cache_tokens as f64 / 1_000_000.0) * cost)
            .unwrap_or(0.0)
    }

    /// 计算总成本（美元）
    #[inline]
    pub fn calculate_total_cost(
        &self,
        input_tokens: usize,
        output_tokens: usize,
        cache_tokens: usize,
    ) -> f64 {
        self.calculate_input_cost(input_tokens)
            + self.calculate_output_cost(output_tokens)
            + self.calculate_cache_cost(cache_tokens)
    }
}

/// Anthropic 已知模型成本映射
const MODEL_COSTS: &[(ModelCostInfo, &[&str])] = &[
    // Claude 3.7 Sonnet
    (
        ModelCostInfo::new(3.0, 15.0, true, Some(0.3)),
        &["claude-sonnet-4-20250514", "claude-3-7-sonnet-20250514"],
    ),
    // Claude 3.7 Haiku
    (
        ModelCostInfo::new(0.25, 1.25, true, Some(0.03)),
        &["claude-haiku-4-20250514", "claude-3-7-haiku-20250514"],
    ),
    // Claude 3.5 Haiku
    (
        ModelCostInfo::new(0.25, 1.25, false, None),
        &["claude-haiku-3-20250514"],
    ),
    // Claude 3.5 Sonnet
    (
        ModelCostInfo::new(3.0, 15.0, true, Some(0.3)),
        &["claude-sonnet-3-20250514", "claude-3-5-sonnet-20250514"],
    ),
    // Claude 3.5 Haiku (new)
    (
        ModelCostInfo::new(0.8, 4.0, true, Some(0.08)),
        &["claude-3-5-haiku-20250620"],
    ),
    // Claude 3 Opus
    (
        ModelCostInfo::new(15.0, 75.0, true, Some(1.5)),
        &["claude-opus-4-20250514", "claude-3-opus-20250514"],
    ),
    // Claude 3 Sonnet
    (
        ModelCostInfo::new(3.0, 15.0, true, Some(0.3)),
        &["claude-3-sonnet-20250514"],
    ),
    // Claude 3 Haiku
    (
        ModelCostInfo::new(0.25, 1.25, false, None),
        &["claude-3-haiku-20250514"],
    ),
];

/// 获取指定模型的输入 token 成本（美元）
///
/// # Arguments
///
/// * `model_name` - 模型名称
/// * `tokens` - 输入 token 数量
///
/// # Returns
///
/// 输入 token 的成本（美元）
#[inline]
pub fn get_model_input_token_cost_usd(model_name: &str, tokens: usize) -> f64 {
    get_model_cost_info(model_name)
        .map(|info| info.calculate_input_cost(tokens))
        .unwrap_or_default()
}

/// 获取指定模型的输出 token 成本（美元）
///
/// # Arguments
///
/// * `model_name` - 模型名称
/// * `tokens` - 输出 token 数量
///
/// # Returns
///
/// 输出 token 的成本（美元）
#[inline]
pub fn get_model_output_token_cost_usd(model_name: &str, tokens: usize) -> f64 {
    get_model_cost_info(model_name)
        .map(|info| info.calculate_output_cost(tokens))
        .unwrap_or_default()
}

/// 获取指定模型的缓存 token 成本（美元）
///
/// # Arguments
///
/// * `model_name` - 模型名称
/// * `tokens` - 缓存 token 数量
///
/// # Returns
///
/// 缓存 token 的成本（美元）
#[inline]
pub fn get_model_cache_token_cost_usd(model_name: &str, tokens: usize) -> f64 {
    get_model_cost_info(model_name)
        .map(|info| info.calculate_cache_cost(tokens))
        .unwrap_or_default()
}

/// 获取指定模型的成本信息
///
/// # Arguments
///
/// * `model_name` - 模型名称
///
/// # Returns
///
/// 模型成本信息，如果未找到则返回默认值
#[inline]
pub fn get_model_cost_info(model_name: &str) -> Option<ModelCostInfo> {
    for (cost_info, model_names) in MODEL_COSTS {
        if model_names.contains(&model_name) {
            return Some(*cost_info);
        }
    }
    None
}

/// 获取指定模型的总成本（美元）
///
/// # Arguments
///
/// * `model_name` - 模型名称
/// * `input_tokens` - 输入 token 数量
/// * `output_tokens` - 输出 token 数量
/// * `cache_tokens` - 缓存 token 数量
///
/// # Returns
///
/// 总成本（美元）
#[inline]
pub fn get_model_total_cost_usd(
    model_name: &str,
    input_tokens: usize,
    output_tokens: usize,
    cache_tokens: usize,
) -> f64 {
    get_model_cost_info(model_name)
        .map(|info| info.calculate_total_cost(input_tokens, output_tokens, cache_tokens))
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_cost_info_creation() {
        let cost_info = ModelCostInfo::new(3.0, 15.0, true, Some(0.3));
        assert_eq!(cost_info.input_cost_per_million, 3.0);
        assert_eq!(cost_info.output_cost_per_million, 15.0);
        assert!(cost_info.supports_cache);
        assert_eq!(cost_info.cache_cost_per_million, Some(0.3));
    }

    #[test]
    fn test_calculate_input_cost() {
        let cost_info = ModelCostInfo::new(3.0, 15.0, true, Some(0.3));
        // 1000 tokens / 1M * $3.0 = $0.003
        assert_eq!(cost_info.calculate_input_cost(1000), 0.003);
        // 1000000 tokens = $3.0
        assert_eq!(cost_info.calculate_input_cost(1_000_000), 3.0);
    }

    #[test]
    fn test_calculate_output_cost() {
        let cost_info = ModelCostInfo::new(3.0, 15.0, true, Some(0.3));
        // 1000 tokens / 1M * $15.0 = $0.015
        assert_eq!(cost_info.calculate_output_cost(1000), 0.015);
        // 1000000 tokens = $15.0
        assert_eq!(cost_info.calculate_output_cost(1_000_000), 15.0);
    }

    #[test]
    fn test_calculate_cache_cost() {
        let cost_info = ModelCostInfo::new(3.0, 15.0, true, Some(0.3));
        // 1000 tokens / 1M * $0.3 = $0.0003
        assert_eq!(cost_info.calculate_cache_cost(1000), 0.0003);
        // 1000000 tokens = $0.3
        assert_eq!(cost_info.calculate_cache_cost(1_000_000), 0.3);
    }

    #[test]
    fn test_calculate_total_cost() {
        let cost_info = ModelCostInfo::new(3.0, 15.0, true, Some(0.3));
        // 1000 input + 500 output + 200 cache
        // input: $0.003, output: $0.0075, cache: $0.00006
        assert!((cost_info.calculate_total_cost(1000, 500, 200) - 0.01056).abs() < 0.0001);
    }

    #[test]
    fn test_get_model_cost_info() {
        let cost_info = get_model_cost_info("claude-sonnet-4-20250514");
        assert!(cost_info.is_some());
        let info = cost_info.unwrap();
        assert_eq!(info.input_cost_per_million, 3.0);
        assert_eq!(info.output_cost_per_million, 15.0);
    }

    #[test]
    fn test_get_model_cost_info_not_found() {
        let cost_info = get_model_cost_info("unknown-model");
        assert!(cost_info.is_none());
    }

    #[test]
    fn test_get_model_input_token_cost_usd() {
        // claude-sonnet-4-20250514: $3.0 per million input tokens
        // 1000 tokens = $0.003
        assert_eq!(
            get_model_input_token_cost_usd("claude-sonnet-4-20250514", 1000),
            0.003
        );
    }

    #[test]
    fn test_get_model_output_token_cost_usd() {
        // claude-sonnet-4-20250514: $15.0 per million output tokens
        // 1000 tokens = $0.015
        assert_eq!(
            get_model_output_token_cost_usd("claude-sonnet-4-20250514", 1000),
            0.015
        );
    }

    #[test]
    fn test_get_model_cache_token_cost_usd() {
        // claude-sonnet-4-20250514: $0.3 per million cache tokens
        // 1000 tokens = $0.0003
        assert_eq!(
            get_model_cache_token_cost_usd("claude-sonnet-4-20250514", 1000),
            0.0003
        );
    }

    #[test]
    fn test_get_model_total_cost_usd() {
        let total = get_model_total_cost_usd("claude-sonnet-4-20250514", 1000, 500, 200);
        // input: $0.003, output: $0.0075, cache: $0.00006
        assert!((total - 0.01056).abs() < 0.0001);
    }
}
