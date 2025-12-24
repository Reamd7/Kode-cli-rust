//! 模型适配器接口
//!
//! 定义与 AI 模型提供商交互的统一接口。

use crate::error::Result;
use crate::message::Message;
use async_trait::async_trait;

use super::types::TokenUsage;

/// 模型响应
///
/// 表示非流式 API 调用的完整响应。
#[derive(Debug, Clone)]
pub struct ModelResponse {
    /// 响应内容
    pub content: String,
    /// Token 使用统计
    pub usage: TokenUsage,
    /// 模型名称
    pub model: String,
}

/// 模型适配器接口
///
/// 所有 AI 模型提供商都需要实现此 trait。
#[async_trait]
pub trait ModelAdapter: Send + Sync {
    /// 发送消息（非流式）
    ///
    /// 向模型发送一系列消息，等待完整响应。
    ///
    /// # Arguments
    ///
    /// * `messages` - 消息列表
    /// * `system_prompt` - 系统提示词（可选）
    /// * `max_tokens` - 最大输出 token 数
    ///
    /// # Returns
    ///
    /// 返回模型响应或错误。
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let response = adapter.send_message(messages, None, 4096).await?;
    /// println!("Response: {}", response.content);
    /// ```
    async fn send_message(
        &self,
        messages: Vec<Message>,
        system_prompt: Option<String>,
        max_tokens: usize,
    ) -> Result<ModelResponse>;

    /// 发送消息（流式）
    ///
    /// 向模型发送一系列消息，返回流式响应。
    ///
    /// # Arguments
    ///
    /// * `messages` - 消息列表
    /// * `system_prompt` - 系统提示词（可选）
    /// * `max_tokens` - 最大输出 token 数
    ///
    /// # Returns
    ///
    /// 返回流块流或错误。
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use futures::stream::StreamExt;
    ///
    /// let mut stream = adapter.stream_message(messages, None, 4096).await?;
    /// while let Some(chunk) = stream.next().await {
    ///     match chunk? {
    ///         StreamChunk::ContentBlockDelta { delta, .. } => {
    ///             print!("{}", delta);
    ///         }
    ///         StreamChunk::MessageStop { usage } => {
    ///             println!("\nTokens: {}", usage.total_tokens.unwrap());
    ///         }
    ///         _ => {}
    ///     }
    /// }
    /// ```
    async fn stream_message(
        &self,
        messages: Vec<Message>,
        system_prompt: Option<String>,
        max_tokens: usize,
    ) -> Result<crate::model::streaming::StreamingResponse>;

    /// 获取模型名称
    ///
    /// 返回此适配器对应的模型名称。
    fn model_name(&self) -> &str;

    /// 检查是否支持流式响应
    ///
    /// 默认返回 `true`，子类可以覆盖。
    fn supports_streaming(&self) -> bool {
        true
    }
}

/// 模型配置
///
/// 用于创建模型适配器的配置信息。
#[derive(Debug, Clone)]
pub struct ModelConfig {
    /// 模型名称
    pub model_name: String,
    /// API 基础 URL
    pub base_url: Option<String>,
    /// API 密钥
    pub api_key: String,
    /// 最大 token 数
    pub max_tokens: usize,
}

#[cfg(test)]
mod tests {
    use super::super::types::StreamChunk;
    use super::*;

    // 测试辅助结构
    struct MockAdapter {
        name: String,
    }

    #[async_trait]
    impl ModelAdapter for MockAdapter {
        async fn send_message(
            &self,
            _messages: Vec<Message>,
            _system_prompt: Option<String>,
            _max_tokens: usize,
        ) -> Result<ModelResponse> {
            Ok(ModelResponse {
                content: "Mock response".to_string(),
                usage: TokenUsage {
                    input_tokens: 10,
                    output_tokens: 5,
                    total_tokens: Some(15),
                },
                model: self.name.clone(),
            })
        }

        async fn stream_message(
            &self,
            _messages: Vec<Message>,
            _system_prompt: Option<String>,
            _max_tokens: usize,
        ) -> Result<crate::model::streaming::StreamingResponse> {
            let (tx, response) = crate::model::streaming::StreamingResponse::channel();

            // 发送模拟数据
            tokio::spawn(async move {
                tx.send(Ok(StreamChunk::content_block_delta(0, "Hello")))
                    .await
                    .ok();
                tx.send(Ok(StreamChunk::message_stop(TokenUsage {
                    input_tokens: 10,
                    output_tokens: 5,
                    total_tokens: Some(15),
                })))
                .await
                .ok();
            });

            Ok(response)
        }

        fn model_name(&self) -> &str {
            &self.name
        }
    }

    #[tokio::test]
    async fn test_model_adapter_trait() {
        let adapter = MockAdapter {
            name: "mock-model".to_string(),
        };

        assert_eq!(adapter.model_name(), "mock-model");
        assert!(adapter.supports_streaming());
    }

    #[tokio::test]
    async fn test_send_message() {
        let adapter = MockAdapter {
            name: "mock-model".to_string(),
        };

        let messages = vec![Message::user("test")];
        let response = adapter.send_message(messages, None, 100).await.unwrap();

        assert_eq!(response.content, "Mock response");
        assert_eq!(response.usage.input_tokens, 10);
    }

    #[tokio::test]
    async fn test_stream_message() {
        use crate::model::types::StreamChunk;
        use futures::StreamExt;

        let adapter = MockAdapter {
            name: "mock-model".to_string(),
        };

        let messages = vec![Message::user("test")];
        let mut stream = adapter.stream_message(messages, None, 100).await.unwrap();

        let first = stream.next().await.unwrap().unwrap();
        match first {
            StreamChunk::ContentBlockDelta { delta, .. } => {
                assert_eq!(delta, "Hello");
            }
            _ => panic!("Expected ContentBlockDelta"),
        }
    }
}
