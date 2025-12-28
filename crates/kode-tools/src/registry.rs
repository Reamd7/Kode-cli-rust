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

    /// 过滤工具
    pub fn filter<F>(&self, predicate: F) -> Vec<Arc<dyn Tool>>
    where
        F: Fn(&Arc<dyn Tool>) -> bool,
    {
        self.tools
            .values()
            .filter(|t| predicate(t))
            .cloned()
            .collect()
    }

    /// 列出所有启用的工具名称
    pub fn list_enabled(&self) -> Vec<String> {
        // TODO: 实现启用/禁用逻辑
        self.list()
    }

    /// 列出所有只读工具名称
    pub fn list_read_only(&self) -> Vec<String> {
        self.tools
            .values()
            .filter(|t| t.is_read_only())
            .map(|t| t.name().to_string())
            .collect()
    }

    /// 批量获取工具
    pub fn get_by_names(&self, names: &[String]) -> Vec<Arc<dyn Tool>> {
        names.iter().filter_map(|name| self.get(name)).collect()
    }

    /// 获取所有工具
    pub fn get_all(&self) -> Vec<Arc<dyn Tool>> {
        self.tools.values().cloned().collect()
    }

    /// 获取只读工具
    pub fn get_read_only(&self) -> Vec<Arc<dyn Tool>> {
        self.filter(|t| t.is_read_only())
    }

    /// 获取并发安全工具
    pub fn get_concurrency_safe(&self) -> Vec<Arc<dyn Tool>> {
        self.filter(|t| t.is_concurrency_safe())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tool::{ToolContext, ToolResult, ToolSchema};
    use async_trait::async_trait;
    use serde_json::json;

    // Mock tool for testing
    struct MockTool {
        name: &'static str,
        read_only: bool,
        concurrency_safe: bool,
    }

    #[async_trait]
    impl Tool for MockTool {
        fn name(&self) -> &str {
            self.name
        }

        fn description(&self) -> &str {
            "Mock tool"
        }

        fn schema(&self) -> ToolSchema {
            ToolSchema {
                name: self.name.to_string(),
                description: "Mock tool".to_string(),
                parameters: json!({}),
            }
        }

        async fn execute(
            &self,
            _params: serde_json::Value,
            _context: &ToolContext,
        ) -> anyhow::Result<ToolResult> {
            Ok(ToolResult::success("OK"))
        }

        fn is_read_only(&self) -> bool {
            self.read_only
        }

        fn is_concurrency_safe(&self) -> bool {
            self.concurrency_safe
        }
    }

    #[test]
    fn test_registry_register_and_get() {
        let mut registry = ToolRegistry::new();
        let tool = Arc::new(MockTool {
            name: "test",
            read_only: false,
            concurrency_safe: false,
        });

        registry.register(tool.clone());
        let retrieved = registry.get("test");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name(), "test");
    }

    #[test]
    fn test_registry_list() {
        let mut registry = ToolRegistry::new();
        registry.register(Arc::new(MockTool {
            name: "tool1",
            read_only: false,
            concurrency_safe: false,
        }));
        registry.register(Arc::new(MockTool {
            name: "tool2",
            read_only: false,
            concurrency_safe: false,
        }));

        let names = registry.list();
        assert_eq!(names.len(), 2);
        assert!(names.contains(&"tool1".to_string()));
        assert!(names.contains(&"tool2".to_string()));
    }

    #[test]
    fn test_registry_filter_read_only() {
        let mut registry = ToolRegistry::new();
        registry.register(Arc::new(MockTool {
            name: "read_tool",
            read_only: true,
            concurrency_safe: false,
        }));
        registry.register(Arc::new(MockTool {
            name: "write_tool",
            read_only: false,
            concurrency_safe: false,
        }));

        let read_only_tools = registry.list_read_only();
        assert_eq!(read_only_tools, vec!["read_tool"]);
    }

    #[test]
    fn test_registry_get_by_names() {
        let mut registry = ToolRegistry::new();
        registry.register(Arc::new(MockTool {
            name: "tool1",
            read_only: false,
            concurrency_safe: false,
        }));
        registry.register(Arc::new(MockTool {
            name: "tool2",
            read_only: false,
            concurrency_safe: false,
        }));

        let tools = registry.get_by_names(&[
            "tool1".to_string(),
            "tool2".to_string(),
            "nonexistent".to_string(),
        ]);
        assert_eq!(tools.len(), 2);
    }
}
