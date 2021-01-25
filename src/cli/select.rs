use crate::{change_config, Users};
use anyhow::{Context, Result};
use clap::ArgMatches;

pub fn main<'a>(users: Users, matches: &ArgMatches<'a>) -> Result<()> {
    let selection = matches.value_of("name").unwrap();

    let selection = users
        .get(selection)
        .context("Couldn't find a git config set matching provided name")?;

    change_config(&selection)
}
