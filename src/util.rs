use std::process::Command;

use crate::hash::Hash;

pub fn git_show_format(f: &str, hash: &Hash) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .arg("show")
        .arg("-s")
        .arg(format!("--pretty=format:{f}"))
        .arg(hash.clone().as_ref())
        .output()?;
    let text = String::from_utf8(output.stdout)
        .unwrap()
        .trim_end()
        .to_string();
    Ok(text)
}
