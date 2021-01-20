use super::{change_config, User, Users};
use anyhow::Result;
use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Confirm, Input};

pub fn main(mut users: Users, term: Term, theme: ColorfulTheme) -> Result<()> {
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

    let alias: String = Input::with_theme(&theme)
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
        .interact_on(&term)?;

    if switch_user {
        change_config(&user)?;
    }

    if let Some(user) = users.insert(alias, user) {
        eprintln!(
            "{log} config {config} overwritten",
            log = style("[WARNING]").yellow().bold(),
            config = user,
        )
    }

    super::write_users(&users)
}
