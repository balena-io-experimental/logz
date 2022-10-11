use anyhow::Result;

use clap::Parser;

mod cli;

use crate::cli::{Cli, Command};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Copy { uuid } => {
            println!("Copy logs from {}", uuid);
        }
    }

    Ok(())
}

// echo '
//   journalctl -o json | gzip > /mnt/data/logs.gz;
//   curl -s -F "file=@/mnt/data/logs.gz" https://file.io;
//   rm /mnt/data/logs.gz; exit;
// ' | balena ssh d4c9510a7e6026eb31eed6340850fd33  | tail -n +4
//
