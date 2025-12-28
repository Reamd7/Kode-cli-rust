//! 事件系统
//!
//! 提供文件操作事件的追踪和通知功能。

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;

/// 文件操作类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FileOperationType {
    /// 读取文件
    Read,
    /// 写入文件（创建）
    Create,
    /// 写入文件（更新）
    Update,
    /// 编辑文件
    Edit,
}

/// 文件操作事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileOperationEvent {
    /// 操作类型
    pub operation: FileOperationType,
    /// 文件路径
    pub path: PathBuf,
    /// 时间戳（Unix 时间戳，秒）
    pub timestamp: u64,
    /// 文件大小（字节）
    pub size: Option<usize>,
    /// 是否为图片
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_image: Option<bool>,
    /// 行数（对于文本文件）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<usize>,
    /// 修改的字符数（对于编辑操作）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changes: Option<usize>,
    /// 原始内容长度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_length: Option<usize>,
    /// 新内容长度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_length: Option<usize>,
}

impl FileOperationEvent {
    /// 创建文件读取事件
    pub fn read(path: PathBuf, size: usize, is_image: bool) -> Self {
        Self {
            operation: FileOperationType::Read,
            path,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            size: Some(size),
            is_image: Some(is_image),
            lines: None,
            changes: None,
            old_length: None,
            new_length: None,
        }
    }

    /// 创建文件写入事件（创建）
    pub fn create(path: PathBuf, size: usize, lines: usize) -> Self {
        Self {
            operation: FileOperationType::Create,
            path,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            size: Some(size),
            is_image: None,
            lines: Some(lines),
            changes: None,
            old_length: None,
            new_length: Some(size),
        }
    }

    /// 创建文件写入事件（更新）
    pub fn update(path: PathBuf, size: usize, lines: usize) -> Self {
        Self {
            operation: FileOperationType::Update,
            path,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            size: Some(size),
            is_image: None,
            lines: Some(lines),
            changes: None,
            old_length: None,
            new_length: Some(size),
        }
    }

    /// 创建文件编辑事件
    pub fn edit(path: PathBuf, old_length: usize, new_length: usize, changes: usize) -> Self {
        Self {
            operation: FileOperationType::Edit,
            path,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            size: None,
            is_image: None,
            lines: None,
            changes: Some(changes),
            old_length: Some(old_length),
            new_length: Some(new_length),
        }
    }
}

/// 文件操作历史记录
#[derive(Debug, Clone)]
pub struct FileOperationHistory {
    /// 操作记录（按时间倒序）
    operations: Vec<FileOperationEvent>,
}

impl FileOperationHistory {
    /// 创建新的历史记录
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }

    /// 添加操作记录
    pub fn add(&mut self, event: FileOperationEvent) {
        self.operations.push(event);
        // 保持最新的 100 条记录
        if self.operations.len() > 100 {
            self.operations.remove(0);
        }
    }

    /// 获取指定文件的操作历史
    pub fn get_file_history(&self, path: &Path) -> Vec<FileOperationEvent> {
        self.operations
            .iter()
            .filter(|op| op.path == path)
            .cloned()
            .collect()
    }

    /// 获取所有操作记录
    pub fn get_all(&self) -> &[FileOperationEvent] {
        &self.operations
    }

    /// 生成修改报告
    pub fn generate_report(&self) -> String {
        if self.operations.is_empty() {
            return "没有文件操作记录".to_string();
        }

        let mut report = String::from("=== 文件操作报告 ===\n\n");

        for op in &self.operations {
            let op_str = match op.operation {
                FileOperationType::Read => "读取",
                FileOperationType::Create => "创建",
                FileOperationType::Update => "更新",
                FileOperationType::Edit => "编辑",
            };

            report.push_str(&format!("- {}: {}\n", op_str, op.path.display()));

            if let Some(size) = op.size {
                report.push_str(&format!("  大小: {} bytes\n", size));
            }

            if let Some(lines) = op.lines {
                report.push_str(&format!("  行数: {}\n", lines));
            }

            if let Some(changes) = op.changes {
                report.push_str(&format!("  变更: {} 字符\n", changes));
            }

            report.push('\n');
        }

        report
    }
}

impl Default for FileOperationHistory {
    fn default() -> Self {
        Self::new()
    }
}

/// 事件发射器
pub struct EventEmitter {
    /// 事件监听器（事件类型 -> 回调列表）
    #[allow(clippy::type_complexity)]
    listeners: Arc<
        RwLock<
            HashMap<
                String,
                Vec<
                    Arc<
                        dyn Fn(FileOperationEvent) -> tokio::task::JoinHandle<()>
                            + Send
                            + Sync
                            + 'static,
                    >,
                >,
            >,
        >,
    >,
}

impl EventEmitter {
    /// 创建新的事件发射器
    pub fn new() -> Self {
        Self {
            listeners: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 注册事件监听器
    pub async fn on<F>(&self, event_type: &str, callback: F) -> tokio::task::JoinHandle<()>
    where
        F: Fn(FileOperationEvent) -> tokio::task::JoinHandle<()> + Send + Sync + 'static,
    {
        let callback = Arc::new(callback);
        let mut listeners = self.listeners.write().await;
        listeners
            .entry(event_type.to_string())
            .or_insert_with(Vec::new)
            .push(callback);

        // 返回一个空 handle 作为占位符
        tokio::spawn(async {})
    }

    /// 触发事件
    pub async fn emit(&self, event_type: &str, event: FileOperationEvent) {
        let listeners = self.listeners.read().await;

        if let Some(handlers) = listeners.get(event_type) {
            for handler in handlers {
                // 异步执行每个监听器
                let handler = handler.clone();
                let event = event.clone();
                tokio::spawn(async move {
                    let _ = handler(event).await;
                });
            }
        }
    }
}

impl Default for EventEmitter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_operation_history() {
        let mut history = FileOperationHistory::new();

        // 添加读取事件
        let event = FileOperationEvent::read(PathBuf::from("/test.txt"), 100, false);
        history.add(event.clone());

        // 添加编辑事件
        let edit_event = FileOperationEvent::edit(PathBuf::from("/test.txt"), 100, 150, 50);
        history.add(edit_event);

        // 验证历史记录
        assert_eq!(history.get_all().len(), 2);
        assert_eq!(
            history
                .get_file_history(PathBuf::from("/test.txt").as_path())
                .len(),
            2
        );
        assert_eq!(
            history
                .get_file_history(PathBuf::from("/nonexistent").as_path())
                .len(),
            0
        );
    }

    #[test]
    fn test_generate_report() {
        let mut history = FileOperationHistory::new();

        let event = FileOperationEvent::read(PathBuf::from("/test.txt"), 100, false);
        history.add(event);

        let report = history.generate_report();
        assert!(report.contains("文件操作报告"));
        assert!(report.contains("/test.txt"));
    }

    #[tokio::test]
    async fn test_event_emitter() {
        let emitter = EventEmitter::new();

        // 注册监听器（使用 tokio::spawn 作为示例）
        emitter
            .on("file_read", |_event| {
                tokio::spawn(async move {
                    // 处理事件
                    println!("Event received");
                })
            })
            .await;

        // 触发事件
        let event = FileOperationEvent::read(PathBuf::from("/test.txt"), 100, false);
        emitter.emit("file_read", event).await;

        // 给异步任务一些时间完成
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    }
}
