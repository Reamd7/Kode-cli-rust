//! Tool trait 定义

use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

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

    /// 是否需要权限
    fn requires_permission(&self) -> bool {
        false
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

/// 工具上下文
#[derive(Debug, Clone)]
pub struct ToolContext {
    // TODO: 添加上下文字段
}

/// 工具执行结果
#[derive(Debug, Clone)]
pub struct ToolResult {
    /// 输出内容
    pub output: String,
}
