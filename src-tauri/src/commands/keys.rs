// Tauri commands for key management

use crate::core::{keyring, storage::Vault};
use crate::models::{KeyMetadata, KeyType, OperationResult, UserId};
use tauri::State;

#[tauri::command]
pub async fn list_keys(vault: State<'_, Vault>) -> Result<OperationResult<Vec<KeyMetadata>>, String> {
    match keyring::list_all_keys(&vault) {
        Ok(keys) => Ok(OperationResult::ok(keys)),
        Err(e) => Ok(OperationResult::err(format!("Failed to list keys: {}", e))),
    }
}

#[tauri::command]
pub async fn delete_key(
    fingerprint: String,
    vault: State<'_, Vault>,
) -> Result<OperationResult<bool>, String> {
    log::info!("DELETE_KEY command called for fingerprint: {}", fingerprint);
    match keyring::delete_key_files(&vault, &fingerprint) {
        Ok(deleted) => {
            log::info!("Deleted key {}: {}", fingerprint, deleted);
            Ok(OperationResult::ok(deleted))
        },
        Err(e) => {
            log::error!("Failed to delete key {}: {}", fingerprint, e);
            Ok(OperationResult::err(format!("Failed to delete key: {}", e)))
        },
    }
}

#[tauri::command]
pub async fn generate_key(
    name: String,
    email: String,
    comment: Option<String>,
    key_type: String,
    passphrase: String,
    expiry_timestamp: Option<i64>, // Unix timestamp in seconds
    vault: State<'_, Vault>,
) -> Result<OperationResult<KeyMetadata>, String> {
    println!("DEBUG: generate_key called with name={}, email={}", name, email);
    // Note: key_type is currently ignored by our simple Sequoia implementation which defaults to sensible modern defaults
    // but in a full implementation we would map it to CipherSuite/Curve options.
    
    // Calculate duration in seconds if expiry is set
    let valid_seconds = expiry_timestamp.map(|ts| {
        let now = chrono::Utc::now().timestamp();
        if ts > now {
             (ts - now) as u64
        } else {
             0 // Should probably error or default to minimal, but 0 often means no-expire or immediate. Sequoia behavior for 0?
        }
    });

    match crate::core::crypto::generate_keypair(&name, &email, &passphrase, valid_seconds) {
        Ok((_public_key, private_key)) => {
            // ... (rest of function) ...
            
            use sequoia_openpgp as openpgp;
            use openpgp::parse::Parse;
            
            let cert = match openpgp::Cert::from_bytes(private_key.as_bytes()) {
                Ok(c) => c,
                Err(e) => return Ok(OperationResult::err(format!("Failed to parse generated key: {}", e))),
            };
            
            // Format fingerprint as hex string without spaces for filename safety
            let fingerprint = cert.fingerprint().to_string().replace(" ", "");
            
            let metadata = KeyMetadata {
                fingerprint: fingerprint.clone(),
                key_type: if key_type == "rsa4096" {
                    KeyType::Rsa4096
                } else {
                    KeyType::Ed25519
                },
                user_id: UserId {
                    name,
                    email,
                    comment,
                },
                created_at: chrono::Utc::now().to_rfc3339(),
                expires_at: expiry_timestamp.map(|ts| {
                    chrono::DateTime::from_timestamp(ts, 0)
                        .map(|dt| dt.to_rfc3339())
                        .unwrap_or_default()
                }),
                is_private: true,
                is_favorite: false,
                tags: vec![],
            };

            // Save key file (private key which includes public parts)
            if let Err(e) = keyring::save_key_to_file(&vault, &fingerprint, &private_key) {
                return Ok(OperationResult::err(format!("Failed to save key file: {}", e)));
            }

            // Save metadata
            match keyring::save_key_metadata(&vault, &metadata) {
                Ok(_) => Ok(OperationResult::ok(metadata)),
                Err(e) => Ok(OperationResult::err(format!("Failed to save key metadata: {}", e))),
            }
        },
        Err(e) => Ok(OperationResult::err(format!("Failed to generate key: {}", e)))
    }
}

