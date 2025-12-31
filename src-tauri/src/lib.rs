mod commands;
mod core;
mod models;

use core::storage::Vault;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let vault = Vault::new().expect("Failed to initialize vault");

    tauri::Builder::default()
        .manage(vault)
        .setup(|app| {
            app.handle().plugin(
                tauri_plugin_log::Builder::default()
                    .targets([
                        tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout),
                        tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir {
                            file_name: Some("armor.log".into()),
                        }),
                        tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Webview),
                    ])
                    .level(log::LevelFilter::Info)
                    .build(),
            )?;
            app.handle().plugin(tauri_plugin_dialog::init())?;
            app.handle().plugin(tauri_plugin_shell::init())?;
            app.handle().plugin(tauri_plugin_fs::init())?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::keys::list_keys,
            commands::keys::delete_key,
            commands::keys::generate_key,
            commands::keys::import_key,
            commands::keys::export_key,
            commands::crypto::encrypt_file_cmd,
            commands::crypto::decrypt_file_cmd,
            commands::settings::get_db_path,
            commands::settings::set_db_path,
            commands::settings::backup_db,
            commands::settings::restore_db,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
