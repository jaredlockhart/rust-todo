FROM rust:latest

WORKDIR /app

# live reload support
RUN cargo install systemfd cargo-watch diesel_cli

# create a new empty project
RUN USER=root cargo init

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build

# remove the dummy hello world build
RUN cargo clean -p rust-todo

# copy your source tree
COPY ./src ./src

EXPOSE 3000