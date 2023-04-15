FROM rust:1-bullseye AS chef
RUN apt-get update && apt-get install -y protobuf-compiler
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
RUN rustup component add clippy
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
# RUN cargo clippy --release
RUN cargo test --release --all
RUN cargo build --release --all