# OpenAI 服务使用示例

本文档展示如何使用 `OpenAIService` 进行 API 调用。

## 基础使用

### 创建服务实例

```rust
use kode_services::openai::{OpenAIService, OpenAIConfig};
use kode_core::model::adapter::ModelAdapter;

let config = OpenAIConfig {
    api_key: "your-api-key".to_string(),
    base_url: "https://api.openai.com/v1".to_string(),
    model_name: "gpt-4".to_string(),
    max_tokens: 4096,
    proxy: None, // 可选：代理配置
};

let service = OpenAIService::new(config);
```

### 发送简单消息

```rust
use kode_core::message::Message;

let messages = vec![
    Message::user("Hello, how are you?"),
];

let response = service.send_message(messages, None, 1000).await?;

println!("Response: {}", response.content);
println!("Input tokens: {}", response.usage.input_tokens);
println!("Output tokens: {}", response.usage.output_tokens);
println!("Cost: ${:.6}", response.cost_usd.unwrap_or(0.0));
```

### 使用系统提示词

```rust
let messages = vec![
    Message::user("Explain quantum computing"),
];

let system_prompt = "You are a helpful assistant specializing in physics.";

let response = service
    .send_message(messages, Some(system_prompt.to_string()), 2000)
    .await?;
```

### 工具调用（Function Calling）

```rust
use serde_json::json;

let messages = vec![
    Message::user("What's the weather in San Francisco?"),
];

let tools = vec![json!({
    "name": "get_weather",
    "description": "Get the current weather for a location",
    "input_schema": {
        "type": "object",
        "properties": {
            "location": {
                "type": "string",
                "description": "The city and state, e.g. San Francisco, CA"
            }
        },
        "required": ["location"]
    }
})];

let response = service
    .send_message_with_tools(messages, None, 1000, &tools)
    .await?;

// 处理工具调用
```

## 高级功能

### 流式响应

```rust
use kode_core::model::streaming::StreamingResponse;
use futures::StreamExt;

let messages = vec![
    Message::user("Tell me a story"),
];

let mut stream = service.stream_message(messages, None, 500).await?;

while let Some(chunk) = stream.next().await {
    match chunk? {
        kode_core::model::StreamChunk::ContentBlockDelta(_, text) => {
            print!("{}", text);
        }
        kode_core::model::StreamChunk::MessageStop(usage) => {
            println!("\nTotal tokens: {}", usage.total_tokens.unwrap());
        }
        _ => {}
    }
}
```

### 代理配置

```rust
let config = OpenAIConfig {
    api_key: "your-api-key".to_string(),
    base_url: "https://api.openai.com/v1".to_string(),
    model_name: "gpt-4".to_string(),
    max_tokens: 4096,
    proxy: Some("http://proxy.example.com:8080".to_string()),
};

let service = OpenAIService::new(config);
// 或使用环境变量 HTTP_PROXY / HTTPS_PROXY
```

### Session 缓存

```rust
use kode_services::openai::SessionCache;

let cache = SessionCache::new();

// 缓存已知的模型错误
cache.cache_model_error("gpt-5", "max_tokens_not_supported");

// 检查错误
if cache.has_model_error("gpt-5", "max_tokens_not_supported") {
    println!("Known error detected");
}

// 记录应用的修复
cache.record_applied_fix("gpt-5", "converted_to_max_completion_tokens");
```

### 成本计算

```rust
use kode_services::openai::{calculate_cost, get_model_pricing};

// 计算成本
let cost = calculate_cost("gpt-4", 1000, 500);
println!("Cost: ${:.6}", cost.unwrap());

// 获取定价信息
if let Some(pricing) = get_model_pricing("gpt-4") {
    println!("Input price: ${}/M tokens", pricing.input_price_per_million);
    println!("Output price: ${}/M tokens", pricing.output_price_per_million);
}
```

## 错误处理

