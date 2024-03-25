# GGRC Boat tracking

A boat inventory system.

Implemented as a server to be ran locally, which serves a single page application for use in web browsers.

The conceptual idea behind the project is to have a computer in the boathouse that can be quickly used to do data entry for which boats were used after each practice.
Eventually, it also aims to support replacing the logbook for reporting issues with boats.
This should give the organization data to know which boats to buy, sell, maintain, etc...

## Features
* Adding boats
* Removing boats (WIP)
* Recording batches of boat uses for a practice or regatta
  * Editing old batches
  * Using an old batch as a template for faster data entry.
* Viewing historical usage of boats (WIP)
* Issue tracking for individual boats (WIP)
* CSV export (WIP)
  * Summary
  * Per-boat usage history
  * All boats usage history


## Development

### Technologies used

This project uses:
* [Rust](https://www.rust-lang.org/) as its primary language.
  * [Axum](https://github.com/tokio-rs/axum) for its server framework.
  * [Dioxus](https://dioxuslabs.com/learn/0.4/) for its web frontend framework.
  * [tailwindcss](https://tailwindcss.com/) for its styling.
  * [Diesel](https://diesel.rs/) for ORM/query-building/migrations management.
* [SQLite](https://www.sqlite.org/) for the database engine.
* [Nix](https://nixos.org/) for external dependency management.

### Getting started

* First install nix (or use NixOs as your operating system): https://nixos.org/download/
* Navigate to the project root and run `nix develop`. This will pull in all dependencies needed to build the project.
* In the shell that `nix-develop` dropped you in, you should run:
  * `watch-tailwind` which will scan the project for classes to include in the css file included in the assets.
  * `watch-dx` which will watch the project for changes to the sections marked as `web` and recompile the frontend.
  * `watch-server` which will watch the project for changes to the backend and rebuild the server.
  * You will probably want to run these in separate terminal windows.
* VS Code is the supported "IDE"
  * You should add the following plugins:
    * Rust-analyzer
    * Dioxus
    * Tailwind Css IntelliSense
    * direnv
  * You will have to bless the directory by running `direnv allow .envrc` to allow VSC to use the dependencies pulled in by `nix`.

Currently these instructions are tailored to the sole developer working on the project on NixOs.
Contributions are welcome regarding developing on MacOs or Windows.

## Contributing
By contributing to this project, you agree to have your contributions liscensed under the GNU GPL 3.0 liscense described in the `LICENSE` file found at the root of this project.