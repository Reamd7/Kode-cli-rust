//! 文件编辑工具

use async_trait::async_trait;
use serde_json::{json, Value};
use std::path::PathBuf;
use std::sync::Arc;

use crate::file_utils::{compute_structured_diff, detect_line_endings, write_text_content};
use crate::secure_file::SecureFileService;
use crate::tool::{Tool, ToolContext, ToolResult, ToolSchema, ValidationResult};
use anyhow::Result;

/// 文件编辑工具
pub struct FileEditTool {
    /// 安全文件服务
    secure_service: Arc<SecureFileService>,
}

impl FileEditTool {
    /// 创建新的文件编辑工具
    pub fn new(secure_service: Arc<SecureFileService>) -> Self {
        Self { secure_service }
    }

    /// 检查文件是否为 Jupyter Notebook
    fn is_notebook_file(path: &str) -> bool {
        std::path::Path::new(path)
            .extension()
            .and_then(|e| e.to_str())
            .map(|ext| ext.eq_ignore_ascii_case("ipynb"))
            .unwrap_or(false)
    }

    /// 应用编辑
    fn apply_edit(content: &str, old_str: &str, new_str: &str) -> Result<String> {
        if !content.contains(old_str) {
            return Err(anyhow::anyhow!("未找到要替换的字符串"));
        }

        let count = content.matches(old_str).count();
        if count > 1 {
            return Err(anyhow::anyhow!(
                "找到 {} 处匹配，需要更精确的 old_str 以确保唯一匹配",
                count
            ));
        }

        Ok(content.replacen(old_str, new_str, 1))
    }

