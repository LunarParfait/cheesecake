
# Cheesecake

Still very much WIP

The end goal is for this to be an opinionated batteries-included meta-framework
for building fullstack backend focused web applications in Rust. Something similar
to ruby on rails and laravel.

This repository contains the CLI tool used to create and manage the
applications, the presets area located at
[cheescake-base](https://github.com/LunarParfait/cheesecake-base).

## Dependencies
- Make (to install CLI tool)
- Rust 2024
- SeaORM CLI

## CLI Installation
1. Clone this repo
2. run `make install`
3. run `cake --help` to get CLI help


## Roadmap

### CLI
- [x] create new project
- [x] lifecycle tasks (setup, clean, etc)
- [x] build task
- [x] test task
- [x] check tasks
- [x] lint tasks
- [x] run tasks
- [ ] migration tasks
- [ ] model generation tasks
- [x] controller generation tasks
- [ ] middleware generation tasks
- [x] view generation tasks
- [ ] change view/controller commands to accept path instead of name
- [ ] move base cheesecake config to a separate crate

### General
- [ ] add Dockerfile
- [ ] remove SeaORM CLI dependency
- [ ] add user storage solution
- [ ] add non SQLite support
- [ ] add webpack build system
- [ ] add builtin caching solutions
- [ ] add i18n support
- [ ] add security solutions
- [ ] add more presets
- [ ] maybe add toml/yaml based configuration?
