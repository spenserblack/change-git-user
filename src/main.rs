use anyhow::{Context, Result};
pub use config::change_config;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use std::fmt;
pub use user::{User, Users};

const USERS_FILENAME: &str = "gcu-users.toml";

fn main() -> Result<()> {
    let term = Term::stderr();
    let theme = ColorfulTheme::default();

    let users = read_users();

    let action_choices = if users.is_some() {
        vec![ActionChoice::Add, ActionChoice::Select]
    } else {
        vec![ActionChoice::Add]
    };

    let users = match users {
        Some(Ok(users)) => users,
        Some(Err(e)) => return Err(e),
        None => Users::default(),
    };

    let selection = Select::with_theme(&theme)
        .with_prompt("What do you want to do?")
        .items(&action_choices)
        .default(0)
        .interact_on_opt(&term)?;

    let selection = match selection {
        Some(selection) => &action_choices[selection],
        None => return Ok(()),
    };

    match selection {
        ActionChoice::Add => add::main(users, term, theme),
        _ => select::main(users, term, theme),
    }
}

fn read_users() -> Option<Result<Users>> {
    use std::fs;

    let bytes = fs::read(USERS_FILENAME).ok()?;
    let users: Result<Users> = toml::from_slice(&bytes).context("Failed to parse users");
    Some(users)
}

fn write_users(users: &Users) -> Result<()> {
    use std::fs;

    let users = toml::to_string(users).context("Failed to write users to TOML")?;

    fs::write(USERS_FILENAME, users).context("Failed to write users to file")
}

enum ActionChoice {
    Add,
    Select,
}

impl fmt::Display for ActionChoice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ActionChoice::*;

        match self {
            Add => write!(f, "Add a new user config"),
            Select => write!(f, "Switch user"),
        }
    }
}

mod add;
mod config;
mod select;
mod user;
