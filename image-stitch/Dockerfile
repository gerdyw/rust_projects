# Multi-stage Docker build for x86_64

# Stage 1: Build the Rust application
FROM messense/rust-musl-cross:x86_64-musl AS builder

WORKDIR /app

# Copy dependency manifests
COPY Cargo.toml Cargo.lock* ./

# Create a dummy main.rs to cache dependencies
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    echo "pub fn dummy() {}" > src/lib.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release --target x86_64-unknown-linux-musl

# Remove dummy files
RUN rm -rf src

# Copy actual source code
COPY src ./src
COPY migrations ./migrations
COPY .sqlx ./.sqlx

# Build the actual application
ENV SQLX_OFFLINE=true
RUN touch src/lib.rs src/main.rs && \
    cargo build --release --target x86_64-unknown-linux-musl

# Stage 2: Create minimal runtime image
FROM alpine:3.19

WORKDIR /app

# Install runtime dependencies
RUN apk add --no-cache ca-certificates tzdata

# Copy the compiled binary from builder
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/image-stitch /usr/local/bin/image-stitch

# Copy migrations directory
COPY --from=builder /app/migrations /app/migrations

# Create non-root user
RUN addgroup -g 1000 appuser && \
    adduser -D -u 1000 -G appuser appuser && \
    chown -R appuser:appuser /app

USER appuser

# Expose the service port
EXPOSE 3000

# Set the entrypoint
ENTRYPOINT ["/usr/local/bin/image-stitch"]
