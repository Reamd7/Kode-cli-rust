//! 文件新鲜度追踪服务
//!
//! 追踪文件的读取和编辑时间戳，检测外部修改冲突。

use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::time::SystemTime;

/// 文件时间戳信息
#[derive(Debug, Clone)]
pub struct FileTimestamp {
    /// 文件路径
    pub path: String,
    /// 最后读取时间
    pub last_read: u64,
    /// 最后修改时间（文件系统）
    pub last_modified: u64,
    /// 文件大小（字节）
    pub size: u64,
    /// Agent 最后编辑时间
    pub last_agent_edit: Option<u64>,
}

/// 文件新鲜度状态
#[derive(Debug, Clone)]
pub struct FreshnessStatus {
    /// 文件是否新鲜（未修改）
    pub is_fresh: bool,
    /// 最后读取时间
    pub last_read: Option<u64>,
    /// 当前修改时间
    pub current_modified: Option<u64>,
    /// 是否存在冲突
    pub conflict: bool,
}

/// 文件新鲜度追踪服务
///
/// 追踪文件的读取和修改，用于检测外部编辑冲突。
#[derive(Debug, Clone, Default)]
pub struct FileFreshnessService {
    /// 文件时间戳记录
    read_timestamps: HashMap<String, FileTimestamp>,
    /// 编辑冲突集合
    edit_conflicts: HashSet<String>,
    /// 会话文件集合
    session_files: HashSet<String>,
    /// TODO 文件监控 (agent_id -> file_path)
    watched_todo_files: HashMap<String, String>,
}

impl FileFreshnessService {
    /// 创建新的文件新鲜度服务
    pub fn new() -> Self {
        Self::default()
    }

    /// 记录文件读取操作
    ///
    /// 记录文件的读取时间戳和当前修改时间。
    ///
    /// # Arguments
    /// * `file_path` - 文件路径
    pub fn record_file_read(&mut self, file_path: &str) {
        if let Ok(metadata) = std::fs::metadata(file_path) {
            let modified = metadata
                .modified()
                .ok()
                .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
                .map(|d| d.as_millis() as u64)
                .unwrap_or(0);

            let size = metadata.len();

            let timestamp = FileTimestamp {
                path: file_path.to_string(),
                last_read: Self::current_time(),
                last_modified: modified,
                size,
                last_agent_edit: None,
            };

            self.read_timestamps.insert(file_path.to_string(), timestamp);
            self.session_files.insert(file_path.to_string());
        }
    }

    /// 记录文件编辑操作
    ///
    /// 更新文件编辑后的时间戳。
    ///
    /// # Arguments
    /// * `file_path` - 文件路径
    pub fn record_file_edit(&mut self, file_path: &str) {
        let now = Self::current_time();

        // 更新记录的时间戳
        if let Ok(metadata) = std::fs::metadata(file_path) {
            let modified = metadata
                .modified()
                .ok()
                .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
                .map(|d| d.as_millis() as u64)
                .unwrap_or(0);

            let size = metadata.len();

            if let Some(existing) = self.read_timestamps.get_mut(file_path) {
                existing.last_modified = modified;
                existing.size = size;
                existing.last_agent_edit = Some(now);
            } else {
                // 创建新记录
                let timestamp = FileTimestamp {
                    path: file_path.to_string(),
                    last_read: now,
                    last_modified: modified,
                    size,
                    last_agent_edit: Some(now),
                };
                self.read_timestamps.insert(file_path.to_string(), timestamp);
            }
        }

        // 从冲突中移除（我们刚刚编辑了它）
        self.edit_conflicts.remove(file_path);
    }

    /// 检查文件新鲜度
    ///
    /// 检查文件是否在最后读取后被修改。
    ///
    /// # Arguments
    /// * `file_path` - 文件路径
    ///
    /// # Returns
    /// 新鲜度状态
    pub fn check_file_freshness(&self, file_path: &str) -> FreshnessStatus {
        let recorded = match self.read_timestamps.get(file_path) {
            Some(r) => r,
            None => {
                return FreshnessStatus {
                    is_fresh: true,
                    last_read: None,
                    current_modified: None,
                    conflict: false,
                }
            }
        };

        // 检查文件是否存在
        if !Path::new(file_path).exists() {
            return FreshnessStatus {
                is_fresh: false,
                last_read: Some(recorded.last_read),
                current_modified: None,
                conflict: true,
            };
        }

        // 检查修改时间
        if let Ok(metadata) = std::fs::metadata(file_path) {
            let current_modified = metadata
                .modified()
                .ok()
                .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
                .map(|d| d.as_millis() as u64)
                .unwrap_or(0);

            let is_fresh = current_modified <= recorded.last_modified;
            let conflict = !is_fresh;

            FreshnessStatus {
                is_fresh,
                last_read: Some(recorded.last_read),
                current_modified: Some(current_modified),
                conflict,
            }
        } else {
            FreshnessStatus {
                is_fresh: false,
                last_read: Some(recorded.last_read),
                current_modified: None,
                conflict: true,
            }
        }
    }

