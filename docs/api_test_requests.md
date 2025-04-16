# Dream Ontology MCP API Testing Guide

This document contains examples of curl commands for testing the various API endpoints of the Dream Ontology MCP service.

## Health Check

Check if the API is up and running:

```bash
curl -X GET http://localhost:3000/health
```

## Symbols API

### List All Symbols

Retrieve all symbols without filtering:

```bash
curl -X GET http://localhost:3000/symbols
```

### List Symbols with Limit

Limit the number of returned symbols:

```bash
curl -X GET http://localhost:3000/symbols -H "Content-Type: application/json" -d '{"limit": 5}'
```

### List Symbols by Category

Filter symbols by category:

```bash
curl -X GET http://localhost:3000/symbols -H "Content-Type: application/json" -d '{"category": "dream"}'
```

### Search Symbols

Search for symbols with a query:

```bash
curl -X GET http://localhost:3000/symbols -H "Content-Type: application/json" -d '{"query": "water"}'
```

### Combined Filtering

Apply multiple filters:

```bash
curl -X GET http://localhost:3000/symbols -H "Content-Type: application/json" -d '{"category": "dream", "query": "water", "limit": 5}'
```

### Get Symbol by ID

Retrieve a specific symbol by its ID:

```bash
curl -X GET http://localhost:3000/symbols/water
```

## Symbol Interpretation

Request interpretation for a symbol:

```bash
curl -X POST http://localhost:3000/interpret -H "Content-Type: application/json" -d '{"symbol_id": "water", "context": "recurring dream", "query": "What might this symbolize?"}'
```

## MCP Testing

### Get Symbols MCP Method

Test the get_symbols MCP method directly (assuming MCP server is running on port 3001):

```bash
curl -X POST http://localhost:3001/mcp -H "Content-Type: application/json" -d '{
  "jsonrpc": "2.0",
  "id": "1",
  "method": "get_symbols",
  "params": {}
}'
```

### Get Symbols with Category

Filter by category in MCP request:

```bash
curl -X POST http://localhost:3001/mcp -H "Content-Type: application/json" -d '{
  "jsonrpc": "2.0",
  "id": "2",
  "method": "get_symbols",
  "params": {
    "category": "dream"
  }
}'
```

### Get Symbols with Search Query

Search symbols in MCP request:

```bash
curl -X POST http://localhost:3001/mcp -H "Content-Type: application/json" -d '{
  "jsonrpc": "2.0",
  "id": "3",
  "method": "get_symbols",
  "params": {
    "query": "water"
  }
}'
```

### Combined MCP Request

Apply multiple filters in MCP request:

```bash
curl -X POST http://localhost:3001/mcp -H "Content-Type: application/json" -d '{
  "jsonrpc": "2.0",
  "id": "4",
  "method": "get_symbols",
  "params": {
    "category": "dream",
    "query": "water",
    "limit": 5
  }
}'
```

## Testing Tips

- Replace `localhost:3000` with your actual server address if deployed elsewhere
- For the MCP endpoint, use port 3001 or whatever port is configured for the MCP server
- If testing with authentication, add authorization headers as needed
- Save common requests in a script file for easy reuse
- Consider using [jq](https://stedolan.github.io/jq/) to format and filter JSON responses

## Error Handling Testing

### Test Invalid Symbol ID

```bash
curl -X GET http://localhost:3000/symbols/nonexistent-symbol
```

### Test Invalid JSON

```bash
curl -X GET http://localhost:3000/symbols -H "Content-Type: application/json" -d '{invalid json'
```

### Test Invalid Parameters

```bash
curl -X GET http://localhost:3000/symbols -H "Content-Type: application/json" -d '{"category": ""}'
```
