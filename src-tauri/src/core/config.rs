use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub db_path: Option<String>,
}

impl AppConfig {
    pub fn default() -> Self {
        Self { db_path: None }
    }
}

pub fn get_config_dir() -> Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("com", "armor", "desktop")
        .context("Could not determine project directories")?;
    Ok(proj_dirs.config_dir().to_path_buf())
}

pub fn load_config() -> Result<AppConfig> {
    let config_dir = get_config_dir()?;
    let config_path = config_dir.join("armor-config.json");

    if !config_path.exists() {
        return Ok(AppConfig::default());
    }

    let content = fs::read_to_string(config_path)?;
    let config: AppConfig = serde_json::from_str(&content).unwrap_or(AppConfig::default());
    Ok(config)
}

pub fn save_config(config: &AppConfig) -> Result<()> {
    let config_dir = get_config_dir()?;
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }

    let config_path = config_dir.join("armor-config.json");
    let content = serde_json::to_string_pretty(config)?;
    fs::write(config_path, content)?;
    Ok(())
}
