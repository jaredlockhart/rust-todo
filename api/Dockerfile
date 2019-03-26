FROM rust:latest

WORKDIR /app

# install dev packages
RUN cargo install systemfd cargo-watch diesel_cli

# create a new empty project
RUN USER=root cargo init

# copy dependency manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# build dependencies
RUN cargo build

# remove the dummy hello world build
RUN cargo clean -p rust-todo

# copy the source tree
COPY ./src ./src

EXPOSE 3000