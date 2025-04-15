use super::provider::ApiType;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ProviderError {
    Configuration(String),
    RequestPreparation(String),
    ApiCall(String),
    ResponseParsing(String),
}

impl fmt::Display for ProviderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProviderError::Configuration(msg) => write!(f, "Configuration error: {}", msg),
            ProviderError::RequestPreparation(msg) => {
                write!(f, "Request preparation error: {}", msg)
            }
            ProviderError::ApiCall(msg) => write!(f, "API call error: {}", msg),
            ProviderError::ResponseParsing(msg) => write!(f, "Response parsing error: {}", msg),
        }
    }
}

impl Error for ProviderError {}

#[derive(Debug, Clone)]
pub struct ProviderConfig {
    pub api_type: ApiType,
    pub api_key: Option<String>,
    pub api_base_url: Option<String>,
    pub model: String,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u16>,
    pub schema: Option<String>,
    // Add other config options as needed
}

impl ProviderConfig {
    pub fn new(api_type: ApiType, model: String) -> Self {
        ProviderConfig {
            api_type,
            api_key: None,
            api_base_url: None,
            model,
            temperature: None,
            max_tokens: None,
            schema: None,
        }
    }

    pub fn with_api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key);
        self
    }

    pub fn with_api_base_url(mut self, api_base_url: String) -> Self {
        self.api_base_url = Some(api_base_url);
        self
    }

    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn with_max_tokens(mut self, max_tokens: u16) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }
    pub fn with_schema(mut self, schema: String) -> Self {
        self.schema = Some(schema);
        self
    }
}
