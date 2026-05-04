# Multi-stage Docker build for linux/amd64 and linux/arm64

# ===== Stage 1: Builder images (always run on the native x86 CI runner) =====
ARG TARGETARCH=amd64
FROM --platform=linux/amd64 messense/rust-musl-cross:x86_64-musl AS builder-amd64
FROM --platform=linux/amd64 messense/rust-musl-cross:aarch64-musl AS builder-arm64

# Select the right cross-compiler based on the target architecture
FROM builder-${TARGETARCH} AS builder

# Persist the Rust target triple in a file so subsequent RUN steps can read it
ARG TARGETARCH=amd64
RUN case "$TARGETARCH" in \
      amd64) echo "x86_64-unknown-linux-musl" ;; \
      arm64) echo "aarch64-unknown-linux-musl" ;; \
    esac > /rust_target

WORKDIR /app

# Copy dependency manifests
COPY Cargo.toml Cargo.lock* ./

# Create a dummy main.rs to cache dependencies
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    echo "pub fn dummy() {}" > src/lib.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release --target $(cat /rust_target)

# Remove dummy files
RUN rm -rf src

# Copy actual source code
COPY src ./src
COPY migrations ./migrations
COPY .sqlx ./.sqlx

# Build the actual application; move binary to a fixed path for the runtime stage
ENV SQLX_OFFLINE=true
RUN touch src/lib.rs src/main.rs && \
    cargo build --release --target $(cat /rust_target) && \
    cp target/$(cat /rust_target)/release/image-stitch /image-stitch

# Stage 2: Create minimal runtime image
FROM alpine:3.19

WORKDIR /app

# Install runtime dependencies
RUN apk add --no-cache ca-certificates tzdata

# Copy the compiled binary from builder
COPY --from=builder /image-stitch /usr/local/bin/image-stitch

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
