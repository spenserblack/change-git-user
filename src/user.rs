use console::style;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Serialize)]
pub struct User {
    pub name: String,
    pub email: String,
    #[serde(rename = "signing-key")]
    pub signing_key: Option<String>,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} <{}>",
            style(&self.name).bold(),
            style(&self.email).underlined()
        )?;
        match &self.signing_key {
            Some(signing_key) => write!(f, " {}", style(signing_key).green().italic())?,
            None => {}
        }

        Ok(())
    }
}

pub type Users = IndexMap<String, User>;
