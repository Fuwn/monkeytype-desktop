# Monkeytype Desktop

[![crates.io](https://img.shields.io/crates/v/monkeytype.svg)](https://crates.io/crates/monkeytype)
[![github.com](https://github.com/Fuwn/monkeytype-desktop/actions/workflows/rust.yaml/badge.svg?branch=main)](https://github.com/Fuwn/monkeytype-desktop/actions/workflows/rust.yaml)

Monkeytype desktop is a desktop client for [monkeytype.com](https://monkeytype.com/) with various
quality-of-life features such as a Discord Rich Presence.

You might know my for authoring the original [monkey-type-desktop](https://github.com/fuwn/monkey-type-desktop)
client which was written using Electron. This is a rewrite of that client in Rust which aims to be
more lightweight and an improvement over the original.

## Usage

There aren't any official installation avenues at the moment, but in the future Moneytype Desktop
will come with an auto-updater and installation will be as simple as downloading a binary or using
Cargo.

If you really want to try it out, you can clone this repository and build it yourself using Tauri
and Cargo.

### Pre-requisites

- [Rust](https://www.rust-lang.org/)
- [The Tauri CLI](https://tauri.app/v1/api/cli) (`cargo install tauri-cli`)

Just clone this repository, `cd` into it, and run `cargo tauri dev` to launch a development server.

## License

This project is licensed with the
[GNU General Public License v3.0](https://github.com/Fuwn/monkeytype-desktop/blob/main/LICENSE).
