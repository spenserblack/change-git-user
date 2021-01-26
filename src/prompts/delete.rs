use crate::{write_users, Users};
use anyhow::Result;
use console::Term;
use dialoguer::{theme::ColorfulTheme, MultiSelect};
use std::path::Path;

pub fn main<P: AsRef<Path>>(
    mut users: Users,
    term: Term,
    theme: ColorfulTheme,
    path: P,
) -> Result<()> {
    let keys: Vec<_> = users.keys().map(String::from).collect();

    let selections = MultiSelect::with_theme(&theme)
        .with_prompt("Select git config(s) to delete:")
        .items(&keys)
        .interact_on(&term)?;

    for key in selections.into_iter().map(|i| &keys[i]) {
        users.remove(key);
    }

    write_users(&users, path)
}
