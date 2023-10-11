FROM rust:1.73.0-buster
WORKDIR /app

RUN apt-get update && rm -rf /var/lib/apt/lists/*
RUN curl -sS https://bootstrap.pypa.io/get-pip.py | python3

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN rustup component add rustfmt

COPY ./src ./src
RUN cargo build

CMD ["./target/release/mediasoup-test-backend"]