# Multi-stage Docker build for ARM64 (Apple Silicon / Raspberry Pi)
# Important: this Dockerfile expects the monorepo root as build context.
# From stitch-images/, use: docker buildx build -f Dockerfile ..

# Stage 1: Build the Rust application
FROM messense/rust-musl-cross:aarch64-musl AS builder

WORKDIR /app

# Copy manifests and shared crate needed to resolve path dependencies.
COPY stitch-images/Cargo.toml stitch-images/Cargo.lock* stitch-images/
COPY crates/db_init/Cargo.toml crates/db_init/
COPY crates/db_init/src crates/db_init/src

# Create dummy sources to cache dependencies.
RUN mkdir -p stitch-images/src && \
    echo "fn main() {}" > stitch-images/src/main.rs && \
    echo "pub fn dummy() {}" > stitch-images/src/lib.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release --target aarch64-unknown-linux-musl --manifest-path stitch-images/Cargo.toml

# Remove dummy files
RUN rm -rf stitch-images/src

# Copy actual source code
COPY stitch-images/src stitch-images/src
COPY stitch-images/migrations stitch-images/migrations
COPY stitch-images/.sqlx stitch-images/.sqlx

# Build the actual application
ENV SQLX_OFFLINE=true
RUN touch stitch-images/src/lib.rs stitch-images/src/main.rs && \
    cargo build --release --target aarch64-unknown-linux-musl --manifest-path stitch-images/Cargo.toml

# Stage 2: Create minimal runtime image
FROM alpine:3.19

WORKDIR /app

ENV TEMP_IMAGES_DIR=/data/temp_images
ENV DEST_IMAGES_DIR=/data/output_images

# Install runtime dependencies
RUN apk add --no-cache ca-certificates tzdata

# Copy the compiled binary from builder
COPY --from=builder /app/stitch-images/target/aarch64-unknown-linux-musl/release/stitch_images /usr/local/bin/stitch-images

# Copy migrations directory
COPY --from=builder /app/stitch-images/migrations /app/migrations

# Create writable output directories and non-root user
RUN mkdir -p /data/output_images /data/temp_images && \
    addgroup -g 1000 appuser && \
    adduser -D -u 1000 -G appuser appuser && \
    chown -R appuser:appuser /app /data

USER appuser

# Expose the service port
EXPOSE 8000

# Set the entrypoint
ENTRYPOINT ["/usr/local/bin/stitch-images"]
