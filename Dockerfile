FROM rust:latest as builder
USER root
RUN apt-get update && apt-get install cmake -y
WORKDIR /compile
RUN mkdir ./src
RUN echo "fn main() {}" > ./src/main.rs
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
RUN cargo build --release
RUN rm -f ./target/release/deps/trovo*
COPY ./src ./src
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update && apt-get install libssl-dev ca-certificates -y && update-ca-certificates
WORKDIR /Trovo
COPY --from=builder ./compile/target/release/trovo /Trovo/trovo
ENTRYPOINT /Trovo/trovo