    /// 获取编辑片段（前后 N 行）
    fn get_snippet(content: &str, old_str: &str, context_lines: usize) -> String {
        let lines: Vec<&str> = content.lines().collect();

        // 找到 old_str 的位置
        let mut target_line = 0;
        for (i, line) in lines.iter().enumerate() {
            if line.contains(old_str) {
                target_line = i;
                break;
            }
        }

        let start = if target_line.checked_sub(context_lines).is_some() {
            target_line - context_lines
        } else {
            0
        };

        let end = (target_line + context_lines + 1).min(lines.len());

        lines[start..end]
            .iter()
            .enumerate()
            .map(|(i, line)| {
                let line_num = start + i + 1;
                let marker = if i == (target_line - start) {
                    ">>>"
                } else {
                    "   "
                };
                format!("{} {:4} | {}", marker, line_num, line)
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[async_trait]
impl Tool for FileEditTool {
    fn name(&self) -> &str {
        "Edit"
    }

    fn description(&self) -> &str {
        "编辑文件，精确字符串替换。确保 old_str 在文件中唯一匹配。"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: "Edit".to_string(),
            description: self.description().to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "file_path": {
                        "type": "string",
                        "description": "文件路径（绝对路径或相对路径）"
                    },
                    "old_string": {
                        "type": "string",
                        "description": "要替换的旧字符串（必须唯一匹配）"
                    },
                    "new_string": {
                        "type": "string",
                        "description": "新字符串"
                    }
                },
                "required": ["file_path", "old_string", "new_string"]
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

        // 检查是否为 Notebook 文件（拒绝编辑）
        if Self::is_notebook_file(file_path) {
            return Ok(ValidationResult::error(
                "不支持编辑 Jupyter Notebook 文件 (.ipynb)，请使用专用工具",
            ));
        }

        // 检查 old_string 参数
        let old_string = match params["old_string"].as_str() {
            Some(s) => s,
            None => return Ok(ValidationResult::error("old_string 参数缺失")),
        };

        // 检查 new_string 参数
        let new_string = match params["new_string"].as_str() {
            Some(s) => s,
            None => return Ok(ValidationResult::error("new_string 参数缺失")),
        };

        // 检查 old_string 和 new_string 不同
        if old_string == new_string {
            return Ok(ValidationResult::error("old_string 和 new_string 不能相同"));
        }

        Ok(ValidationResult::success())
    }

    async fn execute(&self, params: Value, context: &ToolContext) -> Result<ToolResult> {
        // 1. 获取参数
        let file_path = params["file_path"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("file_path 参数缺失"))?;

        let old_string = params["old_string"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("old_string 参数缺失"))?;

        let new_string = params["new_string"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("new_string 参数缺失"))?;

        // 2. 检查 old_string 和 new_string 不同
        if old_string == new_string {
            return Ok(ToolResult::error("old_string 和 new_string 相同"));
        }

        // 3. 验证路径
        let path = PathBuf::from(file_path);
        let _validated_path = self
            .secure_service
            .validate_path(&path)
            .await
            .map_err(|e| anyhow::anyhow!("路径验证失败: {}", e))?;

        // 4. 检查文件是否存在
        if !self.secure_service.safe_exists(&path).await {
            return Ok(ToolResult::error(format!("文件不存在: {}", file_path)));
        }

        // 5. 检查文件新鲜度
        if let Some(read_timestamp) = context.get_read_timestamp(&path) {
            let file_info = self
                .secure_service
                .safe_get_file_info(&path)
                .await
                .map_err(|e| anyhow::anyhow!("无法获取文件信息: {}", e))?;

            if file_info.modified > read_timestamp {
                return Ok(ToolResult::error(
                    "文件已被修改。请重新读取后再编辑。".to_string(),
                ));
            }
        }

        // 6. 读取文件内容
        let content =
            std::fs::read_to_string(&path).map_err(|e| anyhow::anyhow!("无法读取文件: {}", e))?;

        // 7. 生成编辑前片段
        let snippet_before = Self::get_snippet(&content, old_string, 3);

        // 8. 应用编辑
        let new_content = Self::apply_edit(&content, old_string, new_string)?;

        // 8.5. 计算差异
        let diff_info = compute_structured_diff(&content, &new_content);

        // 9. 检测编码和行结束符
        let encoding = match crate::file_utils::detect_encoding(&path) {
            Ok(enc) => enc,
            Err(_) => {
                // 如果检测失败，默认使用 UTF-8
                encoding_rs::UTF_8
            }
        };

        let line_ending = detect_line_endings(&content);

        // 10. 写入文件
        write_text_content(&path, &new_content, encoding, line_ending)?;

        // 11. 获取写入后的文件信息（用于更新时间戳）
        let new_file_info = self
            .secure_service
            .safe_get_file_info(&path)
            .await
            .map_err(|e| anyhow::anyhow!("无法获取文件信息: {}", e))?;

        // 12. 生成编辑后片段
        let snippet_after = Self::get_snippet(&new_content, new_string, 3);

        // 13. 返回结果
        let output = format!(
            "✓ 编辑成功\n\n修改前:\n{}\n\n修改后:\n{}\n\n差异统计: +{} 行, -{} 行, ~{} 行",
            snippet_before,
            snippet_after,
            diff_info.additions,
            diff_info.deletions,
            diff_info.modifications
        );

        Ok(ToolResult::success(output).with_read_update(path.clone(), new_file_info.modified))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_file_edit_tool_name() {
        let service = Arc::new(SecureFileService::default());
        let tool = FileEditTool::new(service);
        assert_eq!(tool.name(), "Edit");
    }

    #[tokio::test]
    async fn test_apply_edit() {
        let content = "line1\nline2\nline3";
        let result = FileEditTool::apply_edit(content, "line2", "line2-edited").unwrap();
        assert_eq!(result, "line1\nline2-edited\nline3");
    }

    #[tokio::test]
    async fn test_edit_file() {
        let temp_dir = TempDir::new().unwrap();
        let canonical_path = fs::canonicalize(temp_dir.path()).unwrap();
        let service = Arc::new(SecureFileService::new(vec![canonical_path]));

        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, "line1\nline2\nline3").unwrap();

        let tool = FileEditTool::new(service);
        let params = json!({
            "file_path": test_file.to_string_lossy().to_string(),
            "old_string": "line2",
            "new_string": "line2-edited"
        });

        let context = ToolContext::new(temp_dir.path().to_path_buf());
        let result = tool.execute(params, &context).await.unwrap();
        assert!(!result.is_error);
        assert!(result.output.contains("编辑成功"));

        let content = fs::read_to_string(&test_file).unwrap();
        assert_eq!(content, "line1\nline2-edited\nline3");
    }
}
