#!/usr/bin/env -S cargo +nightly -Zscript
---
[dependencies]
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", features = ["json"] }
serde_json = "1.0"
dotenvy = "0.15"
---

//! Integration test for Anthropic Service

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Get configuration from environment variables
    let api_key = std::env::var("ANTHROPIC_API_KEY")
        .expect("ANTHROPIC_API_KEY must be set in .env file or environment");
    let base_url = std::env::var("ANTHROPIC_BASE_URL")
        .unwrap_or_else(|_| "https://api.anthropic.com".to_string());
    let model = std::env::var("ANTHROPIC_MODEL")
        .unwrap_or_else(|_| "claude-3-5-sonnet-20241022".to_string());

    println!("🧪 Testing Anthropic Service Integration");
    println!("===================================");
    println!("  API Key: {}...", &api_key[..std::cmp::min(20, api_key.len())]);
    println!("  Base URL: {}", base_url);
    println!("  Model: {}", model);
    println!();

    let client = reqwest::Client::new();

    // Test 1: Verify API Key
    println!("1️⃣  Testing API key verification...");
    
    let response = client
        .get(format!("{}/v1/models", base_url))
        .header("x-api-key", &api_key)
        .header("anthropic-version", "2023-06-01")
        .send()
        .await;

    match response {
        Ok(r) if r.status() == 200 => {
            println!("   ✅ API key is valid!");
        }
        Ok(r) => {
            println!("   ❌ API key returned status: {}", r.status());
        }
        Err(e) => {
            println!("   ❌ Request failed: {}", e);
        }
    }
    println!();

    // Test 2: Send a simple message
    println!("2️⃣  Testing message sending...");
    
    let request_body = serde_json::json!({
        "model": model,
        "max_tokens": 100,
        "messages": [
            {
                "role": "user",
                "content": "Hello! Please respond with 'Integration test successful!'"
            }
        ]
    });

    let response = client
        .post(format!("{}/v1/messages", base_url))
        .header("x-api-key", &api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&request_body)
        .send()
        .await;

    match response {
        Ok(r) if r.status() == 200 => {
            let body: serde_json::Value = r.json().await.unwrap_or_default();
            println!("   ✅ Message sent successfully!");
            
            if let Some(content) = body.get("content").and_then(|c| c.as_array()) {
                for block in content {
                    if let Some(text_str) = block.get("text").and_then(|t| t.as_str()) {
                        println!("   Response: {}", text_str);
                    }
                }
            }
            
            if let Some(usage) = body.get("usage") {
                let input = usage.get("input_tokens").and_then(|t| t.as_u64()).unwrap_or(0);
                let output = usage.get("output_tokens").and_then(|t| t.as_u64()).unwrap_or(0);
                println!("   Input tokens: {}, Output tokens: {}", input, output);
            }
            
            let model_name = body.get("model").and_then(|m| m.as_str()).unwrap_or("unknown");
            println!("   Model: {}", model_name);
        }
        Ok(r) => {
            println!("   ❌ Message failed with status: {}", r.status());
            let text = r.text().await.unwrap_or_default();
            println!("   Error: {}", text.chars().take(300).collect::<String>());
        }
        Err(e) => {
            println!("   ❌ Request failed: {}", e);
        }
    }

    println!();
    println!("===================================");
    println!("🎉 Integration test completed!");
}
