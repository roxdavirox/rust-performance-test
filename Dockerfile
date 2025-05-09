FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /app/target/release/rinha-backend /usr/local/bin/rinha-backend
CMD ["rinha-backend"]
