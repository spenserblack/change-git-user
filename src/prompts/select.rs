use crate::{change_config, Users};
use anyhow::{Context, Result};
use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};

pub fn main(users: Users, term: Term, theme: ColorfulTheme) -> Result<()> {
    let keys: Vec<_> = users.keys().collect();

    let selection = Select::with_theme(&theme)
        .with_prompt("Select a git config:")
        .items(&keys)
        .interact_on_opt(&term)?;

    let selection = selection.context("User exited without making a selection")?;

    let selection = &users[keys[selection]];

    change_config(&selection)
}
