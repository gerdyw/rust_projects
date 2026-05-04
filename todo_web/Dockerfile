# ===== Stage 1: Builder images (always run on the native x86 CI runner) =====
# Build context must be the repo root: docker build -f todo_web/Dockerfile .
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

# Mirror the repo layout so ../crates/db_init resolves correctly
WORKDIR /usr/src/app/todo_web

# Copy the shared db_init crate (one level up, matching path = "../crates/db_init")
COPY crates/db_init /usr/src/app/crates/db_init/

# Create a stub binary for dependency caching
COPY todo_web/Cargo.toml todo_web/Cargo.lock ./
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release --target=$(cat /rust_target) && \
    rm src/main.rs

# Copy the real source and build; move binary to a fixed path for the runtime stage
COPY todo_web/src src/
COPY todo_web/migrations migrations/
RUN cargo build --release --target=$(cat /rust_target) && \
    cp target/$(cat /rust_target)/release/todo_web /todo_web

# ===== Stage 2: Runtime =====
FROM alpine:3.19 AS runtime

WORKDIR /app

COPY --from=builder /todo_web ./todo_web
COPY --from=builder /usr/src/app/todo_web/migrations migrations/
COPY todo_web/assets assets/

EXPOSE 8080
CMD ["./todo_web"]
