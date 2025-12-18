# Kode-Rust 架构设计文档

## 项目概述

Kode-Rust 是基于 TypeScript 版本的 [Kode-cli](https://github.com/shareAI-lab/kode) 的完整 Rust 重写版本，使用 `ratatui` 构建 TUI 界面，`tokio` 作为异步运行时。

### 项目路径
- **原始 TS 版本**: `/Users/gemini/Documents/backup/Kode-cli`
- **Rust 版本**: `/Users/gemini/Documents/backup/Kode-cli-rust`

**核心目标：**
- 完整功能移植：实现所有核心功能（多模型支持、Agent系统、工具系统、MCP集成等）
- 配置兼容：与原版 `.kode.json` 完全兼容
- Agent兼容：支持相同的 markdown + YAML frontmatter 格式
- MCP兼容：支持 Model Context Protocol
- 高性能：利用 Rust 的性能优势和异步IO

## 技术栈

### 核心依赖
- **运行时**: `tokio` (1.x) - 异步运行时
- **TUI框架**: `ratatui` (0.28+) - 终端用户界面
- **HTTP客户端**: `reqwest` (0.12+) - API调用
- **序列化**: `serde`, `serde_json` - 配置和API交互
- **错误处理**: `anyhow` - 应用级错误处理
- **配置**: `serde_json`, `directories` - 配置管理

### 次要依赖
- **Markdown解析**: `pulldown-cmark`, `yaml-rust2` - Agent定义解析
- **进程执行**: `tokio::process` - Bash工具
- **文件操作**: `tokio::fs` - 异步文件IO
- **缓存**: `lru` - LRU缓存
- **正则**: `regex` - Grep工具
- **语法高亮**: `syntect` - 代码高亮
- **事件处理**: `crossterm` - 键盘输入

## 项目结构 (Workspace)

```
kode-cli-rust/
├── Cargo.toml                    # Workspace 配置
├── ARCHITECTURE.md               # 本文档
├── README.md                     # 项目说明
├── .gitignore
│
├── crates/
│   ├── kode-core/                # 核心功能库
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── agent/            # Agent 系统
│   │       ├── config/           # 配置管理
│   │       ├── context/          # 上下文管理
│   │       ├── error.rs          # 错误定义
│   │       ├── message/          # 消息管理
│   │       └── model/            # 模型抽象
│   │
│   ├── kode-tools/               # 工具系统
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── tool.rs           # Tool trait 定义
│   │       ├── registry.rs       # 工具注册表
│   │       ├── bash.rs           # Bash 工具
│   │       ├── file_read.rs      # 文件读取
│   │       ├── file_write.rs     # 文件写入
│   │       ├── file_edit.rs      # 文件编辑
│   │       ├── grep.rs           # 搜索工具
│   │       ├── glob.rs           # 文件匹配
│   │       ├── task.rs           # 任务委托
│   │       └── mcp/              # MCP 工具适配
│   │
│   ├── kode-services/            # 外部服务集成
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── anthropic.rs      # Anthropic API
│   │       ├── openai.rs         # OpenAI-compatible APIs
│   │       ├── streaming.rs      # SSE 流处理
│   │       ├── model_adapter.rs  # 模型适配器
│   │       └── mcp_client.rs     # MCP 客户端
│   │
│   ├── kode-ui/                  # TUI 界面
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── app.rs            # 应用状态
│   │       ├── repl.rs           # REPL 主界面
│   │       ├── components/       # UI 组件
│   │       ├── widgets/          # 自定义 Widget
│   │       ├── input.rs          # 输入处理
│   │       └── renderer.rs       # 渲染逻辑
│   │
│   └── kode-cli/                 # CLI 入口
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs           # 主入口
│           ├── commands/         # CLI 命令
│           └── cli.rs            # 命令行解析
│
└── tests/                        # 集成测试
    ├── config_test.rs
    ├── agent_test.rs
    └── tool_test.rs
```

## 核心模块设计

### 1. kode-core: 核心功能

#### Agent 系统 (`agent/`)
```rust
pub struct Agent {
    pub name: String,
    pub description: String,
    pub tools: ToolFilter,
    pub model: Option<String>,
    pub system_prompt: String,
}

pub enum ToolFilter {
    All,
    Specific(Vec<String>),
}

pub struct AgentLoader {
    cache: LruCache<PathBuf, Agent>,
    directories: Vec<PathBuf>,
}

impl AgentLoader {
    // 5层优先级加载
    // 1. Built-in
    // 2. ~/.claude/agents/
    // 3. ~/.kode/agents/
    // 4. ./.claude/agents/
    // 5. ./.kode/agents/
    pub async fn load_agents(&mut self) -> Result<Vec<Agent>>;
    pub async fn get_agent(&mut self, name: &str) -> Result<Agent>;
}
```

#### 配置管理 (`config/`)
```rust
#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub model_profiles: HashMap<String, ModelProfile>,
    pub default_model: String,
    pub task_model: Option<String>,
    pub reasoning_model: Option<String>,
    pub quick_model: Option<String>,
    pub mcp_servers: Option<Vec<McpServer>>,
    // ... 其他配置
}

impl Config {
    // 分层加载：全局 (~/.kode.json) + 项目 (./.kode.json) + 环境变量
    pub fn load() -> Result<Self>;
    pub fn save(&self) -> Result<()>;
    pub fn merge(&mut self, other: Config);
}
```

#### 模型抽象 (`model/`)
```rust
#[async_trait]
pub trait ModelAdapter: Send + Sync {
    async fn send_message(
        &self,
        messages: Vec<Message>,
        tools: Vec<ToolSchema>,
    ) -> Result<Response>;

    async fn stream_message(
        &self,
        messages: Vec<Message>,
        tools: Vec<ToolSchema>,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<StreamChunk>>>>>;
}

pub struct ModelManager {
    adapters: HashMap<String, Box<dyn ModelAdapter>>,
    current_model: String,
}
```

#### 消息上下文 (`message/`)
```rust
pub struct MessageContextManager {
    messages: Vec<Message>,
    max_tokens: usize,
    token_counter: Box<dyn TokenCounter>,
}

impl MessageContextManager {
    pub fn add_message(&mut self, msg: Message) -> Result<()>;
    pub fn trim_to_fit(&mut self) -> Result<()>;
    pub fn get_context(&self) -> &[Message];
}
```

### 2. kode-tools: 工具系统

#### Tool Trait
```rust
#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn schema(&self) -> ToolSchema;

    async fn execute(
        &self,
        params: Value,
        context: &ToolContext,
    ) -> Result<ToolResult>;

    fn requires_permission(&self) -> bool {
        false
    }
}

pub struct ToolSchema {
    pub name: String,
    pub description: String,
    pub parameters: JsonSchema,
}

pub struct ToolContext {
    pub working_dir: PathBuf,
    pub permissions: Arc<PermissionManager>,
}

pub struct ToolResult {
    pub output: String,
    pub artifacts: Vec<Artifact>,
}
```

#### 工具注册表
```rust
pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn Tool>>,
}

impl ToolRegistry {
    pub fn register(&mut self, tool: Arc<dyn Tool>);
    pub fn get(&self, name: &str) -> Option<Arc<dyn Tool>>;
    pub fn list(&self) -> Vec<&str>;
    pub fn filter(&self, names: &[String]) -> Vec<Arc<dyn Tool>>;
}
```

#### 核心工具实现
- `BashTool`: 执行 shell 命令
- `FileReadTool`: 读取文件
- `FileWriteTool`: 写入文件
- `FileEditTool`: 编辑文件（精确替换）
- `GrepTool`: 内容搜索（基于 ripgrep）
- `GlobTool`: 文件模式匹配
- `TaskTool`: 委托任务给子 Agent

### 3. kode-services: 服务集成

#### Anthropic 服务
```rust
pub struct AnthropicService {
    client: reqwest::Client,
    api_key: String,
    base_url: String,
}

impl ModelAdapter for AnthropicService {
    async fn send_message(...) -> Result<Response> {
        // 实现完整的 Messages API 调用
    }

    async fn stream_message(...) -> Result<...> {
        // 实现 SSE 流式响应
    }
}
```

#### 流式响应处理
```rust
pub struct StreamingResponse {
    stream: Pin<Box<dyn Stream<Item = Result<StreamChunk>>>>,
}

pub enum StreamChunk {
    ContentBlockStart { index: usize },
    ContentBlockDelta { delta: String },
    ContentBlockStop,
    ToolUse { tool_name: String, params: Value },
    MessageStop,
}

impl StreamingResponse {
    pub async fn collect_full_response(self) -> Result<Response>;
    pub fn into_stream(self) -> impl Stream<Item = Result<StreamChunk>>;
}
```

#### MCP 客户端
```rust
pub struct McpClient {
    servers: HashMap<String, McpServer>,
}

impl McpClient {
    pub async fn connect(&mut self, config: McpServerConfig) -> Result<()>;
    pub async fn list_tools(&self, server: &str) -> Result<Vec<ToolSchema>>;
    pub async fn call_tool(&self, server: &str, name: &str, params: Value) -> Result<Value>;
}
```

### 4. kode-ui: TUI 界面

#### 应用状态
```rust
pub struct App {
    pub state: AppState,
    pub repl: ReplState,
    pub model_manager: Arc<ModelManager>,
    pub tool_registry: Arc<ToolRegistry>,
    pub config: Config,
}

pub enum AppState {
    Repl,
    ToolExecution,
    PermissionRequest,
    AgentWorking,
}

pub struct ReplState {
    pub messages: Vec<DisplayMessage>,
    pub input: String,
    pub scroll_offset: usize,
    pub waiting_for_response: bool,
}
```

#### REPL 界面
```rust
pub struct ReplWidget<'a> {
    messages: &'a [DisplayMessage],
    input: &'a str,
    scroll: usize,
}

impl Widget for ReplWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // 实现消息列表 + 输入框的渲染
    }
}
```

#### 组件系统
- `MessageList`: 消息滚动列表
- `InputBox`: 输入框
- `StatusBar`: 状态栏
- `ToolOutput`: 工具输出显示
- `PermissionDialog`: 权限请求对话框
- `ThinkingIndicator`: 思考中动画

### 5. kode-cli: CLI 入口

```rust
#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { prompt } => {
            let app = App::new().await?;
            if let Some(prompt) = prompt {
                app.run_once(prompt).await?;
            } else {
                app.run_interactive().await?;
            }
        }
        Commands::Config { action } => {
            handle_config_command(action).await?;
        }
        Commands::Agents { list } => {
            handle_agents_command(list).await?;
        }
    }

    Ok(())
}
```

## 数据流设计

### 用户输入 -> AI 响应流程
```
User Input (TUI)
    ↓
Input Handler
    ↓
Message Context Manager (add user message)
    ↓
Model Manager (select model)
    ↓
Model Adapter (send request with tools)
    ↓
Streaming Response
    ↓ (real-time)
UI Update (content delta)
    ↓
Tool Use Detection
    ↓
Permission Check
    ↓ (if approved)
Tool Execution
    ↓
Add Tool Result to Context
    ↓
Continue AI Response
    ↓
Complete & Update UI
```

### Agent 任务委托流程
```
Main Agent receives TaskTool use
    ↓
Load Sub-Agent config
    ↓
Filter tools by agent config
    ↓
Create sub-context with system prompt
    ↓
Model Adapter (with filtered tools)
    ↓
Sub-agent responds
    ↓
Return result to main agent
    ↓
Main agent continues
```

## 配置兼容性

### .kode.json 格式（与 TS 版本完全兼容）
```json
{
  "modelProfiles": {
    "claude-sonnet": {
      "provider": "anthropic",
      "apiKey": "sk-ant-...",
      "model": "claude-sonnet-4-5-20250929"
    },
    "deepseek": {
      "provider": "openai-compatible",
      "apiKey": "sk-...",
      "baseURL": "https://api.deepseek.com",
      "model": "deepseek-chat"
    }
  },
  "defaultModel": "claude-sonnet",
  "taskModel": "deepseek",
  "mcpServers": [
    {
      "name": "filesystem",
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-filesystem", "/Users/..."]
    }
  ]
}
```

### Agent 定义格式（markdown + YAML）
```markdown
---
name: code-reviewer
description: "Reviews code for best practices and potential issues"
tools: ["FileRead", "Grep", "Bash"]
model: "claude-sonnet"
---

You are an expert code reviewer. When reviewing code:
1. Check for security vulnerabilities
2. Look for performance issues
3. Ensure code follows best practices
...
```

## 开发阶段计划

### Phase 1: 基础架构 (Week 1-2)
**目标**: 建立项目框架和核心抽象

- [ ] 初始化 Cargo workspace
- [ ] 实现 `kode-core` 基础模块
  - [ ] Config 加载和保存
  - [ ] ModelAdapter trait
  - [ ] Message 数据结构
- [ ] 实现 `kode-tools` 基础
  - [ ] Tool trait 定义
  - [ ] ToolRegistry
  - [ ] FileReadTool, FileWriteTool (基础版)
- [ ] 实现 `kode-services` 基础
  - [ ] AnthropicService (非流式)
  - [ ] 简单的请求/响应
- [ ] 单元测试覆盖

**里程碑**: 能够加载配置、调用 Claude API、执行基础文件操作

### Phase 2: 核心功能 (Week 3-5)
**目标**: 实现主要功能，可以基本使用

- [ ] 完善 Model 系统
  - [ ] 流式响应 (SSE)
  - [ ] OpenAI-compatible adapter
  - [ ] 多模型切换
- [ ] 工具系统完善
  - [ ] BashTool (进程执行)
  - [ ] GrepTool (ripgrep 集成)
  - [ ] GlobTool (文件匹配)
  - [ ] FileEditTool (精确编辑)
- [ ] Agent 系统
  - [ ] Agent 定义解析 (YAML frontmatter)
  - [ ] 5层目录加载
  - [ ] Agent 缓存
  - [ ] TaskTool (子 Agent 委托)
- [ ] 基础 TUI
  - [ ] 简单的 REPL 界面
  - [ ] 消息显示
  - [ ] 输入框
  - [ ] 实时流式输出
- [ ] 权限系统
  - [ ] Permission 检查
  - [ ] 用户批准对话框
- [ ] 单元测试 + 集成测试

**里程碑**: 可以交互式聊天、执行工具、委托任务给 Agent

### Phase 3: 高级特性 (Week 6-8)
**目标**: 完整功能，生产可用

- [ ] MCP 集成
  - [ ] MCP 客户端实现
  - [ ] MCP 工具动态加载
  - [ ] MCP 服务器管理
- [ ] 上下文管理
  - [ ] MessageContextManager
  - [ ] 智能 token 计数
  - [ ] 上下文窗口优化
- [ ] UI 完善
  - [ ] 语法高亮
  - [ ] 代码块渲染
  - [ ] Markdown 格式化
  - [ ] 状态栏
  - [ ] 加载动画
  - [ ] 错误显示
- [ ] 性能优化
  - [ ] 并发工具执行
  - [ ] 缓存优化
  - [ ] 内存管理
- [ ] CLI 命令
  - [ ] `kode config` - 配置管理
  - [ ] `kode agents` - Agent 列表
  - [ ] `kode tools` - 工具列表
- [ ] 完整测试套件

**里程碑**: 功能完整，与 TS 版本功能对等

### Phase 4: 优化和发布 (Week 9-10)
**目标**: 优化、文档、发布

- [ ] 性能基准测试
- [ ] 内存优化
- [ ] 文档完善
  - [ ] README
  - [ ] 用户指南
  - [ ] 开发者文档
  - [ ] API 文档
- [ ] CI/CD 设置
- [ ] 发布准备
  - [ ] Cross-platform builds
  - [ ] Release binaries
  - [ ] crates.io 发布

## 性能目标

- **启动时间**: < 100ms
- **配置加载**: < 50ms
- **Agent 加载**: < 20ms (cached), < 100ms (uncached)
- **工具执行**: 取决于工具，但框架开销 < 5ms
- **UI 渲染**: 60 FPS
- **内存占用**: < 50MB (idle), < 200MB (active)

## 兼容性测试清单

- [ ] 加载原版 `.kode.json` 配置
- [ ] 加载原版 Agent 定义文件
- [ ] MCP 服务器互操作
- [ ] 配置文件修改后原版可读
- [ ] Agent 文件修改后原版可读

## 后续扩展方向

1. **插件系统**: 动态加载工具插件 (dylibstatic)
2. **Web UI**: 可选的 Web 界面（Tauri/Leptos）
3. **分布式 Agent**: 跨机器的 Agent 协作
4. **高级 Context**: 向量数据库集成、代码图谱
5. **IDE 集成**: LSP 支持、VSCode 插件

## 依赖清单

```toml
# Workspace Cargo.toml
[workspace]
members = [
    "crates/kode-core",
    "crates/kode-tools",
    "crates/kode-services",
    "crates/kode-ui",
    "crates/kode-cli",
]

[workspace.dependencies]
# 异步运行时
tokio = { version = "1.35", features = ["full"] }
async-trait = "0.1"

# 序列化
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# HTTP
reqwest = { version = "0.12", features = ["json", "stream"] }

# TUI
ratatui = "0.28"
crossterm = "0.27"

# 错误处理
anyhow = "1.0"

# 配置
directories = "5.0"

# 其他
regex = "1.10"
lru = "0.12"
pulldown-cmark = "0.11"
yaml-rust2 = "0.8"
```

## 总结

本架构设计基于以下原则：
1. **模块化**: Workspace 分离职责
2. **异步优先**: Tokio 异步 IO
3. **类型安全**: 利用 Rust 类型系统
4. **兼容性**: 与 TS 版本配置和 Agent 格式兼容
5. **可扩展**: Trait-based 工具系统
6. **高性能**: 零成本抽象、并发执行
7. **用户友好**: 流式响应、实时 UI 更新

开发采用分阶段方式，确保每个阶段都有可测试的里程碑。
