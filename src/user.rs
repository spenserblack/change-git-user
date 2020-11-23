use console::style;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Serialize)]
pub struct User {
    pub name: String,
    pub email: String,
    pub signing_key: Option<String>,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} <{}>", self.name, self.email)?;
        match &self.signing_key {
            Some(signing_key) => write!(f, " ({})", style(signing_key).green())?,
            None => {}
        }

        Ok(())
    }
}

#[derive(Deserialize, Serialize)]
pub struct Users {
    #[serde(rename = "user")]
    users: Vec<User>,
}

impl Users {
    pub fn new(users: Vec<User>) -> Self {
        Users {
            users,
        }
    }

    pub fn push(&mut self, user: User) {
        self.users.push(user);
    }
}

impl Default for Users {
    fn default() -> Self {
        Users { users: Vec::with_capacity(1) }
    }
}
