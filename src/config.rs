use super::User;
use anyhow::{Context, Result};
use git2::Repository;

const SIGNING_KEY: &str = "user.signingKey";

pub fn change_config(user: &User) -> Result<()> {
    let repo = Repository::discover(".").context("Couldn't find a repository")?;
    let mut config = repo
        .config()
        .context("Couldn't get repository configuration")?;

    config
        .set_str("user.name", &user.name)
        .context("Couldn't set user.name")?;
    config
        .set_str("user.email", &user.email)
        .context("Couldn't set user.email")?;

    if let Some(signing_key) = &user.signing_key {
        config
            .set_str(SIGNING_KEY, &signing_key)
            .context("Couldn't set user.signingKey")?;
    } else {
        config
            .set_str(SIGNING_KEY, "")
            .context("Couldn't set user.signingKey to empty value")?;
    }

    Ok(())
}
