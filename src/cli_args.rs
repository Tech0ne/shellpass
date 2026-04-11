use std::path::PathBuf;

use clap::Parser;

/// shellpass - tui based password manager
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Directory where vault.dat is stored (default to data dir)
    #[arg(short, long, value_name = "DIR")]
    pub vault_dir: Option<PathBuf>,
}
