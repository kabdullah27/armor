// Cryptographic operations using Sequoia OpenPGP

use anyhow::Result;
use openpgp::cert::prelude::*;
use openpgp::crypto::Password;
use openpgp::policy::StandardPolicy;
use openpgp::serialize::stream::{Armorer, Message};
use openpgp::serialize::Serialize; // Added Serialize trait
use sequoia_openpgp as openpgp;

pub fn generate_keypair(
    name: &str,
    email: &str,
    passphrase: &str,
    valid_seconds: Option<u64>,
) -> Result<(String, String)> {
    println!("DEBUG: crypto::generate_keypair started");
    let _p = StandardPolicy::new();

    let userid = format!("{} <{}>", name, email);

    // 1. Generate a CertBuilder with reasonable defaults
    let mut builder = CertBuilder::new()
        .add_userid(userid.as_str())
        .add_signing_subkey()
        .add_transport_encryption_subkey();

    if let Some(seconds) = valid_seconds {
        // use std::time::Duration;
        builder = builder.set_validity_period(std::time::Duration::from_secs(seconds));
    }

    let (mut cert, _) = builder.generate()?;

    // 2. Protect the secret keys with the passphrase
    // Only encrypt if a passphrase is provided
    if !passphrase.is_empty() {
        let password = Password::from(passphrase);
        let packets: Vec<openpgp::packet::Packet> = cert.into();

        // We need to map over packets because encrypt_secret consumes the key
        let packets: Vec<openpgp::packet::Packet> = packets
            .into_iter()
            .map(|packet| match packet {
                openpgp::packet::Packet::SecretKey(key) => {
                    match key.clone().encrypt_secret(&password) {
                        Ok(encrypted_key) => openpgp::packet::Packet::SecretKey(encrypted_key),
                        Err(_) => openpgp::packet::Packet::SecretKey(key),
                    }
                }
                openpgp::packet::Packet::SecretSubkey(key) => {
                    match key.clone().encrypt_secret(&password) {
                        Ok(encrypted_key) => openpgp::packet::Packet::SecretSubkey(encrypted_key),
                        Err(_) => openpgp::packet::Packet::SecretSubkey(key),
                    }
                }
                p => p,
            })
            .collect();

        // Reassemble the Cert
        cert = Cert::try_from(packets)?;
    }

    // 3. Export Public Key (Ascii Armored)
    let mut public_bytes = Vec::new();
    {
        let message = Message::new(&mut public_bytes);
        let mut armor_writer = Armorer::new(message)
            .kind(openpgp::armor::Kind::PublicKey)
            .build()?;
        cert.serialize(&mut armor_writer)?;
        armor_writer.finalize()?;
    }
    let public_key = String::from_utf8(public_bytes)?;

    // 4. Export Private Key (Ascii Armored)
    let mut private_bytes = Vec::new();
    {
        let message = Message::new(&mut private_bytes);
        let mut armor_writer = Armorer::new(message)
            .kind(openpgp::armor::Kind::SecretKey)
            .build()?;
        cert.as_tsk().serialize(&mut armor_writer)?;
        armor_writer.finalize()?;
    }
    let private_key = String::from_utf8(private_bytes)?;

    Ok((public_key, private_key))
}