    /// 生成文件修改提醒
    ///
    /// 如果文件在最后读取后被外部修改，生成提醒消息。
    ///
    /// # Arguments
    /// * `file_path` - 文件路径
    ///
    /// # Returns
    /// 提醒消息（如果没有修改则返回 None）
    pub fn generate_file_modification_reminder(&self, file_path: &str) -> Option<String> {
        let recorded = self.read_timestamps.get(file_path)?;

        // 检查文件是否存在
        if !Path::new(file_path).exists() {
            return Some(format!(
                "Note: {} was deleted since last read.",
                file_path
            ));
        }

        // 检查修改时间
        if let Ok(metadata) = std::fs::metadata(file_path) {
            let current_modified = metadata
                .modified()
                .ok()
                .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
                .map(|d| d.as_millis() as u64)
                .unwrap_or(0);

            let is_modified = current_modified > recorded.last_modified;

            if !is_modified {
                return None;
            }

            // 检查是否为 Agent 修改的
            const TIME_TOLERANCE_MS: u64 = 100;
            if let Some(last_agent_edit) = recorded.last_agent_edit {
                if last_agent_edit >= recorded.last_modified.saturating_sub(TIME_TOLERANCE_MS) {
                    // Agent 最近修改了这个文件，不需要提醒
                    return None;
                }
            }

            // 外部修改检测
            Some(format!(
                "Note: {} was modified externally since last read. The file may have changed outside of this session.",
                file_path
            ))
        } else {
            Some(format!(
                "Note: {} is no longer accessible.",
                file_path
            ))
        }
    }

    /// 获取重要文件列表
    ///
    /// 返回最近访问的文件，用于上下文恢复。
    ///
    /// # Arguments
    /// * `max_files` - 最大返回文件数量
    ///
    /// # Returns
    /// 按访问时间排序的文件信息列表
    pub fn get_important_files(&self, max_files: usize) -> Vec<FileTimestamp> {
        let mut files: Vec<_> = self
            .read_timestamps
            .values()
            .filter(|info| Self::is_valid_for_recovery(&info.path))
            .cloned()
            .map(|mut info| {
                // 移除 last_agent_edit 以便克隆
                info.last_agent_edit = None;
                info
            })
            .collect();

        // 按最后读取时间排序（最新的在前）
        files.sort_by(|a, b| b.last_read.cmp(&a.last_read));

        files.truncate(max_files);
        files
    }

    /// 重置会话状态
    ///
    /// 清除所有会话相关的状态。
    pub fn reset_session(&mut self) {
        self.read_timestamps.clear();
        self.edit_conflicts.clear();
        self.session_files.clear();
        self.watched_todo_files.clear();
    }

    /// 获取冲突文件列表
    ///
    /// # Returns
    /// 有编辑冲突的文件路径列表
    pub fn get_conflicted_files(&self) -> Vec<String> {
        self.edit_conflicts.iter().cloned().collect()
    }

    /// 获取会话文件列表
    ///
    /// # Returns
    /// 会话期间访问的文件路径列表
    pub fn get_session_files(&self) -> Vec<String> {
        self.session_files.iter().cloned().collect()
    }

    /// 获取文件信息
    ///
    /// # Arguments
    /// * `file_path` - 文件路径
    ///
    /// # Returns
    /// 文件时间戳信息（如果存在）
    pub fn get_file_info(&self, file_path: &str) -> Option<FileTimestamp> {
        self.read_timestamps.get(file_path).cloned()
    }

    /// 检查文件是否被追踪
    ///
    /// # Arguments
    /// * `file_path` - 文件路径
    ///
    /// # Returns
    /// 是否正在追踪该文件
    pub fn is_file_tracked(&self, file_path: &str) -> bool {
        self.read_timestamps.contains_key(file_path)
    }

