use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "logz")]
#[clap(version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,

    /// Print debug information
    #[clap(short, long)]
    pub debug: bool,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Copy logs from device
    Copy {
        #[clap(value_parser)]
        uuid: String,
    },
}
