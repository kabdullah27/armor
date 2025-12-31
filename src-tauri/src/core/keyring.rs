// Keyring operations for managing PGP keys using SQLite
use crate::models::KeyMetadata;
use anyhow::Result;
use rusqlite::params;

pub fn save_key_metadata(
    vault: &crate::core::storage::Vault,
    metadata: &KeyMetadata,
) -> Result<()> {
    let conn = vault.conn.lock().unwrap();
    let json = serde_json::to_string(metadata)?;

    // We update the metadata column.
    // Note: We use UPSERT syntax (INSERT OR REPLACE) or ON CONFLICT
    conn.execute(
        "INSERT INTO keys (
            fingerprint, user_name, user_email, key_type, created_at, expires_at, is_private, is_favorite, metadata_json
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
        ON CONFLICT(fingerprint) DO UPDATE SET
            user_name=excluded.user_name,
            user_email=excluded.user_email,
            key_type=excluded.key_type,
            created_at=excluded.created_at,
            expires_at=excluded.expires_at,
            is_private=excluded.is_private,
            is_favorite=excluded.is_favorite,
            metadata_json=excluded.metadata_json",
        params![
            metadata.fingerprint,
            metadata.user_id.name,
            metadata.user_id.email,
            metadata.key_type.to_string(), // Assuming KeyType implements Display
            metadata.created_at,
            metadata.expires_at,
            metadata.is_private,
            metadata.is_favorite,
            json
        ],
    )?;
    Ok(())
}

pub fn load_key_metadata(
    vault: &crate::core::storage::Vault,
    fingerprint: &str,
) -> Result<KeyMetadata> {
    let conn = vault.conn.lock().unwrap();
    let mut stmt = conn.prepare("SELECT metadata_json FROM keys WHERE fingerprint = ?1")?;

    let json: String = stmt.query_row(params![fingerprint], |row| row.get(0))?;

    let metadata = serde_json::from_str(&json)?;
    Ok(metadata)
}

pub fn list_all_keys(vault: &crate::core::storage::Vault) -> Result<Vec<KeyMetadata>> {
    let conn = vault.conn.lock().unwrap();
    let mut stmt = conn.prepare("SELECT metadata_json FROM keys")?;

    let rows = stmt.query_map([], |row| {
        let json: String = row.get(0)?;
        Ok(json)
    })?;

    let mut keys = Vec::new();
    for json_result in rows {
        if let Ok(json) = json_result {
            if let Ok(metadata) = serde_json::from_str(&json) {
                keys.push(metadata);
            }
        }
    }

    Ok(keys)
}

pub fn delete_key_files(vault: &crate::core::storage::Vault, fingerprint: &str) -> Result<bool> {
    log::info!("delete_key_files called for fingerprint: {}", fingerprint);
    let conn = vault.conn.lock().unwrap();
    // This removes the row, effectively deleting key content and metadata together
    let count = conn.execute(
        "DELETE FROM keys WHERE fingerprint = ?1",
        params![fingerprint],
    )?;
    log::info!("Deleted {} rows for fingerprint: {}", count, fingerprint);
    Ok(count > 0)
}

pub fn save_key_to_file(
    vault: &crate::core::storage::Vault,
    fingerprint: &str,
    content: &str,
) -> Result<()> {
    let conn = vault.conn.lock().unwrap();
    // Update the key_content column for the existing row (or insert if not exists, though metadata usually comes first)
    // We'll use UPSERT but just update content if key exists, or create a partial row if not
    conn.execute(
        "INSERT INTO keys (fingerprint, key_content) VALUES (?1, ?2)
        ON CONFLICT(fingerprint) DO UPDATE SET key_content=excluded.key_content",
        params![fingerprint, content],
    )?;
    Ok(())
}
