# Build stage
FROM --platform=linux/arm64 rust:1.94-slim-bookworm as builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    curl \
    ca-certificates

# Copy manifest files
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY migrations ./migrations
COPY .sqlx ./.sqlx

# Build release
RUN rustup target add aarch64-unknown-linux-gnu
RUN cargo build --release --locked  --target aarch64-unknown-linux-gnu
RUN cp ./target/aarch64-unknown-linux-gnu/release/transcoder /app/target/release/transcoder

# Runtime stage
FROM --platform=linux/arm64 debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ffmpeg \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*  

WORKDIR /app  

# Copy binary from builder
COPY --from=builder /app/target/release/transcoder /usr/local/bin

# Create neccessary directories
RUN mkdir -p /app/uploads /app/stream_output

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD [ "curl", "-f", "http://localhost:8080/health", "||", "exit", "1" ]

EXPOSE 3000

CMD ["transcoder"]