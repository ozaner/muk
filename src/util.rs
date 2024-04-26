use std::{path::PathBuf, process::Command};

use crate::hash::Hash;

pub fn do_git(path: &Option<PathBuf>) -> Result<Command, std::io::Error> {
    let mut cmd = Command::new("git");
    if let Some(path) = path {
        cmd.args([
            "-C",
            path.to_str().ok_or(std::io::ErrorKind::Other)?.as_ref(),
        ]);
    }
    Ok(cmd)
}

pub fn git_show_format(
    f: &str,
    hash: &Hash,
    path: &Option<PathBuf>,
) -> Result<String, Box<dyn std::error::Error>> {
    let output = do_git(path)?
        .args(["show", "-s"])
        .arg(format!("--pretty=format:{f}"))
        .arg(hash.clone().resolve_hash(path)?)
        .output()?;
    let text = String::from_utf8(output.stdout)
        .unwrap()
        .trim_end()
        .to_string();
    Ok(text)
}
