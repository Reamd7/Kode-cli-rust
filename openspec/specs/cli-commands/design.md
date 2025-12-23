# 设计文档 - CLI 命令 / Design Document - CLI Commands

## Context

CLI 命令是 Kode-Rust 的用户入口，提供命令行接口来访问各种功能。

The CLI commands are the user entry point for Kode-Rust, providing a command-line interface to access various features.

### 当前状态 / Current State

CLI 命令计划在 `crates/kode-cli/src/` 中实现，尚未开始实现。

The CLI commands are planned to be implemented in `crates/kode-cli/src/`, implementation has not started yet.

## TypeScript 版本参考 / TypeScript Version Reference

在实现本设计时，请参考原版 TypeScript 项目中的以下文件：

When implementing this design, refer to the following files in the original TypeScript project:

### CLI 结构 / CLI Structure
- **命令定义**: `/Users/gemini/Documents/backup/Kode-cli/src/commands.ts`
  - 命令注册和处理
  - 命令参数解析

### 命令实现 / Command Implementations
- **命令目录**: `/Users/gemini/Documents/backup/Kode-cli/src/commands/`
  - 所有 CLI 命令实现
  - 参数验证
  - 执行逻辑

### 入口点 / Entry Points
- **应用入口**: `/Users/gemini/Documents/backup/Kode-cli/src/entrypoints/`
  - CLI 入口点
  - 命令行参数解析
  - 应用初始化

### 实现细节 / Implementation Details
1. **命令解析**: 使用命令行参数解析库（如 clap）
2. **错误处理**: 清晰的错误信息
3. **帮助文档**: 详细的命令帮助
4. **配置交互**: 与配置系统交互

## Goals / Non-Goals

### Goals

- [ ] 使用 Clap 定义 CLI 结构 / Use Clap to define CLI structure
- [ ] 实现配置管理命令 / Implement config management commands
- [ ] 实现 Agent 命令 / Implement Agent commands
- [ ] 实现工具命令 / Implement tool commands
- [ ] 实现模型命令 / Implement model commands

### Non-Goals

- [ ] 交互式 TUI（在 kode-ui 中）/ Interactive TUI (in kode-ui)
- [ ] 调试命令 / Debug commands
- [ ] 插件系统 / Plugin system

## Decisions

### 决策 1: CLI 框架选择 / Decision 1: CLI Framework Selection

**选择**: 使用 Clap with derive 宏

**Choice**: Use Clap with derive macros

**理由 / Rationale**:
- 项目已依赖 Clap / Project already depends on Clap
- 类型安全 / Type-safe
- 自动生成帮助信息 / Automatic help generation
- 派生宏简化定义 / Derive macros simplify definition

### 决策 2: CLI 结构定义 / Decision 2: CLI Structure Definition

**实现 / Implementation**:
```rust
/// 来自 ARCHITECTURE.md L369-393
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

#### 命令结构
```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "kode")]
struct Cli {
    #[arg(conflicts_with = "subcommand")]
    prompt: Option<String>,

    #[command(subcommand)]
    subcommand: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Run { prompt: Option<String> },
    Config(ConfigAction),
    Agents(AgentAction),
    Tools(ToolAction),
}
```

    /// 模型命令
    /// Model commands
    Models {
        #[command(subcommand)]
        action: ModelAction,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    /// 列出所有配置
    /// List all configuration
    List,

    /// 设置配置值
    /// Set configuration value
    Set {
        /// 配置键
        /// Configuration key
        key: String,
        /// 配置值
        /// Configuration value
        value: String,
    },

    /// 获取配置值
    /// Get configuration value
    Get {
        /// 配置键
        /// Configuration key
        key: String,
    },
}

#[derive(Subcommand)]
enum AgentAction {
    /// 列出所有 Agent
    /// List all Agents
    List,

    /// 显示 Agent 详情
    /// Show Agent details
    Show {
        /// Agent 名称
        /// Agent name
        name: String,
    },

    /// 验证 Agent 定义
    /// Validate Agent definition
    Validate {
        /// Agent 名称
        /// Agent name
        name: String,
    },
}

#[derive(Subcommand)]
enum ToolAction {
    /// 列出所有工具
    /// List all tools
    List,

    /// 显示工具详情
    /// Show tool details
    Show {
        /// 工具名称
        /// Tool name
        name: String,
    },
}

#[derive(Subcommand)]
enum ModelAction {
    /// 列出所有模型
    /// List all models
    List,

    /// 切换默认模型
    /// Switch default model
    Switch {
        /// 模型名称
        /// Model name
        name: String,
    },
}
```

