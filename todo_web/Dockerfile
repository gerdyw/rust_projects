# ===== Stage 1: Build =====
FROM messense/rust-musl-cross:aarch64-musl AS builder

WORKDIR /usr/src/app

# Copy manifest files first (for dependency caching)
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --target=aarch64-unknown-linux-musl || true

# Copy the real source
COPY . .

# Build with SQLite bundled
RUN cargo build --release --target=aarch64-unknown-linux-musl --features sqlx/sqlite

# ===== Stage 2: Runtime =====
FROM alpine:3.19 AS runtime

WORKDIR /app

# Copy the static binary
COPY --from=builder /usr/src/app/target/aarch64-unknown-linux-musl/release/todo_web .

EXPOSE 8080
CMD ["./todo_web"]
