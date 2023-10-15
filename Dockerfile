FROM rust:1-slim-buster as builder

WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

FROM debian:buster-slim

RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/madoka /usr/local/bin/madoka

CMD ["madoka"]