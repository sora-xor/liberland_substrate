FROM debian:bookworm-20231030-slim

ENV RUSTUP_HOME="/opt/rust"
ENV CARGO_HOME="/opt/rust"
ENV PATH="$PATH:$RUSTUP_HOME/bin"
ENV RUST_VERSION=nightly-2023-03-21

ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y \ 
    build-essential \
    git \
    clang \
    curl \
    libssl-dev \
    llvm \
    libudev-dev \
    make \
    protobuf-compiler \
    pkg-config &&\
    curl https://sh.rustup.rs -sSf | sh -s -- -y --no-modify-path --default-toolchain ${RUST_VERSION} &&\
    rustup default ${RUST_VERSION} &&\
    rustup target add wasm32-unknown-unknown &&\
    apt-get clean