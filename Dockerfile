# Etapa de build
FROM rust:1.82-slim AS builder

RUN apt-get update && apt-get install -y \
  musl-tools pkg-config libssl-dev ca-certificates curl build-essential && \
  rustup target add x86_64-unknown-linux-musl

WORKDIR /app

# Aproveita cache de dependências
COPY Cargo.toml ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --target x86_64-unknown-linux-musl

# Copia o restante do código
COPY . .

# Build final com stripping
ENV RUSTFLAGS="-C target-feature=+crt-static"
RUN cargo build --release --target x86_64-unknown-linux-musl && \
    strip target/x86_64-unknown-linux-musl/release/rinha-backend

# Etapa final com imagem mínima compatível com logs
FROM gcr.io/distroless/cc
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rinha-backend /rinha-backend
ENTRYPOINT ["/rinha-backend"]
