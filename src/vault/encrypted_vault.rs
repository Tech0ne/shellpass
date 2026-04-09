use aes_gcm::{
    AeadCore, Aes256Gcm, Key, KeyInit, Nonce,
    aead::{Aead, OsRng},
};
use argon2::password_hash::SaltString;
use base64::{Engine, engine::general_purpose::STANDARD as BASE64};
use serde::{Deserialize, Serialize};

use crate::{
    errors::Result,
    vault::{crypto::derive_key, vault_data::VaultData},
};

const VERSION: u8 = 1;

#[derive(Serialize, Deserialize, Debug)]
pub struct EncryptedVault {
    version: u8,
    salt: String,
    nonce: String,
    ciphertext: String,
}

impl EncryptedVault {
    pub fn from_vault_data(data: &VaultData, password: &str) -> Result<Self> {
        let json = serde_json::to_string(data)?;
        let salt = SaltString::generate(&mut OsRng);
        let salt_str = salt.as_str().to_string();
        let key_bytes = derive_key(password, &salt_str)?;
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let ciphertext = cipher.encrypt(&nonce, json.as_bytes())?;
        Ok(Self {
            version: VERSION,
            salt: salt_str,
            nonce: BASE64.encode(nonce),
            ciphertext: BASE64.encode(ciphertext),
        })
    }

    pub fn into_vault_data(&self, password: &str) -> Result<VaultData> {
        let key_bytes = derive_key(password, &self.salt)?;
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);
        let nonce_bytes = BASE64.decode(&self.nonce)?;
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = BASE64.decode(&self.ciphertext)?;
        let plaintext = cipher.decrypt(nonce, ciphertext.as_slice())?;
        Ok(serde_json::from_slice(&plaintext)?)
    }
}
