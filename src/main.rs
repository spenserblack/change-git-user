use anyhow::{Context, Result};
pub use config::change_config;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use std::{
    fmt,
    path::{Path, PathBuf},
};
pub use user::{User, Users};

const DATA_FILENAME: &str = "change-git-user.users.toml";

fn main() -> Result<()> {
    let data_dir = dirs::data_local_dir()
        .map(|p: PathBuf| p.join(DATA_FILENAME))
        .unwrap();
    let data_dir = data_dir.as_os_str();

    let cli = cli::Cli::new(data_dir);

    let users = read_users(cli.data_filepath().context("Couldn't read user data")?);

    let users = match users {
        Some(Ok(users)) => users,
        Some(Err(e)) => return Err(e),
        None => Users::default(),
    };

    if cli.used() {
        return cli.main(users);
    }

    let action_choices = if !users.is_empty() {
        vec![
            ActionChoice::Add,
            ActionChoice::Select,
            ActionChoice::Delete,
        ]
    } else {
        vec![ActionChoice::Add]
    };

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

    let data_filepath = cli.data_filepath();
    match selection {
        ActionChoice::Add => prompts::add::main(
            users,
            term,
            theme,
            data_filepath.context("Couldn't start user add prompt")?,
        ),
        ActionChoice::Select => prompts::select::main(users, term, theme),
        ActionChoice::Delete => prompts::delete::main(
            users,
            term,
            theme,
            data_filepath.context("Couldn't start user delete prompt")?,
        ),
    }
}

fn read_users<P: AsRef<Path>>(path: P) -> Option<Result<Users>> {
    use std::fs;

    let bytes = fs::read(path).ok()?;
    let users: Result<Users> = toml::from_slice(&bytes).context("Failed to parse users");
    Some(users)
}

fn write_users<P: AsRef<Path>>(users: &Users, path: P) -> Result<()> {
    use std::fs;

    let users = toml::to_string(users).context("Failed to write users to TOML")?;

    fs::write(path, users).context("Failed to write users to file")
}

enum ActionChoice {
    Add,
    Select,
    Delete,
}

impl fmt::Display for ActionChoice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ActionChoice::*;

        match self {
            Add => write!(f, "Add a new user config"),
            Select => write!(f, "Switch user"),
            Delete => write!(f, "Remove user config(s)"),
        }
    }
}

mod cli;
mod config;
mod prompts;
mod user;
