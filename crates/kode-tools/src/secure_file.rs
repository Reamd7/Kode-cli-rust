//! 安全文件服务
//!
//! 提供安全的文件操作，包括路径验证、文件大小限制等

use anyhow::{Context, Result};
use regex::Regex;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use tokio::fs;

/// 最大路径长度
const MAX_PATH_LENGTH: usize = 4096;

/// 文本文件最大大小 (250KB)
const MAX_TEXT_FILE_SIZE: usize = 250 * 1024;

/// 图片文件最大大小 (3.75MB)
const MAX_IMAGE_FILE_SIZE: usize = 3_750_000;

/// 安全错误
#[derive(Debug, thiserror::Error)]
pub enum SecurityError {
    #[error("路径遍历攻击检测: {0}")]
    /// 路径遍历攻击
    PathTraversal(String),

    #[error("可疑模式检测: {0}")]
    /// 可疑模式
    SuspiciousPattern(String),

    #[error("路径不在允许列表中: {0}")]
    /// 路径不允许
    PathNotAllowed(String),

    #[error("文件过大: {0} bytes (最大 {1} bytes)")]
    /// 文件过大
    FileTooLarge(usize, usize),

    #[error("文件不存在: {0}")]
    /// 文件未找到
    FileNotFound(String),

    #[error("路径过长: {0} 字符 (最大 {1})")]
    /// 路径过长
    PathTooLong(usize, usize),
}

/// 文件信息
#[derive(Debug, Clone)]
pub struct FileInfo {
    /// 文件大小（字节）
    pub size: u64,
    /// 修改时间（Unix 时间戳）
    pub modified: u64,
    /// 是否为文件
    pub is_file: bool,
    /// 是否为目录
    pub is_dir: bool,
}

/// 安全文件服务
#[derive(Clone)]
pub struct SecureFileService {
    /// 允许的基础路径列表
    allowed_base_paths: Vec<PathBuf>,
    /// 最大文件大小
    max_file_size: usize,
    /// 允许的文件扩展名
    #[allow(dead_code)]
    allowed_extensions: HashSet<String>,
    /// 可疑模式正则表达式
    suspicious_patterns: Vec<Regex>,
}

impl SecureFileService {
    /// 创建新的安全文件服务
    pub fn new(allowed_base_paths: Vec<PathBuf>) -> Self {
        let suspicious_patterns = vec![
            // 检测 URL 编码的路径遍历
            Regex::new(r"%2e%2e").expect("Invalid regex"),
            Regex::new(r"%2E%2E").expect("Invalid regex"),
            // 检测混合编码
            Regex::new(r"\.\.%2f").expect("Invalid regex"),
            Regex::new(r"\.\.%2F").expect("Invalid regex"),
            // 检测常见的转义尝试
            Regex::new(r"\.\.\\").expect("Invalid regex"),
            Regex::new(r"~").expect("Invalid regex"),
        ];

        Self {
            allowed_base_paths,
            max_file_size: MAX_TEXT_FILE_SIZE,
            allowed_extensions: Self::default_allowed_extensions(),
            suspicious_patterns,
        }
    }

    /// 创建带自定义文件大小限制的服务
    pub fn with_max_size(allowed_base_paths: Vec<PathBuf>, max_size: usize) -> Self {
        let mut service = Self::new(allowed_base_paths);
        service.max_file_size = max_size;
        service
    }

