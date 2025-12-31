use anyhow::{Context, Result};
use directories::ProjectDirs;
use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::Mutex;

pub struct Vault {
    pub db_path: PathBuf,
    pub conn: Mutex<Connection>,
}

impl Vault {
    pub fn new() -> Result<Self> {
        let config = super::config::load_config().map_err(|e| {
            log::error!("Vault failed to load config: {}", e);
            e
        })?;
        log::info!("Vault loaded config: {:?}", config);

        let db_path = if let Some(path_str) = config.db_path {
            let path = PathBuf::from(path_str);
            if let Some(parent) = path.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(parent)?;
                }
            }
            path
        } else {
            let proj_dirs = ProjectDirs::from("com", "armor", "desktop")
                .context("Could not determine project directories")?;
            let data_dir = proj_dirs.data_dir();
            if !data_dir.exists() {
                std::fs::create_dir_all(data_dir)?;
            }
            data_dir.join("armor.db")
        };

        // Open connection
        let conn = Connection::open(&db_path)?;

        // Initialize Schema
        conn.execute(
            "CREATE TABLE IF NOT EXISTS keys (
                fingerprint TEXT PRIMARY KEY,
                user_name TEXT,
                user_email TEXT,
                key_type TEXT,
                created_at TEXT,
                expires_at TEXT,
                is_private BOOLEAN,
                is_favorite BOOLEAN,
                key_content TEXT,
                metadata_json TEXT
            )",
            [],
        )?;

        Ok(Self {
            db_path,
            conn: Mutex::new(conn),
        })
    }
}
