# Multi-stage Docker build for ARM64 (Apple Silicon / Raspberry Pi)
# Build context must be the repo root: docker build -f image-stitch/Dockerfile .

# Stage 1: Build the Rust application
FROM messense/rust-musl-cross:aarch64-musl AS builder

# Mirror the repo layout so ../crates/db_init resolves correctly
WORKDIR /usr/src/app/image-stitch

# Copy the shared db_init crate (one level up, matching path = "../crates/db_init")
COPY crates/db_init /usr/src/app/crates/db_init/

# Copy dependency manifests
COPY image-stitch/Cargo.toml image-stitch/Cargo.lock* ./

# Create a dummy main.rs to cache dependencies
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    echo "pub fn dummy() {}" > src/lib.rs

# Build dependencies (this layer will be cached)
ARG TARGETARCH
RUN if [ "$TARGETARCH" = "amd64" ]; then \
      export RUST_TARGET=x86_64-unknown-linux-musl; \
    elif [ "$TARGETARCH" = "arm64" ]; then \
      export RUST_TARGET=aarch64-unknown-linux-musl; \
    else \
      echo "Unsupported arch: $TARGETARCH" && exit 1; \
    fi && \
    cargo build --release --target "$RUST_TARGET"
# Remove dummy files
RUN rm -rf src

# Copy actual source code
COPY image-stitch/src ./src
COPY image-stitch/migrations ./migrations
COPY image-stitch/.sqlx ./.sqlx

# Build the actual application
ENV SQLX_OFFLINE=true
RUN touch src/lib.rs src/main.rs && \
    cargo build --release --target aarch64-unknown-linux-musl

# Stage 2: Create minimal runtime image
FROM alpine:3.19

WORKDIR /app

# Install runtime dependencies
RUN apk add --no-cache ca-certificates tzdata

# Copy the compiled binary from builder
COPY --from=builder /usr/src/app/image-stitch/target/aarch64-unknown-linux-musl/release/image-stitch /usr/local/bin/image-stitch

# Copy migrations directory
COPY --from=builder /usr/src/app/image-stitch/migrations /app/migrations

# Create non-root user
RUN addgroup -g 1000 appuser && \
    adduser -D -u 1000 -G appuser appuser && \
    chown -R appuser:appuser /app

USER appuser

# Expose the service port
EXPOSE 3000

# Set the entrypoint
ENTRYPOINT ["/usr/local/bin/image-stitch"]
