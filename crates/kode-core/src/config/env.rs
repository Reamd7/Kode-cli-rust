//! 环境变量处理
//!
//! 提供环境变量的读取和处理功能。

use std::path::PathBuf;
use std::env;

/// 获取配置文件路径
///
/// 支持环境变量覆盖配置目录：
/// - 优先检查 KODE_CONFIG_DIR
/// - 其次检查 CLAUDE_CONFIG_DIR
/// - 默认使用 ~/.kode.json
///
/// # Returns
///
/// 返回配置文件的完整路径
pub fn get_config_file_path() -> PathBuf {
    // 优先检查 KODE_CONFIG_DIR
    if let Ok(dir) = env::var("KODE_CONFIG_DIR") {
        return PathBuf::from(dir).join("config.json");
    }

    // 其次检查 CLAUDE_CONFIG_DIR（兼容 Claude Code）
    if let Ok(dir) = env::var("CLAUDE_CONFIG_DIR") {
        return PathBuf::from(dir).join("config.json");
    }

    // 默认使用 ~/.kode.json
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".kode.json")
}

/// 获取 OpenAI API Key
///
/// 从环境变量 OPENAI_API_KEY 读取 OpenAI API 密钥。
///
/// # Returns
///
/// 返回 API Key（如果存在），否则返回 None
pub fn get_openai_api_key() -> Option<String> {
    env::var("OPENAI_API_KEY").ok()
}

/// 获取 Anthropic API Key
///
/// 从环境变量 ANTHROPIC_API_KEY 读取 Anthropic API 密钥。
///
/// # Returns
///
/// 返回 API Key，如果不存在则返回空字符串
pub fn get_anthropic_api_key() -> String {
    env::var("ANTHROPIC_API_KEY").unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_config_file_path_default() {
        // 清除环境变量（确保没有残留）
        env::remove_var("KODE_CONFIG_DIR");
        env::remove_var("CLAUDE_CONFIG_DIR");

        let path = get_config_file_path();
        // 检查路径包含 .kode.json 或 config.json（home_dir 可能在测试环境中返回不同路径）
        let path_str = path.to_string_lossy();
        let has_config = path_str.contains(".kode.json") || path_str.contains("config.json");
        assert!(has_config, "Path should contain config filename: {}", path_str);
    }

    #[test]
    fn test_get_config_file_path_with_kode_env() {
        env::set_var("KODE_CONFIG_DIR", "/tmp/kode");
        let path = get_config_file_path();
        assert_eq!(path, PathBuf::from("/tmp/kode/config.json"));
        env::remove_var("KODE_CONFIG_DIR");
    }

    #[test]
    fn test_get_config_file_path_with_claude_env() {
        env::set_var("CLAUDE_CONFIG_DIR", "/tmp/claude");
        let path = get_config_file_path();
        assert_eq!(path, PathBuf::from("/tmp/claude/config.json"));
        env::remove_var("CLAUDE_CONFIG_DIR");
    }

    #[test]
    fn test_get_openai_api_key() {
        env::set_var("OPENAI_API_KEY", "sk-test123");
        let key = get_openai_api_key();
        assert_eq!(key, Some("sk-test123".to_string()));
        env::remove_var("OPENAI_API_KEY");
    }

    #[test]
    fn test_get_openai_api_key_not_set() {
        env::remove_var("OPENAI_API_KEY");
        let key = get_openai_api_key();
        assert!(key.is_none());
    }

    #[test]
    fn test_get_anthropic_api_key() {
        env::set_var("ANTHROPIC_API_KEY", "sk-ant-test123");
        let key = get_anthropic_api_key();
        assert_eq!(key, "sk-ant-test123");
        env::remove_var("ANTHROPIC_API_KEY");
    }

    #[test]
    fn test_get_anthropic_api_key_not_set() {
        env::remove_var("ANTHROPIC_API_KEY");
        let key = get_anthropic_api_key();
        assert_eq!(key, "");
    }

    #[test]
    fn test_kode_config_priority_over_claude() {
        env::set_var("CLAUDE_CONFIG_DIR", "/tmp/claude");
        env::set_var("KODE_CONFIG_DIR", "/tmp/kode");
        let path = get_config_file_path();
        // KODE_CONFIG_DIR 优先级更高
        assert_eq!(path, PathBuf::from("/tmp/kode/config.json"));
        env::remove_var("KODE_CONFIG_DIR");
        env::remove_var("CLAUDE_CONFIG_DIR");
    }
}
