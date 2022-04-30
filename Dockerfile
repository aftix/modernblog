# syntax=docker/dockerfile:1
FROM rust:1.40 as builder
WORKDIR /usr/src/aftback
COPY . .
RUN rustup override set nightly
RUN cargo install diesel_cli
RUN cargo install -Z unstable-options --profile release --path .
RUN /usr/local/cargo/bin/diesel migration run

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/aftback /usr/local/bin/aftback
RUN mkdir /app
RUN apt-get update && apt-get install -y sqlite3
COPY --from=builder /usr/src/aftback/data.db /app/.
COPY --from=builder /usr/src/aftback/Rocket.toml /app/.
WORKDIR /app
EXPOSE 8000
CMD ["aftback"]
