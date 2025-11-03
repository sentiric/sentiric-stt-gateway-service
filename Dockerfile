# Dockerfile - TAM VE NİHAİ VERSİYON

# --- STAGE 1: Builder ---
# 'slim' versiyonunu kullanmak imaj boyutunu küçültür.
FROM rust:1-slim-bookworm AS builder

# === DÜZELTME BURADA: Gerekli derleme araçlarını kuruyoruz ===
# protoc'u ve git'i bu aşamada kuruyoruz ki Cargo derlemesi başarılı olsun.
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    protobuf-compiler \
    git \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Build argümanlarını al (CI'dan gelecek)
ARG GIT_COMMIT
ARG BUILD_DATE
ARG SERVICE_VERSION

WORKDIR /app

# Önce bağımlılıkları kopyalayıp derleyerek Docker'ın katman önbelleklemesini optimize ediyoruz.
COPY Cargo.toml Cargo.lock ./
# sentiric-contracts'ı indirebilmesi için boş bir src dizini ve main.rs oluşturuyoruz.
RUN mkdir src && echo "fn main() {}" > src/main.rs
# Sadece bağımlılıkları build et
RUN cargo build --release

# Şimdi tüm kaynak kodunu kopyala
COPY . .

# Son ve tam build'i yap
RUN cargo build --release

# --- STAGE 2: Final (Minimal) Image ---
FROM debian:bookworm-slim

# Çalışma zamanı için gerekli minimum sistem bağımlılıkları
RUN apt-get update && apt-get install -y --no-install-recommends \
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

# Standart portlarımızı açıyoruz
EXPOSE 15020 15021 15022

ENTRYPOINT ["./sentiric-stt-gateway-service"]