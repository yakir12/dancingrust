FROM rust:1.76 as builder

# Create a new empty shell project
WORKDIR /usr/src/app
RUN cargo new --bin hello_world
WORKDIR /usr/src/app/hello_world

# Copy over your manifest files
COPY ./Cargo.toml ./Cargo.toml

# This is a trick to cache dependencies
# We'll create a dummy main.rs that does nothing but still compiles
RUN echo "fn main() {println!(\"Dummy build\");}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/hello_world*

# Now copy your actual source code
COPY ./src ./src

# Build the actual application
RUN cargo build --release

# Start with a fresh slim image for the runtime stage
FROM debian:bullseye-slim

# Install OpenSSL - common dependency for web frameworks
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /usr/src/app/hello_world/target/release/hello_world /usr/local/bin/hello_world

# Expose the port the app will run on
EXPOSE 8080

# Command to run the application
CMD ["hello_world"]
