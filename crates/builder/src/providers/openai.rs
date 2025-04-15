use super::provider::ApiType;
use crate::providers::config::{ProviderConfig, ProviderError};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

// OpenAI provider implementation using reqwest directly
#[derive(Debug, Clone)]
pub struct OpenAiProvider {
    client: Client,
    config: Arc<ProviderConfig>,
    api_base_url: String,
}

// Structures for serializing/deserializing OpenAI API messages
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

impl ChatMessage {
    pub fn system(content: impl Into<String>) -> Self {
        ChatMessage {
            role: "system".to_string(),
            content: content.into(),
        }
    }

    pub fn user(content: impl Into<String>) -> Self {
        ChatMessage {
            role: "user".to_string(),
            content: content.into(),
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        ChatMessage {
            role: "assistant".to_string(),
            content: content.into(),
        }
    }
}

// Response structures
#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: ChatMessage,
    finish_reason: Option<String>,
    index: usize,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<ChatChoice>,
}

impl OpenAiProvider {
    pub fn new(config: Arc<ProviderConfig>) -> Result<Self, ProviderError> {
        if config.api_type != ApiType::OpenAI {
            return Err(ProviderError::Configuration(
                "Incorrect ApiType for OpenAiProvider".to_string(),
            ));
        }

        let api_key = config
            .api_key
            .as_ref()
            .ok_or_else(|| ProviderError::Configuration("OpenAI API key is missing".to_string()))?;

        // Create a reqwest client with appropriate timeout settings
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(120)) // 2 minutes timeout
            .build()
            .map_err(|e| {
                ProviderError::Configuration(format!("Failed to create HTTP client: {}", e))
            })?;

        // Default to the official OpenAI API endpoint, but allow override via config
        let api_base_url = config
            .api_base_url
            .clone()
            .unwrap_or_else(|| "https://api.openai.com/v1".to_string());

        Ok(OpenAiProvider {
            client,
            config,
            api_base_url,
        })
    }

    pub async fn chat_completion(
        &self,
        messages: Vec<ChatMessage>,
        model_override: Option<String>,
        temperature_override: Option<f32>,
        max_tokens_override: Option<u32>,
        response_schema: Option<serde_json::Value>, // Add schema as an optional parameter
                                                    // Optional schema for response format
                                                    // Add other parameters as needed
    ) -> Result<String, ProviderError> {
        let api_key =
            self.config.api_key.as_ref().ok_or_else(|| {
                ProviderError::Configuration("OpenAI API key is missing".to_string())
            })?;

        let model = model_override.unwrap_or_else(|| self.config.model.clone());
        let temperature = temperature_override.or(self.config.temperature);
        let max_tokens = max_tokens_override.or_else(|| self.config.max_tokens.map(|t| t as u32));

        // Construct the request body
        let mut request_body = json!({
            "model": model,
            "messages": messages,
        });

        // Add optional parameters if provided
        if let Some(temp) = temperature {
            request_body["temperature"] = json!(temp);
        }
        if let Some(tokens) = max_tokens {
            request_body["max_tokens"] = json!(tokens);
        }
        // Add the response_format if a schema is provided
        if let Some(schema) = response_schema {
            request_body["response_format"] = json!({
                "type": "json_schema",
                "json_schema": {
                    "name": "response",
                    "schema": schema
                }
            });
        }

        // Add other parameters as needed
        // request_body["top_p"] = ...

        // Construct the full API URL
        let url = format!("{}/chat/completions", self.api_base_url);

        // Send the request to OpenAI
        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&request_body)
            .send()
            .await
            .map_err(|e| ProviderError::ApiCall(format!("Request to OpenAI failed: {}", e)))?;

        // Check for non-success status codes
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "<could not read error response>".to_string());
            return Err(ProviderError::ApiCall(format!(
                "OpenAI API returned non-success status code {}: {}",
                status, error_text
            )));
        }

        // Parse the response
        let completion_response: ChatCompletionResponse = response.json().await.map_err(|e| {
            ProviderError::ResponseParsing(format!("Failed to parse OpenAI response: {}", e))
        })?;

        // Extract the generated text from the first choice
        completion_response
            .choices
            .into_iter()
            .next()
            .map(|choice| choice.message.content)
            .ok_or_else(|| {
                ProviderError::ResponseParsing("No completions returned from OpenAI".to_string())
            })
    }
}
