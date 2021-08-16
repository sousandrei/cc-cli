use anyhow::{Error, Result};
use std::{
    fs::write,
    path::Path,
    process::{exit, Command, Output},
};

use super::{
    hook::{check_installed, get_git_absolute_path},
    prompt,
};

pub fn as_hook(commit_msg_file: &str) -> Result<()> {
    let commit_message = prompt()?;

    write(commit_msg_file, commit_message)?;
    Ok(())
}

pub fn as_cli() -> Result<()> {
    check_installed()?;

    let commit_message = prompt()?;

    let Output {
        status,
        stdout,
        stderr,
    } = Command::new("git")
        .args(&["commit", "-m", &commit_message])
        .output()
        .expect("Failed run git commit");

    if !status.success() {
        println!("{}", String::from_utf8(stdout).unwrap());
        println!("{}", String::from_utf8(stderr).unwrap());

        let e = Error::msg(format!(
            "git commit failed with code {}",
            status.code().unwrap()
        ));

        return Err(e);
    }

    Ok(())
}

pub fn check_rebase() -> Result<()> {
    let path = get_git_absolute_path()?;

    if Path::new(&(path.clone() + "/rebase-merge")).exists()
        || Path::new(&(path + "/rebase-apply")).exists()
    {
        exit(0)
    }

    Ok(())
}
