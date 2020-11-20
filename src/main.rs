use anyhow::Result;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use std::fmt;

fn main() -> Result<()> {
    let action_choices = [ActionChoice::Add, ActionChoice::Select];
    let term = Term::stderr();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What do you want to do?")
        .items(&action_choices)
        .default(0)
        .interact_on_opt(&term)?;

    let selection = match selection {
        Some(selection) => &action_choices[selection],
        None => return Ok(()),
    };

    println!("User selected: {}", selection);

    Ok(())
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
