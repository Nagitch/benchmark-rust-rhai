FROM rust:1.87.0

WORKDIR /usr/src
COPY . .
RUN cargo build