    /// 获取默认允许的文件扩展名
    fn default_allowed_extensions() -> HashSet<String> {
        vec![
            "txt", "md", "rs", "toml", "json", "yaml", "yml", "js", "ts", "tsx", "jsx", "py", "go",
            "java", "c", "cpp", "h", "hpp", "cs", "php", "rb", "sh", "bash", "zsh", "fish", "ps1",
            "html", "css", "scss", "less", "svg", "png", "jpg", "jpeg", "gif", "bmp", "webp",
            "xml", "pdf", "csv", "tsv",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect()
    }

    /// 验证并规范化路径
    pub async fn validate_path(&self, path: &Path) -> Result<PathBuf> {
        // 1. 转换为字符串检查长度
        let path_str = path.to_string_lossy();
        if path_str.len() > MAX_PATH_LENGTH {
            return Err(SecurityError::PathTooLong(path_str.len(), MAX_PATH_LENGTH).into());
        }

        // 2. 规范化路径
        let normalized = fs::canonicalize(path)
            .await
            .unwrap_or_else(|_| path.to_path_buf());

        let normalized_str = normalized.to_string_lossy();

        // 3. 检测路径遍历攻击
        if normalized_str.contains("..") {
            return Err(SecurityError::PathTraversal("路径包含 ..".to_string()).into());
        }

        if normalized_str.contains('~') {
            return Err(SecurityError::PathTraversal("路径包含 ~".to_string()).into());
        }

        // 4. 检测可疑模式
        for pattern in &self.suspicious_patterns {
            if pattern.is_match(&normalized_str) {
                return Err(SecurityError::SuspiciousPattern(normalized_str.to_string()).into());
            }
        }

        // 5. 白名单检查
        if !self.is_in_allowed_paths(&normalized) {
            return Err(SecurityError::PathNotAllowed(normalized_str.to_string()).into());
        }

        Ok(normalized)
    }

    /// 检查路径是否在允许列表中
    fn is_in_allowed_paths(&self, path: &Path) -> bool {
        self.allowed_base_paths
            .iter()
            .any(|base| path.starts_with(base))
    }

    /// 安全检查文件是否存在
    pub async fn safe_exists(&self, path: &Path) -> bool {
        fs::metadata(path).await.is_ok()
    }

    /// 安全获取文件信息
    pub async fn safe_get_file_info(&self, path: &Path) -> Result<FileInfo> {
        let metadata = fs::metadata(path)
            .await
            .with_context(|| format!("无法获取文件元数据: {:?}", path))?;

        let modified = metadata
            .modified()
            .with_context(|| format!("无法获取修改时间: {:?}", path))?
            .duration_since(std::time::UNIX_EPOCH)
            .with_context(|| format!("无效的修改时间: {:?}", path))?
            .as_millis() as u64;

        Ok(FileInfo {
            size: metadata.len(),
            modified,
            is_file: metadata.is_file(),
            is_dir: metadata.is_dir(),
        })
    }

    /// 检查文件大小是否超过限制
    pub fn check_file_size(&self, size: usize, is_image: bool) -> Result<()> {
        let max_size = if is_image {
            MAX_IMAGE_FILE_SIZE
        } else {
            self.max_file_size
        };

        if size > max_size {
            return Err(SecurityError::FileTooLarge(size, max_size).into());
        }

        Ok(())
    }

    /// 获取最大文件大小
    pub fn max_file_size(&self) -> usize {
        self.max_file_size
    }
}

impl Default for SecureFileService {
    fn default() -> Self {
        Self::new(vec![PathBuf::from(".")])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_validate_path_success() {
        let temp_dir = TempDir::new().unwrap();
        // Use canonicalized path for the service
        let canonical_path = fs::canonicalize(temp_dir.path()).unwrap();
        let service = SecureFileService::new(vec![canonical_path]);

        let test_file = temp_dir.path().join("test.txt");
        fs::File::create(&test_file).unwrap();

        let validated = service.validate_path(&test_file).await.unwrap();
        assert!(validated.exists());
    }

    #[tokio::test]
    async fn test_validate_path_traversal() {
        let temp_dir = TempDir::new().unwrap();
        let canonical_path = fs::canonicalize(temp_dir.path()).unwrap();
        let service = SecureFileService::new(vec![canonical_path]);

        // 尝试访问父目录
        let traversal_path = temp_dir.path().join("..").join("etc");
        let result = service.validate_path(&traversal_path).await;

        // 应该失败或规范化到安全路径
        assert!(result.is_err() || result.unwrap().ends_with("etc"));
    }

    #[tokio::test]
    async fn test_safe_get_file_info() {
        let temp_dir = TempDir::new().unwrap();
        let canonical_path = fs::canonicalize(temp_dir.path()).unwrap();
        let service = SecureFileService::new(vec![canonical_path]);

        let test_file = temp_dir.path().join("test.txt");
        let mut file = fs::File::create(&test_file).unwrap();
        file.write_all(b"test content").unwrap();

        let info = service.safe_get_file_info(&test_file).await.unwrap();
        assert!(info.is_file);
        assert!(!info.is_dir);
        assert_eq!(info.size, 12);
    }

    #[tokio::test]
    async fn test_check_file_size() {
        let service = SecureFileService::default();

        // 小文件应该通过
        assert!(service.check_file_size(1024, false).is_ok());

        // 大文本文件应该失败
        assert!(service.check_file_size(500_000, false).is_err());

        // 图片文件限制更大
        assert!(service.check_file_size(3_000_000, true).is_ok());
        assert!(service.check_file_size(5_000_000, true).is_err());
    }

    #[tokio::test]
    async fn test_safe_exists() {
        let temp_dir = TempDir::new().unwrap();
        let canonical_path = fs::canonicalize(temp_dir.path()).unwrap();
        let service = SecureFileService::new(vec![canonical_path]);

        let test_file = temp_dir.path().join("test.txt");
        assert!(!service.safe_exists(&test_file).await);

        fs::File::create(&test_file).unwrap();
        assert!(service.safe_exists(&test_file).await);
    }
}
