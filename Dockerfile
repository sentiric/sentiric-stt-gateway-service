# --- STAGE 1: Chef ---
FROM lukemathwalker/cargo-chef:latest-rust-1.77-bookworm AS chef
WORKDIR /app

# --- STAGE 2: Planner ---
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# --- STAGE 3: Builder ---
FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release --bin sentiric-stt-gateway-service

# --- STAGE 4: Runtime ---
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m -u 1001 appuser
USER appuser
WORKDIR /app

COPY --from=builder /app/target/release/sentiric-stt-gateway-service /app/

ENV RUST_LOG=info
EXPOSE 15010 15011 15012

ENTRYPOINT ["./sentiric-stt-gateway-service"]