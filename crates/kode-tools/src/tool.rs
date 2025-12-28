//! Tool trait 定义

use crate::events::FileOperationHistory;
use crate::validation::JsonSchemaValidator;
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio_util::sync::CancellationToken;

/// 取消错误
#[derive(Debug, thiserror::Error)]
#[error("操作已取消")]
pub struct CancellationError;

/// Tool trait
#[async_trait]
pub trait Tool: Send + Sync {
    /// 工具名称
    fn name(&self) -> &str;

    /// 工具描述
    fn description(&self) -> &str;

    /// 工具 schema
    fn schema(&self) -> ToolSchema;

    /// 执行工具
    async fn execute(&self, params: Value, context: &ToolContext) -> Result<ToolResult>;

    /// 是否为只读工具（默认 false）
    fn is_read_only(&self) -> bool {
        false
    }

    /// 是否并发安全（默认 false）
    fn is_concurrency_safe(&self) -> bool {
        false
    }

    /// 是否需要权限确认（默认 false）
    fn needs_permission(&self, _params: &Value) -> bool {
        false
    }

    /// 验证输入参数（默认通过）
    fn validate_input(&self, params: &Value, _context: &ToolContext) -> Result<ValidationResult> {
        // 默认实现：只验证 JSON Schema
        self.schema().validate(params)?;
        Ok(ValidationResult::success())
    }
}

/// 工具 Schema
#[derive(Debug, Clone)]
pub struct ToolSchema {
    /// 工具名称
    pub name: String,
    /// 工具描述
    pub description: String,
    /// 参数 schema (JSON Schema)
    pub parameters: Value,
}

impl ToolSchema {
    /// 验证参数是否符合 schema
    ///
    /// 使用 JSON Schema 验证器进行参数验证
    pub fn validate(&self, params: &Value) -> Result<()> {
        // 使用 JsonSchemaValidator 进行验证
        let validator = JsonSchemaValidator::new();

        validator
            .validate(&self.parameters, params)
            .map_err(|e| anyhow::anyhow!("参数验证失败: {}", e))?;

        Ok(())
    }
}

/// 工具上下文
#[derive(Debug, Clone)]
pub struct ToolContext {
    /// 当前工作目录
    pub cwd: PathBuf,
    /// 文件读取时间戳追踪（文件路径 -> 修改时间）
    pub read_timestamps: HashMap<PathBuf, u64>,
    /// 安全模式标志
    pub safe_mode: bool,
    /// 取消令牌
    pub cancellation_token: Option<CancellationToken>,
    /// 文件操作历史记录
    pub file_operations: FileOperationHistory,
}

impl ToolContext {
    /// 创建新的工具上下文
    pub fn new(cwd: PathBuf) -> Self {
        Self {
            cwd,
            read_timestamps: HashMap::new(),
            safe_mode: false,
            cancellation_token: None,
            file_operations: FileOperationHistory::new(),
        }
    }

    /// 创建带安全模式的上下文
    pub fn with_safe_mode(cwd: PathBuf) -> Self {
        Self {
            cwd,
            read_timestamps: HashMap::new(),
            safe_mode: true,
            cancellation_token: None,
            file_operations: FileOperationHistory::new(),
        }
    }

    /// 记录文件读取时间戳（返回新的上下文）
    pub fn with_read_timestamp(mut self, path: PathBuf, timestamp: u64) -> Self {
        self.read_timestamps.insert(path, timestamp);
        self
    }

    /// 获取文件读取时间戳
    pub fn get_read_timestamp(&self, path: &PathBuf) -> Option<u64> {
        self.read_timestamps.get(path).copied()
    }

    /// 记录文件读取时间戳（可变引用版本）
    pub fn track_read(&mut self, path: PathBuf, timestamp: u64) {
        self.read_timestamps.insert(path, timestamp);
    }

    /// 设置取消令牌
    pub fn set_cancellation_token(&mut self, token: CancellationToken) {
        self.cancellation_token = Some(token);
    }

    /// 检查是否已取消
    ///
    /// 如果操作已被取消，返回 CancellationError
    /// 这个方法应该在长时间运行的操作中定期调用
    pub fn check_cancelled(&self) -> Result<(), CancellationError> {
        if let Some(token) = &self.cancellation_token {
            if token.is_cancelled() {
                return Err(CancellationError);
            }
        }
        Ok(())
    }

    /// 检查是否已取消（异步版本）
    ///
    /// 这个版本会等待取消信号
    pub async fn check_cancelled_async(&self) -> Result<(), CancellationError> {
        if let Some(token) = &self.cancellation_token {
            if token.is_cancelled() {
                return Err(CancellationError);
            }
            // 可以选择使用 token.cancelled().await 来等待取消信号
            // 但这会阻塞，所以这里只检查状态
        }
        Ok(())
    }

