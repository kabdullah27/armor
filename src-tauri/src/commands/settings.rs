use crate::core::{config, storage::{self, Vault}};
use crate::models::OperationResult;
use std::path::PathBuf;
use tauri::State;

#[tauri::command]
pub async fn get_db_path(vault: State<'_, Vault>) -> Result<OperationResult<String>, String> {
    // Return the path currently being used by the Vault
    let path_str = vault.db_path.to_string_lossy().to_string();
    Ok(OperationResult::ok(path_str))
}

#[tauri::command]
pub async fn is_first_run() -> Result<OperationResult<bool>, String> {
    let config = config::load_config().map_err(|e| e.to_string())?;
    Ok(OperationResult::ok(config.first_run))
}

#[tauri::command]
pub async fn complete_onboarding() -> Result<OperationResult<bool>, String> {
    let mut config = config::load_config().map_err(|e| e.to_string())?;
    config.first_run = false;
    config::save_config(&config).map_err(|e| e.to_string())?;
    Ok(OperationResult::ok(true))
}

/// Points the app at a new database location (takes effect after restart).
/// If the target doesn't exist yet, a consistent copy of the live vault is written there.
/// Returns the resolved database file path.
#[tauri::command]
pub async fn set_db_path(path: String, vault: State<'_, Vault>) -> Result<OperationResult<String>, String> {
    log::info!("SET_DB_PATH command called with path: {}", path);
    let mut config = match config::load_config() {
        Ok(c) => c,
        Err(e) => return Ok(OperationResult::err(format!("Failed to load config: {}", e))),
    };

    let mut new_path = PathBuf::from(&path);
    if new_path.is_dir() {
        new_path = new_path.join("armor.db");
    }

    if new_path != vault.db_path && !new_path.exists() {
        if let Err(e) = vault.backup_to(&new_path) {
            return Ok(OperationResult::err(format!("Failed to migrate database to new location: {}", e)));
        }
    }

    // Store None when pointing back at the default location
    let path_str = new_path.to_string_lossy().to_string();
    config.db_path = match storage::default_db_path() {
        Ok(default) if default == new_path => None,
        _ => Some(path_str.clone()),
    };

    if let Err(e) = config::save_config(&config) {
        return Ok(OperationResult::err(format!("Failed to save config: {}", e)));
    }

    Ok(OperationResult::ok(path_str))
}

#[tauri::command]
pub async fn backup_db(target_path: String, vault: State<'_, Vault>) -> Result<OperationResult<bool>, String> {
    match vault.backup_to(&PathBuf::from(target_path)) {
        Ok(_) => Ok(OperationResult::ok(true)),
        Err(e) => Ok(OperationResult::err(format!("Failed to backup database: {}", e))),
    }
}

#[tauri::command]
pub async fn restore_db(source_path: String, vault: State<'_, Vault>) -> Result<OperationResult<bool>, String> {
    match vault.restore_from(&PathBuf::from(source_path)) {
        Ok(_) => Ok(OperationResult::ok(true)),
        Err(e) => Ok(OperationResult::err(format!("Failed to restore database: {}", e))),
    }
}
