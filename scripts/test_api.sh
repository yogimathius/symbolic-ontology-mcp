#!/bin/bash

# Dream Ontology MCP API Test Script

API_HOST=${API_HOST:-"localhost:3000"}
MCP_HOST=${MCP_HOST:-"localhost:3001"}

# Set colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print section headers
print_header() {
    echo -e "\n${BLUE}==== $1 ====${NC}\n"
}

# Function to print test name
print_test() {
    echo -e "${GREEN}Testing: $1${NC}"
}

# Health Check
test_health() {
    print_test "Health Check"
    curl -s -X GET http://$API_HOST/health
    echo
}

# List all symbols
list_all_symbols() {
    print_test "List All Symbols"
    curl -s -X GET http://$API_HOST/symbols | jq
}

# List symbols with limit
list_symbols_limit() {
    local limit=${1:-5}
    print_test "List Symbols with Limit: $limit"
    curl -s -X GET http://$API_HOST/symbols -H "Content-Type: application/json" \
        -d "{\"limit\": $limit}" | jq
}

# List symbols by category
list_symbols_category() {
    local category=${1:-"dream"}
    print_test "List Symbols by Category: $category"
    curl -s -X GET http://$API_HOST/symbols -H "Content-Type: application/json" \
        -d "{\"category\": \"$category\"}" | jq
}

# Search symbols
search_symbols() {
    local query=${1:-"water"}
    print_test "Search Symbols: $query"
    curl -s -X GET http://$API_HOST/symbols -H "Content-Type: application/json" \
        -d "{\"query\": \"$query\"}" | jq
}

# Combined filtering
combined_filter() {
    local category=${1:-"dream"}
    local query=${2:-"water"}
    local limit=${3:-5}
    print_test "Combined Filtering: Category=$category, Query=$query, Limit=$limit"
    curl -s -X GET http://$API_HOST/symbols -H "Content-Type: application/json" \
        -d "{\"category\": \"$category\", \"query\": \"$query\", \"limit\": $limit}" | jq
}

# Get symbol by ID
get_symbol() {
    local id=${1:-"water"}
    print_test "Get Symbol by ID: $id"
    curl -s -X GET http://$API_HOST/symbols/$id | jq
}

# Request interpretation
interpret_symbol() {
    local id=${1:-"water"}
    local context=${2:-"recurring dream"}
    local query=${3:-"What might this symbolize?"}
    print_test "Symbol Interpretation: ID=$id, Context=$context"
    curl -s -X POST http://$API_HOST/interpret -H "Content-Type: application/json" \
        -d "{\"symbol_id\": \"$id\", \"context\": \"$context\", \"query\": \"$query\"}" | jq
}

# MCP get_symbols test
mcp_get_symbols() {
    print_test "MCP Get Symbols"
    curl -s -X POST http://$MCP_HOST/mcp -H "Content-Type: application/json" \
        -d '{
            "jsonrpc": "2.0",
            "id": "1",
            "method": "get_symbols",
            "params": {}
        }' | jq
}

# MCP get_symbols with category
mcp_get_symbols_category() {
    local category=${1:-"dream"}
    print_test "MCP Get Symbols by Category: $category"
    curl -s -X POST http://$MCP_HOST/mcp -H "Content-Type: application/json" \
        -d "{
            \"jsonrpc\": \"2.0\",
            \"id\": \"2\",
            \"method\": \"get_symbols\",
            \"params\": {
                \"category\": \"$category\"
            }
        }" | jq
}

# MCP get_symbols with search
mcp_get_symbols_search() {
    local query=${1:-"water"}
    print_test "MCP Get Symbols by Search: $query"
    curl -s -X POST http://$MCP_HOST/mcp -H "Content-Type: application/json" \
        -d "{
            \"jsonrpc\": \"2.0\",
            \"id\": \"3\",
            \"method\": \"get_symbols\",
            \"params\": {
                \"query\": \"$query\"
            }
        }" | jq
}

