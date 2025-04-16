use serde::{Deserialize, Serialize};
use std::error::Error;

/// A simple LLM client for interacting with OpenRouter
#[derive(Debug, Clone)]
pub struct LlmClient {
    api_key: String,
    model: String,
    base_url: String,
}

/// Request structure for LLM completions
#[derive(Debug, Serialize)]
pub struct CompletionRequest {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
}

/// Message structure for LLM conversations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

/// Response from LLM completion requests
#[derive(Debug, Deserialize)]
pub struct CompletionResponse {
    pub id: String,
    pub choices: Vec<Choice>,
}

/// A single completion choice
#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: Message,
    pub finish_reason: String,
}

impl LlmClient {
    /// Create a new LLM client
    pub fn new(api_key: String, model: String) -> Self {
        LlmClient {
            api_key,
            model,
            base_url: "https://openrouter.ai/api/v1".to_string(),
        }
    }

    /// Create a client with the default DeepSeek model
    pub fn with_default_model(api_key: String) -> Self {
        Self::new(api_key, "deepseek/deepseek-chat".to_string())
    }

    /// Generate a completion for the given prompt
    pub async fn generate(&self, prompt: &str) -> Result<String, Box<dyn Error>> {
        // This is a stub implementation
        // In a real implementation, we would make an HTTP request to the OpenRouter API

        // For now, just return a mock response
        Ok(format!("LLM response to: {}", prompt))
    }

    /// Generate a completion with a system prompt and user message
    pub async fn chat(
        &self,
        system_prompt: &str,
        user_message: &str,
    ) -> Result<String, Box<dyn Error>> {
        // This is just preparing the messages - we're not using them yet in this stub
        let _messages = vec![
            Message {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: user_message.to_string(),
            },
        ];

        // This is a stub implementation
        Ok(format!(
            "LLM chat response to system: '{}' and user: '{}'",
            system_prompt, user_message
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate() {
        let client = LlmClient::with_default_model("test-key".to_string());
        let result = client
            .generate("What is the meaning of water in dreams?")
            .await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.contains("LLM response to:"));
    }

    #[tokio::test]
    async fn test_chat() {
        let client = LlmClient::with_default_model("test-key".to_string());
        let result = client
            .chat(
                "You are a dream symbol interpreter",
                "What does water represent in dreams?",
            )
            .await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.contains("LLM chat response"));
    }
}
