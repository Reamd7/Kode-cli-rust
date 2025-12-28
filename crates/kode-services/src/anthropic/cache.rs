//! Anthropic Prompt Caching 模块
//!
//! 提供 Anthropic API 的提示词缓存功能支持。

use serde_json::Value;

/// Anthropic API 缓存控制参数
#[derive(Debug, Clone, Default)]
pub struct CacheControl {
    /// 缓存断点位置（按 token 数）
    pub breakpoints: Vec<usize>,
    /// 最大缓存块数
    pub max_blocks: usize,
}

impl CacheControl {
    /// 创建新的缓存控制配置
    ///
    /// # Arguments
    ///
    /// * `breakpoints` - 缓存断点位置
    /// * `max_blocks` - 最大缓存块数（Anthropic 限制为 4）
    pub fn new(breakpoints: Vec<usize>, max_blocks: usize) -> Self {
        Self {
            breakpoints,
            max_blocks: max_blocks.min(4), // Anthropic 限制为 4 个块
        }
    }

    /// 使用默认 4 块限制创建缓存控制
    ///
    /// # Arguments
    ///
    /// * `breakpoints` - 缓存断点位置
    pub fn with_default_limits(breakpoints: Vec<usize>) -> Self {
        Self::new(breakpoints, 4)
    }

    /// 检查是否启用缓存
    #[inline]
    pub fn is_enabled(&self) -> bool {
        !self.breakpoints.is_empty() && self.max_blocks > 0
    }

    /// 获取有效的缓存断点（考虑最大块数限制）
    #[inline]
    pub fn effective_breakpoints(&self) -> Vec<usize> {
        let effective_count = self.breakpoints.len().min(self.max_blocks);
        self.breakpoints[..effective_count].to_vec()
    }
}

/// Anthropic 消息缓存配置
#[derive(Debug, Clone)]
pub struct AnthropicCacheConfig {
    /// 是否启用提示词缓存
    pub enabled: bool,
    /// 缓存断点
    pub breakpoints: Vec<usize>,
    /// 最大缓存块数
    pub max_blocks: usize,
}

impl Default for AnthropicCacheConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            breakpoints: vec![],
            max_blocks: 4,
        }
    }
}

impl AnthropicCacheConfig {
    /// 创建新的缓存配置
    pub fn new(enabled: bool, breakpoints: Vec<usize>, max_blocks: usize) -> Self {
        Self {
            enabled,
            breakpoints,
            max_blocks: max_blocks.min(4),
        }
    }

    /// 启用缓存，使用默认断点
    pub fn enabled_with_breakpoints(breakpoints: Vec<usize>) -> Self {
        Self::new(true, breakpoints, 4)
    }

    /// 创建禁用缓存的配置
    pub fn disabled() -> Self {
        Self::default()
    }
}

