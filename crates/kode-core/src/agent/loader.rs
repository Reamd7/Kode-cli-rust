//! Agent 加载器
//!
//! 从多个目录加载 Agent 配置，支持五层优先级和 LRU 缓存。

use crate::agent::types::{Agent, AgentLocation};
use crate::error::{Error, Result};

use lru::LruCache;
use serde_yaml::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::fs;

/// Agent 加载器
///
/// 支持五层目录优先级加载 Agent：
/// 1. Built-in agents（内置）
/// 2. ~/.claude/agents/（用户 Claude）
/// 3. ~/.kode/agents/（用户 Kode）
/// 4. ./.claude/agents/（项目 Claude）
/// 5. ./.kode/agents/（项目 Kode）
///
/// 后面的层会覆盖前面的层，允许用户和项目级别的 Agent 覆盖内置 Agent。
///
/// # Examples
///
/// ```no_run
/// use kode_core::agent::loader::AgentLoader;
///
/// # async fn example() -> kode_core::error::Result<()> {
/// let mut loader = AgentLoader::new()?;
/// let agents = loader.load_all_agents().await?;
/// println!("Loaded {} agents", agents.len());
/// # Ok(())
/// # }
/// ```
pub struct AgentLoader {
    /// LRU 缓存，缓存已加载的 Agent
    cache: LruCache<String, Agent>,

    /// 搜索路径列表，按优先级排序
    search_paths: Vec<AgentSource>,

    /// 当前工作目录
    cwd: PathBuf,
}

/// Agent 来源
///
/// 标识 Agent 来自哪个层级。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentSource {
    /// 内置 Agent
    Builtin,
    /// 用户 Claude 目录 (~/.claude/agents/)
    UserClaude,
    /// 用户 Kode 目录 (~/.kode/agents/)
    UserKode,
    /// 项目 Claude 目录 (./.claude/agents/)
    ProjectClaude,
    /// 项目 Kode 目录 (./.kode/agents/)
    ProjectKode,
}

impl AgentSource {
    /// 返回此源的目录路径
    fn path(&self, cwd: &Path) -> Option<PathBuf> {
        match self {
            AgentSource::Builtin => None,
            AgentSource::UserClaude => dirs::home_dir().map(|h| h.join(".claude/agents")),
            AgentSource::UserKode => dirs::home_dir().map(|h| h.join(".kode/agents")),
            AgentSource::ProjectClaude => Some(cwd.join(".claude/agents")),
            AgentSource::ProjectKode => Some(cwd.join(".kode/agents")),
        }
    }

    /// 返回源的名称（用于日志和调试）
    #[allow(dead_code)]
    fn name(&self) -> &'static str {
        match self {
            AgentSource::Builtin => "builtin",
            AgentSource::UserClaude => "user/.claude",
            AgentSource::UserKode => "user/.kode",
            AgentSource::ProjectClaude => "project/.claude",
            AgentSource::ProjectKode => "project/.kode",
        }
    }

    /// 转换为 AgentLocation
    fn location(&self) -> AgentLocation {
        match self {
            AgentSource::Builtin => AgentLocation::Builtin,
            AgentSource::UserClaude | AgentSource::UserKode => AgentLocation::User,
            AgentSource::ProjectClaude | AgentSource::ProjectKode => AgentLocation::Project,
        }
    }
}

impl AgentLoader {
    /// 创建新的 Agent 加载器
    ///
    /// 初始化搜索路径和缓存。
    ///
    /// # Errors
    ///
    /// 如果无法获取当前工作目录，返回错误。
    pub fn new() -> Result<Self> {
        let cwd = std::env::current_dir()
            .map_err(|e| Error::ConfigError(format!("Failed to get current directory: {}", e)))?;

        let search_paths = vec![
            AgentSource::Builtin,
            AgentSource::UserClaude,
            AgentSource::UserKode,
            AgentSource::ProjectClaude,
            AgentSource::ProjectKode,
        ];

        Ok(Self {
            cache: LruCache::new(std::num::NonZeroUsize::new(100).unwrap()),
            search_paths,
            cwd,
        })
    }

