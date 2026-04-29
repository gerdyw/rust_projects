# ===== Stage 1: Build =====
# Build context must be the repo root: docker build -f todo_web/Dockerfile .
FROM messense/rust-musl-cross:aarch64-musl AS builder

# Mirror the repo layout so ../crates/db_init resolves correctly
WORKDIR /usr/src/app/todo_web

# Copy the shared db_init crate (one level up, matching path = "../crates/db_init")
COPY crates/db_init /usr/src/app/crates/db_init/

# Create a stub binary for dependency caching
COPY todo_web/Cargo.toml todo_web/Cargo.lock ./
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release --target=aarch64-unknown-linux-musl && \
    rm src/main.rs

# Copy the real source and build
COPY todo_web/src src/
COPY todo_web/migrations migrations/
RUN cargo build --release --target=aarch64-unknown-linux-musl

# ===== Stage 2: Runtime =====
FROM alpine:3.19 AS runtime

WORKDIR /app

COPY --from=builder /usr/src/app/todo_web/target/aarch64-unknown-linux-musl/release/todo_web .
COPY --from=builder /usr/src/app/todo_web/migrations migrations/
COPY todo_web/assets assets/

EXPOSE 8080
CMD ["./todo_web"]
