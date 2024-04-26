use std::path::PathBuf;

use clap::{Args, ValueEnum};

use crate::{hash::Hash, util, MukSubcommand};

#[derive(Clone, Debug, ValueEnum)]
enum Subcommand {
    Get,
}

#[derive(Debug, Args)]
pub struct Message {
    cmd: Subcommand,

    /// Operate on only the message summary
    #[arg(short, long)]
    summary: bool,

    /// Operate on only the message description
    #[arg(short, long)]
    description: bool,

    #[arg(default_value = "head")]
    hash: Hash,
}

impl MukSubcommand for Message {
    fn run(&self, path: Option<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
        if self.summary && self.description {
            return Err("Cannot specify both --summary (-s) and --description (-d)".into());
        }
        match self.cmd {
            Subcommand::Get => Ok(println!(
                "{}",
                util::git_show_format(
                    if self.summary {
                        "%s"
                    } else if self.description {
                        "%b"
                    } else {
                        "%B"
                    },
                    &self.hash,
                    path
                )?
            )),
        }
    }
}
