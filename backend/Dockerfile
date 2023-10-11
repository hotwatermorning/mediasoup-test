FROM rust:1.73.0-buster as builder
RUN USER=root cargo new --bin app
WORKDIR /app

RUN apt-get update && rm -rf /var/lib/apt/lists/*
RUN curl -sS https://bootstrap.pypa.io/get-pip.py | python3

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN rustup component add rustfmt
RUN --mount=type=cache,target=/usr/local/cargo/registry cargo build --release
RUN rm src/*.rs

COPY ./src ./src
RUN --mount=type=cache,target=/usr/local/cargo/registry <<EOF
  set -e
  touch ./src/main.rs
  cargo install --locked --path .
EOF

FROM debian:buster-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /usr/local/cargo/bin/mediasoup-test-backend .
COPY .env .env
CMD ["./mediasoup-test-backend"]