    /// 记录文件操作
    pub fn track_operation(&mut self, event: crate::events::FileOperationEvent) {
        self.file_operations.add(event);
    }

    /// 获取文件操作历史
    pub fn get_file_history(&self, path: &Path) -> Vec<crate::events::FileOperationEvent> {
        self.file_operations.get_file_history(path)
    }

    /// 生成操作报告
    pub fn generate_operation_report(&self) -> String {
        self.file_operations.generate_report()
    }
}

/// 工具执行结果
#[derive(Debug, Clone)]
pub struct ToolResult {
    /// 输出内容
    pub output: String,
    /// 元数据
    pub metadata: Option<Value>,
    /// 是否为错误
    pub is_error: bool,
    /// 更新的读取时间戳（用于追踪文件修改）
    pub read_updates: Vec<(PathBuf, u64)>,
}

impl ToolResult {
    /// 创建成功结果
    pub fn success(output: impl Into<String>) -> Self {
        Self {
            output: output.into(),
            metadata: None,
            is_error: false,
            read_updates: Vec::new(),
        }
    }

    /// 创建带元数据的结果
    pub fn with_metadata(output: impl Into<String>, metadata: Value) -> Self {
        Self {
            output: output.into(),
            metadata: Some(metadata),
            is_error: false,
            read_updates: Vec::new(),
        }
    }

    /// 创建错误结果
    pub fn error(output: impl Into<String>) -> Self {
        Self {
            output: output.into(),
            metadata: None,
            is_error: true,
            read_updates: Vec::new(),
        }
    }

    /// 添加读取时间戳更新
    pub fn with_read_update(mut self, path: PathBuf, timestamp: u64) -> Self {
        self.read_updates.push((path, timestamp));
        self
    }
}

/// 参数验证结果
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// 是否通过验证
    pub result: bool,
    /// 验证消息
    pub message: Option<String>,
    /// 错误代码
    pub error_code: Option<u32>,
    /// 元数据
    pub meta: Option<Value>,
}

impl ValidationResult {
    /// 创建成功的验证结果
    pub fn success() -> Self {
        Self {
            result: true,
            message: None,
            error_code: None,
            meta: None,
        }
    }

    /// 创建失败的验证结果
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            result: false,
            message: Some(message.into()),
            error_code: None,
            meta: None,
        }
    }

    /// 创建带错误码的验证结果
    pub fn error_with_code(message: impl Into<String>, code: u32) -> Self {
        Self {
            result: false,
            message: Some(message.into()),
            error_code: Some(code),
            meta: None,
        }
    }

    /// 创建带元数据的验证结果
    pub fn with_meta(message: impl Into<String>, meta: Value) -> Self {
        Self {
            result: false,
            message: Some(message.into()),
            error_code: None,
            meta: Some(meta),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_result_success() {
        let result = ValidationResult::success();
        assert!(result.result);
        assert!(result.message.is_none());
    }

    #[test]
    fn test_validation_result_error() {
        let result = ValidationResult::error("Invalid input");
        assert!(!result.result);
        assert_eq!(result.message, Some("Invalid input".to_string()));
    }

    #[test]
    fn test_tool_result() {
        let success = ToolResult::success("OK");
        assert!(!success.is_error);
        assert_eq!(success.output, "OK");

        let error = ToolResult::error("Failed");
        assert!(error.is_error);
        assert_eq!(error.output, "Failed");
    }

    #[test]
    fn test_tool_context() {
        let mut ctx = ToolContext::new(PathBuf::from("/tmp"));
        assert_eq!(ctx.cwd, PathBuf::from("/tmp"));
        assert!(!ctx.safe_mode);

        ctx.track_read(PathBuf::from("/tmp/test.txt"), 12345);
        assert_eq!(
            ctx.get_read_timestamp(&PathBuf::from("/tmp/test.txt")),
            Some(12345)
        );
    }

    #[test]
    fn test_cancellation_not_set() {
        let context = ToolContext::new(PathBuf::from("/test"));
        assert!(context.check_cancelled().is_ok());
    }

    #[test]
    fn test_cancellation_token() {
        let token = tokio_util::sync::CancellationToken::new();
        let mut context = ToolContext::new(PathBuf::from("/test"));
        context.set_cancellation_token(token.clone());

        // 未取消
        assert!(context.check_cancelled().is_ok());

        // 取消
        token.cancel();
        assert!(context.check_cancelled().is_err());
    }

    #[tokio::test]
    async fn test_cancellation_token_async() {
        let token = tokio_util::sync::CancellationToken::new();
        let mut context = ToolContext::new(PathBuf::from("/test"));
        context.set_cancellation_token(token.clone());

        // 未取消
        assert!(context.check_cancelled_async().await.is_ok());

        // 取消
        token.cancel();
        assert!(context.check_cancelled_async().await.is_err());
    }
}
