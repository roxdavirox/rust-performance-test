FROM rust:1.82-slim AS builder

# Instala dependências mínimas para compilar com musl
RUN apt-get update && apt-get install -y \
  musl-tools pkg-config libssl-dev ca-certificates curl build-essential && \
  rustup target add x86_64-unknown-linux-musl

WORKDIR /app

# Copia só o Cargo.toml e Cargo.lock primeiro para aproveitar cache
COPY Cargo.toml ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --target x86_64-unknown-linux-musl

# Agora copia o código real
COPY . .

# Compila o binário final
ENV RUSTFLAGS="-C target-feature=+crt-static"
RUN cargo build --release --target x86_64-unknown-linux-musl && \
    strip target/x86_64-unknown-linux-musl/release/rinha-backend

# Imagem final super enxuta
FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rinha-backend /rinha-backend
ENTRYPOINT ["/rinha-backend"]
