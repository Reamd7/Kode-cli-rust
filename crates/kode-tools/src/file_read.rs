//! 文件读取工具

use async_trait::async_trait;
use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::file_utils::{add_line_numbers, find_similar_file, read_text_content};
use crate::image_utils::process_image;
use crate::secure_file::SecureFileService;
use crate::tool::{Tool, ToolContext, ToolResult, ToolSchema, ValidationResult};
use anyhow::{Context, Result};

/// 文本文件最大大小 (250KB)
const MAX_TEXT_FILE_SIZE: usize = 250 * 1024;

/// 图片文件最大大小 (3.75MB)
const MAX_IMAGE_FILE_SIZE: usize = 3_750_000;

/// 文件读取工具
pub struct FileReadTool {
    /// 安全文件服务
    secure_service: Arc<SecureFileService>,
}

impl FileReadTool {
    /// 创建新的文件读取工具
    pub fn new(secure_service: Arc<SecureFileService>) -> Self {
        Self { secure_service }
    }

    /// 检测文件是否为图片
    fn is_image_file(path: &str) -> bool {
        let ext = std::path::Path::new(path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        matches!(
            ext.to_lowercase().as_str(),
            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp"
        )
    }

    /// 读取文本文件
    async fn read_text_file(
        &self,
        path: &Path,
        offset: usize,
        limit: Option<usize>,
    ) -> Result<ToolResult> {
        let read_result = read_text_content(path, offset, limit)?;

        // 如果没有限制且内容不为空，添加行号
        let content = if offset == 0 && limit.is_none() {
            add_line_numbers(&read_result.content, 0)
        } else {
            read_result.content
        };

        let metadata = json!({
            "line_count": read_result.line_count,
            "total_lines": read_result.total_lines,
            "start_line": read_result.start_line,
        });

        Ok(ToolResult::with_metadata(content, metadata))
    }

    /// 读取图片文件（自动处理尺寸和压缩）
    ///
    /// 使用 `process_image` 函数进行智能图片处理：
    /// - 自动调整超大图片尺寸（> 2000x2000）
    /// - 自动压缩超大文件（> 3.75MB）
    /// - 保持原始格式（除非需要压缩）
    async fn read_image_file(&self, path: &Path) -> Result<ToolResult> {
        // 使用图片处理工具
        process_image(path).with_context(|| format!("图片处理失败: {}", path.display()))
    }
}

#[async_trait]
impl Tool for FileReadTool {
    fn name(&self) -> &str {
        "Read"
    }

