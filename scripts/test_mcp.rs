use reqwest;
use serde_json::{Value, json};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing MCP server on http://127.0.0.1:3001/mcp");

    let client = reqwest::Client::new();

    // Create a standard JSON-RPC request for get_symbols
    let request = json!({
        "jsonrpc": "2.0",
        "id": "test-request",
        "method": "get_symbols",
        "params": {
            "limit": 10
        }
    });

    println!("Sending request: {}", request);

    // Send the request and get the response
    let response = client
        .post("http://127.0.0.1:3001/mcp")
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    let status = response.status();
    let headers = response.headers().clone();
    let body = response.text().await?;

    println!("Status: {}", status);
    println!("Headers: {:#?}", headers);
    println!("Body: {}", body);

    // Parse the body as needed - this might be tricky with SSE format

    Ok(())
}
