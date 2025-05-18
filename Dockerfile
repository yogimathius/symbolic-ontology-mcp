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
RUN cargo build --release --bin mcp_server
RUN cargo build --release --bin mcp_websocket_server
RUN cargo build --release --bin mcp_http_upgrade_server
RUN cargo build --release --bin mcp_streamable_http_server

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy all server binaries
COPY --from=builder /app/target/release/mcp_server /usr/local/bin

# Expose all ports
EXPOSE 3002 3003 3004 3005

# Default to running the original SSE server
ENTRYPOINT ["/usr/local/bin/mcp_server"]
