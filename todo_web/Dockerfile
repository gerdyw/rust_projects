# ====================
# Stage 1: Build
# ====================
FROM --platform=linux/arm64 rust:1.80 AS builder

WORKDIR /usr/src/app

# Install required libs for sqlx/sqlite
RUN apt-get update && apt-get install -y pkg-config libsqlite3-dev musl-tools && rm -rf /var/lib/apt/lists/*

# Copy manifests and fetch dependencies (for better caching)
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true

# Copy full project
COPY . .

# Build release binary for ARM64
RUN cargo build --release

# ====================
# Stage 2: Runtime
# ====================
FROM --platform=linux/arm64 debian:bookworm-slim

WORKDIR /app

# Install sqlite runtime (needed by sqlx if not fully static)
RUN apt-get update && apt-get install -y libsqlite3-0 && rm -rf /var/lib/apt/lists/*

# Copy binary
COPY --from=builder /usr/src/app/target/release/todo_web .

# Copy static assets if you have them
COPY ./assets ./assets

EXPOSE 8080

CMD ["./todo_web"]
