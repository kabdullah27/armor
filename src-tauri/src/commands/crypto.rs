// Tauri commands for crypto operations

use crate::core::{keyring, storage::Vault};
use crate::models::{DecryptionResult, EncryptionResult, OperationResult};
use anyhow::{anyhow, bail, Context};
use sequoia_openpgp as openpgp;
use openpgp::parse::Parse;
use openpgp::policy::StandardPolicy;
use openpgp::Cert;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use tauri::State;

/// Writes to a temporary sibling file and only moves it into place once the
/// whole operation succeeded. This means a failed run never leaves a partial
/// (or empty) output file behind, and using the input path as the output path
/// can't truncate the input before it has been read.
struct AtomicOutput {
    tmp: PathBuf,
    target: PathBuf,
}

impl AtomicOutput {
    fn new(target: &str) -> Self {
        let target = PathBuf::from(target);
        let mut tmp = target.clone().into_os_string();
        tmp.push(".armor-tmp");
        Self { tmp: tmp.into(), target }
    }

    fn create(&self) -> anyhow::Result<File> {
        File::create(&self.tmp).with_context(|| format!("Cannot create {}", self.target.display()))
    }

    fn commit(self) -> anyhow::Result<u64> {
        fs::rename(&self.tmp, &self.target)?;
        Ok(fs::metadata(&self.target)?.len())
    }
}

impl Drop for AtomicOutput {
    fn drop(&mut self) {
        // No-op after a successful commit (the tmp file was renamed away).
        let _ = fs::remove_file(&self.tmp);
    }
}

fn load_certs(vault: &Vault, fingerprints: Option<&[String]>, private_only: bool) -> anyhow::Result<Vec<Cert>> {
    keyring::get_key_contents(vault, fingerprints, private_only)?
        .into_iter()
        .map(|(fp, content)| {
            Cert::from_bytes(content.as_bytes()).with_context(|| format!("Failed to parse key {}", fp))
        })
        .collect()
}

fn encrypt_file(input: &Path, out: AtomicOutput, certs: &[Cert], armor: bool) -> anyhow::Result<u64> {
    use openpgp::serialize::stream::{Armorer, Encryptor2 as Encryptor, LiteralWriter, Message, Recipient};

    let p = StandardPolicy::new();
    let mut recipients = Vec::new();
    for cert in certs {
        let before = recipients.len();
        for k in cert
            .keys()
            .with_policy(&p, None)
            .supported()
            .alive()
            .revoked(false)
            .for_transport_encryption()
            .for_storage_encryption()
        {
            recipients.push(Recipient::from(k.key()));
        }
        if recipients.len() == before {
            bail!("Key {} has no valid encryption subkeys (expired or revoked?)", cert.fingerprint());
        }
    }

    let mut input_file = File::open(input).with_context(|| format!("Cannot open {}", input.display()))?;
    let output_file = out.create()?;

    let mut message = Message::new(output_file);
    if armor {
        message = Armorer::new(message).kind(openpgp::armor::Kind::Message).build()?;
    }
    let message = Encryptor::for_recipients(message, recipients).build()?;
    let mut writer = LiteralWriter::new(message).build()?;
    std::io::copy(&mut input_file, &mut writer)?;
    // Finalizes the whole writer stack (literal -> encryptor -> armorer).
    writer.finalize()?;

    out.commit()
}

#[tauri::command]
pub async fn encrypt_file_cmd(
    input_path: String,
    output_path: String,
    recipient_fingerprints: Vec<String>,
    armor: bool,
    vault: State<'_, Vault>,
) -> Result<OperationResult<EncryptionResult>, String> {
    if recipient_fingerprints.is_empty() {
        return Ok(OperationResult::err("No recipients selected"));
    }

    let certs = match load_certs(&vault, Some(&recipient_fingerprints), false) {
        Ok(c) => c,
        Err(e) => return Ok(OperationResult::err(e.to_string())),
    };
    if certs.len() != recipient_fingerprints.len() {
        return Ok(OperationResult::err("One or more recipient keys were not found in the vault"));
    }

    let out = AtomicOutput::new(&output_path);
    let input = PathBuf::from(&input_path);
    let res = tauri::async_runtime::spawn_blocking(move || encrypt_file(&input, out, &certs, armor))
        .await
        .map_err(|e| e.to_string())?;

    match res {
        Ok(size) => Ok(OperationResult::ok(EncryptionResult {
            output_path,
            success: true,
            size,
            recipients: recipient_fingerprints,
            signed: false,
        })),
        Err(e) => Ok(OperationResult::err(format!("Encryption failed: {:#}", e))),
    }
}

