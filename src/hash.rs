use std::{path::PathBuf, str::FromStr};

use clap::Parser;

use crate::util;

#[derive(Debug, Clone, Parser)]
pub enum Hash {
    Head,
    Root,
    Hash { value: String },
}

impl Hash {
    pub fn resolve_hash(&self, path: &Option<PathBuf>) -> Result<String, std::io::Error> {
        match self {
            Hash::Head => util::get_head(path),
            Hash::Root => util::get_root(path),
            Hash::Hash { value } => Ok(value.clone()),
        }
    }

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
