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

## Usage

```bash
# This will spawn prompts to guide you through managing user configurations
cgu
```
## Features

- Switch `user.name` and `user.email` config
- Assign an optional `user.signingKey` to the config
- Give the config a name to remember it by (e.g. "Work" or "Personal")
- *Does not* execute `git` commands -- uses `git2` to discover a repository
  and manipulate its config

[Crates.io Link]: https://crates.io/crates/change-git-user
