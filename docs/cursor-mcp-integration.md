# Cursor MCP Integration Guide

This guide explains how to configure Cursor to use your Dream Ontology MCP server, allowing AI assistants to access symbolic data directly during conversations.

## Prerequisites

1. **Running MCP Server**: Your Dream Ontology MCP server must be running and accessible.
2. **Cursor with Claude Integration**: You need Cursor with Claude 3 Sonnet or better.

## Step 1: Launch Your MCP Server

Make sure your MCP server is running on the default port (3002):

```bash
cargo run --bin mcp_server
```

Verify it's working with the test client:

```bash
cargo run --bin mcp_client
```

You should see symbols returned in the CLI output.

## Step 2: Configure Cursor for Local MCP

As of the latest Cursor version, connecting to local MCP servers requires a local agent configuration.

### Option 1: Create a Custom Claude Agent (Recommended)

1. In Cursor, click on your profile icon → "Settings" → "AI" → "Custom Agents"
2. Click "Create Custom Agent"
3. Fill out the form:
   - **Name**: "Dream Ontology Claude"
   - **Description**: "Claude with dream symbol knowledge"
   - **Avatar**: Choose any icon you like
   - **Model**: Claude 3 Sonnet or higher
   - **MCP Endpoint**: `http://localhost:3002/sse`
   - **MCP Authentication**: Leave blank (local development)

### Option 2: Temporary Connection via Claude Chat

If your Cursor version supports it, you can directly connect Claude to a local MCP server:

1. In Cursor, open a chat with Claude
2. Type: `/connect_mcp http://localhost:3002/sse`
3. Claude should confirm the connection

## Step 3: Test the MCP Connection

Try these test prompts with Claude in Cursor to verify the connection works:

```
List the dream symbols available in the ontology.
```

```
What's the meaning of the sun symbol in dreams?
```

```
Tell me about the moon symbol and what it represents.
```

Claude should respond with information pulled directly from your MCP server.

## Step 4: Using Dream Ontology MCP in Projects

For ongoing development, you can either:

1. **Keep Using the Custom Agent**: This maintains the connection between sessions
2. **Connect per Project**: Use `/connect_mcp` at the start of each coding session

## Troubleshooting

### MCP Connection Failed

If Claude can't connect to your MCP server:

1. **Check Server Running**: Make sure your MCP server is running
2. **Verify Endpoint URL**: Confirm `http://localhost:3002/sse` is correct
3. **Check Logs**: Look at your MCP server logs for connection attempts
4. **Try Test Client**: Verify the CLI client can connect

### Claude Not Using MCP

If Claude connects but doesn't use your MCP data:

1. **Ask Explicitly**: Frame questions to target the MCP capabilities
2. **Prompt with Details**: Mention "using your dream ontology tools"
3. **Check Format**: Ensure your questions relate to symbols, interpretations, etc.

## Advanced Integration

For production use, consider:

1. **Hosted MCP Server**: Deploy your MCP server with proper authentication
2. **Claude Plugin**: Package as an official Claude plugin
3. **Authentication**: Add API key authentication for secure access

## Example Prompts

Here are effective prompts for working with your Dream Ontology MCP:

```
Use your dream ontology tools to analyze the meaning of water in dreams.
```

```
I dreamed about a tall tree by a lake. Using your tools, what might these symbols represent?
```

```
List symbols related to "light" from your ontology and explain their interpretations.
```
