use anyhow::Result;

use clap::Parser;

use std::{process::{Command, Stdio}, io::Write};

mod cli;

use crate::cli::{Cli, Subcommand};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.subcommand {
        Subcommand::Get { uuid } => {
            println!("Get logs from {}", uuid);

            let on_device = "journalctl -o json | gzip > /mnt/data/logs.gz; \
                curl -s -F 'file=@/mnt/data/logs.gz' https://file.io; \
                rm /mnt/data/logs.gz; \
                exit; \
                \n";

            let mut child = Command::new("balena")
                .arg("ssh")
                .arg(uuid)
                .stdin(Stdio::piped())
                .stderr(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();
            
            child.stdin.as_mut().unwrap().write_all(on_device.as_bytes()).unwrap();


            let output = child.wait_with_output()?;

            if output.status.success() {
                let raw_output = String::from_utf8(output.stdout).unwrap();
                println!("{}", raw_output);
            }
        }
    }

    Ok(())
}
