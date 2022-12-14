use anyhow::Result;

use clap::Parser;

use serde_json::Value;

use flate2::read::GzDecoder;

use std::{
    io::{Read, Write},
    process::{Command, Stdio},
};

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
                sync; \
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

            child
                .stdin
                .as_mut()
                .unwrap()
                .write_all(on_device.as_bytes())
                .unwrap();

            let output = child.wait_with_output()?;

            if !output.status.success() {
                panic!("No success!!!");
            }

            let raw_output = String::from_utf8(output.stdout).unwrap();

            // =============================================================
            //     Welcome to balenaOS
            // =============================================================
            let prelude = format!("{0}\n    Welcome to balenaOS\n{0}\n", "=".repeat(61));

            let (front, back) = raw_output.split_at(prelude.len());

            if prelude != front {
                panic!("Not matching!!!");
            }

            let file_io: Value = serde_json::from_str(back).unwrap();
            let link = file_io["link"].as_str().unwrap();
            println!("{}", link);

            let bytes = reqwest::blocking::get(link).unwrap().bytes().unwrap();

            let mut decoder = GzDecoder::new(&*bytes);
            let mut string = String::new();
            decoder.read_to_string(&mut string).unwrap();

            println!("{}", string);
        }
    }

    Ok(())
}
