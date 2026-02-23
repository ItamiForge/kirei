use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use tokio::fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub scan_paths: Vec<PathBuf>,
    pub ignore_patterns: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        let home = directories::UserDirs::new().unwrap().home_dir().to_path_buf();
        Self {
            scan_paths: vec![
                home.join("Downloads"),
                home.join("Desktop"),
                home.join("Documents"),
            ],
            ignore_patterns: vec![
                "*.git".to_string(),
                "node_modules".to_string(),
                "target".to_string(),
            ],
        }
    }
}

pub async fn load_config() -> Config {
    let config_dir = directories::ProjectDirs::from("com", "kirei", "kirei")
        .map(|p| p.config_dir().to_path_buf());
    
    if let Some(dir) = config_dir {
        let path = dir.join("config.json");
        if let Ok(content) = fs::read_to_string(path).await {
            if let Ok(config) = serde_json::from_str(&content) {
                return config;
            }
        }
    }
    Config::default()
}

pub async fn save_config(config: &Config) -> anyhow::Result<()> {
    let config_dir = directories::ProjectDirs::from("com", "kirei", "kirei")
        .map(|p| p.config_dir().to_path_buf());
    
    if let Some(dir) = config_dir {
        fs::create_dir_all(&dir).await?;
        let path = dir.join("config.json");
        let content = serde_json::to_string_pretty(config)?;
        fs::write(path, content).await?;
    }
    Ok(())
}
