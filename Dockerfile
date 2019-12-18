FROM rust:1-slim-buster as build-stage

COPY src /app/src
COPY Cargo.lock Cargo.toml /app/

WORKDIR /app

RUN cargo install --path .

FROM debian:buster-slim

COPY --from=build-stage /usr/local/cargo/bin/servify /usr/bin/servify
