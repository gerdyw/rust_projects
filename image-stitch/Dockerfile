# Multi-stage Docker build for ARM64 (Apple Silicon / Raspberry Pi)

# Stage 1: Build the Rust application
FROM messense/rust-musl-cross:aarch64-musl AS builder

WORKDIR /app

# Copy manifests and shared crate needed to resolve path dependencies.
COPY image-stitch/Cargo.toml image-stitch/Cargo.lock* image-stitch/
COPY crates/db_init/Cargo.toml crates/db_init/
COPY crates/db_init/src crates/db_init/src

# Create a dummy main.rs to cache dependencies
RUN mkdir -p image-stitch/src && \
    echo "fn main() {}" > image-stitch/src/main.rs && \
    echo "pub fn dummy() {}" > image-stitch/src/lib.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release --target aarch64-unknown-linux-musl --manifest-path image-stitch/Cargo.toml

# Remove dummy files
RUN rm -rf image-stitch/src

# Copy actual source code
COPY image-stitch/src image-stitch/src
COPY image-stitch/migrations image-stitch/migrations
COPY image-stitch/.sqlx image-stitch/.sqlx

# Build the actual application
ENV SQLX_OFFLINE=true
RUN touch image-stitch/src/lib.rs image-stitch/src/main.rs && \
    cargo build --release --target aarch64-unknown-linux-musl --manifest-path image-stitch/Cargo.toml

# Stage 2: Create minimal runtime image
FROM alpine:3.19

WORKDIR /app

# Install runtime dependencies
RUN apk add --no-cache ca-certificates tzdata

# Copy the compiled binary from builder
COPY --from=builder /app/image-stitch/target/aarch64-unknown-linux-musl/release/image-stitch /usr/local/bin/image-stitch

# Copy migrations directory
COPY --from=builder /app/image-stitch/migrations /app/migrations

# Create non-root user
RUN addgroup -g 1000 appuser && \
    adduser -D -u 1000 -G appuser appuser && \
    chown -R appuser:appuser /app

USER appuser

# Expose the service port
EXPOSE 3000

# Set the entrypoint
ENTRYPOINT ["/usr/local/bin/image-stitch"]
