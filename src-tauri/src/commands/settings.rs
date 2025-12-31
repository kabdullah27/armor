use tauri::State;
use crate::core::{config, storage::Vault};
use crate::models::OperationResult;
use std::path::PathBuf;

#[tauri::command]
pub async fn get_db_path(vault: State<'_, Vault>) -> Result<OperationResult<String>, String> {
    // Return the path currently being used by the Vault
    let path_str = vault.db_path.to_string_lossy().to_string();
    Ok(OperationResult::ok(path_str))
}

#[tauri::command]
pub async fn set_db_path(path: String) -> Result<OperationResult<bool>, String> {
    log::info!("SET_DB_PATH command called with path: {}", path);
    let mut config = match config::load_config() {
        Ok(c) => c,
        Err(e) => return Ok(OperationResult::err(format!("Failed to load config: {}", e))),
    };

    // Determine current DB path to copy from
    let current_db_path = if let Some(ref p) = config.db_path {
        std::path::PathBuf::from(p)
    } else {
        use directories::ProjectDirs;
        let proj_dirs = ProjectDirs::from("com", "armor", "desktop")
            .ok_or("Could not determine project directories")?;
        proj_dirs.data_dir().join("armor.db")
    };

    let new_path_buf = PathBuf::from(&path);
    if let Some(parent) = new_path_buf.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    // specific fix: If current DB exists and new one doesn't, copy it.
    if current_db_path.exists() && !new_path_buf.exists() {
        std::fs::copy(&current_db_path, &new_path_buf)
            .map_err(|e| format!("Failed to migrate database to new location: {}", e))?;
    }

    config.db_path = Some(path);

    crate::core::config::save_config(&config).map_err(|e| e.to_string())?;

    Ok(OperationResult::ok(true))
}

#[tauri::command]
pub async fn backup_db(
    target_path: String,
) -> Result<OperationResult<bool>, String> {
    let config = crate::core::config::load_config().map_err(|e| e.to_string())?;
    
    // Determine current DB path
    let current_db_path = if let Some(path_str) = config.db_path {
        std::path::PathBuf::from(path_str)
    } else {
        use directories::ProjectDirs;
        let proj_dirs = ProjectDirs::from("com", "armor", "desktop")
            .ok_or("Could not determine project directories")?;
        proj_dirs.data_dir().join("armor.db")
    };

    if !current_db_path.exists() {
        return Ok(OperationResult::err("Current database file does not exist, cannot backup.".to_string()));
    }

    match std::fs::copy(&current_db_path, &target_path) {
        Ok(_) => Ok(OperationResult::ok(true)),
        Err(e) => Ok(OperationResult::err(format!("Failed to backup database: {}", e))),
    }
}

#[tauri::command]
pub async fn restore_db(
    source_path: String,
) -> Result<OperationResult<bool>, String> {
    let config = crate::core::config::load_config().map_err(|e| e.to_string())?;
    
    // Determine current DB path to overwrite
    let current_db_path = if let Some(path_str) = config.db_path {
        std::path::PathBuf::from(path_str)
    } else {
        use directories::ProjectDirs;
        let proj_dirs = ProjectDirs::from("com", "armor", "desktop")
            .ok_or("Could not determine project directories")?;
        let data_dir = proj_dirs.data_dir();
        if !data_dir.exists() {
             std::fs::create_dir_all(data_dir).map_err(|e| e.to_string())?;
        }
        data_dir.join("armor.db")
    };

    // Copy source to current location (overwrite)
    match std::fs::copy(&source_path, &current_db_path) {
        Ok(_) => Ok(OperationResult::ok(true)),
        Err(e) => Ok(OperationResult::err(format!("Failed to restore database: {}", e))),
    }
}
