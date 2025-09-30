# ===== Stage 1: Build =====
FROM messense/rust-musl-cross:aarch64-musl AS builder

WORKDIR /usr/src/app

# Create a new source file for dependency caching
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release --target=aarch64-unknown-linux-musl && \
    rm src/main.rs

# Copy just the source code (exclude target, etc)
COPY src src/
COPY migrations migrations/
COPY assets assets/

# Build the actual binary
RUN cargo build --release --target=aarch64-unknown-linux-musl

# ===== Stage 2: Runtime =====
FROM alpine:3.19 AS runtime

WORKDIR /app

# Copy the static binary
COPY --from=builder /usr/src/app/target/aarch64-unknown-linux-musl/release/todo_web .

EXPOSE 8080
CMD ["./todo_web"]
