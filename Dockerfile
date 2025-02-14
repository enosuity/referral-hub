# Builder stage
FROM rust:1.81-alpine AS builder

# Install build dependencies
RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    pkgconfig \
    curl \
    build-base

WORKDIR /usr/src/app

# Copy Cargo files for better caching
COPY Cargo.toml Cargo.lock ./

# Create dummy src to build dependencies
RUN mkdir src && \
    echo 'fn main() { println!("dummy"); }' > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy the actual source code and rebuild
COPY src src/
COPY migrations migrations/
COPY .sqlx .sqlx/

# Enable offline mode and build
ENV SQLX_OFFLINE=true
RUN touch src/main.rs && cargo build --release

# Runtime stage
FROM alpine:latest

# Install runtime dependencies
RUN apk add --no-cache \
    ca-certificates \
    openssl \
    curl

# Create a non-root user
RUN adduser -D -s /bin/sh app

# Set working directory
WORKDIR /app

# Copy the binary and migrations
COPY --from=builder /usr/src/app/target/release/referral-service ./
COPY --from=builder /usr/src/app/migrations/ ./migrations/

# Fix permissions
RUN chown -R app:app /app

# Switch to non-root user
USER app

EXPOSE 8080

# Add healthcheck
HEALTHCHECK --interval=30s --timeout=3s \
    CMD curl -f http://localhost:8080/health

CMD ["./referral-service"] 