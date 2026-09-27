# Multi-stage build for ContentGuard AI
# Supports AMD64 and ARM64 architectures

# Stage 1: Builder
FROM rust:1.75-slim-bookworm AS builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libclang-dev \
    clang \
    llvm \
    ffmpeg \
    libavcodec-dev \
    libavformat-dev \
    libavfilter-dev \
    libswscale-dev \
    libvulkan-dev \
    vulkan-tools \
    cmake \
    git \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /build

# Copy Cargo files first for better caching
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build release binary
ENV RUSTFLAGS="-C target-cpu=sandybridge -C opt-level=3"
RUN cargo build --release --features "api,cli"

# Stage 2: Runtime
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    ffmpeg \
    libvulkan1 \
    vulkan-tools \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Create non-root user
RUN groupadd -r contentguard && useradd -r -g contentguard contentguard

# Create directories
RUN mkdir -p /app/data /app/models /app/policies /tmp/contentguard \
    && chown -R contentguard:contentguard /app /tmp/contentguard

# Copy binary from builder
COPY --from=builder /build/target/release/contentguard /app/contentguard

# Set permissions
RUN chmod +x /app/contentguard

# Switch to non-root user
USER contentguard

# Set working directory
WORKDIR /app

# Expose API port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD /app/contentguard health || exit 1

# Default command
ENTRYPOINT ["/app/contentguard"]
CMD ["server", "--bind", "0.0.0.0:8080"]
