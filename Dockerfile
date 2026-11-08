FROM rust:1.80-slim AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/nexus-sphere .
EXPOSE 8080
CMD ["./nexus-sphere"]