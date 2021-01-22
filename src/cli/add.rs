use crate::{write_users, User, Users};
use anyhow::Result;
use clap::ArgMatches;
use console::style;

pub fn main<'a>(mut users: Users, matches: &ArgMatches<'a>) -> Result<()> {
    let name = matches.value_of("user.name").unwrap();
    let email = matches.value_of("user.email").unwrap();
    let signing_key = matches.value_of("user.signingkey").map(String::from);
    let mut alias: String = matches.value_of("name").unwrap_or(name).into();

    let user = User {
        name: name.into(),
        email: email.into(),
        signing_key,
    };

    if users.contains_key(&alias) {
        eprintln!(
            "{log} config with name {name:?} already exists",
            log = style("[WARNING]").yellow().bold(),
            name = alias,
        );
        let mut backup_counter = 1;
        alias = loop {
            let backup_name = format!("{} ({})", alias, backup_counter);
            if !users.contains_key(&backup_name) {
                break backup_name;
            }
            backup_counter += 1;
        };
        println!("Saving as {:?}", alias);
    }
    users.insert(alias, user);

    write_users(&users)
}
