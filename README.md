# `cgu`

[![Crates.io](https://img.shields.io/crates/v/change-git-user)][Crates.io Link]
![Crates.io](https://img.shields.io/crates/d/change-git-user)
![CI](https://github.com/spenserblack/change-git-user/workflows/CI/badge.svg)

Manage multiple git configurations

Inspired by [Git-User-Switch](https://github.com/geongeorge/Git-User-Switch).

## :warning: Archival Notice

I've switched to just using [`includeIf`](https://git-scm.com/docs/git-config#Documentation/git-config.txt-includeIfltconditiongtpath)
in my `.gitconfig`. Honestly, this works a lot better for me than having to constantly switch users via this command.

## Installation

```bash
# this will install the executable `cgu` and `git-change-user`
cargo install change-git-user

# enable only the cli
cargo install --no-default-features --features=cli change-git-user

# enable only prompts
cargo install --no-default-features --features=prompts change-git-user
```

## Features

- Switch `user.name` and `user.email` config
- Assign an optional `user.signingKey` to the config
- Give the config a name to remember it by (e.g. "Work" or "Personal")
- Uses `git2` to discover a repository and manipulate its config

## Usage

***NOTE*** You can use `cgu` and `git change-user` interchangeably.

### Prompts

```bash
# This will spawn prompts to guide you through managing user configurations
cgu
```

### CLI

If you don't want use use prompts, you can use subcommands instead.
The subcommands are:

- `add`
- `select`
- `delete`
- `view`

#### Examples

```bash
# view CLI options
cgu --help

# view subcommand help
cgu <subcommand> --help

# Add a config with the name "My Config"
# Defaults to the user.name value ("My Name" in this example) if --name is not passed
cgu add --name "My Config" "My Name" example@email.com

# View all configs
cgu view --all

# Select the new config
cgu select "My Config"

# Delete the new config
cgu delete "My Config"
```

[Crates.io Link]: https://crates.io/crates/change-git-user
