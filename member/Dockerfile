FROM rust:1.60 as build
RUN apt-get update -y && apt-get install -qq -y libpq-dev openssl libssl-dev  ca-certificates libprotobuf-dev protobuf-compiler
RUN mkdir /app
WORKDIR /app
COPY . ./
RUN cargo build --release

FROM debian:buster-slim
# RUN apt-get update -y && apt-get install -qq -y libpq-dev openssl libssl-dev  ca-certificates libprotobuf-dev protobuf-compiler

COPY --from=build /app/target/release/member .

EXPOSE 50004 8000

ENTRYPOINT ["/member"]


# ARG BASE_IMAGE=ekidd/rust-musl-builder:latest

# FROM ${BASE_IMAGE} AS base

# ENV USER=rust
# WORKDIR /code
# RUN cargo init
# COPY --chown=rust:rust Cargo.toml /code/Cargo.toml
# RUN cargo fetch

# RUN apk --no-cache add protobuf-dev protoc

# COPY src /code/src
# COPY build.rs /code/build.rs
# COPY proto /code/proto

# FROM base AS builder
# RUN cargo build --release

# FROM alpine:latest
# RUN apk --no-cache add ca-certificates
# COPY --from=builder \
#     /code/target/x86_64-unknown-linux-musl/release/dapr-publisher \
#     /usr/local/bin/

# EXPOSE 50004 8000

# ENTRYPOINT [ "/usr/local/bin/member" ]

