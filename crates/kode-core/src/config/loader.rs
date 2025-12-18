//! 配置加载器

use super::Config;
use crate::Result;

/// 配置加载器
pub struct ConfigLoader;

impl ConfigLoader {
    /// 加载配置
    pub async fn load() -> Result<Config> {
        // TODO: 实现配置加载
        Ok(Config::default())
    }
}
