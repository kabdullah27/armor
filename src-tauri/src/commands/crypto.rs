// Tauri commands for crypto operations

use crate::models::{EncryptionResult, DecryptionResult, OperationResult};
use tauri::State;
use crate::core::storage::Vault;

#[tauri::command]
pub async fn encrypt_file_cmd(
    input_path: String,
    output_path: String,
    recipient_fingerprints: Vec<String>,
    armor: bool,
    vault: State<'_, Vault>,
) -> Result<OperationResult<EncryptionResult>, String> {
    use std::fs::File;
    use sequoia_openpgp as openpgp;
    #[allow(deprecated)]
    use openpgp::serialize::stream::{Message, Encryptor, LiteralWriter, Armorer, Recipient};
    use openpgp::policy::StandardPolicy;
    use openpgp::parse::Parse;

    let p = StandardPolicy::new();

    // 1. Load Recipient Keys
    let mut loaded_certs = Vec::new(); // Keep ownership of certs here
    let mut recipients_keys = Vec::new(); 
    
    {
        let conn = vault.conn.lock().unwrap();
        
        for fingerprint in &recipient_fingerprints {
            let mut stmt = conn.prepare("SELECT key_content FROM keys WHERE fingerprint = ?1").map_err(|e| e.to_string())?;
            let key_content: String = match stmt.query_row(rusqlite::params![fingerprint], |row| row.get(0)) {
                Ok(s) => s,
                Err(_) => return Ok(OperationResult::err(format!("Recipient key not found: {}", fingerprint))),
            };

            let cert = openpgp::Cert::from_bytes(key_content.as_bytes()).map_err(|e| format!("Failed to parse key {}: {}", fingerprint, e))?;
            loaded_certs.push(cert);
        }
    } // Drop lock

    // Now process the loaded certs to get recipients
    for cert in &loaded_certs {
        let found_keys: Vec<_> = cert.keys().with_policy(&p, None)
            .supported()
            .alive()
            .revoked(false)
            .for_transport_encryption()
            .collect();
            
        if found_keys.is_empty() {
             // Warning: Skipping key with no valid subkeys? Or fail?
             // For now, let's fail if a requested recipient has no valid keys
             return Ok(OperationResult::err(format!("Key {} has no valid encryption subkeys", cert.fingerprint())));
        }

        for k in found_keys {
             recipients_keys.push(Recipient::from(k.key()));
        }
    }

    if recipients_keys.is_empty() {
        return Ok(OperationResult::err("No valid recipients found".to_string()));
    }

    // 2. Prepare Output Stream
    let output_file = File::create(&output_path).map_err(|e| e.to_string())?;
    
    let message = Message::new(output_file);
    
    // Ownership Model: Each layer consumes the previous one.
    // Message -> Armorer -> Encryptor -> LiteralWriter
    if armor {
        let armor_writer = Armorer::new(message)
            .kind(openpgp::armor::Kind::Message)
            .build()
            .map_err(|e| e.to_string())?;
        
        #[allow(deprecated)]
        let encryptor = Encryptor::for_recipients(armor_writer, recipients_keys)
            .build()
            .map_err(|e| e.to_string())?;

        // 3. Write Literal Data
        // LiteralWriter takes ownership of encryptor
        let mut writer = LiteralWriter::new(encryptor).build().map_err(|e| e.to_string())?;
        
        let mut input_file = File::open(&input_path).map_err(|e| e.to_string())?;
        std::io::copy(&mut input_file, &mut writer).map_err(|e| e.to_string())?;
        
        // 4. Finalize chain (Unwinding)
        // We only finalize the top-level writer. 
        // The inner writers (Encryptor, Armorer) will be finalized on Drop.
        // This is necessary because LiteralWriter::finalize() returns () in this configuration.
        writer.finalize().map_err(|e| e.to_string())?;

    } else {
        // Message -> Encryptor -> LiteralWriter
        #[allow(deprecated)]
        let encryptor = Encryptor::for_recipients(message, recipients_keys)
            .build()
            .map_err(|e| e.to_string())?;

        // 3. Write Literal Data
        let mut writer = LiteralWriter::new(encryptor).build().map_err(|e| e.to_string())?;
        
        let mut input_file = File::open(&input_path).map_err(|e| e.to_string())?;
        std::io::copy(&mut input_file, &mut writer).map_err(|e| e.to_string())?;
        
        // 4. Finalize chain
        writer.finalize().map_err(|e| e.to_string())?;
    }
    // message does not need finalize

    Ok(OperationResult::ok(EncryptionResult {
        output_file: output_path,
        size: 0, 
        recipients: recipient_fingerprints,
        signed: false,
    }))
}

