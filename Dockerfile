# Etapa 1: build com musl para gerar binário 100% estático
FROM rust:1.70-slim as builder

# Instala ferramenta de build estático
RUN apt-get update && apt-get install -y musl-tools pkg-config libssl-dev ca-certificates && \
    rustup target add x86_64-unknown-linux-musl

# Configura ambiente e compila
WORKDIR /app
COPY . .

# Força linking estático de bibliotecas C (openssl pode precisar de ajustes)
ENV RUSTFLAGS="-C target-feature=+crt-static"

# Compila para target musl (binário 100% estático)
RUN cargo build --release --target x86_64-unknown-linux-musl && \
    strip target/x86_64-unknown-linux-musl/release/rinha-backend

# Etapa 2: imagem final sem sistema operacional
FROM scratch

# Copia apenas o binário já compilado
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rinha-backend /rinha-backend

# Define entrada
ENTRYPOINT ["/rinha-backend"]
