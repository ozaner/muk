use std::{env, path::PathBuf};

use clap::{Args, ValueEnum};

use crate::{hash::Hash, util, MukSubcommand};

#[derive(Clone, Debug, ValueEnum)]
enum Subcommand {
    Add,
}

#[derive(Debug, Args)]
pub struct User {
    cmd: Subcommand,
    name: String,
    email: String,
}

impl MukSubcommand for Message {
    fn run(&self, path: &Option<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
        if self.summary && self.description {
            return Err("Cannot specify both --summary (-s) and --description (-d)".into());
        }
        match &self.cmd {
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
            Subcommand::Set => {
                let mut hash = &self.hash;
                if let Hash::Hash { value } = hash {
                    if util::get_root(path)?.starts_with(value) {
                        hash = &Hash::Root;
                    }
                }

                //make amend commit (interactive)
                util::do_git(path)?
                    .args(["commit", "--allow-empty", "--fixup"])
                    .arg(format!("amend:{}", hash.resolve_hash(path)?))
                    .output()?;

                //autosquash amend commit
                env::set_var("GIT_SEQUENCE_EDITOR", ":");
                util::do_git(path)?
                    .args(["rebase", "-i", "--autostash", "--autosquash"])
                    .arg(hash.to_arg_less_one())
                    .output()?;
                Ok(())
            }
        }
    }
}
