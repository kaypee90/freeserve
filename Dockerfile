FROM rust:1.40 as builder
WORKDIR /usr/src/project
COPY . .
RUN cargo install --path . && cargo build --release

FROM debian:buster-slim
RUN apt-get update
COPY --from=builder /usr/local/cargo/bin/project /usr/local/bin/project
CMD ["project"]

