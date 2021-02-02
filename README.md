# `cgu`

[![Crates.io](https://img.shields.io/crates/v/change-git-user)][Crates.io Link]
![Crates.io](https://img.shields.io/crates/d/change-git-user)
![CI](https://github.com/spenserblack/change-git-user/workflows/CI/badge.svg)

Manage multiple git configurations

Inspired by [Git-User-Switch](https://github.com/geongeorge/Git-User-Switch).

## Installation

```bash
# this will install the executable `cgu`
cargo install change-git-user
```

## Features

- Switch `user.name` and `user.email` config
- Assign an optional `user.signingKey` to the config
- Give the config a name to remember it by (e.g. "Work" or "Personal")
- Uses `git2` to discover a repository and manipulate its config

## Usage

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
