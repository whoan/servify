FROM rust:1-slim-buster as build-stage

WORKDIR /app

# compy empty main.rs to allow "install" command to download and compile dependencies
COPY Cargo.lock Cargo.toml /app/
COPY fake-main.rs /app/src/main.rs

# note I am not building the actual code. it is only to download and compile dependencies in a separate layer
RUN \
  cargo build --release && \
  rm -r src/

COPY src /app/src

RUN cargo install --path .


FROM debian:buster-slim

COPY --from=build-stage /usr/local/cargo/bin/servify /usr/bin/servify