    /// 加载指定名称的 Agent
    ///
    /// 首先检查缓存，如果缓存未命中则按优先级搜索各个目录。
    ///
    /// # Arguments
    ///
    /// * `name` - Agent 名称
    ///
    /// # Returns
    ///
    /// 返回找到的 Agent，如果所有路径都找不到则返回错误。
    ///
    /// # Errors
    ///
    /// - 如果 Agent 不存在，返回 `Error::AgentNotFound`
    /// - 如果解析失败，返回 `Error::AgentParseError`
    pub async fn load_agent(&mut self, name: &str) -> Result<Agent> {
        // 检查缓存
        if let Some(agent) = self.cache.get(name) {
            return Ok(agent.clone());
        }

        // 按优先级搜索
        for source in &self.search_paths {
            if let Some(agent) = self.try_load_from_source(source, name).await? {
                self.cache.put(name.to_string(), agent.clone());
                return Ok(agent);
            }
        }

        Err(Error::AgentNotFound(name.to_string()))
    }

    /// 从指定源加载 Agent
    async fn try_load_from_source(
        &self,
        source: &AgentSource,
        name: &str,
    ) -> Result<Option<Agent>> {
        match source {
            AgentSource::Builtin => Ok(self.load_builtin_agent(name)),
            _ => {
                let base = source.path(&self.cwd).unwrap();
                self.load_agent_from_file(&base, name, source).await
            }
        }
    }

    /// 加载内置 Agent
    fn load_builtin_agent(&self, name: &str) -> Option<Agent> {
        match name {
            "general-purpose" => Some(Agent {
                name: "general-purpose".to_string(),
                description: "General-purpose agent for researching complex questions, searching for code, and executing multi-step tasks".to_string(),
                tools: crate::agent::types::ToolFilter::All,
                model: None,
                system_prompt: r#"You are a general-purpose agent. Given the user's task, use the tools available to complete it efficiently and thoroughly.

When to use your capabilities:
- Searching for code, configurations, and patterns across large codebases
- Analyzing multiple files to understand system architecture
- Investigating complex questions that require exploring many files
- Performing multi-step research tasks

Guidelines:
- For file searches: Use Grep or Glob when you need to search broadly. Use FileRead when you know the specific file path.
- For analysis: Start broad and narrow down. Use multiple search strategies if the first doesn't yield results.
- Be thorough: Check multiple locations, consider different naming conventions, look for related files.
- Complete tasks directly using your capabilities."#.to_string(),
                location: AgentLocation::Builtin,
                color: None,
            }),
            _ => None,
        }
    }

    /// 从文件系统加载 Agent
    async fn load_agent_from_file(
        &self,
        base: &Path,
        name: &str,
        source: &AgentSource,
    ) -> Result<Option<Agent>> {
        let agent_file = base.join(format!("{}.md", name));

        // 如果文件不存在，返回 None（不是错误）
        if !agent_file.exists() {
            return Ok(None);
        }

        // 读取文件内容
        let content =
            fs::read_to_string(&agent_file)
                .await
                .map_err(|e| Error::ConfigLoadError {
                    path: agent_file.clone(),
                    message: format!("Failed to read agent file: {}", e),
                })?;

        // 解析 Agent
        let mut agent = self
            .parse_agent(&content)
            .map_err(|e| Error::ConfigParseError {
                path: agent_file,
                message: e.to_string(),
            })?;

        // 设置正确的 location
        agent.location = source.location();

        Ok(Some(agent))
    }

    /// 加载所有可用的 Agent
    ///
    /// 按优先级扫描所有目录，后加载的 Agent 会覆盖先加载的（同名）。
    ///
    /// # Returns
    ///
    /// 返回所有唯一的 Agent 列表（按名称去重）。
    pub async fn load_all_agents(&mut self) -> Result<Vec<Agent>> {
        let mut agents_map: HashMap<String, Agent> = HashMap::new();

        // 按优先级加载，后面的会覆盖前面的
        for source in &self.search_paths {
            let agents = self.load_agents_from_source(source).await?;
            for agent in agents {
                agents_map.insert(agent.name.clone(), agent);
            }
        }

        // 清空缓存并重新填充
        self.cache.clear();
        for agent in agents_map.values() {
            self.cache.put(agent.name.clone(), agent.clone());
        }

        Ok(agents_map.into_values().collect())
    }

