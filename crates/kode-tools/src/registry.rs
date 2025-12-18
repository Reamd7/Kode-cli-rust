//! 工具注册表

use crate::Tool;
use std::collections::HashMap;
use std::sync::Arc;

/// 工具注册表
#[derive(Default)]
pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn Tool>>,
}

impl ToolRegistry {
    /// 创建新的工具注册表
    pub fn new() -> Self {
        Self::default()
    }

    /// 注册工具
    pub fn register(&mut self, tool: Arc<dyn Tool>) {
        self.tools.insert(tool.name().to_string(), tool);
    }

    /// 获取工具
    pub fn get(&self, name: &str) -> Option<Arc<dyn Tool>> {
        self.tools.get(name).cloned()
    }

    /// 列出所有工具名称
    pub fn list(&self) -> Vec<String> {
        self.tools.keys().cloned().collect()
    }
}
