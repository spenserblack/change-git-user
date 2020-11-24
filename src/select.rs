use super::{Users, change_config};
use anyhow::Result;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};

pub fn main(users: Users, term: Term, theme: ColorfulTheme) -> Result<()> {

    let selection = Select::with_theme(&theme)
        .with_prompt("Select a git config:")
        .items(&users)
        .interact_on(&term)?;

    let selection = &users[selection];

    change_config(&selection)
}