FROM rust:slim-buster
WORKDIR /usr/src/subuidless-executor
COPY executor executor
COPY protocol protocol
WORKDIR /usr/src/subuidless-executor/executor

RUN cargo install --path .