# MCP get_symbols combined
mcp_get_symbols_combined() {
    local category=${1:-"dream"}
    local query=${2:-"water"}
    local limit=${3:-5}
    print_test "MCP Get Symbols Combined: Category=$category, Query=$query, Limit=$limit"
    curl -s -X POST http://$MCP_HOST/mcp -H "Content-Type: application/json" \
        -d "{
            \"jsonrpc\": \"2.0\",
            \"id\": \"4\",
            \"method\": \"get_symbols\",
            \"params\": {
                \"category\": \"$category\",
                \"query\": \"$query\",
                \"limit\": $limit
            }
        }" | jq
}

# Test error handling
test_nonexistent_symbol() {
    print_test "Test Nonexistent Symbol"
    curl -s -X GET http://$API_HOST/symbols/nonexistent-symbol | jq
}

test_invalid_json() {
    print_test "Test Invalid JSON"
    curl -s -X GET http://$API_HOST/symbols -H "Content-Type: application/json" -d '{invalid json' | jq
}

test_invalid_params() {
    print_test "Test Invalid Parameters"
    curl -s -X GET http://$API_HOST/symbols -H "Content-Type: application/json" -d '{"category": ""}' | jq
}

# Run all tests
run_all_tests() {
    print_header "API Tests"
    test_health
    list_all_symbols
    list_symbols_limit
    list_symbols_category
    search_symbols
    combined_filter
    get_symbol
    interpret_symbol
    
    print_header "MCP Tests"
    mcp_get_symbols
    mcp_get_symbols_category
    mcp_get_symbols_search
    mcp_get_symbols_combined
    
    print_header "Error Handling Tests"
    test_nonexistent_symbol
    test_invalid_json
    test_invalid_params
}

# Check for jq
if ! command -v jq &> /dev/null; then
    echo -e "${RED}Error: jq is not installed. Please install it for JSON formatting:${NC}"
    echo "    Debian/Ubuntu: sudo apt-get install jq"
    echo "    macOS (Homebrew): brew install jq"
    echo "    Windows: choco install jq"
    echo
    echo "Running without jq formatting..."
    # Define jq as a no-op if not available
    jq() { cat; }
fi

# If no arguments, show usage
if [ "$#" -eq 0 ]; then
    echo "Usage: $0 [command] [args...]"
    echo
    echo "Available commands:"
    echo "  all                - Run all tests"
    echo "  health             - Test health endpoint"
    echo "  list               - List all symbols"
    echo "  limit [num]        - List symbols with limit"
    echo "  category [cat]     - List symbols by category"
    echo "  search [query]     - Search symbols"
    echo "  combined [cat] [q] [limit] - Combined filtering"
    echo "  symbol [id]        - Get symbol by ID"
    echo "  interpret [id] [context] [query] - Interpret symbol"
    echo "  mcp                - Test MCP get_symbols"
    echo "  mcp_cat [cat]      - Test MCP get_symbols with category"
    echo "  mcp_search [query] - Test MCP get_symbols with search"
    echo "  mcp_comb [cat] [q] [limit] - Test MCP combined"
    echo "  error_id           - Test nonexistent symbol ID"
    echo "  error_json         - Test invalid JSON"
    echo "  error_params       - Test invalid parameters"
    echo
    echo "Environment variables:"
    echo "  API_HOST - Set API host (default: localhost:3000)"
    echo "  MCP_HOST - Set MCP host (default: localhost:3001)"
    exit 1
fi

# Process command line arguments
case "$1" in
    all)
        run_all_tests
        ;;
    health)
        test_health
        ;;
    list)
        list_all_symbols
        ;;
    limit)
        list_symbols_limit "$2"
        ;;
    category)
        list_symbols_category "$2"
        ;;
    search)
        search_symbols "$2"
        ;;
    combined)
        combined_filter "$2" "$3" "$4"
        ;;
    symbol)
        get_symbol "$2"
        ;;
    interpret)
        interpret_symbol "$2" "$3" "$4"
        ;;
    mcp)
        mcp_get_symbols
        ;;
    mcp_cat)
        mcp_get_symbols_category "$2"
        ;;
    mcp_search)
        mcp_get_symbols_search "$2"
        ;;
    mcp_comb)
        mcp_get_symbols_combined "$2" "$3" "$4"
        ;;
    error_id)
        test_nonexistent_symbol
        ;;
    error_json)
        test_invalid_json
        ;;
    error_params)
        test_invalid_params
        ;;
    *)
        echo -e "${RED}Unknown command: $1${NC}"
        exit 1
        ;;
esac 