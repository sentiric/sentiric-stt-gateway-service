# path: Dockerfile
# --- STAGE 1: Chef (Planlama) ---
FROM lukemathwalker/cargo-chef:latest-rust-1.84-bookworm AS chef
WORKDIR /app

# --- STAGE 2: Planner ---
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# --- STAGE 3: Builder ---
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

# Protobuf derleyicisini kur
RUN apt-get update && apt-get install -y protobuf-compiler cmake && rm -rf /var/lib/apt/lists/*

# Bağımlılıkları derle ve cachele
RUN cargo chef cook --release --recipe-path recipe.json

# Kaynak kodları kopyala ve binary'i derle
COPY . .
RUN cargo build --release --bin sentiric-stt-gateway-service

# --- STAGE 4: Runtime (Minimal) ---
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Güvenlik: Root olmayan kullanıcı
RUN useradd -m -u 1001 appuser
USER appuser
WORKDIR /app

# Derlenen binary'i al
COPY --from=builder /app/target/release/sentiric-stt-gateway-service /app/

# Varsayılan Env Vars
ENV RUST_LOG=info
EXPOSE 15020 15021 15022

ENTRYPOINT ["./sentiric-stt-gateway-service"]