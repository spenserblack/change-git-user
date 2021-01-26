use crate::{write_users, Users};
use anyhow::Result;
use clap::ArgMatches;
use std::path::Path;

pub fn main<'a, P: AsRef<Path>>(mut users: Users, matches: &ArgMatches<'a>, path: P) -> Result<()> {
    let selections = matches.values_of("names").unwrap();

    for key in selections {
        users.remove(key);
    }

    write_users(&users, path)
}
