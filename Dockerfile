FROM rust:1.81-slim as builder

WORKDIR /usr/src/app

# Copy the Cargo files
COPY Cargo.toml Cargo.lock ./
COPY app/Cargo.toml ./app/
COPY application/Cargo.toml ./application/
COPY domain/Cargo.toml ./domain/
COPY infrastructure/Cargo.toml ./infrastructure/

# Create dummy source files to build dependencies
RUN mkdir -p app/src && echo "fn main() {}" > app/src/main.rs
RUN mkdir -p application/src && echo "pub fn dummy() {}" > application/src/lib.rs
RUN mkdir -p domain/src && echo "pub fn dummy() {}" > domain/src/lib.rs
RUN mkdir -p infrastructure/src && echo "pub fn dummy() {}" > infrastructure/src/lib.rs

# Build dependencies
RUN cargo build --release

# Remove the dummy source files
RUN rm -rf app/src application/src domain/src infrastructure/src

# Copy the actual source code
COPY app/src ./app/src
COPY application/src ./application/src
COPY domain/src ./domain/src
COPY infrastructure/src ./infrastructure/src
COPY migrations ./migrations

# Build the application
RUN cargo build --release

# Create a smaller runtime image
FROM debian:bookworm-slim

WORKDIR /app

# Install PostgreSQL client for migrations
RUN apt-get update && apt-get install -y libpq5 ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/app /app/server
COPY --from=builder /usr/src/app/migrations /app/migrations
COPY .env.example /app/.env

# Expose the port the server listens on
EXPOSE 8080

# Run the server
CMD ["/app/server"]
