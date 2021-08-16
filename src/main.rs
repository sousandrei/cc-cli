mod commands;

use anyhow::Result;
use argh::FromArgs;
use commands::commit::check_rebase;
use std::process::exit;

#[derive(FromArgs, Debug)]
/// Easy peasy Conventional Commits
struct CcCli {
    /// hooks your commits
    #[argh(switch, short = 'i')]
    hook: bool,

    /// removes the hook
    #[argh(switch, short = 'r')]
    unhook: bool,

    /// positional args
    #[argh(positional)]
    positionals: Vec<String>,
}

fn main() -> Result<()> {
    let CcCli {
        hook,
        unhook,
        positionals,
    }: CcCli = argh::from_env();

    if unhook {
        commands::hook::uninstall()?;
        return Ok(());
    }

    if hook {
        commands::hook::install()?;
        return Ok(());
    }

    check_rebase()?;

    if positionals.len() > 1 {
        exit(0)
    }

    match positionals.is_empty() {
        true => commands::commit::as_cli()?,
        false => commands::commit::as_hook(&positionals[0])?,
    }

    Ok(())
}
