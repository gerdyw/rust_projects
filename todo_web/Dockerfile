# ===== Stage 1: Build the Rust binary =====
FROM rust:1.89 as builder

WORKDIR /usr/src/app

# Copy manifest files first to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Dummy main.rs to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true

# Copy full source code
COPY . .

# Add musl target and build a static binary
RUN rustup target add aarch64-unknown-linux-musl
RUN cargo build --release --target aarch64-unknown-linux-musl

# ===== Stage 2: Minimal Alpine runtime =====
FROM alpine:3.18

WORKDIR /app

# Copy the statically linked binary
COPY --from=builder /usr/src/app/target/aarch64-unknown-linux-musl/release/todo_web .

# Expose port from env (for documentation)
EXPOSE 8080

# Run the binary
CMD ["./todo_web"]
