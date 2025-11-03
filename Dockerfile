# --- STAGE 1: Builder ---
# Bu aşama, projenin derlenmesinden sorumludur.
FROM rust:1-slim-bookworm AS builder

# Gerekli tüm derleme araçlarını kuruyoruz.
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

# ÖNCE TÜM PROJE DOSYALARINI KOPYALA
# Bu, Cargo'nun hem `src` hem de `tests` klasörlerini görmesini sağlar.
COPY . .

# Şimdi projeyi derle. Cargo, bağımlılıkları ve kodu tek seferde halleder.
RUN cargo build --release --locked

# --- STAGE 2: Final (Minimal) Image ---
# Bu aşama, sadece çalıştırılabilir binary'yi içeren küçük bir imaj oluşturur.
FROM debian:bookworm-slim

# Çalışma zamanı için gerekli minimum sistem bağımlılıkları.
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

# Derlenmiş binary'yi builder aşamasından kopyala.
COPY --from=builder /app/target/release/sentiric-stt-gateway-service .
RUN chmod +x ./sentiric-stt-gateway-service

# Güvenlik için non-root kullanıcı oluştur ve kullan.
RUN useradd -m -u 1001 appuser
USER appuser

# Standart portları dışarı aç.
EXPOSE 15020 15021 15022
ENTRYPOINT ["./sentiric-stt-gateway-service"]