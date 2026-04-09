use std::path::PathBuf;

pub fn default_vault_dir_path() -> PathBuf {
    let mut path = dirs::data_local_dir().unwrap_or(PathBuf::from("."));
    path.push("shellsafe");
    path.push("vault.dat");
    path
}
