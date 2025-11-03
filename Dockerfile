# Dockerfile - TAM, EKSİKSİZ VE STANDARTLARA UYGUN FİNAL VERSİYON

# --- STAGE 1: Builder ---
FROM rust:1-slim-bookworm AS builder

# === NİHAİ DÜZELTME: Gerekli tüm derleme araçlarını kuruyoruz ===
# Bu, `sentiric-contracts`'ın `build.rs` script'inin CI'da çalışabilmesi için zorunludur.
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    protobuf-compiler \
    libprotobuf-dev \
    git \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Build argümanları (CI'dan gelecek)
ARG GIT_COMMIT
ARG BUILD_DATE
ARG SERVICE_VERSION
WORKDIR /app

# Docker katman önbelleklemesini optimize etmek için adımlı build
COPY Cargo.toml Cargo.lock ./
# `sentiric-contracts`'ı ve diğer bağımlılıkları indirmek ve derlemek için
RUN mkdir src && echo "fn main() {}" > src/main.rs
# Sadece bağımlılıkları derle. En uzun süren adım budur.
RUN cargo build --release --locked

# Şimdi tüm kaynak kodunu kopyala
COPY src ./src

# Son ve hızlı build (sadece bizim kodumuz derlenecek)
RUN cargo build --release --locked

# --- STAGE 2: Final (Minimal) Image ---
FROM debian:bookworm-slim

# Çalışma zamanı için gerekli minimum sistem bağımlılıkları
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Build argümanlarını runtime'a taşı
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

# Standart portlar
EXPOSE 15020 15021 15022
ENTRYPOINT ["./sentiric-stt-gateway-service"]
