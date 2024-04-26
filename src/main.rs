use std::{path::PathBuf, process};

use clap::Parser;
use muk::{message, MukSubcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct RootCmd {
    #[arg(short = 'C', long)]
    path: Option<PathBuf>,

    #[command(subcommand)]
    command: Subcommand,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    /// Prints the message of the given commit hash
    #[clap(alias = "m")]
    Message(message::Message),
}

fn main() {
    let root = RootCmd::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    let res = match &root.command {
        Subcommand::Message(cmd) => cmd.run(root.path),
    };

    match res {
        Ok(()) => process::exit(0),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1)
        }
    }
}
