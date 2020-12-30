# Learning Rust

This repo contains a bunch of mini projects to learn the programming language ["Rust"][rust].

Most if not all projects and the code within are inspired by ["The Rust Programming Language"][book] book
by Steve Klabnik and Carol Nichols.

## Usage

To quickly set up a new mini project copy the `rust-setup` project, rename and run it.

- `$ cp -r rust_setup NEW_PROJECT`
- `$ cd NEW_PROJECT`
- `$ vim Cargo.toml` (change value of name)
- `$ docker-compose up -d`
- `$ docker-compose exec cargo run`

[rust]: https://www.rust-lang.org
[book]: https://doc.rust-lang.org/book/
