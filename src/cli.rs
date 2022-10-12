use clap::Parser;

#[derive(Debug, Parser)]
#[clap(name = "logz")]
#[clap(version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: Subcommand,

    /// Print debug information
    #[clap(short, long)]
    pub debug: bool,
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
    /// Copy logs from device
    Get {
        #[clap(value_parser)]
        uuid: String,
    },
}
