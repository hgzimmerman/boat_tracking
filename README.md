# GGRC Boat tracking

A boat inventory system.

Implemented as a server to be ran locally, which serves HTML pages with HTMX for interactivity.

The conceptual idea behind the project is to have a computer in the boathouse that can be quickly used to do data entry for which boats were used after each practice.
Eventually, it also aims to support replacing the logbook for reporting issues with boats.
This should give the organization data to know which boats to buy, sell, maintain, etc...

## Features
* Adding boats
* Editing/Removing boats
* Recording batches of boat uses for a practice or regatta
  * Editing old batches
  * Using an old batch as a template for faster data entry.
* Viewing historical usage of boats
* Issue tracking for individual boats
* CSV export
  * Summary
  * Per-boat usage history
  * All boats usage history


## Development

### Technologies used

This project uses:
* [Rust](https://www.rust-lang.org/) as its primary language.
  * [Axum](https://github.com/tokio-rs/axum) for its server framework.
  * [Maud](https://maud.lambda.xyz/) for HTML templating.
  * [HTMX](https://htmx.org/) for interactive frontend behavior.
  * [Alpine.js](https://alpinejs.dev/) for client-side state management.
  * [Tailwind CSS](https://tailwindcss.com/) for styling.
  * [Diesel](https://diesel.rs/) for ORM/query-building/migrations management.
* [SQLite](https://www.sqlite.org/) for the database engine.
* [Nix](https://nixos.org/) for external dependency management.

### Getting started

* First install nix (or use NixOs as your operating system): https://nixos.org/download/
* Navigate to the project root and run `nix develop`. This will pull in all dependencies needed to build the project.
* In the shell that `nix-develop` dropped you in, you should run:
  * `npx tailwindcss -i ./input.css -o ./public/tailwind.css --watch` to watch and rebuild CSS.
  * `cargo run` to run the server.
* The server will be available at http://127.0.0.1:3000
* VS Code is the supported "IDE"
  * You should add the following plugins:
    * Rust-analyzer
    * Tailwind Css IntelliSense
    * direnv
  * You will have to bless the directory by running `direnv allow .envrc` to allow VSC to use the dependencies pulled in by `nix`.

Currently these instructions are tailored to the sole developer working on the project on NixOs.
Contributions are welcome regarding developing on MacOs or Windows.

## Contributing
By contributing to this project, you agree to have your contributions liscensed under the GNU GPL 3.0 liscense described in the `LICENSE` file found at the root of this project.
