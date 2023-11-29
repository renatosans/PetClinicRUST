# Build stage
FROM rust:1.69-buster as builder

WORKDIR /app

# pass env vars
ENV DATABASE_URL=$DATABASE_URL

COPY . . 

ENV SQLX_OFFLINE true

RUN cargo build --release

# Production stage
FROM debian:buster-slim

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/rust-pet-clinic .
RUN apt-get update && apt install -y openssl

CMD ["./rust-pet-clinic"]
