//! VCR (Video Cassette Recorder) - 请求录制/回放支持
//!
//! 用于测试时录制 API 请求/响应并回放，无需真实 API 调用。

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

/// VCR 模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VcrMode {
    /// 录制模式：发送真实请求并保存到文件
    Record,
    /// 回放模式：从文件读取响应
    Replay,
    /// 关闭：正常发送请求
    Off,
}

/// VCR 配置
#[derive(Debug, Clone)]
pub struct VcrConfig {
    /// VCR 模式
    pub mode: VcrMode,
    /// cassettes 目录路径
    pub cassette_dir: PathBuf,
    /// 是否正在测试环境
    pub is_test: bool,
}

impl Default for VcrConfig {
    fn default() -> Self {
        Self {
            mode: VcrMode::Off,
            cassette_dir: PathBuf::from("fixtures"),
            is_test: cfg!(test),
        }
    }
}

/// 录制的请求/响应对
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cassette {
    /// 请求的哈希值（用于生成文件名）
    pub request_hash: String,
    /// 输入消息（脱水后的）
    pub input: Vec<String>,
    /// 响应内容
    pub output: RecordedResponse,
    /// 录制时间戳
    pub recorded_at: chrono::DateTime<chrono::Utc>,
}

/// 录制的响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordedResponse {
    /// 响应内容
    pub content: String,
    /// 输入 token 数
    pub input_tokens: usize,
    /// 输出 token 数
    pub output_tokens: usize,
    /// 成本（美元）
    pub cost_usd: f64,
    /// 耗时（毫秒）
    pub duration_ms: u64,
}

/// VCR 录制/回放器
#[derive(Debug)]
pub struct Vcr {
    config: VcrConfig,
    /// 全局模式（可以覆盖配置）
    global_mode: Mutex<VcrMode>,
}

impl Vcr {
    /// 创建新的 VCR 实例
    pub fn new(config: VcrConfig) -> Self {
        Self {
            config,
            global_mode: Mutex::new(VcrMode::Off),
        }
    }

    /// 获取当前模式
    #[allow(dead_code)]
    fn mode(&self) -> VcrMode {
        if !self.config.is_test {
            return VcrMode::Off;
        }
        *self.global_mode.lock().unwrap()
    }

    /// 设置全局模式（可覆盖配置）
    pub fn set_mode(&self, mode: VcrMode) {
        *self.global_mode.lock().unwrap() = mode;
    }

    /// 生成 cassette 文件名
    fn cassette_path(&self, request_hash: &str) -> PathBuf {
        self.config
            .cassette_dir
            .join(format!("{}.json", request_hash))
    }

    /// 生成请求哈希
    #[allow(dead_code)]
    fn hash_request(&self, messages: &[String]) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        for msg in messages {
            hasher.update(msg);
        }
        let result = hasher.finalize();
        hex::encode(&result[..6]) // 只取前 6 个字符
    }

    /// 脱水消息（替换动态值）
    #[allow(dead_code)]
    fn dehydrate_message(msg: &str) -> String {
        let mut result = msg.to_string();
        // 替换路径
        if let Ok(cwd) = std::env::current_dir() {
            let cwd_str = cwd.to_string_lossy();
            result = result.replace(&*cwd_str, "[CWD]");
        }
        // 替换数字（简化版本）
        result = result.replace("\\", "/");
        result
    }

    /// 检查 cassette 是否存在
    pub fn cassette_exists(&self, request_hash: &str) -> bool {
        self.cassette_path(request_hash).exists()
    }

    /// 读取录制的响应
    pub fn read_cassette(&self, request_hash: &str) -> Option<RecordedResponse> {
        let path = self.cassette_path(request_hash);
        if !path.exists() {
            return None;
        }
        let content = std::fs::read_to_string(path).ok()?;
        let cassette: Cassette = serde_json::from_str(&content).ok()?;
        Some(cassette.output)
    }

    /// 写入 cassette
    pub async fn write_cassette(
        &self,
        request_hash: &str,
        input: &[String],
        output: &RecordedResponse,
    ) -> Result<()> {
        let path = self.cassette_path(request_hash);
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let cassette = Cassette {
            request_hash: request_hash.to_string(),
            input: input.to_vec(),
            output: output.clone(),
            recorded_at: chrono::Utc::now(),
        };

        let content = serde_json::to_string_pretty(&cassette)?;
        tokio::fs::write(&path, content).await?;
        Ok(())
    }
}

/// 构建器模式创建 VCR
#[derive(Debug, Default)]
pub struct VcrBuilder {
    config: VcrConfig,
}

impl VcrBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置 cassette 目录
    pub fn cassette_dir<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.config.cassette_dir = path.as_ref().to_path_buf();
        self
    }

    /// 设置模式
    pub fn mode(mut self, mode: VcrMode) -> Self {
        self.config.mode = mode;
        self
    }

    /// 构建 VCR 实例
    pub fn build(self) -> Vcr {
        Vcr::new(self.config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vcr_builder() {
        let vcr = VcrBuilder::new()
            .cassette_dir("/tmp/cassettes")
            .mode(VcrMode::Record)
            .build();

        assert_eq!(vcr.config.cassette_dir, PathBuf::from("/tmp/cassettes"));
    }

    #[test]
    fn test_dehydrate_message() {
        let cwd = std::env::current_dir().unwrap();
        let cwd_str = cwd.to_string_lossy();
        let msg = format!("Reading file {}/project/src/main.rs", cwd_str);
        let dehydrated = Vcr::dehydrate_message(&msg);
        // 路径应该被替换
        assert!(!dehydrated.contains(&*cwd_str));
        assert!(dehydrated.contains("[CWD]"));
    }

    #[test]
    fn test_recorded_response_serialization() {
        let response = RecordedResponse {
            content: "Hello!".to_string(),
            input_tokens: 10,
            output_tokens: 5,
            cost_usd: 0.0001,
            duration_ms: 100,
        };

        let json = serde_json::to_string(&response).unwrap();
        let parsed: RecordedResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.content, "Hello!");
        assert_eq!(parsed.input_tokens, 10);
        assert_eq!(parsed.output_tokens, 5);
    }
}
