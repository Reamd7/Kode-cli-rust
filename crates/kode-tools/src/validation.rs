//! 参数验证框架

use serde_json::Value;

/// 验证错误类型
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("文件不存在: {0}")]
    /// 文件不存在
    FileNotFound(String),

    #[error("文件过大: {0} bytes (最大 {1} bytes)")]
    /// 文件过大
    FileTooLarge(usize, usize),

    #[error("无效路径: {0}")]
    /// 无效路径
    InvalidPath(String),

    #[error("缺少参数: {0}")]
    /// 缺少参数
    MissingParameter(String),

    #[error("无效类型: {0}")]
    /// 无效类型
    InvalidType(String),

    #[error("字符串未找到")]
    /// 字符串未找到
    StringNotFound,

    #[error("多个匹配: 找到 {0} 处匹配")]
    /// 多个匹配
    MultipleMatches(usize),

    #[error("文件已被修改")]
    /// 文件已修改
    FileModified,

    #[error("验证失败: {0}")]
    /// 其他错误
    Other(String),
}

/// JSON Schema 验证器（占位符）
pub struct JsonSchemaValidator;

impl JsonSchemaValidator {
    /// 验证参数符合 JSON Schema
    pub fn validate(_schema: &Value, _params: &Value) -> Result<(), ValidationError> {
        // TODO: 实现 JSON Schema 验证（需要 jsonschema crate）
        Ok(())
    }
}

/// 路径验证器
pub struct PathValidator;

impl PathValidator {
    /// 验证路径存在性
    pub fn exists(path: &str) -> Result<(), ValidationError> {
        let path = std::path::Path::new(path);
        if !path.exists() {
            return Err(ValidationError::FileNotFound(
                path.to_string_lossy().to_string(),
            ));
        }
        Ok(())
    }

    /// 验证路径安全性
    pub fn is_safe(path: &str) -> Result<(), ValidationError> {
        let path_str = path.to_string();
        if path_str.contains("..") {
            return Err(ValidationError::InvalidPath("路径包含 ..".to_string()));
        }
        if path_str.contains('~') {
            return Err(ValidationError::InvalidPath("路径包含 ~".to_string()));
        }
        Ok(())
    }
}

/// 文件验证器
pub struct FileValidator;

impl FileValidator {
    /// 验证文件大小
    pub fn check_size(size: usize, max_size: usize) -> Result<(), ValidationError> {
        if size > max_size {
            return Err(ValidationError::FileTooLarge(size, max_size));
        }
        Ok(())
    }

    /// 验证文件修改时间
    pub fn check_modified(read_ts: u64, current_ts: u64) -> Result<(), ValidationError> {
        if current_ts > read_ts {
            return Err(ValidationError::FileModified);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_validator_exists() {
        assert!(PathValidator::exists("/nonexistent").is_err());
        assert!(PathValidator::exists(".").is_ok());
    }

    #[test]
    fn test_path_validator_is_safe() {
        assert!(PathValidator::is_safe("/etc/passwd").is_ok());
        assert!(PathValidator::is_safe("/etc/../passwd").is_err());
        assert!(PathValidator::is_safe("~/.bashrc").is_err());
    }

    #[test]
    fn test_file_validator_check_size() {
        assert!(FileValidator::check_size(1024, 4096).is_ok());
        assert!(FileValidator::check_size(8192, 4096).is_err());
    }

    #[test]
    fn test_file_validator_check_modified() {
        assert!(FileValidator::check_modified(1000, 2000).is_err());
        assert!(FileValidator::check_modified(2000, 1000).is_ok());
    }
}
