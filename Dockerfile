FROM rust:1-slim-buster as build-stage

WORKDIR /app

# compy empty main.rs to allow "install" command to download and compile dependencies
COPY fake-main.rs /app/src/main.rs
COPY Cargo.lock Cargo.toml /app/

# note this is not the actual code. only to download and compile dependencies in a separate layer
RUN \
  cargo install --path . && \
  cargo uninstall && \
  rm -r src/

COPY src /app/src
RUN cargo install --path .


FROM debian:buster-slim

COPY --from=build-stage /usr/local/cargo/bin/servify /usr/bin/servify
