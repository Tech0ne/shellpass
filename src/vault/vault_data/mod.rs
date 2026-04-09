pub mod profile;

use std::{fs, path::PathBuf};

use profile::Profile;
use serde::{Deserialize, Serialize};

use crate::{
    errors::{Error, Result},
    vault::encrypted_vault::EncryptedVault,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VaultData {
    pub profiles: Vec<Profile>,
}

impl VaultData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn save(&self, path: &PathBuf, password: &str) -> Result<()> {
        let encrypted = EncryptedVault::from_vault_data(&self, password)?;
        fs::write(path, serde_json::to_string(&encrypted)?).map_err(|e| Error::write(e))?;
        Ok(())
    }

    pub fn load(path: &PathBuf, password: &str) -> Result<Self> {
        let content: EncryptedVault = serde_json::from_slice(
            fs::read_to_string(path)
                .map_err(|e| Error::read(e))?
                .as_bytes(),
        )?;
        content.into_vault_data(password)
    }
}
