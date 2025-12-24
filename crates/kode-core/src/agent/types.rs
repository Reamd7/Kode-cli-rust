//! Agent 类型定义
//!
//! 定义 Agent 结构体和工具过滤器。

use serde::{Deserialize, Serialize};

/// Agent 来源位置
///
/// 标识 Agent 来自哪个层级，对应 TypeScript 版本中的 `location` 字段。
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AgentLocation {
    /// 内置 Agent
    #[serde(rename = "built-in")]
    Builtin,

    /// 用户级 Agent (~/.claude/agents/ 或 ~/.kode/agents/)
    #[serde(rename = "user")]
    User,

    /// 项目级 Agent (./.claude/agents/ 或 ./.kode/agents/)
    #[serde(rename = "project")]
    Project,
}

/// Agent 定义
///
/// 表示一个专门化的 AI 助手，包含其配置和行为指令。
/// 参考 TypeScript 版本中的 `AgentConfig` 接口。
///
/// # Examples
///
/// ```
/// use kode_core::agent::{Agent, AgentLocation, ToolFilter};
///
/// let agent = Agent {
///     name: "code-reviewer".to_string(),
///     description: "Reviews code for best practices".to_string(),
///     tools: ToolFilter::Specific(vec!["FileRead".to_string(), "Grep".to_string()]),
///     model: Some("claude-sonnet".to_string()),
///     system_prompt: "You are an expert code reviewer.".to_string(),
///     location: AgentLocation::User,
///     color: Some("#FF5733".to_string()),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Agent {
    /// Agent 唯一标识符
    ///
    /// 用于匹配用户的 agent 选择，对应 TypeScript 中的 `agentType`。
    pub name: String,

    /// Agent 描述
    ///
    /// 描述何时应该使用此 Agent，对应 TypeScript 中的 `whenToUse`。
    pub description: String,

    /// 工具访问权限
    ///
    /// 定义 Agent 可以访问哪些工具。
    pub tools: ToolFilter,

    /// 可选的模型覆盖
    ///
    /// 如果指定，使用此模型而不是默认模型。
    /// 对应 TypeScript 中的 `model_name`。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// 系统提示词
    ///
    /// Agent 的行为指令，作为上下文中的第一条消息。
    pub system_prompt: String,

    /// Agent 来源位置
    ///
    /// 标识此 Agent 来自哪个层级（内置、用户、项目）。
    /// 对应 TypeScript 版本中的 `location` 字段。
    pub location: AgentLocation,

    /// 可选的 UI 颜色
    ///
    /// 用于 UI 显示的主题颜色，对应 TypeScript 版本中的 `color` 字段。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

impl Agent {
    /// 创建新的 Agent
    ///
    /// # Arguments
    ///
    /// * `name` - Agent 名称
    /// * `description` - Agent 描述
    /// * `tools` - 工具过滤器
    /// * `system_prompt` - 系统提示词
    /// * `location` - Agent 来源位置
    pub fn new(
        name: String,
        description: String,
        tools: ToolFilter,
        system_prompt: String,
        location: AgentLocation,
    ) -> Self {
        Self {
            name,
            description,
            tools,
            model: None,
            system_prompt,
            location,
            color: None,
        }
    }

    /// 设置模型覆盖
    pub fn with_model(mut self, model: String) -> Self {
        self.model = Some(model);
        self
    }

    /// 设置颜色
    pub fn with_color(mut self, color: String) -> Self {
        self.color = Some(color);
        self
    }
}

/// 工具过滤器
///
/// 控制 Agent 可以访问哪些工具。
/// 对应 TypeScript 中的 `parseTools` 函数返回值。
///
/// # Examples
///
/// ```
/// use kode_core::agent::ToolFilter;
///
/// // 允许访问所有工具
/// let all_tools = ToolFilter::All;
///
/// // 只允许访问特定工具
/// let specific_tools = ToolFilter::Specific(vec![
///     "FileRead".to_string(),
///     "Grep".to_string(),
/// ]);
///
/// // 检查是否可以访问某个工具
/// assert!(all_tools.allows("FileRead"));
/// assert!(specific_tools.allows("FileRead"));
/// assert!(!specific_tools.allows("Bash"));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(untagged)]
pub enum ToolFilter {
    /// 访问所有工具
    ///
    /// 对应 TypeScript 中的 `"*"` 或 `"all"`。
    #[default]
    All,

    /// 访问指定的工具列表
    ///
    /// 对应 TypeScript 中的字符串数组。
    Specific(Vec<String>),
}

impl ToolFilter {
    /// 检查是否允许访问指定的工具
    ///
    /// # Arguments
    ///
    /// * `tool_name` - 要检查的工具名称
    ///
    /// # Returns
    ///
    /// 如果允许访问返回 `true`，否则返回 `false`。
    pub fn allows(&self, tool_name: &str) -> bool {
        match self {
            ToolFilter::All => true,
            ToolFilter::Specific(tools) => tools.iter().any(|t| t == tool_name),
        }
    }

    /// 返回允许的工具列表
    ///
    /// 如果是 `All`，返回 `None`（表示没有限制）。
    /// 如果是 `Specific`，返回工具列表。
    pub fn tools(&self) -> Option<&[String]> {
        match self {
            ToolFilter::All => None,
            ToolFilter::Specific(tools) => Some(tools),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_creation() {
        let agent = Agent::new(
            "test".to_string(),
            "Test agent".to_string(),
            ToolFilter::All,
            "Test prompt".to_string(),
            AgentLocation::User,
        );
        assert_eq!(agent.name, "test");
        assert_eq!(agent.description, "Test agent");
        assert!(matches!(agent.tools, ToolFilter::All));
        assert_eq!(agent.system_prompt, "Test prompt");
        assert!(agent.model.is_none());
        assert_eq!(agent.location, AgentLocation::User);
        assert!(agent.color.is_none());
    }

    #[test]
    fn test_agent_with_model() {
        let agent = Agent::new(
            "test".to_string(),
            "Test agent".to_string(),
            ToolFilter::All,
            "Test prompt".to_string(),
            AgentLocation::Project,
        )
        .with_model("claude-sonnet".to_string())
        .with_color("#FF5733".to_string());

        assert_eq!(agent.model, Some("claude-sonnet".to_string()));
        assert_eq!(agent.color, Some("#FF5733".to_string()));
        assert_eq!(agent.location, AgentLocation::Project);
    }

    #[test]
    fn test_tool_filter_all() {
        let filter = ToolFilter::All;
        assert!(filter.allows("FileRead"));
        assert!(filter.allows("Grep"));
        assert!(filter.allows("Bash"));
        assert!(filter.tools().is_none());
    }

    #[test]
    fn test_tool_filter_specific() {
        let filter = ToolFilter::Specific(vec!["FileRead".to_string(), "Grep".to_string()]);
        assert!(filter.allows("FileRead"));
        assert!(filter.allows("Grep"));
        assert!(!filter.allows("Bash"));

        let tools = filter.tools().unwrap();
        assert_eq!(tools.len(), 2);
        assert_eq!(tools[0], "FileRead");
        assert_eq!(tools[1], "Grep");
    }

    #[test]
    fn test_tool_filter_default() {
        let filter: ToolFilter = Default::default();
        assert!(matches!(filter, ToolFilter::All));
    }
}
