//! OpenAI API 类型定义
//!
//! 定义 OpenAI API 请求和响应的数据结构。

use serde::{Deserialize, Serialize};

/// OpenAI API 配置
///
/// 用于配置 OpenAI API 客户端。
#[derive(Debug, Clone)]
pub struct OpenAIConfig {
    /// API 密钥
    pub api_key: String,
    /// API 基础 URL
    pub base_url: String,
    /// 模型名称
    pub model_name: String,
    /// 最大输出 token 数
    pub max_tokens: usize,
}

/// OpenAI 聊天消息
///
/// 表示 OpenAI API 的消息格式。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum ChatMessage {
    /// 系统消息
    #[serde(rename = "system")]
    System {
        /// 消息内容
        content: String,
    },
    /// 用户消息
    #[serde(rename = "user")]
    User {
        /// 消息内容（支持字符串或内容块数组）
        content: ChatMessageContent,
    },
    /// 助手消息
    #[serde(rename = "assistant")]
    Assistant {
        /// 消息内容（可选）
        #[serde(skip_serializing_if = "Option::is_none")]
        content: Option<String>,
        /// 工具调用列表（可选）
        #[serde(skip_serializing_if = "Option::is_none")]
        tool_calls: Option<Vec<ToolCall>>,
        /// 可选的 tool_call_id（用于工具响应消息）
        #[serde(skip_serializing_if = "Option::is_none")]
        tool_call_id: Option<String>,
    },
    /// 工具响应消息
    #[serde(rename = "tool")]
    Tool {
        /// 工具调用 ID
        tool_call_id: String,
        /// 工具响应内容（必须是字符串）
        content: String,
    },
}

/// 聊天消息内容
///
/// 支持简单字符串或内容块数组。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum ChatMessageContent {
    /// 简单文本内容
    Text(String),
    /// 内容块数组（用于支持图片等多模态内容）
    Blocks(Vec<ContentBlock>),
}

impl From<String> for ChatMessageContent {
    fn from(s: String) -> Self {
        Self::Text(s)
    }
}

impl From<&str> for ChatMessageContent {
    fn from(s: &str) -> Self {
        Self::Text(s.to_string())
    }
}

/// 内容块
///
/// 用于多模态内容（如图片）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentBlock {
    /// 内容块类型
    #[serde(rename = "type")]
    pub content_type: String,
    /// 文本内容（如果是文本块）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// 图片 URL（如果是图片块）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<ImageUrl>,
}

/// 图片 URL
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImageUrl {
    /// URL 字符串
    pub url: String,
}

/// 工具定义
///
/// OpenAI Function Calling 格式的工具定义。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ToolDefinition {
    /// 工具类型（固定为 "function"）
    #[serde(rename = "type")]
    pub tool_type: ToolType,
    /// 工具函数定义
    pub function: FunctionDefinition,
}

/// 工具类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ToolType {
    /// 函数类型
    Function,
}

/// 函数定义
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FunctionDefinition {
    /// 函数名称
    pub name: String,
    /// 函数描述（OpenAI 限制为 1024 字符）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 函数参数 JSON Schema
    pub parameters: serde_json::Value,
}

/// 工具调用
///
/// 表示模型请求调用工具。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ToolCall {
    /// 工具调用 ID
    pub id: String,
    /// 工具调用类型
    #[serde(rename = "type")]
    pub call_type: ToolType,
    /// 函数调用详情
    pub function: FunctionCall,
}

/// 函数调用
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FunctionCall {
    /// 函数名称
    pub name: String,
    /// 函数参数（JSON 字符串）
    pub arguments: String,
}

/// Token 使用统计
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Usage {
    /// 输入提示 token 数
    pub prompt_tokens: usize,
    /// 输出完成 token 数
    pub completion_tokens: usize,
    /// 总 token 数
    pub total_tokens: usize,
}

/// 聊天完成请求
///
/// 发送给 OpenAI API 的请求体。
#[derive(Debug, Clone, Serialize)]
pub struct ChatCompletionRequest {
    /// 模型名称
    pub model: String,
    /// 消息列表
    pub messages: Vec<ChatMessage>,
    /// 最大输出 token 数
    #[serde(skip_serializing_if = "should_skip_max_tokens")]
    pub max_tokens: Option<usize>,
    /// 最大完成 token 数（GPT-5/o1 系列）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<usize>,
    /// 温度参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// 工具定义列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ToolDefinition>>,
    /// 工具选择策略
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    /// 是否流式输出
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// 流式选项（某些提供商不支持）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<StreamOptions>,
}

/// 是否跳过 max_tokens 序列化
fn should_skip_max_tokens(val: &Option<usize>) -> bool {
    val.is_none() || val == &Some(0)
}

/// 工具选择策略
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ToolChoice {
    /// 自动选择
    Auto,
    /// 必须调用工具
    Required,
    /// 不调用工具
    None,
    /// 调用特定工具
    #[serde(rename = "")]
    Specific {
        /// 工具调用类型
        #[serde(rename = "type")]
        call_type: ToolType,
        /// 特定函数
        function: SpecificFunction,
    },
}

