// Tauri commands for key management

use crate::core::{crypto, keyring, storage::Vault};
use crate::models::{KeyMetadata, KeyType, OperationResult, UserId};
use sequoia_openpgp as openpgp;
use openpgp::parse::Parse;
use openpgp::Cert;
use tauri::State;

fn to_rfc3339(t: std::time::SystemTime) -> String {
    chrono::DateTime::<chrono::Utc>::from(t).to_rfc3339()
}

/// Canonical fingerprint format used as the DB primary key (hex, no spaces).
fn fingerprint_of(cert: &Cert) -> String {
    cert.fingerprint().to_hex()
}

fn detect_key_type(cert: &Cert) -> KeyType {
    use openpgp::types::PublicKeyAlgorithm::*;
    let primary = cert.primary_key();
    match primary.pk_algo() {
        EdDSA => KeyType::Ed25519,
        ECDH => KeyType::Curve25519,
        #[allow(deprecated)]
        RSAEncryptSign | RSAEncrypt | RSASign => match primary.mpis().bits() {
            Some(bits) if bits <= 2048 => KeyType::Rsa2048,
            _ => KeyType::Rsa4096,
        },
        _ => KeyType::Rsa4096,
    }
}

/// Builds metadata from a cert, keeping user-managed fields (favorite, tags)
/// from an existing record when present.
fn metadata_from_cert(cert: &Cert, existing: Option<&KeyMetadata>) -> KeyMetadata {
    let mut user_id = UserId { name: String::new(), email: String::new(), comment: None };

    if let Some(uid) = cert.userids().next() {
        let uid = uid.userid();
        let name = uid.name2().ok().flatten();
        let email = uid.email2().ok().flatten();
        match (name, email) {
            (None, None) => user_id.name = String::from_utf8_lossy(uid.value()).into_owned(),
            (name, email) => {
                user_id.name = name.unwrap_or_default().to_string();
                user_id.email = email.unwrap_or_default().to_string();
            }
        }
        user_id.comment = uid.comment2().ok().flatten().map(str::to_string);
    }

    let p = openpgp::policy::StandardPolicy::new();
    let expires_at = cert
        .with_policy(&p, None)
        .ok()
        .and_then(|vc| vc.primary_key().key_expiration_time())
        .map(to_rfc3339);

    KeyMetadata {
        fingerprint: fingerprint_of(cert),
        key_type: detect_key_type(cert),
        user_id,
        created_at: to_rfc3339(cert.primary_key().creation_time()),
        expires_at,
        is_private: cert.is_tsk(),
        is_favorite: existing.is_some_and(|m| m.is_favorite),
        tags: existing.map(|m| m.tags.clone()).unwrap_or_default(),
    }
}

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
    match keyring::delete_key(&vault, &fingerprint) {
        Ok(deleted) => Ok(OperationResult::ok(deleted)),
        Err(e) => {
            log::error!("Failed to delete key {}: {}", fingerprint, e);
            Ok(OperationResult::err(format!("Failed to delete key: {}", e)))
        }
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
    log::info!("generate_key called with key_type={}", key_type);

    let name = name.trim().to_string();
    let email = email.trim().to_string();
    if name.is_empty() || email.is_empty() {
        return Ok(OperationResult::err("Name and email are required"));
    }

    let valid_seconds = match expiry_timestamp {
        Some(ts) => {
            let now = chrono::Utc::now().timestamp();
            if ts <= now {
                return Ok(OperationResult::err("Expiry date must be in the future"));
            }
            Some((ts - now) as u64)
        }
        None => None,
    };

    // Key generation (especially RSA 4096) is CPU-heavy; keep it off the async runtime.
    let generated = tauri::async_runtime::spawn_blocking(move || {
        crypto::generate_keypair(&name, &email, &passphrase, valid_seconds, &key_type)
    })
    .await
    .map_err(|e| e.to_string())?;

    let (cert, private_key) = match generated {
        Ok(v) => v,
        Err(e) => return Ok(OperationResult::err(format!("Failed to generate key: {}", e))),
    };

    let mut metadata = metadata_from_cert(&cert, None);
    metadata.user_id.comment = comment.filter(|c| !c.trim().is_empty());

    match keyring::save_key(&vault, &metadata, &private_key) {
        Ok(_) => Ok(OperationResult::ok(metadata)),
        Err(e) => Ok(OperationResult::err(format!("Failed to save key: {}", e))),
    }
}

/// Imports an ASCII-armored or binary key. If the key already exists in the
/// vault, the two are merged so importing a public key never discards a
/// previously stored secret key.
#[tauri::command]
pub async fn import_key(
    key_data: Vec<u8>,
    vault: State<'_, Vault>,
) -> Result<OperationResult<KeyMetadata>, String> {
    let mut cert = match Cert::from_bytes(&key_data) {
        Ok(c) => c,
        Err(e) => return Ok(OperationResult::err(format!("Failed to parse key: {}", e))),
    };

    let fingerprint = fingerprint_of(&cert);
    let existing = keyring::get_key(&vault, &fingerprint).map_err(|e| e.to_string())?;

    let mut existing_meta = None;
    if let Some((content, meta)) = existing {
        existing_meta = meta;
        match Cert::from_bytes(content.as_bytes()).and_then(|old| old.merge_public_and_secret(cert.clone())) {
            Ok(merged) => cert = merged,
            Err(e) => log::warn!("Could not merge with stored key {}: {}", fingerprint, e),
        }
    }

    let content = match crypto::armor_cert(&cert, cert.is_tsk()) {
        Ok(c) => c,
        Err(e) => return Ok(OperationResult::err(format!("Failed to serialize key: {}", e))),
    };

    let metadata = metadata_from_cert(&cert, existing_meta.as_ref());
    match keyring::save_key(&vault, &metadata, &content) {
        Ok(_) => Ok(OperationResult::ok(metadata)),
        Err(e) => Ok(OperationResult::err(format!("Failed to save key: {}", e))),
    }
}

#[tauri::command]
pub async fn export_key(
    fingerprint: String,
    export_private: bool,
    vault: State<'_, Vault>,
) -> Result<OperationResult<String>, String> {
    log::info!("EXPORT_KEY called for fingerprint: {}, private: {}", fingerprint, export_private);

    let key_content = match keyring::get_key(&vault, &fingerprint) {
        Ok(Some((content, _))) => content,
        Ok(None) => return Ok(OperationResult::err("Key not found")),
        Err(e) => return Ok(OperationResult::err(format!("Database error: {}", e))),
    };

    let cert = match Cert::from_bytes(key_content.as_bytes()) {
        Ok(c) => c,
        Err(e) => return Ok(OperationResult::err(format!("Failed to parse key: {}", e))),
    };

    if export_private && !cert.is_tsk() {
        return Ok(OperationResult::err("Selected key is a public key, cannot export private key."));
    }

    match crypto::armor_cert(&cert, export_private) {
        Ok(armored) => Ok(OperationResult::ok(armored)),
        Err(e) => Ok(OperationResult::err(format!("Failed to export key: {}", e))),
    }
}