```rust
use kode_services::openai::OpenAIError;

match service.send_message(messages, None, 1000).await {
    Ok(response) => {
        println!("Success: {}", response.content);
    }
    Err(OpenAIError::RateLimitError(msg)) => {
        eprintln!("Rate limited: {}", msg);
    }
    Err(OpenAIError::ApiError { code, message }) => {
        eprintln!("API error [{}]: {}", code.unwrap_or_default(), message);
    }
    Err(e) => {
        eprintln!("Error: {}", e);
    }
}
```

## 支持的提供商

OpenAIService 支持多种 OpenAI 兼容的提供商：

- **OpenAI** (GPT-4, GPT-3.5-Turbo)
- **DeepSeek** (deepseek-chat, deepseek-coder)
- **MiniMax** (自动端点回退)
- 其他兼容 OpenAI API 的服务

### DeepSeek 示例

```rust
let config = OpenAIConfig {
    api_key: "your-deepseek-key".to_string(),
    base_url: "https://api.deepseek.com".to_string(),
    model_name: "deepseek-chat".to_string(),
    max_tokens: 4096,
    proxy: None,
};

let service = OpenAIService::new(config);
```

### MiniMax 示例（自动端点回退）

```rust
let config = OpenAIConfig {
    api_key: "your-minimax-key".to_string(),
    base_url: "https://api.minimax.chat".to_string(),
    model_name: "abab6.5s-chat".to_string(),
    max_tokens: 4096,
    proxy: None,
};

let service = OpenAIService::new(config);
// 服务会自动尝试多个端点：
// 1. /chat/completions (标准 OpenAI 格式)
// 2. /text/chatcompletion_v2 (MiniMax 旧格式)
```

## 重试和错误恢复

服务自动处理：
- ✅ 指数退避重试（最多 10 次）
- ✅ 随机抖动避免惊群效应
- ✅ retry-after header 支持
- ✅ 自动参数修复（GPT-5/o1 系列）
- ✅ 端点自动回退（MiniMax 等）
- ✅ 详细的调试日志

### GPT-5/o1 系列自动适配

```rust
let config = OpenAIConfig {
    api_key: "your-key".to_string(),
    base_url: "https://api.openai.com/v1".to_string(),
    model_name: "gpt-5".to_string(), // 或 "o1-preview", "o3-mini" 等
    max_tokens: 4096,
    proxy: None,
};

let service = OpenAIService::new(config);
// 服务会自动：
// - 使用 max_completion_tokens 而非 max_tokens
// - 设置 temperature = 1
// - 修复参数不兼容问题
```

## 最佳实践

1. **使用系统提示词**：提供清晰的上下文和角色定义
2. **合理设置 max_tokens**：根据需求设置，避免不必要的成本
3. **启用流式响应**：改善用户体验，特别是长文本生成
4. **监控成本**：使用 `cost_usd` 字段跟踪 API 调用成本
5. **处理错误**：实现适当的错误处理和重试逻辑
6. **使用缓存**：对于重复请求，考虑使用 SessionCache

## 完整示例

```rust
use kode_services::openai::{OpenAIService, OpenAIConfig, SessionCache};
use kode_core::model::adapter::ModelAdapter;
use kode_core::message::Message;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建服务
    let config = OpenAIConfig {
        api_key: std::env::var("OPENAI_API_KEY")?,
        base_url: "https://api.openai.com/v1".to_string(),
        model_name: "gpt-4".to_string(),
        max_tokens: 4096,
        proxy: None,
    };

    let service = OpenAIService::new(config);

    // 发送消息
    let messages = vec![
        Message::user("Explain Rust's ownership system in simple terms"),
    ];

    let response = service
        .send_message(
            messages,
            Some("You are a helpful programming tutor.".to_string()),
            1000,
        )
        .await?;

    // 输出结果
    println!("Response: {}", response.content);
    println!("Tokens: {} in, {} out", response.usage.input_tokens, response.usage.output_tokens);
    println!("Cost: ${:.6}", response.cost_usd.unwrap_or(0.0));

    Ok(())
}
```

## 更多资源

- [OpenAI API 文档](https://platform.openai.com/docs/api-reference)
- [DeepSeek API 文档](https://platform.deepseek.com/api-docs/)
- [kode-core 文档](../../kode-core/README.md)
