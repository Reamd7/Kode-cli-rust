//! 消息类型定义
//!
//! 定义 AI 对话中使用的消息结构和内容块类型。

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// 消息角色
///
/// 表示消息的发送者类型。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Role {
    /// 用户消息
    User,
    /// AI 助手消息
    Assistant,
    /// 系统消息（提示词）
    System,
}

/// 文本内容块
///
/// 表示纯文本内容。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TextBlock {
    /// 文本内容
    pub text: String,
}

/// 工具使用块
///
/// 表示 AI 请求调用工具。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ToolUseBlock {
    /// 工具使用 ID（用于关联结果）
    pub tool_use_id: String,
    /// 工具名称
    pub tool_name: String,
    /// 工具参数（JSON 对象）
    pub parameters: serde_json::Value,
}

/// 工具结果块
///
/// 表示工具执行的结果。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ToolResultBlock {
    /// 关联的工具使用 ID
    pub tool_use_id: String,
    /// 结果内容
    pub content: String,
    /// 是否为错误结果
    #[serde(default)]
    pub is_error: bool,
}

/// 图片内容块
///
/// 表示图片内容（支持 base64 或 URL）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImageBlock {
    /// 图片类型
    #[serde(rename = "type")]
    pub image_type: String,
    /// 媒体类型（例如 "image/jpeg"）
    pub media_type: String,
    /// 图片数据（base64 编码或 URL）
    pub data: String,
}

/// 内容块类型
///
/// 表示消息内容的不同类型。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum ContentBlock {
    /// 文本内容
    Text(TextBlock),
    /// 工具使用
    ToolUse(ToolUseBlock),
    /// 工具结果
    ToolResult(ToolResultBlock),
    /// 图片内容
    Image(ImageBlock),
}

/// 消息内容
///
/// 支持简单字符串或复杂内容块数组。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum MessageContent {
    /// 简单文本内容
    Text(String),
    /// 复杂内容块数组
    Blocks(Vec<ContentBlock>),
}

/// 用户消息选项
///
/// 用户消息的附加选项。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct UserMessageOptions {
    /// 是否为 koding 请求
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_koding_request: Option<bool>,

    /// Koding 上下文信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub koding_context: Option<String>,
}

/// 消息结构
///
/// 表示 AI 对话中的单条消息。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Message {
    /// 消息唯一标识
    pub id: Uuid,

    /// 消息角色
    pub role: Role,

    /// 消息内容（支持单个或多个内容块）
    pub content: MessageContent,

    /// 创建时间
    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub timestamp: Option<DateTime<Utc>>,

    /// 成本追踪（美元）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_usd: Option<f64>,

    /// 执行耗时（毫秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u64>,

    /// API 错误消息标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_api_error_message: Option<bool>,

    /// 响应 ID（用于追踪多轮对话）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_id: Option<String>,

    /// 工具执行结果（仅用户消息）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_use_result: Option<serde_json::Value>,

    /// 用户消息选项（仅用户消息）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<UserMessageOptions>,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            role: Role::User,
            content: MessageContent::Text(String::new()),
            timestamp: Some(Utc::now()),
            cost_usd: None,
            duration_ms: None,
            is_api_error_message: None,
            response_id: None,
            tool_use_result: None,
            options: None,
        }
    }
}

impl Message {
    /// 创建新的用户消息
    ///
    /// # Examples
    /// ```
    /// use kode_core::message::Message;
    ///
    /// let msg = Message::user("Hello, world!");
    /// ```
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            role: Role::User,
            content: MessageContent::Text(content.into()),
            timestamp: Some(Utc::now()),
            ..Default::default()
        }
    }

    /// 创建新的助手消息
    ///
    /// # Examples
    /// ```
    /// use kode_core::message::Message;
    ///
    /// let msg = Message::assistant("Hi there!");
    /// ```
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            role: Role::Assistant,
            content: MessageContent::Text(content.into()),
            timestamp: Some(Utc::now()),
            ..Default::default()
        }
    }

    /// 创建新的系统消息
    ///
    /// # Examples
    /// ```
    /// use kode_core::message::Message;
    ///
    /// let msg = Message::system("You are a helpful assistant.");
    /// ```
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            role: Role::System,
            content: MessageContent::Text(content.into()),
            timestamp: Some(Utc::now()),
            ..Default::default()
        }
    }

    /// 获取消息角色的字符串表示
    pub fn role_str(&self) -> &'static str {
        match self.role {
            Role::User => "user",
            Role::Assistant => "assistant",
            Role::System => "system",
        }
    }

    /// 设置成本追踪信息
    pub fn with_cost_tracking(mut self, cost_usd: f64, duration_ms: u64) -> Self {
        self.cost_usd = Some(cost_usd);
        self.duration_ms = Some(duration_ms);
        self
    }

    /// 设置响应 ID
    pub fn with_response_id(mut self, response_id: impl Into<String>) -> Self {
        self.response_id = Some(response_id.into());
        self
    }

    /// 标记为 API 错误消息
    pub fn with_api_error(mut self) -> Self {
        self.is_api_error_message = Some(true);
        self
    }

    /// 添加工具执行结果
    pub fn with_tool_result(mut self, tool_result: serde_json::Value) -> Self {
        self.tool_use_result = Some(tool_result);
        self
    }

    /// 添加用户消息选项
    pub fn with_options(mut self, options: UserMessageOptions) -> Self {
        self.options = Some(options);
        self
    }

    /// 设置内容为 Blocks
    pub fn with_blocks(mut self, blocks: Vec<ContentBlock>) -> Self {
        self.content = MessageContent::Blocks(blocks);
        self
    }
}

