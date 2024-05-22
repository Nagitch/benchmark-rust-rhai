FROM rust:1.77

WORKDIR /usr/src
COPY . .
RUN cargo build
