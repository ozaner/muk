pub mod hash;
pub mod message;
pub mod util;

pub trait MukSubcommand {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>>;
}
