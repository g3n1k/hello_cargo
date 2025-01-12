# Stage 1: Build
FROM rust:latest AS builder

WORKDIR /usr/src/app

# Copy source code and dependencies
COPY . .
RUN cargo build --release

# Stage 2: Final image
FROM debian:bullseye-slim

WORKDIR /app

# Copy the binary from the build stage
COPY --from=builder /usr/src/app/target/release/my_app /app/

# Set executable
CMD ["./my_app"]
