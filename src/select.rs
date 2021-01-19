use super::{change_config, Users};
use anyhow::Result;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};

pub fn main(users: Users, term: Term, theme: ColorfulTheme) -> Result<()> {
    let keys: Vec<_> = users.keys().collect();

    let selection = Select::with_theme(&theme)
        .with_prompt("Select a git config:")
        .items(&keys)
        .interact_on(&term)?;

    let selection = &users[keys[selection]];

    change_config(&selection)
}