    /// 开始监控 TODO 文件
    ///
    /// # Arguments
    /// * `agent_id` - Agent ID
    /// * `file_path` - TODO 文件路径
    pub fn start_watching_todo_file(&mut self, agent_id: &str, file_path: &str) {
        self.watched_todo_files.insert(agent_id.to_string(), file_path.to_string());

        // 记录初始状态
        if Path::new(file_path).exists() {
            self.record_file_read(file_path);
        }
    }

    /// 停止监控 TODO 文件
    ///
    /// # Arguments
    /// * `agent_id` - Agent ID
    pub fn stop_watching_todo_file(&mut self, agent_id: &str) {
        self.watched_todo_files.remove(agent_id);
    }

    /// 获取当前时间（毫秒时间戳）
    fn current_time() -> u64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0)
    }

    /// 判断文件是否适合恢复
    ///
    /// 过滤掉不重要的文件（依赖、构建产物等）。
    ///
    /// # Arguments
    /// * `file_path` - 文件路径
    ///
    /// # Returns
    /// 是否适合用于上下文恢复
    fn is_valid_for_recovery(file_path: &str) -> bool {
        let invalid_patterns = [
            "node_modules",
            ".git",
            "/tmp",
            ".cache",
            "dist/",
            "build/",
            "target/",
            "__pycache__",
            ".venv",
            "venv/",
        ];

        !invalid_patterns.iter().any(|pattern| file_path.contains(pattern))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::PathBuf;

    fn create_temp_file(content: &str) -> PathBuf {
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join(format!("test_file_{}.txt", uuid::Uuid::new_v4()));

        let mut file = File::create(&file_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();

        file_path
    }

    #[test]
    fn test_record_file_read() {
        let mut service = FileFreshnessService::new();
        let temp_file = create_temp_file("Hello, World!");
        let file_path = temp_file.to_str().unwrap();

        service.record_file_read(file_path);

        assert!(service.is_file_tracked(file_path));
        assert!(service.get_session_files().contains(&file_path.to_string()));

        // 清理
        fs::remove_file(file_path).ok();
    }

    #[test]
    fn test_check_file_freshness() {
        let mut service = FileFreshnessService::new();
        let temp_file = create_temp_file("Initial content");
        let file_path = temp_file.to_str().unwrap();

        service.record_file_read(file_path);

        let status = service.check_file_freshness(file_path);
        assert!(status.is_fresh);
        assert!(!status.conflict);

        // 清理
        fs::remove_file(file_path).ok();
    }

    #[test]
    fn test_get_important_files() {
        let mut service = FileFreshnessService::new();
        let temp_file1 = create_temp_file("File 1");
        let temp_file2 = create_temp_file("File 2");

        let path1 = temp_file1.to_str().unwrap();
        let path2 = temp_file2.to_str().unwrap();

        service.record_file_read(path1);
        std::thread::sleep(std::time::Duration::from_millis(10));
        service.record_file_read(path2);

        let important = service.get_important_files(10);
        assert_eq!(important.len(), 2);

        // 最新的文件应该在前
        assert_eq!(important[0].path, path2);
        assert_eq!(important[1].path, path1);

        // 清理
        fs::remove_file(path1).ok();
        fs::remove_file(path2).ok();
    }

    #[test]
    fn test_reset_session() {
        let mut service = FileFreshnessService::new();
        let temp_file = create_temp_file("Test");
        let file_path = temp_file.to_str().unwrap();

        service.record_file_read(file_path);
        assert!(service.is_file_tracked(file_path));

        service.reset_session();
        assert!(!service.is_file_tracked(file_path));
        assert!(service.get_session_files().is_empty());

        // 清理
        fs::remove_file(file_path).ok();
    }

    #[test]
    fn test_is_valid_for_recovery() {
        assert!(!FileFreshnessService::is_valid_for_recovery("node_modules/package.json"));
        assert!(!FileFreshnessService::is_valid_for_recovery("/tmp/file.txt"));
        assert!(!FileFreshnessService::is_valid_for_recovery("target/debug/test"));

        assert!(FileFreshnessService::is_valid_for_recovery("src/main.rs"));
        assert!(FileFreshnessService::is_valid_for_recovery("README.md"));
    }

    #[test]
    fn test_watched_todo_files() {
        let mut service = FileFreshnessService::new();

        service.start_watching_todo_file("agent-1", "/tmp/todo.md");

        assert_eq!(
            service.watched_todo_files.get("agent-1"),
            Some(&"/tmp/todo.md".to_string())
        );

        service.stop_watching_todo_file("agent-1");

        assert_eq!(service.watched_todo_files.get("agent-1"), None);
    }
}
