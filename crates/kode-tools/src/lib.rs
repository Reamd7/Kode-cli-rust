//! Kode Tools Library
//!
//! 提供工具系统：Tool trait、工具注册表、各种工具实现。

#![warn(missing_docs)]
#![warn(clippy::all)]

/// Tool trait 定义
pub mod tool;

/// 工具注册表
pub mod registry;

/// 安全文件服务
pub mod secure_file;

/// 文件工具函数
pub mod file_utils;

/// 文件读取工具
pub mod file_read;

/// 文件写入工具
pub mod file_write;

/// 文件编辑工具
pub mod file_edit;

/// 参数验证框架
pub mod validation;

// 重新导出主要类型
pub use file_edit::FileEditTool;
pub use file_read::FileReadTool;
pub use file_utils::{LineEnding, ReadResult};
pub use file_write::FileWriteTool;
pub use registry::ToolRegistry;
pub use secure_file::{FileInfo, SecureFileService, SecurityError};
pub use tool::{Tool, ToolContext, ToolResult, ToolSchema, ValidationResult};
pub use validation::{FileValidator, PathValidator, ValidationError};

use std::path::PathBuf;
use std::sync::Arc;

/// 创建所有文件工具
pub fn create_file_tools(secure_service: Arc<SecureFileService>) -> Vec<Arc<dyn Tool>> {
    vec![
        Arc::new(FileReadTool::new(secure_service.clone())) as Arc<dyn Tool>,
        Arc::new(FileWriteTool::new(secure_service.clone())) as Arc<dyn Tool>,
        Arc::new(FileEditTool::new(secure_service)) as Arc<dyn Tool>,
    ]
}

/// 注册所有文件工具到注册表
pub fn register_all_tools(registry: &mut ToolRegistry, allowed_paths: Vec<PathBuf>) {
    let secure_service = Arc::new(SecureFileService::new(allowed_paths));
    let tools = create_file_tools(secure_service);

    for tool in tools {
        registry.register(tool);
    }
}