    /// 从指定源加载所有 Agent
    async fn load_agents_from_source(&self, source: &AgentSource) -> Result<Vec<Agent>> {
        match source {
            AgentSource::Builtin => Ok(self.load_builtin_agents()),
            _ => {
                let base = source.path(&self.cwd).unwrap();
                self.scan_directory(&base, source).await
            }
        }
    }

    /// 加载所有内置 Agent
    fn load_builtin_agents(&self) -> Vec<Agent> {
        vec![self.load_builtin_agent("general-purpose").unwrap()]
    }

    /// 扫描目录中的所有 Agent 文件
    async fn scan_directory(&self, dir: &Path, source: &AgentSource) -> Result<Vec<Agent>> {
        // 如果目录不存在，返回空列表
        if !dir.exists() {
            return Ok(Vec::new());
        }

        let mut agents = Vec::new();
        let mut entries = fs::read_dir(dir)
            .await
            .map_err(|e| Error::ConfigLoadError {
                path: dir.to_path_buf(),
                message: format!("Failed to read directory: {}", e),
            })?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| Error::ConfigLoadError {
                path: dir.to_path_buf(),
                message: format!("Failed to read directory entry: {}", e),
            })?
        {
            let path = entry.path();

            // 只处理 .md 文件
            if path.extension().and_then(|s| s.to_str()) != Some("md") {
                continue;
            }

            // 跳过非文件
            if !path.is_file() {
                continue;
            }

            // 读取并解析文件
            let content = fs::read_to_string(&path)
                .await
                .map_err(|e| Error::ConfigLoadError {
                    path: path.clone(),
                    message: format!("Failed to read agent file: {}", e),
                })?;

            match self.parse_agent(&content) {
                Ok(mut agent) => {
                    agent.location = source.location();
                    agents.push(agent);
                }
                Err(e) => {
                    // 记录警告但不中断，继续处理其他 agent
                    eprintln!(
                        "Warning: Failed to parse agent file {:?}, skipping: {}",
                        path, e
                    );
                }
            }
        }

