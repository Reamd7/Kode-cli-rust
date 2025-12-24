//! 流式响应包装器
//!
//! 提供流式响应的类型别名和辅助函数。

use std::pin::Pin;

use futures::Stream;
use tokio::sync::mpsc;

use super::types::StreamChunk;
use crate::error::Result;

/// 流式响应类型
///
/// 包装流块的流，便于使用。
pub struct StreamingResponse {
    inner: Pin<Box<dyn Stream<Item = Result<StreamChunk>> + Send>>,
}

impl StreamingResponse {
    /// 创建新的流式响应
    pub fn new(stream: Pin<Box<dyn Stream<Item = Result<StreamChunk>> + Send>>) -> Self {
        Self { inner: stream }
    }

    /// 从通道创建流式响应
    pub fn from_channel(rx: mpsc::Receiver<Result<StreamChunk>>) -> Self {
        Self {
            inner: Box::pin(tokio_stream::wrappers::ReceiverStream::new(rx)),
        }
    }

    /// 创建新的通道对，用于发送流式响应
    pub fn channel() -> (mpsc::Sender<Result<StreamChunk>>, Self) {
        let (tx, rx) = mpsc::channel(64);
        (tx, Self::from_channel(rx))
    }
}

impl Stream for StreamingResponse {
    type Item = Result<StreamChunk>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        std::pin::Pin::new(&mut self.inner).poll_next(cx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;

    #[tokio::test]
    async fn test_streaming_response_channel() {
        let (tx, response) = StreamingResponse::channel();

        // 发送一些数据
        tx.send(Ok(StreamChunk::content_block_delta(0, "Hello")))
            .await
            .unwrap();
        tx.send(Ok(StreamChunk::message_stop(super::super::TokenUsage {
            input_tokens: 10,
            output_tokens: 5,
            total_tokens: Some(15),
        })))
        .await
        .unwrap();

        // 接收数据
        let mut response = Box::pin(response);
        let chunk1 = response.next().await.unwrap().unwrap();
        match chunk1 {
            StreamChunk::ContentBlockDelta { delta, .. } => {
                assert_eq!(delta, "Hello");
            }
            _ => panic!("Expected ContentBlockDelta"),
        }
    }
}
