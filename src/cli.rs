use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Clone, Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Clone, Debug, Subcommand)]
pub enum Commands {
    /// Displays the Merkle tree from provided dataset
    Display { data_file: PathBuf },
    /// Verifies a membership proof against the given Merkle tree
    Verify {
        data_file: PathBuf,
        leaf: String,
        proof_file: PathBuf,
    },
}