/// 进度消息类型
///
/// 用于在工具执行期间显示进度更新。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProgressMessage {
    /// 助手消息内容
    pub content: Message,

    /// 工具使用 ID
    pub tool_use_id: String,

    /// 关联的工具使用 ID（并发工具场景）
    pub sibling_tool_use_ids: HashSet<String>,

    /// 可用工具列表
    pub tools: Vec<serde_json::Value>,

    /// 规范化的消息列表
    pub normalized_messages: Vec<serde_json::Value>,
}

impl ProgressMessage {
    /// 创建新的进度消息
    ///
    /// # Examples
    /// ```
    /// use kode_core::message::ProgressMessage;
    /// use kode_core::message::Message;
    /// use std::collections::HashSet;
    ///
    /// let msg = Message::assistant("Running command...");
    /// let ids = HashSet::from(["tool-1", "tool-2"]);
    ///
    /// let progress = ProgressMessage::new(
    ///     &msg,
    ///     "tool-123",
    ///     &ids,
    ///     &[],
    ///     &[],
    /// );
    /// ```
    pub fn new(
        content: &Message,
        tool_use_id: impl Into<String>,
        sibling_tool_use_ids: &HashSet<String>,
        tools: &[serde_json::Value],
        normalized_messages: &[serde_json::Value],
    ) -> Self {
        Self {
            content: content.clone(),
            tool_use_id: tool_use_id.into(),
            sibling_tool_use_ids: sibling_tool_use_ids.clone(),
            tools: tools.to_vec(),
            normalized_messages: normalized_messages.to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let msg = Message::user("test");
        assert_eq!(msg.role, Role::User);
        assert_eq!(msg.content, MessageContent::Text("test".to_string()));
        assert!(msg.id != Uuid::nil());
        assert!(msg.timestamp.is_some());
    }

    #[test]
    fn test_role_str() {
        assert_eq!(Message::user("test").role_str(), "user");
        assert_eq!(Message::assistant("test").role_str(), "assistant");
        assert_eq!(Message::system("test").role_str(), "system");
    }

    #[test]
    fn test_message_serialization() {
        let msg = Message::user("Hello");
        let json = serde_json::to_string(&msg).unwrap();
        let parsed: Message = serde_json::from_str(&json).unwrap();
        // 不能直接比较因为 ID 不同，只比较结构
        assert_eq!(parsed.role, Role::User);
    }

    #[test]
    fn test_message_with_metadata() {
        let msg = Message::user("Hello")
            .with_cost_tracking(0.002, 1500)
            .with_response_id("resp-123");

        assert_eq!(msg.cost_usd, Some(0.002));
        assert_eq!(msg.duration_ms, Some(1500));
        assert_eq!(msg.response_id, Some("resp-123".to_string()));
    }

    #[test]
    fn test_content_blocks() {
        let tool_use = ToolUseBlock {
            tool_use_id: "test-id".to_string(),
            tool_name: "bash".to_string(),
            parameters: serde_json::json!({"command": "ls"}),
        };

        let block = ContentBlock::ToolUse(tool_use);
        let json = serde_json::to_string(&block).unwrap();
        let parsed: ContentBlock = serde_json::from_str(&json).unwrap();
        assert_eq!(block, parsed);
    }

    #[test]
    fn test_progress_message() {
        let msg = Message::assistant("Running...");
        let ids = HashSet::from(["tool-1".to_string(), "tool-2".to_string()]);

        let progress = ProgressMessage::new(
            &msg,
            "tool-123",
            &ids,
            &[],
            &[],
        );

        assert_eq!(progress.tool_use_id, "tool-123");
        assert_eq!(progress.sibling_tool_use_ids.len(), 2);
    }
}
