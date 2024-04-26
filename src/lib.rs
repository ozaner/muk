use std::path::PathBuf;

pub mod hash;
pub mod message;
pub mod util;

pub trait MukSubcommand {
    fn run(&self, dir: Option<PathBuf>) -> Result<(), Box<dyn std::error::Error>>;
}
