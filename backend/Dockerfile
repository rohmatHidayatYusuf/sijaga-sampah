# Multi-stage Dockerfile for SIJAGA SAMPAH Rust Backend
FROM rust:1.80-slim as builder

WORKDIR /usr/src/sijaga

# Install required build tools
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock* ./
# Pre-build dummy to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src

COPY . .
RUN cargo build --release

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates curl libssl3 && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /usr/src/sijaga/target/release/sijaga_backend /app/sijaga_backend
COPY migrations /app/migrations

ENV HOST=0.0.0.0
ENV PORT=8080

EXPOSE 8080

CMD ["/app/sijaga_backend"]
