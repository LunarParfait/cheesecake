
# Cheesecake

Still very much WIP

The end goal is for this to be an opinionated batteries-included meta-framework
for building fullstack hypermedia driven web applications in Rust. Something similar
to ruby on rails and laravel.

This repository contains the CLI tool used to create and manage the
applications, the base scaffolding is located at
[cheescake-base](https://github.com/LunarParfait/cheesecake-base).

## Dependencies
- Make
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
- [ ] controller generation tasks
- [ ] view generation tasks

### Functionality
- [ ] add Dockerfile
- [ ] add user storage solution
- [ ] add webpack build system
- [ ] add starter kits
- [ ] add builtin caching solutions
- [ ] add i18n support
- [ ] add security solutions