### 决策 3: 主入口实现 / Decision 3: Main Entry Implementation

**设计 / Design**:
```rust
#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.subcommand {
        Some(Commands::Config { action }) => {
            handle_config_command(action).await?;
        }
        Some(Commands::Agents { action }) => {
            handle_agents_command(action).await?;
        }
        Some(Commands::Tools { action }) => {
            handle_tools_command(action).await?;
        }
        Some(Commands::Models { action }) => {
            handle_models_command(action).await?;
        }
        None => {
            // 直接运行或启动 REPL
            if let Some(prompt) = cli.prompt {
                run_once(prompt).await?;
            } else {
                run_repl().await?;
            }
        }
    }

    Ok(())
}
```

### 决策 4: 配置命令实现 / Decision 4: Config Commands Implementation

**设计 / Design**:
```rust
async fn handle_config_command(action: ConfigAction) -> Result<()> {
    let loader = ConfigLoader::new();

    match action {
        ConfigAction::List => {
            let global = loader.load_global().await?;
            let project = loader.load_current_project().await?;

            println!("Global Configuration (~/.kode.json):");
            println!("{}", serde_json::to_string_pretty(&global)?);

            println!("\nProject Configuration (./.kode.json):");
            println!("{}", serde_json::to_string_pretty(&project)?);
        }
        ConfigAction::Set { key, value } => {
            let mut config = loader.load_global().await?;
            // 设置配置值（简化实现）
            match key.as_str() {
                "theme" => config.theme = Some(value),
                "verbose" => config.verbose = Some(value.parse()?),
                _ => return Err(Error::UnknownConfigKey(key)),
            }
            loader.save(&config, &dirs::home_dir().unwrap().join(".kode.json")).await?;
            println!("Configuration updated: {} = {}", key, value);
        }
        ConfigAction::Get { key } => {
            let config = loader.load_global().await?;
            let value = match key.as_str() {
                "theme" => config.theme,
                "verbose" => config.verbose.map(|v| v.to_string()),
                _ => return Err(Error::UnknownConfigKey(key)),
            };
            match value {
                Some(v) => println!("{}: {}", key, v),
                None => println!("{}: <not set>", key),
            }
        }
    }

    Ok(())
}
```

### 决策 5: Agent 命令实现 / Decision 5: Agent Commands Implementation

**设计 / Design**:
```rust
async fn handle_agents_command(action: AgentAction) -> Result<()> {
    let mut loader = AgentLoader::new();

    match action {
        AgentAction::List => {
            let agents = loader.list_agents().await?;
            println!("Available Agents:");
            for agent in agents {
                println!("  - {}: {}", agent.name,
                    agent.description.as_ref().unwrap_or(&"(no description)".to_string()));
            }
        }
        AgentAction::Show { name } => {
            let agent = loader.load_agent(&name).await?;
            println!("Name: {}", agent.name);
            if let Some(desc) = agent.description {
                println!("Description: {}", desc);
            }
            println!("Tools: {:?}", agent.tools);
            if let Some(model) = agent.model {
                println!("Model: {}", model);
            }
            println!("\nSystem Prompt:");
            println!("{}", agent.system_prompt);
        }
        AgentAction::Validate { name } => {
            match loader.load_agent(&name).await {
                Ok(agent) => {
                    println!("Agent '{}' is valid.", name);
                    println!("  Tools: {:?}", agent.tools);
                }
                Err(e) => {
                    println!("Agent '{}' validation failed: {}", name, e);
                    return Err(e);
                }
            }
        }
    }

    Ok(())
}
```

### 决策 6: 工具命令实现 / Decision 6: Tool Commands Implementation

**设计 / Design**:
```rust
async fn handle_tools_command(action: ToolAction) -> Result<()> {
    let registry = create_tool_registry().await?;

    match action {
        ToolAction::List => {
            let tools = registry.get_schemas();
            println!("Available Tools:");
            for tool in tools {
                println!("  - {}: {}", tool.name, tool.description);
            }
        }
        ToolAction::Show { name } => {
            let tool = registry.get(&name)
                .ok_or_else(|| Error::ToolNotFound(name))?;
            println!("Name: {}", tool.name());
            println!("Description: {}", tool.description());
            println!("Parameters:");
            println!("{}", serde_json::to_string_pretty(&tool.schema())?);
        }
    }

    Ok(())
}

async fn create_tool_registry() -> Result<ToolRegistry> {
    let mut registry = ToolRegistry::new();
    registry.register(Arc::new(FileReadTool))?;
    registry.register(Arc::new(FileWriteTool))?;
    registry.register(Arc::new(BashTool::new(Duration::from_secs(30))))?;
    // ... 更多工具
    Ok(registry)
}
```