#[tauri::command]
pub async fn import_key(
    key_text: String,
    vault: State<'_, Vault>,
) -> Result<OperationResult<KeyMetadata>, String> {
    use sequoia_openpgp as openpgp;
    use openpgp::parse::Parse;
    use openpgp::Cert; // Can use Cert directly via openpgp::Cert too
    
    // Parse the key
    let cert = match Cert::from_bytes(key_text.as_bytes()) {
        Ok(c) => c,
        Err(e) => return Ok(OperationResult::err(format!("Failed to parse key: {}", e))),
    };

    let fingerprint = cert.fingerprint().to_string().replace(" ", "");
    
    // Extract User ID (first one)
    let mut name = String::new();
    let mut email = String::new();
    let comment = None;

    if let Some(uid) = cert.userids().next() {
        // uid is a Packet, uid.userid() returns &UserID
        let s = String::from_utf8_lossy(uid.userid().value()).to_string();
        
        // Basic parsing of "Name (Comment) <Email>"
        if let Some(start) = s.find('<') {
            if let Some(end) = s.find('>') {
                email = s[start+1..end].to_string();
                name = s[0..start].trim().to_string();
            }
        } else {
            name = s;
        }
    }
    
    // Determine key type
    let key_type = match cert.primary_key().pk_algo() {
        openpgp::types::PublicKeyAlgorithm::EdDSA => KeyType::Ed25519,
        _ => KeyType::Rsa4096, // Default fallback/mapping
    };

    // Use standard policy to check validity and get expiration
    let p = openpgp::policy::StandardPolicy::new();
    let expires_at = if let Ok(valid_cert) = cert.with_policy(&p, None) {
        valid_cert.primary_key().key_expiration_time().map(|t| {
            let datetime: chrono::DateTime<chrono::Utc> = t.into();
            datetime.to_rfc3339()
        })
    } else {
        None
    };

    let creation_time: chrono::DateTime<chrono::Utc> = cert.primary_key().creation_time().into();

    let metadata = KeyMetadata {
        fingerprint: fingerprint.clone(),
        key_type,
        user_id: UserId {
            name,
            email,
            comment,
        },
        created_at: creation_time.to_rfc3339(),
        expires_at,
        is_private: cert.is_tsk(),
        is_favorite: false,
        tags: vec![],
    };

    // Save content
    if let Err(e) = keyring::save_key_to_file(&vault, &fingerprint, &key_text) {
        return Ok(OperationResult::err(format!("Failed to save key content: {}", e)));
    }

    // Save metadata
    match keyring::save_key_metadata(&vault, &metadata) {
        Ok(_) => Ok(OperationResult::ok(metadata)),
        Err(e) => Ok(OperationResult::err(format!("Failed to save key metadata: {}", e))),
    }
}

#[tauri::command]
pub async fn export_key(
    fingerprint: String,
    export_private: bool,
    vault: State<'_, Vault>,
) -> Result<OperationResult<String>, String> {
    log::info!("EXPORT_KEY command called for fingerprint: {}, private: {}", fingerprint, export_private);
    
    use sequoia_openpgp as openpgp;
    use openpgp::parse::Parse;
    use openpgp::serialize::Serialize;

    let conn = vault.conn.lock().unwrap();
    let mut stmt = match conn.prepare("SELECT key_content FROM keys WHERE fingerprint = ?1") {
         Ok(s) => s,
         Err(e) => {
             log::error!("Database prepare error: {}", e);
             return Ok(OperationResult::err(format!("Database error: {}", e)));
         }
    };
    
    let key_content: String = match stmt.query_row(rusqlite::params![fingerprint], |row| row.get(0)) {
        Ok(s) => {
            log::info!("Key content loaded from database");
            s
        },
        Err(e) => {
            log::error!("Key not found in database: {}", e);
            return Ok(OperationResult::err("Key not found".to_string()));
        }
    };

    if export_private {
        log::info!("Exporting private key");
        let cert = match openpgp::Cert::from_bytes(key_content.as_bytes()) {
            Ok(c) => c,
            Err(e) => {
                log::error!("Failed to parse cert: {}", e);
                return Ok(OperationResult::err(format!("Failed to parse key: {}", e)));
            }
        };
        
        if !cert.is_tsk() {
             log::warn!("Key is not a private key");
             return Ok(OperationResult::err("Selected key is a public key, cannot export private key.".to_string()));
        }
        log::info!("Private key export successful");
        Ok(OperationResult::ok(key_content))
    } else {
        log::info!("Exporting public key from cert");
        let cert = match openpgp::Cert::from_bytes(key_content.as_bytes()) {
            Ok(c) => c,
            Err(e) => {
                log::error!("Failed to parse cert: {}", e);
                return Ok(OperationResult::err(format!("Failed to parse key: {}", e)));
            }
        };
        
        let mut public_bytes = Vec::new();
        let message = openpgp::serialize::stream::Message::new(&mut public_bytes);
        let mut armor_writer = match openpgp::serialize::stream::Armorer::new(message)
            .kind(openpgp::armor::Kind::PublicKey)
            .build() {
                Ok(w) => w,
                Err(e) => {
                    log::error!("Failed to create armorer: {}", e);
                    return Ok(OperationResult::err(format!("Failed to create armorer: {}", e)));
                }
            };
            
        if let Err(e) = cert.serialize(&mut armor_writer) {
            log::error!("Failed to serialize cert: {}", e);
            return Ok(OperationResult::err(format!("Failed to serialize cert: {}", e)));
        }
        
        if let Err(e) = armor_writer.finalize() {
            log::error!("Failed to finalize armorer: {}", e);
            return Ok(OperationResult::err(format!("Failed to finalize armorer: {}", e)));
        }
        
        let public_key = match String::from_utf8(public_bytes) {
            Ok(s) => s,
            Err(e) => {
                log::error!("Failed to convert to UTF-8: {}", e);
                return Ok(OperationResult::err(format!("Failed to convert to UTF-8: {}", e)));
            }
        };
        
        Ok(OperationResult::ok(public_key))
    }
}
