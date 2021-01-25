use crate::{write_users, Users};
use anyhow::Result;
use clap::ArgMatches;

pub fn main<'a>(mut users: Users, matches: &ArgMatches<'a>) -> Result<()> {
    let selections = matches.values_of("names").unwrap();

    for key in selections {
        users.remove(key);
    }

    write_users(&users)
}
