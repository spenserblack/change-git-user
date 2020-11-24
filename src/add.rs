use super::{User, Users};
use anyhow::Result;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Input};

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

    users.push(user);

    super::write_users(&users)
}
