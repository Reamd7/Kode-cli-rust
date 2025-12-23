//! 配置类型定义
//!
//! 定义了 Kode 的所有配置结构，包括全局配置、项目配置、模型配置等。

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 通知渠道类型
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum NotificationChannel {
    #[default]
    /// iTerm2
    Iterm2,
    /// 终端响铃
    TerminalBell,
    /// iTerm2 with bell
    Iterm2WithBell,
    /// 禁用通知
    NotificationsDisabled,
}

/// 自动更新状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AutoUpdaterStatus {
    /// 禁用
    Disabled,
    /// 启用
    Enabled,
    /// 无权限
    NoPermissions,
    /// 未配置
    NotConfigured,
}

/// 账户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    /// 账户 UUID
    pub account_uuid: String,
    /// 邮箱地址
    pub email_address: String,
    /// 组织 UUID
    pub organization_uuid: Option<String>,
}

/// MCP 服务器配置 - STDIO 类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct McpStdioServerConfig {
    /// 启动命令
    pub command: String,
    /// 命令参数
    pub args: Vec<String>,
    /// 环境变量
    pub env: Option<HashMap<String, String>>,
}

/// MCP 服务器配置 - SSE 类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct McpSseServerConfig {
    /// 服务器 URL
    pub url: String,
}

/// MCP 服务器配置（联合类型）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum McpServerConfig {
    /// STDIO 传输
    Stdio(McpStdioServerConfig),
    /// SSE 传输
    Sse(McpSseServerConfig),
}

/// 模型提供商标识
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProviderType {
    /// Anthropic
    Anthropic,
    /// OpenAI
    Openai,
    /// Mistral
    Mistral,
    /// DeepSeek
    Deepseek,
    /// Kimi
    Kimi,
    /// Qwen
    Qwen,
    /// GLM
    Glm,
    /// MiniMax
    Minimax,
    /// Baidu Qianfan
    BaiduQianfan,
    /// SiliconFlow
    Siliconflow,
    /// BigDream
    Bigdream,
    /// OpenDev
    Opendev,
    /// XAI
    Xai,
    /// Groq
    Groq,
    /// Gemini
    Gemini,
    /// Ollama
    Ollama,
    /// Azure
    Azure,
    /// 自定义
    Custom,
    /// 自定义 OpenAI 兼容
    CustomOpenai,
}

/// 模型配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelProfile {
    /// 配置名称
    pub name: String,
    /// 提供商类型
    pub provider: ProviderType,
    /// 模型名称（唯一标识）
    pub model_name: String,
    /// 自定义 API 基础 URL
    pub base_url: Option<String>,
    /// API 密钥
    pub api_key: String,
    /// 最大 token 数
    pub max_tokens: u32,
    /// 上下文长度
    pub context_length: u32,
    /// 推理强度（GPT-5）
    pub reasoning_effort: Option<String>,
    /// 是否激活
    pub is_active: bool,
    /// 创建时间戳
    pub created_at: u64,
    /// 最后使用时间
    pub last_used: Option<u64>,
    /// 是否为 GPT-5 模型
    pub is_gpt5: Option<bool>,
    /// 验证状态
    pub validation_status: Option<String>,
    /// 最后验证时间
    pub last_validation: Option<u64>,
}

/// 模型指针配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ModelPointers {
    /// 主要对话模型
    pub main: Option<String>,
    /// 任务执行模型
    pub task: Option<String>,
    /// 推理模型
    pub reasoning: Option<String>,
    /// 快速响应模型
    pub quick: Option<String>,
}

/// 项目配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProjectConfig {
    /// 允许使用的工具列表
    #[serde(default)]
    pub allowed_tools: Vec<String>,
    /// 上下文信息
    #[serde(default)]
    pub context: HashMap<String, String>,
    /// 上下文文件列表
    pub context_files: Option<Vec<String>>,
    /// 历史记录
    #[serde(default)]
    pub history: Vec<String>,
    /// 是否爬取目录
    pub dont_crawl_directory: Option<bool>,
    /// 是否启用架构工具
    pub enable_architect_tool: Option<bool>,
    /// MCP 上下文 URI
    #[serde(default)]
    pub mcp_context_uris: Vec<String>,
    /// MCP 服务器配置
    pub mcp_servers: Option<HashMap<String, McpServerConfig>>,
    /// 已批准的 mcprc 服务器
    pub approved_mcprc_servers: Option<Vec<String>>,
    /// 已拒绝的 mcprc 服务器
    pub rejected_mcprc_servers: Option<Vec<String>>,
    /// 上次 API 调用时长
    pub last_api_duration: Option<u64>,
    /// 上次成本
    pub last_cost: Option<f64>,
    /// 上次时长
    pub last_duration: Option<u64>,
    /// 上次会话 ID
    pub last_session_id: Option<String>,
    /// 示例文件列表
    pub example_files: Option<Vec<String>>,
    /// 示例文件生成时间
    pub example_files_generated_at: Option<u64>,
    /// 是否接受信任对话框
    pub has_trust_dialog_accepted: Option<bool>,
    /// 是否完成项目入职
    pub has_completed_project_onboarding: Option<bool>,
}

/// 全局配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GlobalConfig {
    /// 项目配置集合 - key 是绝对路径
    pub projects: Option<HashMap<String, ProjectConfig>>,
    /// 启动次数
    #[serde(default)]
    pub num_startups: u32,
    /// 自动更新状态
    pub auto_updater_status: Option<AutoUpdaterStatus>,
    /// 用户 ID
    pub user_id: Option<String>,
    /// 主题设置
    pub theme: Option<String>,
    /// 是否完成入职
    pub has_completed_onboarding: Option<bool>,
    /// 最后入职版本
    pub last_onboarding_version: Option<String>,
    /// 最后查看的版本说明
    pub last_release_notes_seen: Option<String>,
    /// MCP 服务器配置
    pub mcp_servers: Option<HashMap<String, McpServerConfig>>,
    /// 通知渠道
    #[serde(default)]
    pub preferred_notif_channel: NotificationChannel,
    /// 详细输出
    #[serde(default)]
    pub verbose: bool,
    /// 自定义 API key 响应
    pub custom_api_key_responses: Option<CustomApiKeyResponses>,
    /// 主要提供商
    pub primary_provider: Option<ProviderType>,
    /// 最大 token 数
    pub max_tokens: Option<u32>,
    /// 是否确认成本阈值
    pub has_acknowledged_cost_threshold: Option<bool>,
    /// OAuth 账户
    pub oauth_account: Option<AccountInfo>,
    /// iTerm2 快捷键绑定（已弃用）
    #[serde(rename = "iterm2KeyBindingInstalled")]
    pub iterm2_key_binding_installed: Option<bool>,
    /// Shift+Enter 快捷键绑定
    #[serde(rename = "shiftEnterKeyBindingInstalled")]
    pub shift_enter_key_binding_installed: Option<bool>,
    /// 代理设置
    pub proxy: Option<String>,
    /// 流式响应
    #[serde(default)]
    pub stream: bool,
    /// 模型配置列表（数组类型）
    pub model_profiles: Option<Vec<ModelProfile>>,
    /// 模型指针
    pub model_pointers: Option<ModelPointers>,
    /// 默认模型名称
    pub default_model_name: Option<String>,
    /// 最后忽略的更新版本
    pub last_dismissed_update_version: Option<String>,
}

/// 自定义 API key 响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CustomApiKeyResponses {
    /// 已批准的 key
    pub approved: Option<Vec<String>>,
    /// 已拒绝的 key
    pub rejected: Option<Vec<String>>,
}
