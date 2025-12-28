//! OpenAI API 成本计算
//!
//! 根据 token 使用量计算 API 调用成本。

use serde::{Deserialize, Serialize};

/// OpenAI 模型定价信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pricing {
    /// 输入 token 价格（每百万 token 的美元价格）
    pub input_price_per_million: f64,
    /// 输出 token 价格（每百万 token 的美元价格）
    pub output_price_per_million: f64,
}

impl Pricing {
    /// 创建新的定价信息
    pub const fn new(input_price_per_million: f64, output_price_per_million: f64) -> Self {
        Self {
            input_price_per_million,
            output_price_per_million,
        }
    }
}

/// 获取模型定价
///
/// # Arguments
///
/// * `model_name` - 模型名称
///
/// # Returns
///
/// 定价信息，如果模型未知则返回 None
pub fn get_model_pricing(model_name: &str) -> Option<Pricing> {
    let model_lower = model_name.to_lowercase();

    // GPT-4 系列
    if model_lower.contains("gpt-4") {
        if model_lower.contains("turbo") {
            return Some(Pricing::new(0.10, 0.30)); // GPT-4 Turbo
        } else if model_lower.contains("32k") || model_lower.contains("preview") {
            return Some(Pricing::new(0.60, 1.20)); // GPT-4 32K
        }
        return Some(Pricing::new(0.30, 0.60)); // GPT-4 标准版
    }

    // GPT-3.5 系列
    if model_lower.contains("gpt-3.5") {
        if model_lower.contains("turbo") {
            return Some(Pricing::new(0.50, 1.50)); // GPT-3.5 Turbo 16K
        }
        return Some(Pricing::new(0.10, 0.20)); // GPT-3.5 Turbo 4K
    }

    // GPT-5 系列（估计价格）
    if model_lower.contains("gpt-5") {
        return Some(Pricing::new(1.0, 2.0)); // 估计价格
    }

    // o1/o3 系列
    if model_lower.contains("o1") || model_lower.contains("o3") {
        return Some(Pricing::new(5.0, 15.0)); // o1 系列估计价格
    }

    // DeepSeek
    if model_lower.contains("deepseek") {
        if model_lower.contains("coder") {
            return Some(Pricing::new(0.14, 0.28)); // DeepSeek Coder
        }
        return Some(Pricing::new(0.14, 0.28)); // DeepSeek Chat
    }

    // 其他模型
    None
}

/// 计算 API 调用成本
///
/// # Arguments
///
/// * `model_name` - 模型名称
/// * `input_tokens` - 输入 token 数
/// * `output_tokens` - 输出 token 数
///
/// # Returns
///
/// 成本（美元），如果模型定价未知则返回 None
pub fn calculate_cost(model_name: &str, input_tokens: u32, output_tokens: u32) -> Option<f64> {
    let pricing = get_model_pricing(model_name)?;

    let input_cost = (input_tokens as f64 / 1_000_000.0) * pricing.input_price_per_million;
    let output_cost = (output_tokens as f64 / 1_000_000.0) * pricing.output_price_per_million;

    Some(input_cost + output_cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_model_pricing_gpt4() {
        let pricing = get_model_pricing("gpt-4");
        assert!(pricing.is_some());
        let p = pricing.unwrap();
        assert_eq!(p.input_price_per_million, 0.30);
        assert_eq!(p.output_price_per_million, 0.60);
    }

    #[test]
    fn test_get_model_pricing_gpt35_turbo() {
        let pricing = get_model_pricing("gpt-3.5-turbo");
        assert!(pricing.is_some());
        let p = pricing.unwrap();
        assert_eq!(p.input_price_per_million, 0.50);
        assert_eq!(p.output_price_per_million, 1.50);
    }

    #[test]
    fn test_get_model_pricing_deepseek() {
        let pricing = get_model_pricing("deepseek-chat");
        assert!(pricing.is_some());
        let p = pricing.unwrap();
        assert_eq!(p.input_price_per_million, 0.14);
        assert_eq!(p.output_price_per_million, 0.28);
    }

    #[test]
    fn test_calculate_cost() {
        let cost = calculate_cost("gpt-4", 1000, 500);
        assert!(cost.is_some());
        let c = cost.unwrap();
        // 1000 tokens @ $0.30/M = $0.0003
        // 500 tokens @ $0.60/M = $0.0003
        // Total = $0.0006
        assert!((c - 0.0006).abs() < 0.0001);
    }

    #[test]
    fn test_calculate_cost_unknown_model() {
        let cost = calculate_cost("unknown-model", 1000, 500);
        assert!(cost.is_none());
    }
}
