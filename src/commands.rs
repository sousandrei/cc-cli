use anyhow::Result;
use inquire::{max_length, min_length, Confirm, Select, Text};

const ITEMS: [&str; 10] = [
    "build", "chore", "ci", "docs", "feat", "fix", "perf", "refactor", "style", "test",
];

pub mod commit;
pub mod hook;

pub fn prompt() -> Result<String> {
    let commit_type = Select::new("What is the commit type?", ITEMS.to_vec()).prompt()?;

    let mut scope = Text::new("What is the commit scope?")
        .with_validator(max_length!(15, "Maximum of 15 characters"))
        .prompt()?;

    let breaking_change = Confirm::new("Does it contain a breaking change?")
        .with_default(false)
        .prompt()?;

    let message = Text::new("Commit message?")
        .with_validator(min_length!(3, "Minimum of 5 characters"))
        .prompt()?;

    if !scope.is_empty() {
        scope = format!("({scope})");
    }

    let is_breaking = match breaking_change {
        true => "!",
        false => "",
    };

    let commit_message = format!("{commit_type}{scope}{is_breaking}: {message}");

    Ok(commit_message)
}