        Ok(agents)
    }

    /// 解析 Agent 定义
    ///
    /// 支持两种格式：
    /// 1. Markdown + YAML frontmatter（类似 gray-matter）
    /// 2. 纯 YAML（向后兼容）
    ///
    /// # Arguments
    ///
    /// * `content` - 文件内容
    ///
    /// # Returns
    ///
    /// 返回解析后的 Agent。
    ///
    /// # Errors
    ///
    /// - 如果格式无效（缺少分隔符），返回错误
    /// - 如果缺少必需字段，返回错误
    /// - 如果 YAML 解析失败，返回错误
    fn parse_agent(&self, content: &str) -> Result<Agent> {
        // 尝试分离 YAML frontmatter 和 Markdown 正文
        // 支持 --- 分隔符
        let parts: Vec<&str> = content.splitn(3, "---").collect();

        let (yaml_content, markdown_body) = if parts.len() >= 3 && parts[0].is_empty() {
            // 格式: "\n---\nyaml\n---\nmarkdown"
            (parts[1].trim(), parts[2].trim())
        } else if parts.len() == 3 {
            // 格式: "prefix---\nyaml\n---\nmarkdown"
            (parts[1].trim(), parts[2].trim())
        } else if parts.len() == 2 {
            // 可能是 "---\nyaml\n---\n" 格式（没有正文）
            (parts[0].trim(), parts[1].trim())
        } else {
            // 没有 frontmatter，纯 Markdown 或纯 YAML
            // 尝试解析为纯 YAML
            return self.parse_yaml_agent(content);
        };

        // 解析 YAML
        let yaml: Value =
            serde_yaml::from_str(yaml_content).map_err(|e| Error::ConfigParseError {
                path: PathBuf::from("<unknown>"),
                message: format!("YAML parsing error: {}", e),
            })?;

        // 提取必需字段
        let name = yaml
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::ConfigParseError {
                path: PathBuf::from("<unknown>"),
                message: "Missing required field: 'name'".to_string(),
            })?
            .to_string();

        let description = yaml
            .get("description")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::ConfigParseError {
                path: PathBuf::from("<unknown>"),
                message: "Missing required field: 'description'".to_string(),
            })?
            .to_string();

        // 解析 tools 字段
        let tools = self.parse_tools_field(yaml.get("tools"));

        // 解析可选的 model_name 字段
        let model = yaml
            .get("model_name")
            .or_else(|| yaml.get("model")) // 支持已弃用的 'model' 字段
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        // 解析可选的 color 字段
        let color = yaml
            .get("color")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        Ok(Agent {
            name,
            description,
            tools,
            model,
            system_prompt: markdown_body.to_string(),
            location: AgentLocation::Project, // 默认值，调用者会覆盖
            color,
        })
    }

    /// 解析纯 YAML 格式的 Agent（向后兼容）
    fn parse_yaml_agent(&self, content: &str) -> Result<Agent> {
        let yaml: Value = serde_yaml::from_str(content).map_err(|e| Error::ConfigParseError {
            path: PathBuf::from("<unknown>"),
            message: format!("YAML parsing error: {}", e),
        })?;

        let name = yaml
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::ConfigParseError {
                path: PathBuf::from("<unknown>"),
                message: "Missing required field: 'name'".to_string(),
            })?
            .to_string();

        let description = yaml
            .get("description")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::ConfigParseError {
                path: PathBuf::from("<unknown>"),
                message: "Missing required field: 'description'".to_string(),
            })?
            .to_string();

        let tools = self.parse_tools_field(yaml.get("tools"));

        let model = yaml
            .get("model_name")
            .or_else(|| yaml.get("model"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let system_prompt = yaml
            .get("system_prompt")
            .or_else(|| yaml.get("systemPrompt")) // 支持驼峰命名
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::ConfigParseError {
                path: PathBuf::from("<unknown>"),
                message: "Missing required field: 'system_prompt'".to_string(),
            })?
            .to_string();

        // 解析可选的 color 字段
        let color = yaml
            .get("color")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        Ok(Agent {
            name,
            description,
            tools,
            model,
            system_prompt,
            location: AgentLocation::Project, // 默认值，调用者会覆盖
            color,
        })
    }

    /// 解析 tools 字段
    ///
    /// 支持：
    /// - "all" 或 "*" -> ToolFilter::All
    /// - 字符串数组 -> ToolFilter::Specific
    /// - 缺失 -> ToolFilter::All（默认）
    fn parse_tools_field(&self, tools_value: Option<&Value>) -> crate::agent::types::ToolFilter {
        use crate::agent::types::ToolFilter;

        match tools_value {
            None => ToolFilter::All,
            Some(value) => {
                if let Some(s) = value.as_str() {
                    if s == "all" || s == "*" {
                        ToolFilter::All
                    } else {
                        // 单个工具名
                        ToolFilter::Specific(vec![s.to_string()])
                    }
                } else if let Some(arr) = value.as_sequence() {
                    let tools: Vec<String> = arr
                        .iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect();

                    if tools.is_empty() {
                        ToolFilter::All
                    } else {
                        ToolFilter::Specific(tools)
                    }
                } else {
                    ToolFilter::All
                }
            }
        }
    }

    /// 清除缓存
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// 获取指定名称的 Agent
    ///
    /// 对应 TypeScript 版本的 `getAgentByType` 函数。
    ///
    /// # Arguments
    ///
    /// * `name` - Agent 名称
    ///
    /// # Returns
    ///
    /// 返回找到的 Agent，如果不存在则返回 `None`。
    pub async fn get_agent_by_type(&mut self, name: &str) -> Option<Agent> {
        self.load_agent(name).await.ok()
    }

    /// 获取所有可用的 Agent 类型名称
    ///
    /// 对应 TypeScript 版本的 `getAvailableAgentTypes` 函数。
    ///
    /// # Returns
    ///
    /// 返回所有 Agent 的名称列表。
    pub async fn get_available_agent_types(&mut self) -> Result<Vec<String>> {
        let agents = self.load_all_agents().await?;
        Ok(agents.iter().map(|a| a.name.clone()).collect())
    }

    /// 获取所有 Agent（包括被覆盖的）
    ///
    /// 对应 TypeScript 版本的 `getAllAgents` 函数。
    /// 返回所有 Agent 定义，包括被更高优先级覆盖的版本。
    ///
    /// # Returns
    ///
    /// 返回元组：(active_agents, all_agents)
    /// - active_agents: 去重后的 Agent（高优先级覆盖低优先级）
    /// - all_agents: 所有 Agent（包括被覆盖的）
    pub async fn get_all_agents(&mut self) -> Result<(Vec<Agent>, Vec<Agent>)> {
        let mut active_agents_map: HashMap<String, Agent> = HashMap::new();
        let mut all_agents: Vec<Agent> = Vec::new();

        // 按优先级加载
        for source in &self.search_paths {
            let agents = self.load_agents_from_source(source).await?;
            for agent in agents {
                // 记录所有 agents
                all_agents.push(agent.clone());
                // 活跃 agents 只保留每个名称的最新（最高优先级）版本
                active_agents_map.insert(agent.name.clone(), agent);
            }
        }

        // 清空缓存并重新填充
        self.cache.clear();
        for agent in active_agents_map.values() {
            self.cache.put(agent.name.clone(), agent.clone());
        }

        let active_agents = active_agents_map.into_values().collect();
        Ok((active_agents, all_agents))
    }

    /// 获取活跃的 Agent（去重后的）
    ///
    /// 对应 TypeScript 版本的 `getActiveAgents` 函数。
    ///
    /// # Returns
    ///
    /// 返回去重后的 Agent 列表（高优先级覆盖低优先级）。
    pub async fn get_active_agents(&mut self) -> Result<Vec<Agent>> {
        let (active_agents, _) = self.get_all_agents().await?;
        Ok(active_agents)
    }

    /// 获取搜索路径列表
    ///
    /// 返回 (路径, 位置) 的列表，用于文件监控。
    pub fn get_search_paths(&self) -> Vec<(PathBuf, AgentLocation)> {
        let mut paths = Vec::new();
        for source in &self.search_paths {
            if let Some(path) = source.path(&self.cwd) {
                paths.push((path, source.location()));
            }
        }
        paths
    }
}

