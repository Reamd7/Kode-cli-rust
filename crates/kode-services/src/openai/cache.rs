//! Session 状态缓存
//!
//! 用于缓存模型错误和已知的参数修复，避免重复检测和修复。

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Session 状态缓存
///
/// 线程安全的内存缓存，用于存储会话期间的模型错误和修复历史。
#[derive(Debug, Clone)]
pub struct SessionCache {
    /// 内部状态（使用 Arc 共享）
    inner: Arc<RwLock<SessionCacheInner>>,
}

/// Session 缓存内部状态
#[derive(Debug, Default)]
struct SessionCacheInner {
    /// 模型错误缓存（模型名称 -> 已知的错误类型）
    model_errors: HashMap<String, Vec<String>>,
    /// 应用的修复（模型名称 -> 应用的修复列表）
    applied_fixes: HashMap<String, Vec<String>>,
}

impl SessionCache {
    /// 创建新的 Session 缓存
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(SessionCacheInner::default())),
        }
    }

    /// 缓存模型错误
    ///
    /// # Arguments
    ///
    /// * `model_name` - 模型名称
    /// * `error_type` - 错误类型（例如 "max_tokens_not_supported"）
    pub fn cache_model_error(&self, model_name: &str, error_type: &str) {
        let mut inner = self.inner.write().unwrap();
        inner
            .model_errors
            .entry(model_name.to_string())
            .or_default()
            .push(error_type.to_string());
    }

    /// 检查模型是否有已知的错误类型
    ///
    /// # Arguments
    ///
    /// * `model_name` - 模型名称
    /// * `error_type` - 错误类型
    ///
    /// # Returns
    ///
    /// 如果该错误类型已被缓存则返回 true
    pub fn has_model_error(&self, model_name: &str, error_type: &str) -> bool {
        let inner = self.inner.read().unwrap();
        inner
            .model_errors
            .get(model_name)
            .map(|errors| errors.contains(&error_type.to_string()))
            .unwrap_or(false)
    }

    /// 获取模型的所有已知错误
    ///
    /// # Arguments
    ///
    /// * `model_name` - 模型名称
    ///
    /// # Returns
    ///
    /// 已知错误类型的列表
    pub fn get_model_errors(&self, model_name: &str) -> Vec<String> {
        let inner = self.inner.read().unwrap();
        inner
            .model_errors
            .get(model_name)
            .cloned()
            .unwrap_or_default()
    }

    /// 记录应用的修复
    ///
    /// # Arguments
    ///
    /// * `model_name` - 模型名称
    /// * `fix_description` - 修复描述（例如 "converted_max_tokens_to_max_completion_tokens"）
    pub fn record_applied_fix(&self, model_name: &str, fix_description: &str) {
        let mut inner = self.inner.write().unwrap();
        inner
            .applied_fixes
            .entry(model_name.to_string())
            .or_default()
            .push(fix_description.to_string());
    }

    /// 获取模型的所有已应用修复
    ///
    /// # Arguments
    ///
    /// * `model_name` - 模型名称
    ///
    /// # Returns
    ///
    /// 已应用修复的列表
    pub fn get_applied_fixes(&self, model_name: &str) -> Vec<String> {
        let inner = self.inner.read().unwrap();
        inner
            .applied_fixes
            .get(model_name)
            .cloned()
            .unwrap_or_default()
    }

    /// 清空缓存
    pub fn clear(&self) {
        let mut inner = self.inner.write().unwrap();
        inner.model_errors.clear();
        inner.applied_fixes.clear();
    }

    /// 获取缓存统计信息
    ///
    /// # Returns
    ///
    /// (错误数量, 修复数量)
    pub fn stats(&self) -> (usize, usize) {
        let inner = self.inner.read().unwrap();
        let error_count = inner.model_errors.values().map(|v| v.len()).sum();
        let fix_count = inner.applied_fixes.values().map(|v| v.len()).sum();
        (error_count, fix_count)
    }
}

impl Default for SessionCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_model_error() {
        let cache = SessionCache::new();

        // 缓存错误
        cache.cache_model_error("gpt-5", "max_tokens_not_supported");

        // 检查错误存在
        assert!(cache.has_model_error("gpt-5", "max_tokens_not_supported"));
        assert!(!cache.has_model_error("gpt-5", "other_error"));
        assert!(!cache.has_model_error("other-model", "max_tokens_not_supported"));
    }

    #[test]
    fn test_get_model_errors() {
        let cache = SessionCache::new();

        cache.cache_model_error("gpt-5", "error1");
        cache.cache_model_error("gpt-5", "error2");

        let errors = cache.get_model_errors("gpt-5");
        assert_eq!(errors.len(), 2);
        assert!(errors.contains(&"error1".to_string()));
        assert!(errors.contains(&"error2".to_string()));
    }

    #[test]
    fn test_record_applied_fix() {
        let cache = SessionCache::new();

        cache.record_applied_fix("gpt-5", "converted_max_tokens");

        let fixes = cache.get_applied_fixes("gpt-5");
        assert_eq!(fixes.len(), 1);
        assert_eq!(fixes[0], "converted_max_tokens");
    }

    #[test]
    fn test_cache_stats() {
        let cache = SessionCache::new();

        cache.cache_model_error("model1", "error1");
        cache.cache_model_error("model1", "error2");
        cache.record_applied_fix("model1", "fix1");
        cache.record_applied_fix("model2", "fix2");

        let (error_count, fix_count) = cache.stats();
        assert_eq!(error_count, 2);
        assert_eq!(fix_count, 2);
    }

    #[test]
    fn test_clear() {
        let cache = SessionCache::new();

        cache.cache_model_error("model1", "error1");
        cache.record_applied_fix("model1", "fix1");

        cache.clear();

        let (error_count, fix_count) = cache.stats();
        assert_eq!(error_count, 0);
        assert_eq!(fix_count, 0);
        assert!(!cache.has_model_error("model1", "error1"));
    }
}
