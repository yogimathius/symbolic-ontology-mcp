# Symbol Ontology MCP API Documentation

## Overview

This document provides guidance for using the Symbol Ontology MCP API. The API allows you to search and retrieve dream symbols and their interpretations.

## Important Note on Parameter Handling

Due to a known issue with optional parameters in the MCP protocol implementation ([rust-sdk issue #135](https://github.com/modelcontextprotocol/rust-sdk/issues/135)), this API provides specialized endpoints for different query types:

- Use `search_symbols` for text-based searches
- Use `filter_by_category` for category filtering
- Use `get_symbols` for listing all symbols (without filters)
- Use `get_categories` to retrieve all available categories

## Available Endpoints

### 1. `search_symbols` (Recommended for Text Search)

**Description:** Search for symbols matching a text query.

**Parameters:**

- `query` (required): Text to search for in symbol names and descriptions
- `limit` (optional): Maximum number of results to return (default: 50)

**Example:**

```
// Search for symbols related to water
search_symbols(query: "water", limit: 10)
```

### 2. `filter_by_category` (Recommended for Category Filtering)

**Description:** Get symbols belonging to a specific category.

**Parameters:**

- `category` (required): Category name to filter by
- `limit` (optional): Maximum number of results to return (default: 50)

**Example:**

```
// Get symbols in the "nature" category
filter_by_category(category: "nature", limit: 10)
```

### 3. `get_symbols` (For Listing All Symbols)

**Description:** List all symbols (without filtering).

**Parameters:**

- `limit` (optional): Maximum number of results to return (default: 50)

**Example:**

```
// Get all symbols
get_symbols(limit: 20)
```

**Note:** The implementation has issues with the optional `category` parameter. For category filtering, please use the `filter_by_category` endpoint instead.

### 4. `get_categories` (For Retrieving Available Categories)

**Description:** Get a list of all available symbol categories.

**Parameters:** None

**Example:**

```
// Get all available categories
get_categories()
```

## Response Format

All endpoints return responses in the following format:

```json
{
  "symbols": [
    {
      "id": "water",
      "name": "Water",
      "category": "nature",
      "description": "Symbolizes emotions and the unconscious",
      "related_symbols": ["ocean", "river", "rain"]
    }
    // Additional symbols...
  ],
  "total_count": 10
}
```

For `get_categories`, the response format is:

```json
{
  "categories": ["animal", "nature", "mythological", "jungian"],
  "count": 4
}
```

## Troubleshooting

If you encounter issues with parameter handling:

1. Always use the specialized endpoints for each query type
2. For text searches, use `search_symbols` instead of `get_symbols`
3. For category filtering, use `filter_by_category` instead of `get_symbols`
4. If parameters are not being recognized correctly, try the specialized endpoints with non-optional parameters

## Examples for Common Use Cases

### Finding Symbols by Keyword

```
search_symbols(query: "shadow")
```

### Listing Symbols in a Category

```
filter_by_category(category: "jungian")
```

### Combining Searches

For combined searches, make sequential calls:

```
// First, get symbols in the "jungian" category
let jungian_symbols = filter_by_category(category: "jungian")

// Then, search within results for those related to "unconscious"
// (client-side filtering may be needed)
```

### Finding Available Categories

```
get_categories()
```
