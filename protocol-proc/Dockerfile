FROM rust:slim-buster
WORKDIR /usr/src/subuidless-executor
COPY client client
COPY protocol protocol
COPY protocol-proc protocol-proc
WORKDIR /usr/src/subuidless-executor/client

RUN cargo install --bin executor --path .