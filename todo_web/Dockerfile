# ===== Stage 1: Build the Rust binary =====
FROM rust:1.89 as builder

WORKDIR /usr/src/app

# Copy manifest files first to leverage caching
COPY Cargo.toml Cargo.lock ./

# Build dependencies only
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true

# Copy the source code
COPY . .

# Build the actual project
RUN cargo build --release

# ===== Stage 2: Minimal runtime image =====
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /usr/src/app/target/release/todo_web .

# Expose the port the app listens on
EXPOSE 8080

# Set the default command
CMD ["./todo_web"]
