use std::{path::PathBuf, str::FromStr};

use clap::Parser;

use crate::util::do_git;

#[derive(Debug, Clone, Parser)]
pub enum Hash {
    Head,
    Root,
    Hash { value: String },
}

impl Hash {
    pub fn resolve_hash(&self, path: &Option<PathBuf>) -> Result<String, std::io::Error> {
        match self {
            Hash::Head => Self::get_head(path),
            Hash::Root => Self::get_root(path),
            Hash::Hash { value } => Ok(value.clone()),
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

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let value = value.to_lowercase();
        match value.as_str() {
            "head" => Ok(Self::Head),
            "root" => Ok(Self::Root),
            _ => {
                assert!(value.len() >= 7, "hashes must be at least 7 digits long");
                Ok(Self::Hash { value })
            }
        }
    }
}
