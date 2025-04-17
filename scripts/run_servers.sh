#!/bin/bash
# Run both the API server and MCP server for development

# Exit on error
set -e

# Function to clean up child processes on exit
cleanup() {
  echo "Shutting down servers..."
  kill $(jobs -p) 2>/dev/null || true
}

# Set trap for clean exit
trap cleanup EXIT

# Print header
echo "==============================================="
echo "Starting Dream Ontology Servers"
echo "==============================================="

# Start the main API server
echo "Starting API Server on http://127.0.0.1:3000..."
cargo run &
API_PID=$!
echo "API Server started with PID: $API_PID"

# Wait a moment to ensure first server starts
sleep 2

# Start the MCP server
echo "Starting MCP Server on http://127.0.0.1:3001..."
cargo run --bin mcp_server &
MCP_PID=$!
echo "MCP Server started with PID: $MCP_PID"

echo "==============================================="
echo "Both servers are running!"
echo "Press Ctrl+C to shut down both servers"
echo "==============================================="

# Wait for user to press Ctrl+C
wait 