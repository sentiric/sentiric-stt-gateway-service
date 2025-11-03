# --- STAGE 1: Builder ---
FROM rust:1-slim-bookworm AS builder

# Gerekli derleme araçlarını kur
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    protobuf-compiler \
    git \
    curl \
    libssl-dev \
    pkg-config \
    && \
    rm -rf /var/lib/apt/lists/*

ARG GIT_COMMIT
ARG BUILD_DATE
ARG SERVICE_VERSION
WORKDIR /app
COPY . .
ENV GIT_COMMIT=${GIT_COMMIT}
ENV BUILD_DATE=${BUILD_DATE}
ENV SERVICE_VERSION=${SERVICE_VERSION}
RUN cargo build --release

# --- STAGE 2: Final (Minimal) Image ---
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    netcat-openbsd \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

ARG GIT_COMMIT
ARG BUILD_DATE
ARG SERVICE_VERSION
ENV GIT_COMMIT=${GIT_COMMIT}
ENV BUILD_DATE=${BUILD_DATE}
ENV SERVICE_VERSION=${SERVICE_VERSION}

WORKDIR /app
COPY --from=builder /app/target/release/sentiric-stt-gateway-service .
RUN chmod +x ./sentiric-stt-gateway-service

RUN useradd -m -u 1001 appuser
USER appuser
EXPOSE 15020 15021 15022
ENTRYPOINT ["./sentiric-stt-gateway-service"]