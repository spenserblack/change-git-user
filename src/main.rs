use anyhow::Result;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use std::fmt;
pub use user::User;

fn main() -> Result<()> {
    let action_choices = [ActionChoice::Add, ActionChoice::Select];
    let term = Term::stderr();
    let theme = ColorfulTheme::default();

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
        ActionChoice::Add => add::main(term, theme),
        _ => unimplemented!(),
    }
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
mod user;
