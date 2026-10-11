use anyhow::{bail, Context, Result};
use directories::ProjectDirs;
use rusqlite::{Connection, DatabaseName, OpenFlags};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

pub struct Vault {
    pub db_path: PathBuf,
    pub conn: Mutex<Connection>,
}

/// Default database location (`<data_dir>/armor.db`), used when no custom path is configured.
pub fn default_db_path() -> Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("com", "armor", "desktop")
        .context("Could not determine project directories")?;
    Ok(proj_dirs.data_dir().join("armor.db"))
}

fn ensure_parent_dir(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
    }
    Ok(())
}

fn init_schema(conn: &Connection) -> Result<()> {
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
    Ok(())
}

impl Vault {
    pub fn new() -> Result<Self> {
        let config = super::config::load_config().map_err(|e| {
            log::error!("Vault failed to load config: {}", e);
            e
        })?;
        log::info!("Vault loaded config: {:?}", config);

        let db_path = match config.db_path {
            Some(path_str) => PathBuf::from(path_str),
            None => default_db_path()?,
        };
        ensure_parent_dir(&db_path)?;

        let conn = Connection::open(&db_path)?;
        init_schema(&conn)?;

        Ok(Self {
            db_path,
            conn: Mutex::new(conn),
        })
    }

    /// Writes a consistent snapshot of the live database to `target` using the
    /// SQLite online backup API (safe while the connection is open).
    pub fn backup_to(&self, target: &Path) -> Result<()> {
        ensure_parent_dir(target)?;
        let conn = self.conn.lock().unwrap();
        conn.backup(DatabaseName::Main, target, None)?;
        Ok(())
    }

    /// Replaces the live database contents with `source`, after verifying that
    /// it is an ARMOR vault. Takes effect immediately, no restart needed.
    pub fn restore_from(&self, source: &Path) -> Result<()> {
        if source == self.db_path {
            bail!("Backup file is the active database");
        }
        let src = Connection::open_with_flags(source, OpenFlags::SQLITE_OPEN_READ_ONLY)
            .context("Could not open backup file")?;
        let has_keys_table: bool = src
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='keys')",
                [],
                |row| row.get(0),
            )
            .context("File is not a valid SQLite database")?;
        if !has_keys_table {
            bail!("File is not an ARMOR database backup");
        }
        drop(src);

        let mut conn = self.conn.lock().unwrap();
        conn.restore(DatabaseName::Main, source, None::<fn(rusqlite::backup::Progress)>)?;
        init_schema(&conn)?;
        Ok(())
    }
}
