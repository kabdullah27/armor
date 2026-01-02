use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub db_path: Option<String>,
    #[serde(default = "default_first_run")]
    pub first_run: bool,
}

fn default_first_run() -> bool {
    true
}

impl AppConfig {
    pub fn default() -> Self {
        Self {
            db_path: None,
            first_run: true,
        }
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

    let content = fs::read_to_string(&config_path)?;
    let config: AppConfig = serde_json::from_str(&content).map_err(|e| {
        log::error!("Failed to parse config file at {:?}: {}", config_path, e);
        anyhow::anyhow!("Failed to parse config file: {}", e)
    })?;
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