/// Turns low-level errors into messages a user can act on.
fn explain_error(e: anyhow::Error) -> String {
    let text = format!("{:#}", e);
    if text.contains("AEAD Encrypted Data Packet") {
        return "This file uses GnuPG's non-standard OCB encryption (LibrePGP), which Armor \
                can't open. GnuPG 2.3+ picks it when your key was created by GnuPG. To fix it, \
                run `gpg --edit-key <your key>`, then `setpref AES256 AES192 AES SHA512 SHA384 \
                SHA256 ZLIB BZIP2 ZIP Uncompressed` and `save`, send your updated public key \
                to the sender, and ask them to encrypt the file again."
            .to_string();
    }
    text
}

/// Rejects PKESK packets that would trigger RUSTSEC-2025-0136 in Sequoia 1.x:
/// an ECDH session key shorter than one AES key-wrap block underflows inside
/// `aes_key_unwrap` and panics, which crashes the app (release builds abort on
/// panic). A valid RFC 3394 wrapped key is at least 24 bytes. The upstream fix
/// is only in Sequoia 2.x, which can't read many GnuPG-exported keys yet.
fn is_well_formed(pkesk: &openpgp::packet::PKESK) -> bool {
    use openpgp::crypto::mpi::Ciphertext;
    match pkesk.esk() {
        Ciphertext::ECDH { key, .. } => key.len() >= 24,
        _ => true,
    }
}

struct DecryptHelper {
    certs: Vec<Cert>,
    passphrase: String,
    restrict_to_target: bool,
    used: Option<openpgp::Fingerprint>,
}

impl openpgp::parse::stream::VerificationHelper for DecryptHelper {
    fn get_certs(&mut self, _ids: &[openpgp::KeyHandle]) -> openpgp::Result<Vec<Cert>> {
        Ok(Vec::new())
    }
    fn check(&mut self, _structure: openpgp::parse::stream::MessageStructure) -> openpgp::Result<()> {
        Ok(())
    }
}

impl openpgp::parse::stream::DecryptionHelper for DecryptHelper {
    fn decrypt<D>(
        &mut self,
        pkesks: &[openpgp::packet::PKESK],
        _skesks: &[openpgp::packet::SKESK],
        sym_algo: Option<openpgp::types::SymmetricAlgorithm>,
        mut decrypt: D,
    ) -> openpgp::Result<Option<openpgp::Fingerprint>>
    where
        D: FnMut(openpgp::types::SymmetricAlgorithm, &openpgp::crypto::SessionKey) -> bool,
    {
        for pkesk in pkesks {
            if !is_well_formed(pkesk) {
                continue;
            }
            let recipient = pkesk.recipient();
            for cert in &self.certs {
                for key in cert.keys().secret() {
                    // A wildcard recipient means an anonymous PKESK: try every key.
                    if !recipient.is_wildcard() && key.key().keyid() != *recipient {
                        continue;
                    }

                    let key = key.key().clone();
                    let keypair = if key.has_unencrypted_secret() {
                        key.into_keypair()
                    } else if self.passphrase.is_empty() {
                        return Err(anyhow!("Key {} is protected, please enter its passphrase", cert.fingerprint()));
                    } else {
                        match key.decrypt_secret(&self.passphrase.as_str().into()) {
                            Ok(k) => k.into_keypair(),
                            Err(e) => {
                                return Err(anyhow!(
                                    "Could not unlock key {} (wrong passphrase?): {}",
                                    cert.fingerprint(),
                                    e
                                ))
                            }
                        }
                    };

                    if let Ok(mut keypair) = keypair {
                        if let Some((algo, session_key)) = pkesk.decrypt(&mut keypair, sym_algo) {
                            if decrypt(algo, &session_key) {
                                self.used = Some(cert.fingerprint());
                                return Ok(Some(cert.fingerprint()));
                            }
                        }
                    }
                }
            }
        }

        if self.restrict_to_target {
            return Err(anyhow!("File is not encrypted for the selected key"));
        }
        Err(anyhow!("None of your private keys can decrypt this file"))
    }
}

