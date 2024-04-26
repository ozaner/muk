use std::{process::Command, str::FromStr};

use clap::Parser;

#[derive(Debug, Clone, Parser)]
pub struct Hash {
    #[clap(default_value = "head")]
    value: String,
}

impl Hash {
    pub fn get_root() -> Result<Self, std::io::Error> {
        let output = Command::new("git")
            .arg("rev-list")
            .arg("--all")
            .arg("--max-parents=0")
            .arg("head")
            .output()?;
        let hash = String::from_utf8(output.stdout)
            .unwrap()
            .trim_end()
            .to_string();
        Ok(Self { value: hash })
    }
}

impl AsRef<str> for Hash {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

impl FromStr for Hash {
    type Err = std::io::Error;

    fn from_str(hash: &str) -> Result<Self, Self::Err> {
        let hash = hash.to_lowercase();
        match hash.as_str() {
            "head" => Ok(Self { value: hash }),
            "root" => Ok(Self::get_root()?),
            _ => {
                assert!(hash.len() >= 7, "hashes must be at least 7 digits long");
                Ok(Self { value: hash })
            }
        }
    }
}
