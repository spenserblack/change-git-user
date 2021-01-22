use crate::Users;
use anyhow::{Context, Result};
use clap::ArgMatches;

pub fn main<'a>(users: Users, matches: &ArgMatches<'a>) -> Result<()> {
    let name = matches.value_of("name").unwrap();

    let user = users
        .get(name)
        .context("Couldn't find a git config set matching provided name")?;

    println!("{name}: {user}", name = name, user = user);

    Ok(())
}
