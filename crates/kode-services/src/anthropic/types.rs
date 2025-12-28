//! Anthropic API 类型定义
//!
//! 定义 Anthropic Messages API 请求和响应的数据结构。

use kode_core::message::ContentBlock;
use serde::{Deserialize, Serialize};

/// 思考类型
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ThinkingType {
    /// 启用思考模式
    Enabled,
    /// 禁用思考模式
    Disabled,
    /// 遵守系统提示词中的指示
    #[serde(rename = "collapsed")]
    Collapsed,
    /// 隐藏思考内容
    #[serde(rename = "hidden")]
    Hidden,
}

/// 思考配置
///
/// 用于配置 Claude 的思考模式。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct ThinkingConfig {
    /// 思考类型
    #[serde(rename = "type")]
    pub thinking_type: ThinkingType,
    /// 思考预算 token 数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_tokens: Option<usize>,
}

impl ThinkingConfig {
    /// 创建新的思考配置
    pub fn new(thinking_type: ThinkingType, budget_tokens: Option<usize>) -> Self {
        Self {
            thinking_type,
            budget_tokens,
        }
    }

    /// 启用思考模式
    pub fn enabled(budget_tokens: Option<usize>) -> Self {
        Self::new(ThinkingType::Enabled, budget_tokens)
    }

    /// 禁用思考模式
    pub fn disabled() -> Self {
        Self::new(ThinkingType::Disabled, None)
    }
}

/// Anthropic API 消息请求
///
/// 用于构建发送到 Anthropic API 的请求体。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct ApiRequest {
    /// 模型名称
    pub model: String,
    /// 消息列表
    pub messages: Vec<ApiMessage>,
    /// 系统提示词
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    /// 最大输出 token 数
    pub max_tokens: usize,
    /// 是否流式响应
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

/// Anthropic API 消息
///
/// 表示单条消息的格式。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct ApiMessage {
    /// 角色
    pub role: String,
    /// 内容
    pub content: serde_json::Value,
}

/// Anthropic API 响应
///
/// 表示 Anthropic API 的非流式响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct ApiResponse {
    /// 响应 ID
    pub id: String,
    /// 响应类型
    pub r#type: String,
    /// 角色
    pub role: String,
    /// 内容块
    pub content: Vec<ContentBlock>,
    /// 停止原因
    pub stop_reason: Option<String>,
    /// 停止序列
    pub stop_sequence: Option<String>,
    /// 模型名称
    pub model: String,
    /// Token 使用统计
    pub usage: ApiUsage,
}

/// Token 使用统计
///
/// API 响应的 token 使用情况。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct ApiUsage {
    /// 输入 token 数
    #[serde(rename = "input_tokens")]
    pub input_tokens: usize,
    /// 输出 token 数
    #[serde(rename = "output_tokens")]
    pub output_tokens: usize,
    /// 思考 token 数
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "thinking_tokens")]
    pub thinking_tokens: Option<usize>,
}

/// 服务器发送事件 (SSE)
///
/// 用于解析流式响应的 SSE 格式。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct ServerSentEvent {
    /// 事件类型
    #[serde(rename = "type")]
    pub r#type: String,
    /// 角色（仅 message_start）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// 内容块（仅 content_block_start）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_block: Option<ContentBlockStartEvent>,
    /// 增量数据（仅 content_block_delta）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta: Option<DeltaEvent>,
    /// 索引（仅 content_block_stop）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
    /// 停止原因（仅 message_delta）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_reason: Option<String>,
    /// 使用统计
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<ApiUsage>,
}

/// 内容块开始事件
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct ContentBlockStartEvent {
    /// 内容块索引
    pub index: usize,
    /// 内容块类型
    #[serde(rename = "type")]
    pub block_type: String,
    /// 工具名称（仅 tool_use）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 工具使用 ID（仅 tool_use）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Delta 事件
///
/// 表示内容增量的不同类型。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct DeltaEvent {
    /// 文本增量
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "text_delta")]
    pub text_delta: Option<String>,
    /// JSON 输入增量
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "input_json_delta")]
    pub input_json_delta: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use kode_core::message::TextBlock;

    #[test]
    fn test_api_response_serialization() {
        let response = ApiResponse {
            id: "test-id".to_string(),
            r#type: "message".to_string(),
            role: "assistant".to_string(),
            content: vec![ContentBlock::Text(TextBlock {
                text: "Hello!".to_string(),
            })],
            stop_reason: Some("stop".to_string()),
            stop_sequence: None,
            model: "claude-sonnet-4-20250514".to_string(),
            usage: ApiUsage {
                input_tokens: 10,
                output_tokens: 5,
                thinking_tokens: None,
            },
        };

        let json = serde_json::to_string(&response).unwrap();
        let parsed: ApiResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.id, "test-id");
        assert_eq!(parsed.model, "claude-sonnet-4-20250514");
        assert_eq!(parsed.usage.input_tokens, 10);
        assert_eq!(parsed.usage.output_tokens, 5);
    }

    #[test]
    fn test_server_sent_event_serialization() {
        let event = ServerSentEvent {
            r#type: "content_block_delta".to_string(),
            role: None,
            content_block: None,
            delta: Some(DeltaEvent {
                text_delta: Some("Hello".to_string()),
                input_json_delta: None,
            }),
            index: Some(0),
            stop_reason: None,
            usage: None,
        };

        let json = serde_json::to_string(&event).unwrap();
        let parsed: ServerSentEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.r#type, "content_block_delta");
        assert_eq!(parsed.delta.unwrap().text_delta.unwrap(), "Hello");
    }

    #[test]
    fn test_api_usage_serialization() {
        let usage = ApiUsage {
            input_tokens: 100,
            output_tokens: 50,
            thinking_tokens: None,
        };

        let json = serde_json::to_string(&usage).unwrap();
        let parsed: ApiUsage = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.input_tokens, 100);
        assert_eq!(parsed.output_tokens, 50);
    }
}

/// API 错误响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct ApiErrorResponse {
    /// 错误详情
    pub error: ApiErrorDetail,
}

/// API 错误详情
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct ApiErrorDetail {
    /// 错误消息
    pub message: String,
    /// 错误类型
    pub error_type: String,
}

/// 模型列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct ModelsResponse {
    /// 模型数据列表
    pub data: Vec<ModelData>,
}

/// 模型数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct ModelData {
    /// 模型 ID
    pub id: String,
    /// 模型类型
    #[serde(rename = "type")]
    pub model_type: String,
    /// 显示名称
    pub display_name: String,
    /// 描述
    pub description: String,
    /// 输入 token 限制
    pub input_token_limit: usize,
    /// 输出 token 限制
    pub output_token_limit: usize,
    /// 模型能力
    pub capabilities: ModelCapabilities,
}

/// 模型能力
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct ModelCapabilities {
    /// 是否支持思考模式
    pub thinking: bool,
    /// 是否支持流式响应
    pub streaming: bool,
    /// 是否支持计算机使用
    pub computer_use: bool,
}
