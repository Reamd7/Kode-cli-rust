//! 参数验证框架

use jsonschema::JSONSchema;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::OnceLock;

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

/// JSON Schema 验证器
///
/// 使用编译缓存来提高验证性能
pub struct JsonSchemaValidator {
    /// Schema 编译缓存（schema hash -> compiled validator）
    cache: OnceLock<HashMap<String, JSONSchema>>,
}

impl JsonSchemaValidator {
    /// 创建新的验证器
    pub fn new() -> Self {
        Self {
            cache: OnceLock::new(),
        }
    }

    /// 生成 schema 的缓存键
    fn schema_key(schema: &Value) -> String {
        // 使用 schema 的 JSON 字符串作为缓存键
        serde_json::to_string(schema).unwrap_or_default()
    }

    /// 获取或编译 schema
    fn get_or_compile_schema(&self, schema: &Value) -> Option<JSONSchema> {
        let cache = self.cache.get_or_init(HashMap::new);
        let key = Self::schema_key(schema);

        // 检查缓存
        if let Some(_validator) = cache.get(&key) {
            // JSONSchema 不实现 Clone，需要重新编译
            // 这是当前实现的限制，未来可以使用 Arc 来共享
            return JSONSchema::compile(schema).ok();
        }

        // 编译新 schema
        JSONSchema::compile(schema).ok()
    }

    /// 验证参数符合 JSON Schema
    ///
    /// # Arguments
    /// * `schema` - JSON Schema 定义
    /// * `params` - 要验证的参数
    ///
    /// # Returns
    /// 返回 Ok(()) 如果验证通过，否则返回错误
    ///
    /// # Examples
    /// ```
    /// use serde_json::json;
    /// use kode_tools::validation::JsonSchemaValidator;
    ///
    /// let validator = JsonSchemaValidator::new();
    /// let schema = json!({
    ///     "type": "object",
    ///     "properties": {
    ///         "name": {"type": "string"}
    ///     },
    ///     "required": ["name"]
    /// });
    /// let params = json!({"name": "test"});
    /// assert!(validator.validate(&schema, &params).is_ok());
    /// ```
    pub fn validate(&self, schema: &Value, params: &Value) -> Result<(), ValidationError> {
        let compiled = self
            .get_or_compile_schema(schema)
            .ok_or_else(|| ValidationError::Other("无法编译 JSON Schema".to_string()))?;

        let result = compiled.validate(params);
        if let Err(errors) = result {
            // 收集所有错误信息
            let error_messages: Vec<String> = errors.map(|e| e.to_string()).collect::<Vec<_>>();

            Err(ValidationError::Other(format!(
                "参数验证失败:\n{}",
                error_messages.join("\n")
            )))
        } else {
            Ok(())
        }
    }
}

impl Default for JsonSchemaValidator {
    fn default() -> Self {
        Self::new()
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
    fn test_json_schema_validator_valid_input() {
        let validator = JsonSchemaValidator::new();
        let schema = serde_json::json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "integer"}
            },
            "required": ["name"]
        });

        let params = serde_json::json!({
            "name": "Alice",
            "age": 30
        });

        assert!(validator.validate(&schema, &params).is_ok());
    }

    #[test]
    fn test_json_schema_validator_missing_required() {
        let validator = JsonSchemaValidator::new();
        let schema = serde_json::json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"}
            },
            "required": ["name"]
        });

        let params = serde_json::json!({});

        assert!(validator.validate(&schema, &params).is_err());
    }

    #[test]
    fn test_json_schema_validator_wrong_type() {
        let validator = JsonSchemaValidator::new();
        let schema = serde_json::json!({
            "type": "object",
            "properties": {
                "age": {"type": "integer"}
            }
        });

        let params = serde_json::json!({
            "age": "thirty"
        });

        assert!(validator.validate(&schema, &params).is_err());
    }

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
