use anyhow::{Context, Result};
pub use config::change_config;
#[cfg(feature = "prompts")]
use console::Term;
#[cfg(feature = "prompts")]
use dialoguer::{theme::ColorfulTheme, Select};
use std::path::{Path, PathBuf};
pub use user::{User, Users};

const DATA_FILENAME: &str = "change-git-user.users.toml";

#[cfg(any(feature = "cli", feature = "prompts"))]
fn main() -> Result<()> {
    let data_dir = dirs::data_local_dir()
        .map(|p: PathBuf| p.join(DATA_FILENAME))
        .unwrap();
    let data_dir = data_dir.as_os_str();

    #[cfg(feature = "cli")]
    let cli = cli::Cli::new(data_dir);

    #[cfg(feature = "cli")]
    let data_filepath = cli.data_filepath();

    #[cfg(all(not(feature = "cli"), feature = "prompts"))]
    let data_filepath: Result<_, anyhow::Error> = Ok(data_dir);

    let users = read_users(data_filepath.context("Couldn't read user data")?);

    let users = match users {
        Some(Ok(users)) => users,
        Some(Err(e)) => return Err(e),
        None => Users::default(),
    };

    #[cfg(feature = "cli")]
    if cli.used() {
        return cli.main(users);
    }

    #[cfg(all(feature = "cli", not(feature = "prompts")))]
    {
        println!("{}", cli.usage());
        println!(concat!(
            "Use ",
            env!("CARGO_BIN_NAME"),
            " --help for more details"
        ));
        Ok(())
    }

    #[cfg(all(feature = "cli", feature = "prompts"))]
    let data_filepath = cli.data_filepath();

    #[cfg(all(not(feature = "cli"), feature = "prompts"))]
    let data_filepath = Ok(data_dir);

    #[cfg(feature = "prompts")]
    run_prompts(users, data_filepath)
}

#[cfg(feature = "prompts")]
fn run_prompts<P: AsRef<Path>>(users: Users, data_filepath: Result<P>) -> Result<()> {
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

#[cfg(not(any(feature = "cli", feature = "prompts")))]
fn main() {
    compile_error!(r#"Cannot function when neither "cli" nor "prompts" features are enabled"#);
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

#[cfg(feature = "prompts")]
enum ActionChoice {
    Add,
    Select,
    Delete,
}

#[cfg(feature = "prompts")]
impl std::fmt::Display for ActionChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use ActionChoice::*;

        match self {
            Add => write!(f, "Add a new user config"),
            Select => write!(f, "Switch user"),
            Delete => write!(f, "Remove user config(s)"),
        }
    }
}

#[cfg(feature = "cli")]
mod cli;
mod config;
#[cfg(feature = "prompts")]
mod prompts;
mod user;