    fn description(&self) -> &str {
        "读取文件内容，支持行号范围和图片文件。文本文件返回带行号的内容，图片文件返回 Base64 编码。"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: "Read".to_string(),
            description: self.description().to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "file_path": {
                        "type": "string",
                        "description": "文件路径（绝对路径或相对路径）"
                    },
                    "offset": {
                        "type": "integer",
                        "description": "起始行号（从 0 开始）",
                        "default": 0
                    },
                    "limit": {
                        "type": "integer",
                        "description": "读取行数限制"
                    }
                },
                "required": ["file_path"]
            }),
        }
    }

    fn is_read_only(&self) -> bool {
        true
    }

    fn is_concurrency_safe(&self) -> bool {
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

        // 检查 offset 参数（如果存在）
        if let Some(offset) = params["offset"].as_u64() {
            if offset > usize::MAX as u64 {
                return Ok(ValidationResult::error("offset 值过大"));
            }
        }

        // 检查 limit 参数（如果存在）
        if let Some(limit) = params["limit"].as_u64() {
            if limit == 0 {
                return Ok(ValidationResult::error("limit 必须大于 0"));
            }
        }

        Ok(ValidationResult::success())
    }

    async fn execute(&self, params: Value, context: &ToolContext) -> Result<ToolResult> {
        // 1. 获取参数
        let file_path = params["file_path"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("file_path 参数缺失"))?;

        let offset = params["offset"].as_u64().unwrap_or(0) as usize;
        let limit = params["limit"].as_u64().map(|l| l as usize);

        // 2. 验证和安全检查路径
        let path = PathBuf::from(file_path);
        let validated_path = self
            .secure_service
            .validate_path(&path)
            .await
            .map_err(|e| anyhow::anyhow!("路径验证失败: {}", e))?;

        // 3. 检查文件是否存在
        if !self.secure_service.safe_exists(&validated_path).await {
            // 文件不存在时，尝试查找相似文件
            let file_name = validated_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");

            let parent_dir = validated_path.parent().unwrap_or(&context.cwd);

            let similar_files = find_similar_file(file_name, parent_dir, 5, 0.6);

            if !similar_files.is_empty() {
                let suggestions: Vec<String> = similar_files
                    .iter()
                    .filter_map(|p| p.file_name().and_then(|n| n.to_str()).map(String::from))
                    .collect();

                let suggestion_msg = if suggestions.len() == 1 {
                    format!(
                        "文件不存在: {}\n\n您是否想找: {}?",
                        file_path, suggestions[0]
                    )
                } else {
                    format!(
                        "文件不存在: {}\n\n相似的文件:\n{}",
                        file_path,
                        suggestions
                            .iter()
                            .enumerate()
                            .map(|(i, s)| format!("  {}. {}", i + 1, s))
                            .collect::<Vec<_>>()
                            .join("\n")
                    )
                };

                return Ok(ToolResult::error(suggestion_msg));
            }

            return Ok(ToolResult::error(format!("文件不存在: {}", file_path)));
        }

        // 4. 获取文件信息
        let file_info = self
            .secure_service
            .safe_get_file_info(&validated_path)
            .await
            .map_err(|e| anyhow::anyhow!("无法获取文件信息: {}", e))?;

        if !file_info.is_file {
            return Ok(ToolResult::error(format!("路径不是文件: {}", file_path)));
        }

        // 5. 文件大小检查
        let is_image = Self::is_image_file(file_path);
        let max_size = if is_image {
            MAX_IMAGE_FILE_SIZE
        } else {
            MAX_TEXT_FILE_SIZE
        };

        if file_info.size as usize > max_size {
            let has_offset = params.get("offset").is_some();
            let has_limit = params.get("limit").is_some();

            if !has_offset && !has_limit {
                return Ok(ToolResult::error(format!(
                    "文件过大 ({} bytes)。请使用 offset/limit 参数分块读取。",
                    file_info.size
                )));
            }
        }

        // 6. 根据文件类型读取
        let mut result = if is_image {
            self.read_image_file(&validated_path).await?
        } else {
            self.read_text_file(&validated_path, offset, limit).await?
        };

        // 7. 记录读取时间戳
        result
            .read_updates
            .push((validated_path.clone(), file_info.modified));

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_file_read_tool_name() {
        let service = Arc::new(SecureFileService::default());
        let tool = FileReadTool::new(service);
        assert_eq!(tool.name(), "Read");
    }

    #[tokio::test]
    async fn test_file_read_tool_is_read_only() {
        let service = Arc::new(SecureFileService::default());
        let tool = FileReadTool::new(service);
        assert!(tool.is_read_only());
        assert!(tool.is_concurrency_safe());
    }

    #[tokio::test]
    async fn test_read_text_file() {
        let temp_dir = TempDir::new().unwrap();
        let canonical_path = fs::canonicalize(temp_dir.path()).unwrap();
        let service = Arc::new(SecureFileService::new(vec![canonical_path]));

        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, "line1\nline2\nline3").unwrap();

        let tool = FileReadTool::new(service);
        let params = json!({
            "file_path": test_file.to_string_lossy().to_string(),
        });

        let context = ToolContext::new(temp_dir.path().to_path_buf());
        let result = tool.execute(params, &context).await.unwrap();
        assert!(!result.is_error);
        assert!(result.output.contains("line1"));
        assert!(result.output.contains("line2"));
    }
}
