//! Agent 系统模块
//!
//! 提供 Agent 定义、加载和管理功能。
//!
//! # 模块结构
//!
//! - [`types`](types): Agent 类型定义（Agent、ToolFilter）
//! - [`loader`](loader): Agent 加载器（五层优先级、LRU 缓存、高容错）
//! - [`storage`](storage): Agent 存储工具（基于文件的状态隔离）
//!
//! # 示例
//!
//! ```no_run
//! use kode_core::agent::loader::AgentLoader;
//!
//! # async fn example() -> kode_core::error::Result<()> {
//! let mut loader = AgentLoader::new()?;
//! let agents = loader.load_all_agents().await?;
//!
//! for agent in agents {
//!     println!("{}: {}", agent.name, agent.description);
//! }
//! # Ok(())
//! # }
//! ```

pub mod loader;
pub mod storage;
pub mod types;

// 重新导出常用类型
pub use loader::{AgentLoader, AgentSource};
pub use types::{Agent, AgentLocation, ToolFilter};
