FROM rust:1.74.1-buster

RUN USER=root cargo new --bin app
WORKDIR /app

RUN apt-get update && rm -rf /var/lib/apt/lists/*
RUN curl -sS https://bootstrap.pypa.io/get-pip.py | python3

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN rustup component add rustfmt
RUN cargo build
RUN rm src/*.rs

COPY ./src ./src
RUN touch ./src/main.rs
RUN cargo build

COPY .env.docker .env
EXPOSE 3010
ENTRYPOINT ["./target/debug/mediasoup-test-backend"]