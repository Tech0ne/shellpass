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
        let mut buf = Vec::new();
        ciborium::into_writer(&encrypted, &mut buf)?;
        fs::write(path, buf).map_err(|e| Error::write(e))?;
        Ok(())
    }

    pub fn load(path: &PathBuf, password: &str) -> Result<Self> {
        let content: EncryptedVault = ciborium::from_reader(
            fs::read(path)
                .map_err(|e| Error::read(e))?
                .iter()
                .as_slice(),
        )?;
        content.into_vault_data(password)
    }
}
