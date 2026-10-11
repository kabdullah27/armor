// Keyring operations for managing PGP keys using SQLite
use crate::core::storage::Vault;
use crate::models::KeyMetadata;
use anyhow::Result;
use rusqlite::{params, OptionalExtension};

/// Inserts or updates a key's content and metadata in a single statement so a
/// row can never end up with content but no metadata (or vice versa).
pub fn save_key(vault: &Vault, metadata: &KeyMetadata, content: &str) -> Result<()> {
    let conn = vault.conn.lock().unwrap();
    let json = serde_json::to_string(metadata)?;

    conn.execute(
        "INSERT INTO keys (
            fingerprint, user_name, user_email, key_type, created_at, expires_at,
            is_private, is_favorite, key_content, metadata_json
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
        ON CONFLICT(fingerprint) DO UPDATE SET
            user_name=excluded.user_name,
            user_email=excluded.user_email,
            key_type=excluded.key_type,
            created_at=excluded.created_at,
            expires_at=excluded.expires_at,
            is_private=excluded.is_private,
            is_favorite=excluded.is_favorite,
            key_content=excluded.key_content,
            metadata_json=excluded.metadata_json",
        params![
            metadata.fingerprint,
            metadata.user_id.name,
            metadata.user_id.email,
            metadata.key_type.to_string(),
            metadata.created_at,
            metadata.expires_at,
            metadata.is_private,
            metadata.is_favorite,
            content,
            json
        ],
    )?;
    Ok(())
}

/// Returns the stored key content and metadata for a fingerprint, if present.
pub fn get_key(vault: &Vault, fingerprint: &str) -> Result<Option<(String, Option<KeyMetadata>)>> {
    let conn = vault.conn.lock().unwrap();
    let row = conn
        .query_row(
            "SELECT key_content, metadata_json FROM keys WHERE fingerprint = ?1",
            params![fingerprint],
            |row| Ok((row.get::<_, Option<String>>(0)?, row.get::<_, Option<String>>(1)?)),
        )
        .optional()?;

    Ok(row.and_then(|(content, json)| {
        let metadata = json.and_then(|j| serde_json::from_str(&j).ok());
        content.map(|c| (c, metadata))
    }))
}

/// Returns key content for the given fingerprints (or all private keys when
/// `fingerprints` is None), in a single lock acquisition.
pub fn get_key_contents(vault: &Vault, fingerprints: Option<&[String]>, private_only: bool) -> Result<Vec<(String, String)>> {
    let conn = vault.conn.lock().unwrap();
    let mut out = Vec::new();
    match fingerprints {
        Some(fps) => {
            let sql = if private_only {
                "SELECT fingerprint, key_content FROM keys WHERE fingerprint = ?1 AND is_private = 1 AND key_content IS NOT NULL"
            } else {
                "SELECT fingerprint, key_content FROM keys WHERE fingerprint = ?1 AND key_content IS NOT NULL"
            };
            let mut stmt = conn.prepare_cached(sql)?;
            for fp in fps {
                if let Some(row) = stmt
                    .query_row(params![fp], |row| Ok((row.get(0)?, row.get(1)?)))
                    .optional()?
                {
                    out.push(row);
                }
            }
        }
        None => {
            let sql = if private_only {
                "SELECT fingerprint, key_content FROM keys WHERE is_private = 1 AND key_content IS NOT NULL"
            } else {
                "SELECT fingerprint, key_content FROM keys WHERE key_content IS NOT NULL"
            };
            let mut stmt = conn.prepare(sql)?;
            let rows = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?;
            for row in rows {
                out.push(row?);
            }
        }
    }
    Ok(out)
}

pub fn list_all_keys(vault: &Vault) -> Result<Vec<KeyMetadata>> {
    let conn = vault.conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT fingerprint, metadata_json FROM keys
         WHERE metadata_json IS NOT NULL
         ORDER BY is_favorite DESC, created_at DESC",
    )?;

    let rows = stmt.query_map([], |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)))?;

    let mut keys = Vec::new();
    for row in rows {
        let (fingerprint, json) = row?;
        match serde_json::from_str(&json) {
            Ok(metadata) => keys.push(metadata),
            Err(e) => log::warn!("Skipping key {} with unreadable metadata: {}", fingerprint, e),
        }
    }

    Ok(keys)
}

pub fn delete_key(vault: &Vault, fingerprint: &str) -> Result<bool> {
    let conn = vault.conn.lock().unwrap();
    let count = conn.execute("DELETE FROM keys WHERE fingerprint = ?1", params![fingerprint])?;
    log::info!("Deleted {} rows for fingerprint: {}", count, fingerprint);
    Ok(count > 0)
}
