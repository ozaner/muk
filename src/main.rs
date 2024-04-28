// muk message -s/-d/None get <hash>=HEAD
// muk message set <message>=open-editor

// muk committer get <hash>
// muk author set <hash> <user>=current
// muk committer set <hash> <user>=current

// muk cdate get <hash>
// muk cdate set <hash> <ISO 8601>=now
// muk adate sync <hash>     (can only synx adate to cdate, not vice-versa)

// muk user add <user>
// muk user rm <user>
// muk user list
// muk user switch <user>

// #[derive(Debug, Default, Clone, Parser)]
// pub struct HashRange {
//     start: Hash,
//     end: Option<Hash>,
// }

// impl ToString for HashRange {
//     fn to_string(&self) -> String {
//         let mut str = self.start.value;
//         if let Some(end) = self.end.as_ref() {
//             str.push_str(&format!("..{}", end.value));
//         }
//         str
//     }
// }

// impl FromStr for HashRange {
//     type Err = std::io::Error;

//     fn from_str(hash: &str) -> Result<Self, Self::Err> {
//         let hash = hash.to_lowercase();
//         if hash.contains("..") {
//             let (start, end) = hash.split_once("..").unwrap();
//             Ok(Self {
//                 start: Hash::from_str(start)?,
//                 end: Some(Hash::from_str(end)?),
//             })
//         } else {
//             Ok(Self {
//                 start: Hash::from_str(&hash)?,
//                 end: None,
//             })
//         }
//     }
// }

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
        Subcommand::Message(cmd) => cmd.run(&root.path),
    };

    match res {
        Ok(()) => process::exit(0),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1)
        }
    }
}
