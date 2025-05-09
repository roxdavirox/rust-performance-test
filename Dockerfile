FROM rust:1.70-slim AS builder

RUN apt-get update && apt-get install -y musl-tools pkg-config libssl-dev ca-certificates && \
    rustup target add x86_64-unknown-linux-musl

WORKDIR /app
COPY . .

ENV RUSTFLAGS="-C target-feature=+crt-static"

RUN cargo build --release --target x86_64-unknown-linux-musl && \
    strip target/x86_64-unknown-linux-musl/release/rinha-backend

FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rinha-backend /rinha-backend
ENTRYPOINT ["/rinha-backend"]
