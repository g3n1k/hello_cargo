# Stage 1: Builder
FROM rust:latest AS builder

# Install dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Create a new user to avoid running as root
RUN useradd -m rustuser
WORKDIR /home/rustuser/app

# Copy the Rust project files
COPY . .

# Set the user for better security
USER rustuser

# Build the project
RUN cargo build --release

# Stage 2: Final Image
FROM debian:buster-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -m rustuser
WORKDIR /home/rustuser

# Copy the compiled binary from the builder stage
COPY --from=builder /home/rustuser/app/target/release/my_app /usr/local/bin/my_app

# Set the user and entrypoint
USER rustuser
ENTRYPOINT ["/usr/local/bin/my_app"]
