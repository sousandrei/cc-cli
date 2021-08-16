use anyhow::{Error, Result};
use inquire::Confirm;
use std::{
    fs::{read_to_string, remove_file, set_permissions, File, Permissions},
    io::Write,
    os::unix::prelude::PermissionsExt,
    path::Path,
    process::{Command, Output},
};

const PERMISSIONS: u32 = 0o775;
const FILENAME: &str = "prepare-commit-msg";
const CONTENTS: &str = "#!/bin/sh
# cc-cli as a commit hook
exec < /dev/tty
cc-cli \"$@\"";

pub fn get_git_absolute_path() -> Result<String> {
    let Output {
        status,
        stdout,
        stderr,
    } = Command::new("git")
        .args(&["rev-parse", "--absolute-git-dir"])
        .output()
        .expect("Failed run git rev-parse");

    if !status.success() {
        println!("{}", String::from_utf8(stdout).unwrap());
        println!("{}", String::from_utf8(stderr).unwrap());

        let e = Error::msg(format!(
            "Failed to find hooks dir\ngit rev-parse failed with code {}",
            status.code().unwrap()
        ));

        return Err(e);
    }

    let mut path = String::from_utf8(stdout)?;

    path.pop();

    Ok(path)
}

fn get_hook_absolute_path() -> Result<String> {
    let mut path = get_git_absolute_path()?;

    path.push_str("/hooks/");
    path.push_str(FILENAME);

    Ok(path)
}

pub fn install() -> Result<()> {
    let hooks_path = get_hook_absolute_path()?;

    let mut file = File::create(hooks_path.clone())?;
    file.write_all(CONTENTS.as_bytes())?;

    set_permissions(hooks_path, Permissions::from_mode(PERMISSIONS))?;

    println!("Hook installed for this repo!");

    Ok(())
}

pub fn uninstall() -> Result<()> {
    let hook_path = get_hook_absolute_path()?;

    if !Path::new(&hook_path).exists() {
        let e = Error::msg("Hook is not installed in this repo");
        return Err(e);
    }

    let existing_hook = read_to_string(&hook_path)?;

    if existing_hook != CONTENTS {
        let continue_deletion =
            Confirm::new("Hook installed is different than cc-cli, do you want to proceed?")
                .with_default(false)
                .prompt()?;

        if !continue_deletion {
            println!("Cancelled");
            return Ok(());
        }
    }

    remove_file(hook_path)?;
    println!("Hook uninstalled for this repo!");

    Ok(())
}

pub fn check_installed() -> Result<()> {
    let hook_path = get_hook_absolute_path()?;

    if Path::new(&hook_path).exists() {
        let existing_hook = read_to_string(&hook_path)?;

        let e = match existing_hook == CONTENTS {
            true => Error::msg("Hook is already installed! Just run 'git commit' normaly"),
            false => Error::msg("There is an existing 'prepare-commit-msg' hook"),
        };

        return Err(e);
    }

    println!("{:#?}", Path::new(&hook_path));

    Ok(())
}
