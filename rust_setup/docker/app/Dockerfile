FROM rust:1.47-alpine3.12

ENV USER=rust

RUN adduser -D rust
USER rust

RUN rustup component add rustfmt

WORKDIR /usr/src/app

