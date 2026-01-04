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
        output_path: output_path, // Updated field name
        success: true,            // Added field
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
    target_fingerprint: Option<String>,
    vault: State<'_, Vault>,
) -> Result<OperationResult<DecryptionResult>, String> {
    use std::fs::File;
    use sequoia_openpgp as openpgp;
    use openpgp::parse::stream::{DecryptorBuilder, DecryptionHelper, VerificationHelper, MessageStructure};
    use openpgp::policy::StandardPolicy;
    use openpgp::{KeyHandle, Cert};
    use openpgp::parse::Parse; // Import Parse for from_bytes/from_reader

    let p = StandardPolicy::new();

    struct Helper<'a> {
        conn: &'a std::sync::Mutex<rusqlite::Connection>,
        passphrase: String,
        target_fingerprint: Option<String>,
    }

    impl<'a> VerificationHelper for Helper<'a> {
        fn get_certs(&mut self, _ids: &[KeyHandle]) -> openpgp::Result<Vec<Cert>> {
            Ok(Vec::new()) 
        }
        fn check(&mut self, _structure: MessageStructure) -> openpgp::Result<()> {
            Ok(())
        }
    }

    impl<'a> DecryptionHelper for Helper<'a> {
        fn decrypt<D>(&mut self, pkesks: &[openpgp::packet::PKESK], _skesks: &[openpgp::packet::SKESK], sym_algo: Option<openpgp::types::SymmetricAlgorithm>, mut decrypt: D) -> openpgp::Result<Option<openpgp::Fingerprint>>
        where D: FnMut(openpgp::types::SymmetricAlgorithm, &openpgp::crypto::SessionKey) -> bool
        {
            let conn = self.conn.lock().unwrap();
            let mut candidate_keys = Vec::new();

            // 1. Fetch Candidate Keys
            if let Some(target) = &self.target_fingerprint {
                 let mut stmt = conn.prepare("SELECT key_content FROM keys WHERE fingerprint = ?1 AND is_private = 1").map_err(|e| anyhow::anyhow!(e))?;
                 let rows = stmt.query_map([target], |row| row.get::<_, String>(0)).map_err(|e| anyhow::anyhow!(e))?;
                 for r in rows { candidate_keys.push(r?); }
                 
                 // If specific key requested but not found in DB
                 if candidate_keys.is_empty() {
                     return Err(anyhow::anyhow!("Selected key not found in vault").into());
                 }
            } else {
                 // Try all private keys (legacy behavior)
                 let mut stmt = conn.prepare("SELECT key_content FROM keys WHERE is_private = 1").map_err(|e| anyhow::anyhow!(e))?;
                 let rows = stmt.query_map([], |row| row.get::<_, String>(0)).map_err(|e| anyhow::anyhow!(e))?;
                 for r in rows { candidate_keys.push(r?); }
            }

            // 2. Iterate PKESKs in the file
            for pkesk in pkesks {
                let key_id = pkesk.recipient();
                
                for key_str in &candidate_keys {
                     if let Ok(cert) = Cert::from_bytes(key_str.as_bytes()) {
                         for key in cert.keys().secret() {
                             if key.key().keyid() == *key_id {
                                 // Found a matching private key for this encrypted packet
                                 let key_clone = key.key().clone();
                                 let fp = cert.fingerprint();

                                 // Unlock attempt
                                 let decrypted_key_res = if self.passphrase.is_empty() {
                                     key_clone.into_keypair()
                                 } else {
                                     key_clone.decrypt_secret(&self.passphrase.clone().into())
                                        .and_then(|k| k.into_keypair())
                                 };

                                 match decrypted_key_res {
                                     Ok(mut decrypted_key) => {
                                         if pkesk.decrypt(&mut decrypted_key, sym_algo).map(|(algo, session_key)| decrypt(algo, &session_key)).unwrap_or(false) {
                                              return Ok(Some(fp));
                                         }
                                     },
                                     Err(_) => {
                                         // Found the RIGHT key but WRONG password.
                                         // Return explicit error to user.
                                         // (Using generic error message to avoid oracle if paranoia needed, but user wants clarity)
                                         return Err(anyhow::anyhow!("Wrong passphrase for key {}", fp).into());
                                     }
                                 }
                             }
                         }
                     }
                }
            }
            
            // 3. Fallback: If we had a target but didn't find its PKESK
            if self.target_fingerprint.is_some() {
                return Err(anyhow::anyhow!("File is not encrypted for the selected key").into());
            }

            Ok(None)
        }
    }

    let helper = Helper {
        conn: &vault.conn,
        passphrase,
        target_fingerprint,
    };

    let mut input_file = File::open(&input_path).map_err(|e| e.to_string())?;
    
    let mut decryptor = DecryptorBuilder::from_reader(&mut input_file).map_err(|e| e.to_string())?
        .with_policy(&p, None, helper)
        .map_err(|e| e.to_string())?;

    let mut output_file = File::create(&output_path).map_err(|e| e.to_string())?;
    
    std::io::copy(&mut decryptor, &mut output_file).map_err(|e| e.to_string())?;

    Ok(OperationResult::ok(DecryptionResult {
        output_path: output_path,
        success: true,
        size: 0,
        decrypted_with: None,
        signatures: Vec::new(),
    }))
}
