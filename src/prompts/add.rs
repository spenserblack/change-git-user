use crate::{change_config, write_users, User, Users};
use anyhow::Result;
use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use std::path::Path;

pub fn main<P: AsRef<Path>>(
    mut users: Users,
    term: Term,
    theme: ColorfulTheme,
    path: P,
) -> Result<()> {
    let name: String = Input::with_theme(&theme)
        .with_prompt("user.name")
        .interact_text_on(&term)?;

    let email: String = Input::with_theme(&theme)
        .with_prompt("user.email")
        .validate_with(|input: &String| {
            if input.contains('@') {
                Ok(())
            } else {
                Err("Must be an email address")
            }
        })
        .interact_text_on(&term)?;

    let signing_key: String = Input::with_theme(&theme)
        .with_prompt("(Optional) user.signingKey")
        .allow_empty(true)
        .validate_with(|input: &String| {
            if input.contains(' ') {
                Err("Invalid signing key format")
            } else {
                Ok(())
            }
        })
        .interact_text_on(&term)?;

    let mut alias: String = Input::with_theme(&theme)
        .with_prompt("Give this config a name")
        .with_initial_text(&name)
        .interact_text_on(&term)?;

    let signing_key = if signing_key.is_empty() {
        None
    } else {
        Some(signing_key)
    };

    let user = User {
        name,
        email,
        signing_key,
    };

    let switch_user = Confirm::with_theme(&theme)
        .with_prompt("Switch to this user?")
        .default(true)
        .interact_on_opt(&term)?;

    let switch_user = match switch_user {
        Some(b) => b,
        None => return Ok(()),
    };

    if switch_user {
        change_config(&user)?;
    }

    if users.contains_key(&alias) {
        eprintln!(
            "{log} config with name {name:?} already exists",
            log = style("[WARNING]").yellow().bold(),
            name = alias,
        );
        let mut backup_counter = 1;
        alias = loop {
            let backup_name = format!("{} ({})", alias, backup_counter);
            if !users.contains_key(&backup_name) {
                break backup_name;
            }
            backup_counter += 1;
        };
        println!("Saving as {:?}", alias);
    }
    users.insert(alias, user);

    write_users(&users, path)
}