/// 特定函数选择
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SpecificFunction {
    /// 函数名称
    pub name: String,
}

/// 流式选项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StreamOptions {
    /// 是否包含使用统计
    pub include_usage: bool,
}

/// 聊天完成响应
///
/// OpenAI API 的非流式响应。
#[derive(Debug, Clone, Deserialize)]
pub struct ChatCompletionResponse {
    /// 响应 ID
    pub id: String,
    /// 对象类型
    pub object: String,
    /// 创建时间戳
    pub created: u64,
    /// 模型名称
    pub model: String,
    /// 选择列表（通常只有一个）
    pub choices: Vec<Choice>,
    /// Token 使用统计
    pub usage: Usage,
}

/// 选择项
#[derive(Debug, Clone, Deserialize)]
pub struct Choice {
    /// 结束原因
    pub finish_reason: Option<String>,
    /// 索引
    pub index: usize,
    /// 消息内容
    pub message: ChatMessage,
    /// 工具调用列表
    #[serde(default)]
    pub tool_calls: Vec<ToolCall>,
}

/// 聊天完成流式块
///
/// OpenAI SSE 流的单个数据块。
#[derive(Debug, Clone, Deserialize)]
pub struct ChatCompletionChunk {
    /// 响应 ID
    pub id: String,
    /// 对象类型
    pub object: String,
    /// 创建时间戳
    pub created: u64,
    /// 模型名称
    pub model: String,
    /// 选择列表
    pub choices: Vec<StreamChoice>,
    /// Token 使用统计（仅在流结束时包含）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
}

/// 流式选择项
#[derive(Debug, Clone, Deserialize)]
pub struct StreamChoice {
    /// 结束原因（流结束时）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
    /// 索引
    pub index: usize,
    /// 增量内容
    pub delta: StreamDelta,
}

/// 流式增量
#[derive(Debug, Clone, Deserialize)]
pub struct StreamDelta {
    /// 增量内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 工具调用增量
    #[serde(default)]
    pub tool_calls: Vec<StreamToolCall>,
}

/// 流式工具调用增量
#[derive(Debug, Clone, Deserialize)]
pub struct StreamToolCall {
    /// 索引
    pub index: i32,
    /// 工具调用 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 函数调用增量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<StreamFunctionCall>,
}

/// 流式函数调用增量
#[derive(Debug, Clone, Deserialize)]
pub struct StreamFunctionCall {
    /// 函数名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 函数参数增量（JSON 片段）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<String>,
}

/// OpenAI API 错误响应
#[derive(Debug, Clone, Deserialize)]
pub struct OpenAIErrorResponse {
    /// 错误信息
    pub error: OpenAIErrorInfo,
}

/// OpenAI 错误信息
#[derive(Debug, Clone, Deserialize)]
pub struct OpenAIErrorInfo {
    /// 错误消息
    pub message: String,
    /// 错误类型
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub error_type: Option<String>,
    /// HTTP 状态码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

/// 模型特性标志
///
/// 用于检测模型支持的功能特性。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct OpenAIModelFeatures {
    /// 是否使用 max_completion_tokens 而非 max_tokens
    pub uses_max_completion_tokens: bool,
    /// temperature 是否必须为 1
    pub requires_temperature_one: bool,
    /// 是否支持 Responses API（GPT-5）
    pub supports_responses_api: bool,
    /// 是否支持自定义工具
    pub supports_custom_tools: bool,
    /// 是否支持 allowed_tools
    pub supports_allowed_tools: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_message_serialization() {
        let msg = ChatMessage::User {
            content: "Hello".into(),
        };
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"role\":\"user\""));

        let parsed: ChatMessage = serde_json::from_str(&json).unwrap();
        assert_eq!(msg, parsed);
    }

    #[test]
    fn test_tool_definition() {
        let tool = ToolDefinition {
            tool_type: ToolType::Function,
            function: FunctionDefinition {
                name: "bash".to_string(),
                description: Some("Execute bash commands".to_string()),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "command": {"type": "string"}
                    }
                }),
            },
        };

        let json = serde_json::to_string(&tool).unwrap();
        let parsed: ToolDefinition = serde_json::from_str(&json).unwrap();
        assert_eq!(tool, parsed);
    }

    #[test]
    fn test_function_call() {
        let call = FunctionCall {
            name: "bash".to_string(),
            arguments: "{\"command\":\"ls\"}".to_string(),
        };

        let json = serde_json::to_string(&call).unwrap();
        let parsed: FunctionCall = serde_json::from_str(&json).unwrap();
        assert_eq!(call, parsed);
    }

    #[test]
    fn test_usage() {
        let usage = Usage {
            prompt_tokens: 100,
            completion_tokens: 50,
            total_tokens: 150,
        };

        let json = serde_json::to_string(&usage).unwrap();
        let parsed: Usage = serde_json::from_str(&json).unwrap();
        assert_eq!(usage, parsed);
    }
}