impl Default for AgentLoader {
    fn default() -> Self {
        Self::new().expect("Failed to create AgentLoader")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::types::ToolFilter;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_parse_agent_with_frontmatter() {
        let loader = AgentLoader::new().unwrap();
        let content = r#"---
name: test-agent
description: "Test agent for testing"
tools:
  - FileRead
  - Grep
model_name: claude-sonnet
---

You are a test agent.
Your job is to test things."#;

        let agent = loader.parse_agent(content).unwrap();
        assert_eq!(agent.name, "test-agent");
        assert_eq!(agent.description, "Test agent for testing");
        assert!(matches!(agent.tools, ToolFilter::Specific(_)));
        assert_eq!(agent.model, Some("claude-sonnet".to_string()));
        assert!(agent.system_prompt.contains("test agent"));
    }

    #[tokio::test]
    async fn test_parse_agent_all_tools() {
        let loader = AgentLoader::new().unwrap();
        let content = r#"---
name: all-tools
description: "Agent with all tools"
tools: all
---

System prompt"#;

        let agent = loader.parse_agent(content).unwrap();
        assert!(matches!(agent.tools, ToolFilter::All));
    }

    #[tokio::test]
    async fn test_parse_agent_star_tools() {
        let loader = AgentLoader::new().unwrap();
        let content = r#"---
name: star-tools
description: "Agent with star tools"
tools: "*"
---

System prompt"#;

        let agent = loader.parse_agent(content).unwrap();
        assert!(matches!(agent.tools, ToolFilter::All));
    }

    #[tokio::test]
    async fn test_parse_agent_missing_fields() {
        let loader = AgentLoader::new().unwrap();
        let content = r#"---
name: incomplete
---

System prompt"#;

        let result = loader.parse_agent(content);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_load_builtin_agent() {
        let mut loader = AgentLoader::new().unwrap();
        let agent = loader.load_agent("general-purpose").await.unwrap();
        assert_eq!(agent.name, "general-purpose");
        assert!(matches!(agent.tools, ToolFilter::All));
    }

    #[tokio::test]
    async fn test_load_nonexistent_agent() {
        let mut loader = AgentLoader::new().unwrap();
        let result = loader.load_agent("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_load_agent_from_file() {
        let temp_dir = TempDir::new().unwrap();
        let agent_file = temp_dir.path().join("test-agent.md");

        let content = r#"---
name: test-agent
description: "Test agent"
tools: all
---

Test prompt"#;

        fs::write(&agent_file, content).await.unwrap();

        let loader = AgentLoader::new().unwrap();
        // 直接调用 scan_directory 测试，使用 UserKode 作为源
        let agents = loader
            .scan_directory(temp_dir.path(), &AgentSource::UserKode)
            .await
            .unwrap();
        assert!(!agents.is_empty());
        assert_eq!(agents[0].name, "test-agent");
        assert_eq!(agents[0].location, AgentLocation::User);
    }

    #[tokio::test]
    async fn test_cache() {
        let mut loader = AgentLoader::new().unwrap();

        // 第一次加载
        let agent1 = loader.load_agent("general-purpose").await.unwrap();

        // 检查缓存
        assert!(loader.cache.contains("general-purpose"));

        // 第二次应该从缓存加载
        let agent2 = loader.load_agent("general-purpose").await.unwrap();

        assert_eq!(agent1.name, agent2.name);
    }

    #[tokio::test]
    async fn test_clear_cache() {
        let mut loader = AgentLoader::new().unwrap();

        loader.load_agent("general-purpose").await.unwrap();
        assert!(loader.cache.contains("general-purpose"));

        loader.clear_cache();
        assert!(!loader.cache.contains("general-purpose"));
    }

    #[tokio::test]
    async fn test_load_all_agents() {
        let mut loader = AgentLoader::new().unwrap();
        let agents = loader.load_all_agents().await.unwrap();

        // 至少应该有内置的 general-purpose
        assert!(!agents.is_empty());
        assert!(agents.iter().any(|a| a.name == "general-purpose"));
    }

    #[tokio::test]
    async fn test_scan_directory_not_exist() {
        let loader = AgentLoader::new().unwrap();
        let nonexist = PathBuf::from("/tmp/this_should_not_exist_12345");

        let agents = loader
            .scan_directory(&nonexist, &AgentSource::UserKode)
            .await
            .unwrap();
        assert!(agents.is_empty());
    }

    #[tokio::test]
    async fn test_get_agent_by_type() {
        let mut loader = AgentLoader::new().unwrap();
        let agent = loader.get_agent_by_type("general-purpose").await;
        assert!(agent.is_some());
        assert_eq!(agent.unwrap().name, "general-purpose");

        // 测试不存在的 agent
        let nonexistent = loader.get_agent_by_type("nonexistent").await;
        assert!(nonexistent.is_none());
    }

    #[tokio::test]
    async fn test_get_available_agent_types() {
        let mut loader = AgentLoader::new().unwrap();
        let types = loader.get_available_agent_types().await.unwrap();
        assert!(!types.is_empty());
        assert!(types.contains(&"general-purpose".to_string()));
    }

    #[tokio::test]
    async fn test_get_all_agents() {
        let mut loader = AgentLoader::new().unwrap();
        let (active_agents, all_agents) = loader.get_all_agents().await.unwrap();

        // active_agents 应该去重
        assert!(!active_agents.is_empty());

        // all_agents 包含所有（可能重复）
        assert!(!all_agents.is_empty());

        // 至少有内置的 general-purpose
        assert!(active_agents.iter().any(|a| a.name == "general-purpose"));
        assert!(all_agents.iter().any(|a| a.name == "general-purpose"));
    }

    #[tokio::test]
    async fn test_get_active_agents() {
        let mut loader = AgentLoader::new().unwrap();
        let agents = loader.get_active_agents().await.unwrap();
        assert!(!agents.is_empty());
        assert!(agents.iter().any(|a| a.name == "general-purpose"));
    }
}
