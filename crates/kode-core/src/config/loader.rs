//! 配置加载器
//!
//! 提供配置文件的加载、保存和管理功能。使用单文件架构，所有配置存储在 ~/.kode.json 中。

use crate::config::types::{GlobalConfig, ProjectConfig};
use crate::error::Error;
use serde_json;
use std::collections::HashMap;
use std::path::Path;
use tokio::fs;

/// 配置加载器
#[derive(Default)]
pub struct ConfigLoader;

impl ConfigLoader {
    /// 创建新的配置加载器
    pub fn new() -> Self {
        ConfigLoader
    }

    /// 异步加载全局配置
    ///
    /// 从 ~/.kode.json 加载全局配置。如果文件不存在，返回默认配置。
    ///
    /// # Returns
    ///
    /// 返回加载的全局配置或错误
    pub async fn load_global(&self) -> Result<GlobalConfig, Error> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| Error::ConfigError("Cannot find home directory".to_string()))?;
        let config_path = home_dir.join(".kode.json");
        self.load(&config_path).await
    }

    /// 从指定路径加载配置
    ///
    /// 尝试从指定路径加载配置。如果文件不存在，返回默认配置。
    ///
    /// # Arguments
    ///
    /// * `path` - 配置文件路径
    ///
    /// # Returns
    ///
    /// 返回加载的配置或错误
    pub async fn load(&self, path: &Path) -> Result<GlobalConfig, Error> {
        match fs::read_to_string(path).await {
            Ok(content) => {
                let config: GlobalConfig =
                    serde_json::from_str(&content).map_err(|e| Error::ConfigParseError {
                        path: path.to_path_buf(),
                        message: e.to_string(),
                    })?;
                Ok(config)
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    Ok(GlobalConfig::default())
                } else {
                    Err(Error::ConfigLoadError {
                        path: path.to_path_buf(),
                        message: e.to_string(),
                    })
                }
            }
        }
    }

    /// 异步保存全局配置
    ///
    /// 将全局配置保存到 ~/.kode.json
    ///
    /// # Arguments
    ///
    /// * `config` - 要保存的全局配置
    ///
    /// # Returns
    ///
    /// 返回操作结果
    pub async fn save_global(&self, config: &GlobalConfig) -> Result<(), Error> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| Error::ConfigError("Cannot find home directory".to_string()))?;
        let config_path = home_dir.join(".kode.json");
        self.save(config, &config_path).await
    }

    /// 异步保存配置到指定路径
    ///
    /// 将配置保存到指定路径
    ///
    /// # Arguments
    ///
    /// * `config` - 要保存的配置
    /// * `path` - 目标文件路径
    ///
    /// # Returns
    ///
    /// 返回操作结果
    pub async fn save(&self, config: &GlobalConfig, path: &Path) -> Result<(), Error> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| Error::ConfigSaveError {
                    path: path.to_path_buf(),
                    message: e.to_string(),
                })?;
        }

        let content = serde_json::to_string_pretty(config).map_err(|e| Error::ConfigSaveError {
            path: path.to_path_buf(),
            message: e.to_string(),
        })?;

        fs::write(path, content)
            .await
            .map_err(|e| Error::ConfigSaveError {
                path: path.to_path_buf(),
                message: e.to_string(),
            })?;

        Ok(())
    }

    /// 获取当前项目配置
    ///
    /// 从全局配置的 projects 字段中查找当前工作目录的项目配置。
    /// 如果未找到，返回默认项目配置。
    ///
    /// # Returns
    ///
    /// 返回当前项目配置
    pub async fn get_current_project_config(&self) -> Result<ProjectConfig, Error> {
        let global_config = self.load_global().await?;
        let current_dir = std::env::current_dir()
            .map_err(|e| Error::ConfigError(e.to_string()))?
            .canonicalize()
            .map_err(|e| Error::ConfigError(e.to_string()))?;

        let project_path = current_dir.to_string_lossy().to_string();

        if let Some(projects) = &global_config.projects {
            if let Some(project_config) = projects.get(&project_path) {
                return Ok(project_config.clone());
            }
        }

        // 返回默认项目配置
        Ok(ProjectConfig::default())
    }

    /// 保存当前项目配置
    ///
    /// 将项目配置保存到全局配置的 projects 字段中，使用当前目录的绝对路径作为 key。
    ///
    /// # Arguments
    ///
    /// * `project_config` - 要保存的项目配置
    ///
    /// # Returns
    ///
    /// 返回操作结果
    pub async fn save_current_project_config(
        &self,
        project_config: ProjectConfig,
    ) -> Result<(), Error> {
        let mut global_config = self.load_global().await?;

        let current_dir = std::env::current_dir()
            .map_err(|e| Error::ConfigError(e.to_string()))?
            .canonicalize()
            .map_err(|e| Error::ConfigError(e.to_string()))?;

        let project_path = current_dir.to_string_lossy().to_string();

        if global_config.projects.is_none() {
            global_config.projects = Some(HashMap::new());
        }

        if let Some(ref mut projects) = global_config.projects {
            projects.insert(project_path, project_config);
        }

        self.save_global(&global_config).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_load_config_from_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let config_content = r#"{
            "theme": "dark",
            "verbose": true
        }"#;
        temp_file.write_all(config_content.as_bytes()).unwrap();

        let loader = ConfigLoader::new();
        let config = loader.load(temp_file.path()).await.unwrap();

        assert_eq!(config.theme, Some("dark".to_string()));
        assert_eq!(config.verbose, true);
    }

    #[tokio::test]
    async fn test_load_nonexistent_config() {
        let loader = ConfigLoader::new();
        let config = loader
            .load(Path::new("/nonexistent/path/config.json"))
            .await
            .unwrap();

        // 应该返回默认配置
        assert_eq!(config.verbose, false);
        assert_eq!(config.num_startups, 0);
    }

    #[tokio::test]
    async fn test_save_config() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path().join("config.json");

        let mut config = GlobalConfig::default();
        config.theme = Some("light".to_string());

        let loader = ConfigLoader::new();
        loader.save(&config, &config_path).await.unwrap();

        let loaded_config = loader.load(&config_path).await.unwrap();
        assert_eq!(loaded_config.theme, Some("light".to_string()));
    }

    #[tokio::test]
    async fn test_project_config_in_global() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path().join("config.json");

        let mut global_config = GlobalConfig::default();
        global_config.projects = Some(HashMap::new());

        let project_config = ProjectConfig {
            allowed_tools: vec!["test_tool".to_string()],
            ..Default::default()
        };

        let project_path = "/test/project/path";
        global_config
            .projects
            .as_mut()
            .unwrap()
            .insert(project_path.to_string(), project_config);

        let loader = ConfigLoader::new();
        loader.save(&global_config, &config_path).await.unwrap();

        let loaded_config = loader.load(&config_path).await.unwrap();
        assert!(loaded_config.projects.is_some());
        let projects = loaded_config.projects.as_ref().unwrap();
        assert!(projects.contains_key(project_path));
        assert_eq!(
            projects.get(project_path).unwrap().allowed_tools,
            vec!["test_tool".to_string()]
        );
    }
}
