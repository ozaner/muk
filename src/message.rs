use clap::{Args, ValueEnum};

use crate::{hash::Hash, util, MukSubcommand};

#[derive(Clone, Debug, ValueEnum)]
enum Subcommand {
    Get,
}

#[derive(Debug, Args)]
pub struct Message {
    cmd: Subcommand,

    #[arg(default_value = "head")]
    hash: Hash,
}

impl MukSubcommand for Message {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self.cmd {
            Subcommand::Get => Ok(println!("{}", util::git_show_format("%B", &self.hash)?)),
        }
    }
}