fn decrypt_file(input: &Path, out: AtomicOutput, helper: DecryptHelper) -> anyhow::Result<(u64, Option<String>)> {
    use openpgp::parse::stream::DecryptorBuilder;

    let p = StandardPolicy::new();
    let input_file = File::open(input).with_context(|| format!("Cannot open {}", input.display()))?;
    let mut decryptor = DecryptorBuilder::from_reader(input_file)?.with_policy(&p, None, helper)?;

    let mut output_file = out.create()?;
    std::io::copy(&mut decryptor, &mut output_file)?;
    output_file.sync_all()?;
    drop(output_file);

    let used = decryptor.into_helper().used.map(|fp| fp.to_hex());
    Ok((out.commit()?, used))
}

#[tauri::command]
pub async fn decrypt_file_cmd(
    input_path: String,
    output_path: String,
    passphrase: String,
    target_fingerprint: Option<String>,
    vault: State<'_, Vault>,
) -> Result<OperationResult<DecryptionResult>, String> {
    // Load and parse candidate keys once, up front, instead of re-parsing
    // every stored key for each encrypted session-key packet.
    let targets = target_fingerprint.map(|fp| vec![fp]);
    let certs = match load_certs(&vault, targets.as_deref(), true) {
        Ok(c) => c,
        Err(e) => return Ok(OperationResult::err(e.to_string())),
    };
    if certs.is_empty() {
        let msg = if targets.is_some() { "Selected key not found in vault" } else { "No private keys in vault" };
        return Ok(OperationResult::err(msg));
    }

    let helper = DecryptHelper {
        certs,
        passphrase,
        restrict_to_target: targets.is_some(),
        used: None,
    };
    let out = AtomicOutput::new(&output_path);
    let input = PathBuf::from(&input_path);
    let res = tauri::async_runtime::spawn_blocking(move || decrypt_file(&input, out, helper))
        .await
        .map_err(|e| e.to_string())?;

    match res {
        Ok((size, decrypted_with)) => Ok(OperationResult::ok(DecryptionResult {
            output_path,
            success: true,
            size,
            decrypted_with,
            signatures: Vec::new(),
        })),
        Err(e) => Ok(OperationResult::err(format!("Decryption failed: {}", explain_error(e)))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::crypto::generate_keypair;

    fn helper(cert: &Cert, pass: &str) -> DecryptHelper {
        DecryptHelper { certs: vec![cert.clone()], passphrase: pass.into(), restrict_to_target: true, used: None }
    }

    #[test]
    fn encrypt_decrypt_roundtrip() {
        let dir = std::env::temp_dir().join(format!("armor-test-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let plain = dir.join("plain.txt");
        let enc = dir.join("plain.txt.asc");
        let dec = dir.join("out.txt");
        fs::write(&plain, b"hello armor").unwrap();

        let (cert, armored) = generate_keypair("Alice", "alice@example.org", "s3cret", None, "ed25519").unwrap();
        assert!(cert.is_tsk());
        // Secrets must be passphrase-protected.
        assert!(cert.keys().secret().all(|k| !k.key().has_unencrypted_secret()));
        assert!(armored.contains("PRIVATE KEY BLOCK"));

        let size = encrypt_file(&plain, AtomicOutput::new(enc.to_str().unwrap()), std::slice::from_ref(&cert), true).unwrap();
        assert!(size > 0);

        // Wrong passphrase: error, and no output file left behind.
        let err = decrypt_file(&enc, AtomicOutput::new(dec.to_str().unwrap()), helper(&cert, "nope")).unwrap_err();
        assert!(format!("{:#}", err).contains("wrong passphrase"), "{:#}", err);
        assert!(!dec.exists());
        assert!(!dir.join("out.txt.armor-tmp").exists());

        let (_, used) = decrypt_file(&enc, AtomicOutput::new(dec.to_str().unwrap()), helper(&cert, "s3cret")).unwrap();
        assert_eq!(fs::read(&dec).unwrap(), b"hello armor");
        assert_eq!(used, Some(cert.fingerprint().to_hex()));

        // Encrypting a file onto itself must not destroy the input.
        encrypt_file(&plain, AtomicOutput::new(plain.to_str().unwrap()), std::slice::from_ref(&cert), false).unwrap();
        let (_, _) = decrypt_file(&plain, AtomicOutput::new(dec.to_str().unwrap()), helper(&cert, "s3cret")).unwrap();
        assert_eq!(fs::read(&dec).unwrap(), b"hello armor");

        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn unprotected_key_decrypts_even_if_passphrase_given() {
        let dir = std::env::temp_dir().join(format!("armor-test2-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let plain = dir.join("p");
        let enc = dir.join("p.gpg");
        let dec = dir.join("d");
        fs::write(&plain, b"x").unwrap();
        let (cert, _) = generate_keypair("Bob", "bob@example.org", "", None, "ed25519").unwrap();
        encrypt_file(&plain, AtomicOutput::new(enc.to_str().unwrap()), std::slice::from_ref(&cert), false).unwrap();
        decrypt_file(&enc, AtomicOutput::new(dec.to_str().unwrap()), helper(&cert, "typed-anyway")).unwrap();
        assert_eq!(fs::read(&dec).unwrap(), b"x");
        fs::remove_dir_all(&dir).ok();
    }

    /// Interoperability with GnuPG in both directions. Needs `gpg` on PATH, so
    /// it's opt-in: `cargo test --lib -- --ignored gpg_interop`
    #[test]
    #[ignore]
    fn gpg_interop() {
        use std::process::Command;
        let dir = std::env::temp_dir().join(format!("armor-gpg-{}", std::process::id()));
        let home = dir.join("gnupg");
        fs::create_dir_all(&home).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&home, fs::Permissions::from_mode(0o700)).unwrap();
        }
        let gpg = |args: &[&str]| {
            let out = Command::new("gpg")
                .env("GNUPGHOME", &home)
                .args(["--batch", "--yes", "--pinentry-mode", "loopback", "--trust-model", "always"])
                .args(args)
                .output()
                .expect("gpg not found");
            assert!(out.status.success(), "gpg {:?} failed: {}", args, String::from_utf8_lossy(&out.stderr));
            out.stdout
        };
        let plain = dir.join("plain.txt");
        fs::write(&plain, b"interop works").unwrap();

        // 1. GnuPG key + GnuPG ciphertext -> Armor imports the key and decrypts.
        gpg(&["--passphrase", "gpgpass", "--quick-gen-key", "Gee <gee@example.org>", "ed25519", "default", "never"]);
        let listing = String::from_utf8(gpg(&["--list-keys", "--with-colons", "gee@example.org"])).unwrap();
        let gee_fpr = listing.lines().find(|l| l.starts_with("fpr:")).unwrap().split(':').nth(9).unwrap();
        gpg(&["--quick-add-key", "--passphrase", "gpgpass", gee_fpr, "cv25519", "encr", "never"]);
        let secret = gpg(&["--passphrase", "gpgpass", "--armor", "--export-secret-keys", "gee@example.org"]);
        let gpg_cert = Cert::from_bytes(&secret).unwrap();
        let from_gpg = dir.join("from-gpg.gpg");
        let out1 = dir.join("out1.txt");

        // GnuPG >= 2.3 defaults to its non-standard OCB (LibrePGP) format for its
        // own keys. Sequoia doesn't implement it; we must at least explain why.
        gpg(&["--recipient", "gee@example.org", "--output", from_gpg.to_str().unwrap(), "--encrypt", plain.to_str().unwrap()]);
        let err = decrypt_file(&from_gpg, AtomicOutput::new(out1.to_str().unwrap()), helper(&gpg_cert, "gpgpass")).unwrap_err();
        assert!(explain_error(err).contains("setpref"));

        // Following the advice in that message switches GnuPG back to standard
        // OpenPGP, which Armor decrypts.
        let fpr = gpg_cert.fingerprint().to_hex();
        let mut child = Command::new("gpg")
            .env("GNUPGHOME", &home)
            .args(["--batch", "--yes", "--pinentry-mode", "loopback", "--passphrase", "gpgpass", "--command-fd", "0", "--edit-key", &fpr])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .unwrap();
        use std::io::Write;
        child.stdin.take().unwrap()
            .write_all(b"setpref AES256 AES192 AES SHA512 SHA384 SHA256 ZLIB BZIP2 ZIP Uncompressed\ny\nsave\n")
            .unwrap();
        assert!(child.wait().unwrap().success());
        gpg(&["--recipient", "gee@example.org", "--output", from_gpg.to_str().unwrap(), "--encrypt", plain.to_str().unwrap()]);
        decrypt_file(&from_gpg, AtomicOutput::new(out1.to_str().unwrap()), helper(&gpg_cert, "gpgpass")).unwrap();
        assert_eq!(fs::read(&out1).unwrap(), b"interop works");

        // 2. Armor key + Armor ciphertext -> GnuPG imports the key and decrypts.
        for kind in ["ed25519", "rsa2048"] {
            let (cert, armored) = generate_keypair("Ari", &format!("ari-{kind}@example.org"), "armorpass", None, kind).unwrap();
            assert_eq!(cert.primary_key().key().version(), 4, "keys must stay v4 for GnuPG");
            let key_file = dir.join(format!("ari-{kind}.asc"));
            fs::write(&key_file, armored).unwrap();
            gpg(&["--passphrase", "armorpass", "--import", key_file.to_str().unwrap()]);
            let from_armor = dir.join(format!("from-armor-{kind}.asc"));
            encrypt_file(&plain, AtomicOutput::new(from_armor.to_str().unwrap()), &[cert], true).unwrap();
            let out2 = gpg(&["--passphrase", "armorpass", "--decrypt", from_armor.to_str().unwrap()]);
            assert_eq!(out2, b"interop works", "gpg could not decrypt {kind}");
        }

        let _ = Command::new("gpgconf").env("GNUPGHOME", &home).args(["--kill", "all"]).status();
        fs::remove_dir_all(&dir).ok();
    }

    /// RUSTSEC-2025-0136: a crafted ECDH PKESK must produce an error, not a crash.
    #[test]
    fn malformed_ecdh_pkesk_is_rejected() {
        use openpgp::crypto::mpi::{Ciphertext, PublicKey};
        use openpgp::packet::{pkesk::PKESK3, PKESK};
        use openpgp::parse::stream::DecryptionHelper;
        use openpgp::types::PublicKeyAlgorithm;

        let (cert, _) = generate_keypair("Mal", "mal@example.org", "", None, "ed25519").unwrap();
        let p = StandardPolicy::new();
        let subkey = cert.keys().with_policy(&p, None).for_transport_encryption().next().unwrap();
        let q = match subkey.key().mpis() {
            PublicKey::ECDH { q, .. } => q.clone(),
            _ => unreachable!(),
        };
        let evil: PKESK = PKESK3::new(
            subkey.key().keyid(),
            PublicKeyAlgorithm::ECDH,
            Ciphertext::ECDH { e: q, key: Vec::new().into_boxed_slice() },
        )
        .unwrap()
        .into();

        // Without the guard, Sequoia 1.x panics (an abort in release builds).
        let mut keypair = cert.keys().secret().key_handle(subkey.key().keyid()).next().unwrap()
            .key().clone().into_keypair().unwrap();
        let raw = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| evil.decrypt(&mut keypair, None)));
        assert!(raw.is_err(), "upstream fixed the panic; the guard may no longer be needed");

        // Through Armor's helper it's just an error.
        let mut h = helper(&cert, "");
        let res = h.decrypt(&[evil], &[], None, |_, _| false);
        assert!(res.is_err());
        assert!(h.used.is_none());
    }
}