#[tauri::command]
pub async fn decrypt_file_cmd(
    input_path: String,
    output_path: String,
    passphrase: String,
    vault: State<'_, Vault>,
) -> Result<OperationResult<DecryptionResult>, String> {
    use std::fs::File;
    use sequoia_openpgp as openpgp;
    use openpgp::parse::stream::{DecryptorBuilder, DecryptionHelper, VerificationHelper, MessageStructure};
    use openpgp::policy::StandardPolicy;
    use openpgp::{KeyHandle, Cert};
    use openpgp::parse::Parse;

    let p = StandardPolicy::new();

    // Helper struct - we need to keep vault reference
    struct Helper {
        vault: std::sync::Arc<std::sync::Mutex<rusqlite::Connection>>,
        passphrase: String,
    }

    impl VerificationHelper for Helper {
        fn get_certs(&mut self, _ids: &[KeyHandle]) -> openpgp::Result<Vec<Cert>> {
            // Implementation optional for strictly decryption, but good for signature verification
            Ok(Vec::new()) 
        }
        fn check(&mut self, _structure: MessageStructure) -> openpgp::Result<()> {
            Ok(())
        }
    }

    impl DecryptionHelper for Helper {
        fn decrypt<D>(&mut self, pkesks: &[openpgp::packet::PKESK], _skesks: &[openpgp::packet::SKESK], _sym_algo: Option<openpgp::types::SymmetricAlgorithm>, mut decrypt: D) -> openpgp::Result<Option<openpgp::Fingerprint>>
        where D: FnMut(openpgp::types::SymmetricAlgorithm, &openpgp::crypto::SessionKey) -> bool
        {
            let conn = self.vault.lock().unwrap();

            for pkesk in pkesks {
                let key_id = pkesk.recipient();
                
                // Try to find key in DB by Key ID (or Subkey ID)
                // Stored fingerprints are hex strings. KeyID is last 16 chars (64 bit) or 8 chars (32 bit).
                // Let's iterate all private keys? Or better, query.
                // SQLite doesn't have easy "endswith" for blob/text unless we use LIKE.
                // KeyID (hex) is 8 bytes = 16 hex chars.
                // Let's load all private keys. optimize later.
                let mut stmt = conn.prepare("SELECT key_content FROM keys WHERE is_private = 1").map_err(|e| anyhow::anyhow!(e))?;
                
                let keys_iter = stmt.query_map([], |row| {
                    Ok(row.get::<_, String>(0)?)
                }).map_err(|e| anyhow::anyhow!(e))?;

                for key_res in keys_iter {
                    if let Ok(key_str) = key_res {
                         if let Ok(cert) = Cert::from_bytes(key_str.as_bytes()) {
                             // Check if this cert has the key we need
                             for key in cert.keys().secret() {
                                 if key.key().keyid() == *key_id {
                                     // Found a matching secret key!
                                     // Try to decrypt it
                                     let key_clone = key.clone();

                                     // Unlock key
                                     let decrypted_key = if self.passphrase.is_empty() {
                                         // Provide empty password if logic requires, but unencrypted keys usually work directly or need empty password
                                         key_clone.key().clone().into_keypair().ok()
                                     } else {
                                        key_clone.key().clone().decrypt_secret(&self.passphrase.clone().into())
                                            .ok()
                                            .and_then(|k| k.into_keypair().ok())
                                     };

                                     if let Some(mut decrypted_key) = decrypted_key {
                                          if pkesk.decrypt(&mut decrypted_key, None).map(|(algo, session_key)| decrypt(algo, &session_key)).unwrap_or(false) {
                                              return Ok(Some(cert.fingerprint()));
                                          }
                                     }
                                 }
                             }
                         }
                    }
                }
            }
            Ok(None)
        }
    }

    // Create helper with Arc-wrapped connection
    let vault_conn = std::sync::Arc::new(std::sync::Mutex::new(
        rusqlite::Connection::open(vault.db_path.clone()).map_err(|e| e.to_string())?
    ));
    
    let helper = Helper {
        vault: vault_conn,
        passphrase,
    };

    let input_file = File::open(&input_path).map_err(|e| e.to_string())?;
    
    // Create decryptor
    let mut decryptor = DecryptorBuilder::from_reader(input_file).map_err(|e| e.to_string())?
        .with_policy(&p, None, helper)
        .map_err(|e| e.to_string())?;

    // Prepare output
    let mut output_file = File::create(&output_path).map_err(|e| e.to_string())?;
    
    // Read entire decrypted stream
    std::io::copy(&mut decryptor, &mut output_file).map_err(|e| e.to_string())?;

    Ok(OperationResult::ok(DecryptionResult {
        output_file: output_path,
        size: 0,
        decrypted_with: String::from("private_key"),
        signatures: Vec::new(),
    }))
}
