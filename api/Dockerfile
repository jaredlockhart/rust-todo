FROM rust:latest

WORKDIR /app

# live reload support
RUN cargo install systemfd cargo-watch

# create a new empty shell project
RUN USER=root cargo init

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build

# copy your source tree
COPY ./src ./src

# remove the dummy hello world build
RUN cargo clean -p rust-todo

EXPOSE 3000