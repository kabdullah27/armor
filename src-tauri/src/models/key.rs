use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMetadata {
    pub fingerprint: String,
    pub key_type: KeyType,
    pub user_id: UserId,
    pub created_at: String,
    pub expires_at: Option<String>,
    pub is_private: bool,
    pub is_favorite: bool,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum KeyType {
    Rsa2048,
    Rsa4096,
    Ed25519,
    Curve25519,
}

impl std::fmt::Display for KeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyType::Rsa2048 => write!(f, "RSA 2048"),
            KeyType::Rsa4096 => write!(f, "RSA 4096"),
            KeyType::Ed25519 => write!(f, "Ed25519"),
            KeyType::Curve25519 => write!(f, "Curve25519"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserId {
    pub name: String,
    pub email: String,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionResult {
    pub output_file: String,
    pub size: u64,
    pub recipients: Vec<String>,
    pub signed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecryptionResult {
    pub output_file: String,
    pub size: u64,
    pub decrypted_with: String,
    pub signatures: Vec<SignatureInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureInfo {
    pub signer: String,
    pub fingerprint: String,
    pub created_at: String,
    pub valid: bool,
}