### 决策 7: 模型命令实现 / Decision 7: Model Commands Implementation

**设计 / Design**:
```rust
async fn handle_models_command(action: ModelAction) -> Result<()> {
    let loader = ConfigLoader::new();
    let config = loader.load_global().await?;

    match action {
        ModelAction::List => {
            if let Some(profiles) = config.model_profiles {
                println!("Available Models:");
                for (name, profile) in profiles {
                    println!("  - {} (provider: {:?}, model: {})",
                        name, profile.provider, profile.model_name);
                }
            } else {
                println!("No models configured.");
            }
        }
        ModelAction::Switch { name } => {
            let profiles = config.model_profiles.as_ref()
                .ok_or_else(|| Error::NoModelsConfigured)?;

            if !profiles.contains_key(&name) {
                return Err(Error::ModelNotFound(name));
            }

            let mut config = config;
            config.default_model = Some(name.clone());

            let path = dirs::home_dir().unwrap().join(".kode.json");
            loader.save(&config, &path).await?;
            println!("Default model switched to: {}", name);
        }
    }

    Ok(())
}
```

## 命令示例 / Command Examples

### 基本使用 / Basic Usage

```bash
# 启动交互式 REPL
kode

# 运行单次提示
kode "List all files in the current directory"
```

### 配置管理 / Configuration Management

```bash
# 列出所有配置
kode config list

# 设置主题
kode config set theme dark

# 获取配置值
kode config get theme
```

### Agent 管理 / Agent Management

```bash
# 列出所有 Agent
kode agents list

# 显示 Agent 详情
kode agents show code-reviewer

# 验证 Agent
kode agents validate code-reviewer
```

### 工具管理 / Tool Management

```bash
# 列出所有工具
kode tools list

# 显示工具详情
kode tools show read_file
```

### 模型管理 / Model Management

```bash
# 列出所有模型
kode models list

# 切换默认模型
kode models switch claude-sonnet
```

## 错误处理 / Error Handling

| 错误类型 / Error Type | 处理方式 / Handling |
|---------------------|-------------------|
| `UnknownConfigKey` | 提示有效的配置键 / Suggest valid config keys |
| `ToolNotFound` | 列出可用工具 / List available tools |
| `AgentNotFound` | 列出可用 Agent / List available agents |
| `ModelNotFound` | 列出可用模型 / List available models |

## 测试策略 / Testing Strategy

### 单元测试 / Unit Tests

```rust
#[test]
fn test_cli_parsing() {
    let cli = Cli::try_parse_from(["kode", "config", "list"]).unwrap();
    assert!(matches!(cli.subcommand, Some(Commands::Config { .. })));
}
```

## Risks / Trade-offs

### 风险 1: 命令冲突 / Risk 1: Command Conflicts

**风险 / Risk**: prompt 和子命令可能冲突

**缓解措施 / Mitigation**:
- 使用 `conflicts_with` 属性 / Use `conflicts_with` attribute
- 清晰的错误消息 / Clear error messages

### 风险 2: 配置验证 / Risk 2: Configuration Validation

**风险 / Risk**: 用户可能设置无效配置

**缓解措施 / Mitigation**:
- 验证配置值 / Validate configuration values
- 提供回滚机制 / Provide rollback mechanism

## Open Questions

1. **是否需要支持配置文件路径参数？**
   - 建议：暂不实现，使用默认路径
   - Rationale: Don't implement for now, use default paths

2. **是否需要支持输出格式化（JSON/YAML）？**
   - 建议：添加 `--output` 选项
   - Rationale: Add `--output` option

3. **是否需要支持别名？**
   - 建议：支持常用命令的简短别名
   - Rationale: Support short aliases for common commands

## 参考资源 / References

- [Clap 文档](https://docs.rs/clap/)
- [Clap 派生教程](https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html)
- [命令行设计最佳实践](https://clig.dev/#summary)
