# Build stage
FROM rust:1.80-slim AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

# Final runtime image
FROM debian:bookworm-slim
WORKDIR /workspace

LABEL org.opencontainers.image.title="offpkg"
LABEL org.opencontainers.image.description="High-performance, universal offline-first package manager for Bun (npm), Python (uv), and Flutter (pub.dev) with fullstack template scaffolding."
LABEL org.opencontainers.image.url="https://github.com/aswin402/offpkg"
LABEL org.opencontainers.image.source="https://github.com/aswin402/offpkg"
LABEL org.opencontainers.image.licenses="MIT"

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    git \
    tar \
    gzip \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/offpkg /usr/local/bin/offpkg

ENTRYPOINT ["offpkg"]
CMD ["--help"]
