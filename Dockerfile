FROM rust:latest
RUN mkdir -p /usr/src/app/
WORKDIR /usr/src/app
# RUN rustup component add rust-analysis --toolchain 1.59.0-x86_64-unknown-linux-gnu
# RUN rustup component add rust-src --toolchain 1.59.0-x86_64-unknown-linux-gnu
# RUN rustup component add rls --toolchain 1.59.0-x86_64-unknown-linux-gnu