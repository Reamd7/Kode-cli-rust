//! SSE 流式响应处理
//!
//! 处理 OpenAI SSE 流的解析和事件生成。

use crate::openai::types::ChatCompletionChunk;
use crate::openai::OpenAIError;
use futures::stream::Stream;
use kode_core::model::types::{StreamChunk, TokenUsage};
use pin_project::pin_project;
use reqwest_eventsource::{Event, EventSource};
use std::collections::HashMap;
use std::pin::Pin;
use std::task::{Context, Poll};
use tracing::debug;

/// SSE 流处理器
///
/// 解析 OpenAI SSE 流并生成统一的事件流。
#[pin_project]
pub struct SseStreamProcessor {
    #[pin]
    event_source: EventSource,
    accumulated_content: String,
    accumulated_tools: HashMap<i32, AccumulatedToolCall>,
    complete: bool,
}

/// 累积的工具调用
#[derive(Debug, Clone, serde::Serialize)]
struct AccumulatedToolCall {
    /// 工具调用 ID
    id: String,
    /// 函数名称
    name: String,
    /// 累积的参数（JSON 片段）
    arguments: String,
}

impl SseStreamProcessor {
    /// 创建新的 SSE 流处理器
    pub fn new(event_source: EventSource) -> Self {
        Self {
            event_source,
            accumulated_content: String::new(),
            accumulated_tools: HashMap::new(),
            complete: false,
        }
    }
}

impl Stream for SseStreamProcessor {
    type Item = Result<StreamChunk, OpenAIError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();

        if *this.complete {
            return Poll::Ready(None);
        }

        match this.event_source.poll_next(cx) {
            Poll::Ready(Some(Ok(event))) => {
                match event {
                    Event::Message(message) => {
                        if message.data == "[DONE]" {
                            debug!("Received [DONE] marker");
                            *this.complete = true;

                            // 返回所有完成的工具调用
                            if !this.accumulated_tools.is_empty() {
                                let tools = std::mem::take(this.accumulated_tools);
                                return Poll::Ready(Some(Ok(StreamChunk::tool_use_complete(
                                    0,
                                    serde_json::to_string(&tools)
                                        .map_err(|e| OpenAIError::stream(e.to_string()))?,
                                ))));
                            }

                            return Poll::Ready(None);
                        }

                        // 解析 JSON 数据块
                        match serde_json::from_str::<ChatCompletionChunk>(&message.data) {
                            Ok(chunk) => Poll::Ready(Some(process_chunk(
                                chunk,
                                this.accumulated_content,
                                this.accumulated_tools,
                            ))),
                            Err(e) => {
                                debug!("Failed to parse chunk: {}", e);
                                Poll::Ready(Some(Err(OpenAIError::stream(format!(
                                    "Invalid chunk JSON: {}",
                                    e
                                )))))
                            }
                        }
                    }
                    Event::Open => Poll::Pending,
                }
            }
            Poll::Ready(Some(Err(e))) => {
                debug!("EventSource error: {:?}", e);
                Poll::Ready(Some(Err(OpenAIError::stream(e.to_string()))))
            }
            Poll::Ready(None) => {
                *this.complete = true;
                Poll::Ready(None)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

/// 处理单个数据块
fn process_chunk(
    chunk: ChatCompletionChunk,
    accumulated_content: &mut String,
    accumulated_tools: &mut HashMap<i32, AccumulatedToolCall>,
) -> Result<StreamChunk, OpenAIError> {
    // 检查是否有 usage 信息（流结束）
    if let Some(usage) = chunk.usage {
        let token_usage = TokenUsage {
            input_tokens: usage.prompt_tokens,
            output_tokens: usage.completion_tokens,
            total_tokens: Some(usage.total_tokens),
            thinking_tokens: None,
        };
        return Ok(StreamChunk::message_stop(token_usage));
    }

    // 处理选择项
    if let Some(choice) = chunk.choices.first() {
        let index = choice.index;

        // 处理内容增量
        if let Some(delta) = &choice.delta.content {
            if !delta.is_empty() {
                accumulated_content.push_str(delta);
                return Ok(StreamChunk::content_block_delta(index, delta.clone()));
            }
        }

        // 处理工具调用增量
        for tool_call in &choice.delta.tool_calls {
            let call_entry =
                accumulated_tools
                    .entry(tool_call.index)
                    .or_insert_with(|| AccumulatedToolCall {
                        id: tool_call.id.clone().unwrap_or_default(),
                        name: tool_call
                            .function
                            .as_ref()
                            .and_then(|f| f.name.clone())
                            .unwrap_or_default(),
                        arguments: String::new(),
                    });

            // 更新 ID
            if let Some(id) = &tool_call.id {
                call_entry.id = id.clone();
            }

            // 更新函数名
            if let Some(func) = &tool_call.function {
                if let Some(name) = &func.name {
                    call_entry.name = name.clone();
                }
                // 累积参数片段
                if let Some(args) = &func.arguments {
                    call_entry.arguments.push_str(args);
                }
            }
        }

        // 检查是否流结束
        if choice.finish_reason.is_some() {
            // 返回累积的工具调用
            if !accumulated_tools.is_empty() {
                let tools: Vec<_> = accumulated_tools
                    .iter()
                    .map(|(idx, tool)| {
                        serde_json::json!({
                            "index": idx,
                            "id": tool.id,
                            "name": tool.name,
                            "arguments": tool.arguments,
                        })
                    })
                    .collect();

                return Ok(StreamChunk::tool_use(
                    "tool_calls".to_string(),
                    "stream".to_string(),
                    serde_json::json!(tools),
                ));
            }
        }
    }

    // 如果没有新事件，跳过
    Err(OpenAIError::stream("No new content in chunk"))
}

/// 将响应转换为流式响应（用于非流式到流式的适配）
pub async fn response_to_streaming(
    response: String,
    usage: TokenUsage,
) -> Result<kode_core::model::streaming::StreamingResponse, OpenAIError> {
    use kode_core::model::streaming::StreamingResponse;

    let (tx, rx) = StreamingResponse::channel();

    tokio::spawn(async move {
        if !response.is_empty() {
            tx.send(Ok(StreamChunk::content_block_delta(0, response)))
                .await
                .ok();
        }
        tx.send(Ok(StreamChunk::message_stop(usage))).await.ok();
    });

    Ok(rx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accumulated_tool_call() {
        let mut tool = AccumulatedToolCall {
            id: "call-123".to_string(),
            name: "bash".to_string(),
            arguments: String::new(),
        };

        tool.arguments.push_str("{\"command\":");
        tool.arguments.push_str("\"ls\"}");
        tool.arguments.push_str("}");

        assert_eq!(tool.arguments, "{\"command\":\"ls\"}}");
    }
}