/// 应用缓存控制到请求体
///
/// # Arguments
///
/// * `request_body` - 请求体 JSON
/// * `cache_config` - 缓存配置
/// * `total_tokens` - 消息总 token 数
///
/// # Returns
///
/// 修改后的请求体
pub fn apply_cache_control_with_limits(
    mut request_body: Value,
    cache_config: &AnthropicCacheConfig,
    total_tokens: usize,
) -> Value {
    if !cache_config.enabled || cache_config.breakpoints.is_empty() {
        return request_body;
    }

    let effective_breakpoints = cache_config
        .breakpoints
        .iter()
        .filter(|&&bp| bp < total_tokens)
        .cloned()
        .take(cache_config.max_blocks)
        .collect::<Vec<_>>();

    if effective_breakpoints.is_empty() {
        return request_body;
    }

    // Anthropic API 格式：在消息内容中使用 cache_control
    if let Some(messages) = request_body
        .get_mut("messages")
        .and_then(|m| m.as_array_mut())
    {
        for (index, msg) in messages.iter_mut().enumerate() {
            if let Some(breakpoint) = effective_breakpoints.get(index) {
                if *breakpoint > 0 {
                    // 在消息中添加 cache_control
                    if let Some(content) = msg.get_mut("content") {
                        match content {
                            Value::String(_) => {
                                // 如果是文本内容，转换为 blocks 格式
                                let text = content.to_string();
                                let new_content = serde_json::json!([
                                    {
                                        "type": "text",
                                        "text": text,
                                        "cache_control": {
                                            "type": "ephemeral"
                                        }
                                    }
                                ]);
                                *content = new_content;
                            }
                            Value::Array(blocks) => {
                                // 在第一个 block 添加 cache_control
                                if let Some(first_block) = blocks.first_mut() {
                                    if let Some(obj) = first_block.as_object_mut() {
                                        obj.insert(
                                            "cache_control".to_string(),
                                            serde_json::json!({
                                                "type": "ephemeral"
                                            }),
                                        );
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    request_body
}

/// 创建 Anthropic 兼容的缓存控制参数
///
/// # Arguments
///
/// * `breakpoints` - 缓存断点位置
/// * `max_blocks` - 最大块数
///
/// # Returns
///
/// 缓存控制 JSON 对象
pub fn create_cache_control_param(breakpoints: &[usize], max_blocks: usize) -> Value {
    let effective_count = breakpoints.len().min(max_blocks);
    let cache_points: Vec<Value> = (0..effective_count)
        .filter_map(|i| breakpoints.get(i).copied())
        .map(|offset| {
            serde_json::json!({
                "type": "ephemeral",
                "offset": offset
            })
        })
        .collect();

    serde_json::json!({
        "type": "cache_control",
        "cache_points": cache_points
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_cache_control_new() {
        let cache = CacheControl::new(vec![1000, 2000, 3000], 4);
        assert_eq!(cache.max_blocks, 4);
        assert_eq!(cache.breakpoints, vec![1000, 2000, 3000]);
    }

    #[test]
    fn test_cache_control_max_blocks_limit() {
        // 超过 4 块应该被限制为 4
        let cache = CacheControl::new(vec![1000, 2000, 3000, 4000, 5000], 10);
        assert_eq!(cache.max_blocks, 4);
    }

    #[test]
    fn test_cache_control_is_enabled() {
        let cache_enabled = CacheControl::new(vec![1000], 4);
        let cache_disabled = CacheControl::new(vec![], 4);
        let cache_no_limit = CacheControl::new(vec![1000], 0);

        assert!(cache_enabled.is_enabled());
        assert!(!cache_disabled.is_enabled());
        assert!(!cache_no_limit.is_enabled());
    }

    #[test]
    fn test_effective_breakpoints() {
        let cache = CacheControl::new(vec![1000, 2000, 3000, 4000, 5000], 3);
        let effective = cache.effective_breakpoints();
        assert_eq!(effective, vec![1000, 2000, 3000]);
    }

    #[test]
    fn test_anthropic_cache_config_default() {
        let config = AnthropicCacheConfig::default();
        assert!(!config.enabled);
        assert!(config.breakpoints.is_empty());
        assert_eq!(config.max_blocks, 4);
    }

    #[test]
    fn test_anthropic_cache_config_enabled() {
        let config = AnthropicCacheConfig::enabled_with_breakpoints(vec![1000, 2000]);
        assert!(config.enabled);
        assert_eq!(config.breakpoints, vec![1000, 2000]);
        assert_eq!(config.max_blocks, 4);
    }

    #[test]
    fn test_apply_cache_control_disabled() {
        let request_body = json!({
            "model": "claude-sonnet-4-20250514",
            "messages": [{"role": "user", "content": "Hello"}]
        });

        let cache_config = AnthropicCacheConfig::disabled();
        let result = apply_cache_control_with_limits(request_body.clone(), &cache_config, 100);

        assert_eq!(result, request_body);
    }

    #[test]
    fn test_apply_cache_control_with_limits() {
        let request_body = json!({
            "model": "claude-sonnet-4-20250514",
            "messages": [
                {"role": "user", "content": "First message"},
                {"role": "assistant", "content": "Response"},
                {"role": "user", "content": "Second message"}
            ]
        });

        let cache_config = AnthropicCacheConfig::enabled_with_breakpoints(vec![100, 200]);
        let result = apply_cache_control_with_limits(request_body, &cache_config, 300);

        // 验证第一条消息被添加了 cache_control
        if let Some(messages) = result["messages"].as_array() {
            if let Some(first_msg) = messages.first() {
                if let Some(content) = first_msg.get("content") {
                    // 内容应该包含 cache_control
                    assert!(content.is_array());
                }
            }
        }
    }

    #[test]
    fn test_create_cache_control_param() {
        let param = create_cache_control_param(&[1000, 2000, 3000], 4);
        assert!(param.is_object());
        assert_eq!(param["type"], "cache_control");
        assert!(param.get("cache_points").is_some());
    }
}
