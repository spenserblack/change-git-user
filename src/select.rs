use super::User;
use anyhow::Result;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};

pub fn main(term: Term, theme: ColorfulTheme) -> Result<()> {
    let fake_users = [
        User { name: "Foo Bar".into(), email: "foobar@email.com".into(), signing_key: None },
        User { name: "Git User".into(), email: "gituser@email.com".into(), signing_key: Some("ASDFASF234234AFSADF".into()) },
    ];

    let selection = Select::with_theme(&theme)
        .with_prompt("Select a git config:")
        .items(&fake_users)
        .interact_on(&term)?;

    let selection = &fake_users[selection];

    println!("User selected {}", selection);

    Ok(())
}
