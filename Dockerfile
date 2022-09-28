FROM rust:1.64.0 as builder

WORKDIR /usr/src/recorder

COPY . .

RUN cargo install --path .

# FROM debian:buster-slim

# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*

# COPY --from=builder /usr/local/cargo/bin/recorder /usr/local/bin/recorder

CMD ["recorder"]