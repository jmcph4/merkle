use std::fs::read_to_string;

use clap::Parser;
use merkle::{Bytes, Digest};

use crate::{
    cli::{Commands, Opts},
    error::MerkleError,
    merkle::MerkleTree,
};

mod cli;
mod error;
mod merkle;

fn main() -> Result<(), MerkleError> {
    let opts: Opts = Opts::parse();

    match opts.command {
        Commands::Display { data_file } => {
            let tree: MerkleTree = MerkleTree::new(
                read_to_string(data_file)
                    .expect("I/O failed")
                    .lines()
                    .map(|line| line.bytes().collect::<Vec<u8>>())
                    .collect(),
            );

            MerkleTree::flatten(tree)
                .iter()
                .map(|x| x.root())
                .enumerate()
                .for_each(|(i, x)| println!("({}, {})", i, x));
        }
        Commands::Verify {
            data_file,
            leaf,
            proof_file,
        } => {
            let tree: MerkleTree = MerkleTree::new(
                read_to_string(data_file)
                    .expect("I/O failed")
                    .lines()
                    .map(|line| line.bytes().collect::<Vec<u8>>())
                    .collect(),
            );
            if tree.verify(
                leaf.into_bytes().into(),
                read_to_string(proof_file)
                    .expect("I/O failed")
                    .lines()
                    .map(|line| line.bytes().collect::<Bytes>())
                    .map(Digest::from)
                    .collect(),
            ) {
                println!("1");
            } else {
                println!("0");
            }
        }
    }

    Ok(())
}
