# Use the official Rust image as a parent image
FROM rust:latest

# Install additional tools
RUN cargo install cargo-edit
RUN cargo install cargo-make

# Since the Rust official image already includes rustup and the basic toolchain,
# we directly add the components needed for linting and formatting.
RUN rustup component add clippy
RUN rustup component add rustfmt

# Set the working directory in the container
RUN mkdir -p /workspaces/visual-hash
WORKDIR /workspaces/visual-hash

ENV RUST_LOG=info

