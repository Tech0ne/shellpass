use argon2::{Argon2, PasswordHasher, password_hash::SaltString};

use crate::errors::{Error, Result};

pub fn derive_key(password: &str, salt_str: &str) -> Result<[u8; 32]> {
    let salt = SaltString::from_b64(salt_str)?;
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(password.as_bytes(), &salt)?;
    let hash_output = hash.hash.ok_or(Error::hash("No hash output"))?;
    let bytes = hash_output.as_bytes();
    if bytes.len() < 32 {
        return Err(Error::hash("Hash too short"));
    }
    let mut key = [0u8; 32];
    key.copy_from_slice(&bytes[..32]);
    Ok(key)
}
