# File: sentiric-stt-gateway-service/Dockerfile
# --- STAGE 1: Builder ---
FROM rust:1.93-slim-bookworm AS builder

# Protoc ve gerekli derleme araçları
RUN apt-get update && \
    apt-get install -y git pkg-config libssl-dev protobuf-compiler curl cmake && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

# Release derlemesi
RUN cargo build --release --bin sentiric-stt-gateway-service

# --- STAGE 2: Final ---
FROM debian:bookworm-slim

# Healthcheck için netcat ve curl
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates libssl-dev netcat-openbsd curl \
    && rm -rf /var/lib/apt/lists/*

# Güvenlik: Non-root kullanıcı
RUN useradd -m -u 1001 appuser
USER appuser
WORKDIR /app

# Binary'i al
COPY --from=builder /app/target/release/sentiric-stt-gateway-service .

# Varsayılan ortam değişkenleri
ENV RUST_LOG=info
ENV STT_GATEWAY_SERVICE_LISTEN_ADDRESS=0.0.0.0
ENV STT_GATEWAY_SERVICE_HTTP_PORT=15020
ENV STT_GATEWAY_SERVICE_GRPC_PORT=15021

EXPOSE 15020 15021 15022

ENTRYPOINT ["./sentiric-stt-gateway-service"]