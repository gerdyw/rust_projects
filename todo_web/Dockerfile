# Use minimal Debian image for ARM64
FROM debian:bookworm-slim

WORKDIR /app

# Copy the ARM64 binary built on Apple Silicon
COPY target/aarch64-unknown-linux-gnu/release/todo_web .

# Copy static assets
COPY assets ./assets

# Expose the port
EXPOSE 8080

# Run the app
CMD ["./todo_web"]
