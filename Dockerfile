FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
# Build only the symbol-mcp-client
RUN cargo build --release --package symbol-mcp-client --bin symbol-mcp

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary
COPY --from=builder /app/target/release/symbol-mcp /usr/local/bin

# Create a directory for logs
RUN mkdir -p /app/logs

# Expose MCP port
EXPOSE 3002

# Default to running the MCP client
ENTRYPOINT ["/usr/local/bin/symbol-mcp"]
