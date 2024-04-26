use std::{path::PathBuf, str::FromStr};

use clap::Parser;

use crate::util::do_git;

#[derive(Debug, Clone, Parser)]
pub struct Hash {
    #[clap(default_value = "head")]
    value: String,
}

impl Hash {
    pub fn resolve_hash(&self, path: &Option<PathBuf>) -> Result<String, std::io::Error> {
        match self.value.as_str() {
            "head" => Self::get_head(path),
            "root" => Self::get_root(path),
            _ => Ok(self.value.clone()),
        }
    }

    fn get_head(path: &Option<PathBuf>) -> Result<String, std::io::Error> {
        let output = do_git(path)?.arg("rev-parse").arg("head").output()?;
        let hash = String::from_utf8(output.stdout)
            .unwrap()
            .trim_end()
            .to_string();
        Ok(hash)
    }

    fn get_root(path: &Option<PathBuf>) -> Result<String, std::io::Error> {
        let output = do_git(path)?
            .arg("rev-list")
            .arg("--all")
            .arg("--max-parents=0")
            .arg("head")
            .output()?;
        let hash = String::from_utf8(output.stdout)
            .unwrap()
            .trim_end()
            .to_string();
        Ok(hash)
    }
}


impl FromStr for Hash {
    type Err = std::io::Error;

    fn from_str(hash: &str) -> Result<Self, Self::Err> {
        let hash = hash.to_lowercase();
        match hash.as_str() {
            "head" | "root" => Ok(Self { value: hash }),
            _ => {
                assert!(hash.len() >= 7, "hashes must be at least 7 digits long");
                Ok(Self { value: hash })
            }
        }
    }
}
