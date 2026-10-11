// Cryptographic operations using Sequoia OpenPGP

use anyhow::Result;
use openpgp::cert::prelude::*;
use openpgp::crypto::Password;
use openpgp::serialize::stream::{Armorer, Message};
use openpgp::serialize::Serialize;
use sequoia_openpgp as openpgp;

/// Generates a new certificate. Returns the cert and its ASCII-armored TSK
/// (which also contains the public parts).
pub fn generate_keypair(
    name: &str,
    email: &str,
    passphrase: &str,
    valid_seconds: Option<u64>,
    key_type: &str,
) -> Result<(Cert, String)> {
    log::info!("generate_keypair started with key_type={}", key_type);

    let userid = format!("{} <{}>", name, email);

    let cipher_suite = match key_type {
        "rsa2048" => CipherSuite::RSA2k,
        "rsa4096" => CipherSuite::RSA4k,
        _ => CipherSuite::Cv25519, // Ed25519 / Curve25519 (default)
    };

    // The builder encrypts every secret key with the password itself, so a
    // failure aborts generation instead of silently storing an unprotected key.
    let password = (!passphrase.is_empty()).then(|| Password::from(passphrase));

    let mut builder = CertBuilder::new()
        .add_userid(userid.as_str())
        .set_cipher_suite(cipher_suite)
        .add_signing_subkey()
        .add_transport_encryption_subkey()
        .set_password(password);

    if let Some(seconds) = valid_seconds {
        builder = builder.set_validity_period(std::time::Duration::from_secs(seconds));
    }

    let (cert, _) = builder.generate()?;
    let private_key = armor_cert(&cert, true)?;
    Ok((cert, private_key))
}

/// Serializes a cert as ASCII armor, either the full TSK or just the public parts.
pub fn armor_cert(cert: &Cert, include_secret: bool) -> Result<String> {
    let mut bytes = Vec::new();
    {
        let message = Message::new(&mut bytes);
        let kind = if include_secret {
            openpgp::armor::Kind::SecretKey
        } else {
            openpgp::armor::Kind::PublicKey
        };
        let mut writer = Armorer::new(message).kind(kind).build()?;
        if include_secret {
            cert.as_tsk().serialize(&mut writer)?;
        } else {
            cert.serialize(&mut writer)?;
        }
        writer.finalize()?;
    }
    Ok(String::from_utf8(bytes)?)
}
