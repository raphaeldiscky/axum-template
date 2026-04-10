# Build context must be the workspace root:
#   docker build -t user-service -f services/user-service/Dockerfile .

FROM rust:1.87-slim-bookworm AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config=1.8.1-1 \
    libssl-dev=3.0.18-1~deb12u1 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .
RUN cargo build --release -p user-service

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates=20230311+deb12u1 \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -r -s /bin/false appuser
USER appuser

COPY --from=builder /app/target/release/user-service /usr/local/bin/user-service

EXPOSE 7000
ENTRYPOINT ["user-service"]
