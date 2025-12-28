//! 文件写入工具

use async_trait::async_trait;
use serde_json::{json, Value};
use std::path::PathBuf;
use std::sync::Arc;

use crate::file_utils::{detect_line_endings, detect_repo_line_endings, write_text_content};
use crate::secure_file::SecureFileService;
use crate::tool::{Tool, ToolContext, ToolResult, ToolSchema, ValidationResult};
use anyhow::Result;

/// 文件写入工具
pub struct FileWriteTool {
    /// 安全文件服务
    secure_service: Arc<SecureFileService>,
}

impl FileWriteTool {
    /// 创建新的文件写入工具
    pub fn new(secure_service: Arc<SecureFileService>) -> Self {
        Self { secure_service }
    }
}

#[async_trait]
impl Tool for FileWriteTool {
    fn name(&self) -> &str {
        "Write"
    }

    fn description(&self) -> &str {
        "写入文件内容，创建或覆盖文件。自动创建父目录，检测并保持文件编码和行结束符。"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: "Write".to_string(),
            description: self.description().to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "file_path": {
                        "type": "string",
                        "description": "文件路径（绝对路径或相对路径）"
                    },
                    "content": {
                        "type": "string",
                        "description": "要写入的内容"
                    }
                },
                "required": ["file_path", "content"]
            }),
        }
    }

    fn is_read_only(&self) -> bool {
        false
    }

    fn is_concurrency_safe(&self) -> bool {
        false
    }

    fn needs_permission(&self, _params: &Value) -> bool {
        true
    }

    fn validate_input(&self, params: &Value, _context: &ToolContext) -> Result<ValidationResult> {
        // 检查 file_path 参数
        let file_path = match params["file_path"].as_str() {
            Some(p) => p,
            None => return Ok(ValidationResult::error("file_path 参数缺失")),
        };

        // 检查路径是否为空
        if file_path.trim().is_empty() {
            return Ok(ValidationResult::error("file_path 不能为空"));
        }

        // 检查 content 参数
        let content = match params["content"].as_str() {
            Some(c) => c,
            None => return Ok(ValidationResult::error("content 参数缺失")),
        };

        // 检查 content 是否为空（允许空字符串）
        let _content = content;

        Ok(ValidationResult::success())
    }

    async fn execute(&self, params: Value, context: &ToolContext) -> Result<ToolResult> {
        // 1. 获取参数
        let file_path = params["file_path"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("file_path 参数缺失"))?;

        let content = params["content"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("content 参数缺失"))?;

        // 2. 验证路径
        let path = PathBuf::from(file_path);
        let _validated_path = self
            .secure_service
            .validate_path(&path)
            .await
            .map_err(|e| anyhow::anyhow!("路径验证失败: {}", e))?;

        // 3. 检查文件是否存在
        let file_exists = self.secure_service.safe_exists(&path).await;

        // 4. 检查文件新鲜度
        if file_exists {
            if let Some(read_timestamp) = context.get_read_timestamp(&path) {
                let file_info = self
                    .secure_service
                    .safe_get_file_info(&path)
                    .await
                    .map_err(|e| anyhow::anyhow!("无法获取文件信息: {}", e))?;

                if file_info.modified > read_timestamp {
                    return Ok(ToolResult::error(format!(
                        "文件已被修改（读取时间: {}, 当前时间: {}）。请重新读取后再写入。",
                        read_timestamp, file_info.modified
                    )));
                }
            }
        }

        // 5. 确定编码和行结束符
        let encoding = if file_exists {
            // 现有文件：检测编码
            match crate::file_utils::detect_encoding(&path) {
                Ok(enc) => enc,
                Err(_) => {
                    // 如果检测失败，默认使用 UTF-8
                    // 注意：我们无法轻易检测编码，因为文件可能不是 UTF-8
                    encoding_rs::UTF_8
                }
            }
        } else {
            // 新文件：使用 UTF-8
            encoding_rs::UTF_8
        };

        let line_ending = if file_exists {
            // 读取现有文件检测行结束符
            if let Ok(existing_content) = std::fs::read_to_string(&path) {
                detect_line_endings(&existing_content)
            } else {
                // 如果无法读取，检测仓库行结束符
                detect_repo_line_endings(&context.cwd).unwrap_or(crate::file_utils::LineEnding::LF)
            }
        } else {
            // 新文件：检测仓库行结束符
            detect_repo_line_endings(&context.cwd).unwrap_or(crate::file_utils::LineEnding::LF)
        };

        // 6. 写入文件
        write_text_content(&path, content, encoding, line_ending)
            .map_err(|e| anyhow::anyhow!("写入文件失败: {}", e))?;

        // 7. 获取写入后的文件信息（用于更新时间戳）
        let new_file_info = self
            .secure_service
            .safe_get_file_info(&path)
            .await
            .map_err(|e| anyhow::anyhow!("无法获取文件信息: {}", e))?;

        // 8. 返回结果
        let line_count = content.lines().count();
        let operation = if file_exists { "update" } else { "create" };

        let metadata = json!({
            "operation": operation,
            "line_count": line_count,
        });

        Ok(ToolResult::with_metadata(
            format!("文件已{}（{} 行）", operation, line_count),
            metadata,
        )
        .with_read_update(path.clone(), new_file_info.modified))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_file_write_tool_name() {
        let service = Arc::new(SecureFileService::default());
        let tool = FileWriteTool::new(service);
        assert_eq!(tool.name(), "Write");
    }

    #[tokio::test]
    async fn test_write_new_file() {
        let temp_dir = TempDir::new().unwrap();
        let canonical_path = fs::canonicalize(temp_dir.path()).unwrap();
        let service = Arc::new(SecureFileService::new(vec![canonical_path.clone()]));

        let test_file = canonical_path.join("new_file.txt");

        let tool = FileWriteTool::new(service);
        let params = json!({
            "file_path": test_file.to_string_lossy().to_string(),
            "content": "Hello, World!"
        });

        let context = ToolContext::new(canonical_path);
        let result = tool.execute(params, &context).await.unwrap();
        assert!(!result.is_error);
        assert!(result.output.contains("create"));

        let content = fs::read_to_string(&test_file).unwrap();
        assert_eq!(content, "Hello, World!");
    }
}
