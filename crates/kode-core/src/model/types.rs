//! 模型类型定义
//!
//! 定义流式响应和相关的数据结构。

use serde::{Deserialize, Serialize};

/// Token 使用统计
///
/// 表示一次 API 调用的 token 使用情况。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TokenUsage {
    /// 输入 token 数量
    pub input_tokens: usize,
    /// 输出 token 数量
    pub output_tokens: usize,
    /// 总 token 数量（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tokens: Option<usize>,
}

/// 流块类型
///
/// 表示流式响应中的不同事件类型。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StreamChunk {
    /// 内容块开始
    #[serde(rename = "content_block_start")]
    ContentBlockStart {
        /// 内容块索引
        index: usize,
    },

    /// 内容增量（文本片段）
    #[serde(rename = "content_block_delta")]
    ContentBlockDelta {
        /// 内容块索引
        index: usize,
        /// 增量文本内容
        delta: String,
    },

    /// 内容块结束
    #[serde(rename = "content_block_stop")]
    ContentBlockStop {
        /// 内容块索引
        index: usize,
    },

    /// 工具使用请求
    #[serde(rename = "tool_use")]
    ToolUse {
        /// 工具名称
        tool_name: String,
        /// 工具使用 ID
        tool_use_id: String,
        /// 工具参数（JSON 对象）
        parameters: serde_json::Value,
    },

    /// 消息结束（包含使用统计）
    #[serde(rename = "message_stop")]
    MessageStop {
        /// Token 使用统计
        usage: TokenUsage,
    },

    /// 错误事件
    #[serde(rename = "error")]
    Error {
        /// 错误消息
        message: String,
    },
}

impl StreamChunk {
    /// 创建内容块开始事件
    pub fn content_block_start(index: usize) -> Self {
        Self::ContentBlockStart { index }
    }

    /// 创建内容增量事件
    pub fn content_block_delta(index: usize, delta: impl Into<String>) -> Self {
        Self::ContentBlockDelta {
            index,
            delta: delta.into(),
        }
    }

    /// 创建内容块结束事件
    pub fn content_block_stop(index: usize) -> Self {
        Self::ContentBlockStop { index }
    }

    /// 创建工具使用事件
    pub fn tool_use(
        tool_name: impl Into<String>,
        tool_use_id: impl Into<String>,
        parameters: serde_json::Value,
    ) -> Self {
        Self::ToolUse {
            tool_name: tool_name.into(),
            tool_use_id: tool_use_id.into(),
            parameters,
        }
    }

    /// 创建消息结束事件
    pub fn message_stop(usage: TokenUsage) -> Self {
        Self::MessageStop { usage }
    }

    /// 创建错误事件
    pub fn error(message: impl Into<String>) -> Self {
        Self::Error {
            message: message.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream_chunk_serialization() {
        let chunk = StreamChunk::content_block_delta(0, "Hello");
        let json = serde_json::to_string(&chunk).unwrap();
        assert!(json.contains("content_block_delta"));
    }

    #[test]
    fn test_stream_chunk_tool_use() {
        let chunk = StreamChunk::tool_use("bash", "id-123", serde_json::json!({"command": "ls"}));
        let json = serde_json::to_string(&chunk).unwrap();
        let parsed: StreamChunk = serde_json::from_str(&json).unwrap();
        assert_eq!(chunk, parsed);
    }

    #[test]
    fn test_token_usage() {
        let usage = TokenUsage {
            input_tokens: 100,
            output_tokens: 50,
            total_tokens: Some(150),
        };

        let json = serde_json::to_string(&usage).unwrap();
        let parsed: TokenUsage = serde_json::from_str(&json).unwrap();
        assert_eq!(usage, parsed);
    }
}
