# GGRC Boat tracking

A boat inventory system.

## Features
* Adding boats
* Removing boats (WIP)
* Recording batches of boat uses for a practice or regatta
  * Editing old batches (WIP)
  * Using an old batch as a template for faster data entry (WIP).
* Viewing historical useage of boats (WIP)
* Issue tracking for individual boats (WIP)
* CSV export (WIP)


## Development

### Technologies used

This project uses:
* [Rust](https://www.rust-lang.org/) as its primary language.
  * [Axum](https://github.com/tokio-rs/axum) for its server framework.
  * [Dioxus](https://dioxuslabs.com/learn/0.4/) for its web frontend framework.
  * [tailwindcss](https://tailwindcss.com/) for its styling.
* [Nix](https://nixos.org/) for external dependency management.

### Getting started

* First install nix (or use NixOs as your operating system): https://nixos.org/download/
* Navigate to the project root and run `nix develop`. This will pull in all dependencies needed to build the project.
* In the shell that `nix-develop` dropped you in, you should run:
  * `watch-tailwind` which will scan the project for classes to include in the css file included in the assets.
  * `watch-dx` which will watch the project for changes to the sections marked as `web` and recompile the frontend.
  * `watch-server` which will watch the project for changes to the backend and rebuild the server.
  * You will probably want to run these in separate terminal windows.
