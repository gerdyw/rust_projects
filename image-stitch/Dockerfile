# Multi-stage Docker build for ARM64 (Apple Silicon / Raspberry Pi)

FROM messense/rust-musl-cross:aarch64-musl AS builder

# Build from repo-root layout:
# /app/image-stitch
# /app/crates/db_init
WORKDIR /app/image-stitch

# 1) Copy manifests first for cacheable dependency layer
COPY image-stitch/Cargo.toml ./
COPY crates/db_init/Cargo.toml ../crates/db_init/Cargo.toml

# 2) Create dummy sources so cargo can resolve/build deps
RUN mkdir -p src ../crates/db_init/src && \
    echo "fn main() {}" > src/main.rs && \
    echo "pub fn dummy() {}" > src/lib.rs && \
    echo "pub fn dummy() {}" > ../crates/db_init/src/lib.rs

# 3) Build deps (cached layer)
RUN cargo build --release --target aarch64-unknown-linux-musl

# 4) Replace dummy code with real code
RUN rm -rf src ../crates/db_init/src
COPY image-stitch/src ./src
COPY image-stitch/migrations ./migrations
COPY image-stitch/.sqlx ./.sqlx
COPY crates/db_init ../crates/db_init

# 5) Build real app
ENV SQLX_OFFLINE=true
RUN cargo build --release --target aarch64-unknown-linux-musl

FROM alpine:3.19

WORKDIR /app
RUN apk add --no-cache ca-certificates tzdata

COPY --from=builder /app/image-stitch/target/aarch64-unknown-linux-musl/release/image-stitch /usr/local/bin/image-stitch
COPY --from=builder /app/image-stitch/migrations /app/migrations

RUN addgroup -g 1000 appuser && \
    adduser -D -u 1000 -G appuser appuser && \
    chown -R appuser:appuser /app

USER appuser
EXPOSE 3000
ENTRYPOINT ["/usr/local/bin/image-stitch"]