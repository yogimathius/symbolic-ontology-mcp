use dream_ontology_mcp::llm::client::LlmClient;

#[test]
fn test_llm_client_creation() {
    let _client = LlmClient::new("test-api-key".to_string(), "test-model".to_string());
    let _default_client = LlmClient::with_default_model("test-api-key".to_string());
}

#[tokio::test]
async fn test_llm_client_generate() {
    let _client = LlmClient::new("test-api-key".to_string(), "test-model".to_string());
}

#[tokio::test]
async fn test_llm_client_chat() {
    let _client = LlmClient::new("test-api-key".to_string(), "test-model".to_string());
}

#[test]
fn test_message_struct() {
    use dream_ontology_mcp::llm::client::Message;

    let message = Message {
        role: "user".to_string(),
        content: "test content".to_string(),
    };

    assert_eq!(message.role, "user");
    assert_eq!(message.content, "test content");
}

#[test]
fn test_completion_request_serialization() {
    use dream_ontology_mcp::llm::client::{CompletionRequest, Message};
    use serde_json::Value;

    let request = CompletionRequest {
        model: "test-model".to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: "You are a helpful assistant".to_string(),
            },
            Message {
                role: "user".to_string(),
                content: "Hello".to_string(),
            },
        ],
        temperature: Some(0.7),
        max_tokens: Some(100),
    };

    let json = serde_json::to_value(request).unwrap();

    assert_eq!(json["model"], "test-model");
    assert_eq!(json["messages"][0]["role"], "system");
    assert_eq!(
        json["messages"][0]["content"],
        "You are a helpful assistant"
    );
    assert_eq!(json["messages"][1]["role"], "user");
    assert_eq!(json["messages"][1]["content"], "Hello");

    assert!(json["temperature"].is_number());
    if let Value::Number(temp) = &json["temperature"] {
        let temp_f64 = temp.as_f64().unwrap();
        assert!((temp_f64 - 0.7).abs() < 0.001);
    }

    assert_eq!(json["max_tokens"], 100);
}
