FROM rust:1.82-slim AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src/ src/
COPY static/ static/

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends curl && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/mappy /app/mappy
COPY static/ static/

RUN addgroup --system appgroup && adduser --system --ingroup appgroup appuser
USER appuser

EXPOSE 3000
CMD ["/app/mappy"]
