use serde::{Deserialize, Serialize};
use std::string::ToString;
use std::sync::Arc;
use strum_macros::Display;
use strum_macros::EnumString;
use strum_macros::IntoStaticStr;

use super::config::{ProviderConfig, ProviderError};
use super::models::Models;
use super::openai::OpenAiProvider;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Provider {
    pub name: String,
    pub base_url: String,
    pub api_key: String,
    pub provider_type: ProviderType,
    pub api_type: ApiType,
    pub models: Vec<Models>,
    pub default_model: Models,
}

#[derive(
    Serialize,
    Deserialize,
    Debug,
    PartialEq,
    Default,
    Clone,
    Copy,
    IntoStaticStr,
    EnumString,
    Display,
)]
pub enum ProviderType {
    #[default]
    Anthropic,
    Cerebras,
    DeepSeek,
    Groq,
    OpenAI,
    Llamafile,
    Ollama,
    LmStudio,
    XAI,
}

#[derive(
    Serialize,
    Deserialize,
    Debug,
    PartialEq,
    Default,
    Clone,
    Copy,
    IntoStaticStr,
    EnumString,
    Display,
)]
pub enum ApiType {
    Anthropic,
    #[default]
    OpenAI,
}

// A builder struct for configuring and executing requests to a model
pub struct ModelBuilder {
    provider: Provider,
    model: Models,
    temperature: Option<f32>,
    max_tokens: Option<u16>,
    ctx_size: Option<u16>,
    top_p: Option<f32>,
    top_k: Option<u16>,
    schema: Option<String>,
    // Add other parameters as needed
}

impl ModelBuilder {
    // Set temperature
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    // Set max tokens
    pub fn with_max_tokens(mut self, max_tokens: u16) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    // Set context size
    pub fn with_ctx(mut self, ctx_size: u16) -> Self {
        self.ctx_size = Some(ctx_size);
        self
    }

    // Set top_p
    pub fn with_top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }

    // Set top_k
    pub fn with_top_k(mut self, top_k: u16) -> Self {
        self.top_k = Some(top_k);
        self
    }
    pub fn with_schema(mut self, schema: String) -> Self {
        self.schema = Some(schema);
        self
    }

    // Build the provider based on the API type
    pub fn build(&self) -> Result<OpenAiProvider, ProviderError> {
        match self.provider.api_type {
            ApiType::OpenAI => {
                // Create a provider config
                let config = Arc::new(ProviderConfig {
                    api_type: self.provider.api_type,
                    api_key: Some(self.provider.api_key.clone()),
                    api_base_url: Some(self.provider.base_url.clone()),
                    model: self.model.model.clone(),
                    temperature: self
                        .temperature
                        .or_else(|| self.model.params.as_ref().and_then(|p| p.temperature)),
                    max_tokens: self.max_tokens.or_else(|| {
                        self.model
                            .params
                            .as_ref()
                            .and_then(|p| p.max_tokens.map(|t| t as u16))
                    }),
                    schema: self.schema.clone(),
                    // Add other parameters as needed
                });

                // Create the OpenAI provider
                OpenAiProvider::new(config)
            }
            ApiType::Anthropic => {
                // For future implementation
                Err(ProviderError::Configuration(
                    "Anthropic API not yet implemented".to_string(),
                ))
            }
        }
    }
}

impl Provider {
    pub fn provider(provider_type: ProviderType) -> Self {
        match provider_type {
            ProviderType::OpenAI => Self {
                name: ProviderType::OpenAI.to_string(),
                base_url: "https://api.openai.com/v1".to_string(),
                api_key: "".to_string(),
                provider_type: ProviderType::OpenAI,
                models: vec!["gpt4".into()],
                default_model: "gpt4".into(),
                api_type: ApiType::OpenAI,
            },
            ProviderType::Ollama => Self {
                name: ProviderType::Ollama.to_string(),
                base_url: "http://127.0.0.1:1143/v1".to_string(),
                api_key: "xxx".to_string(),
                provider_type: ProviderType::Ollama,
                models: vec!["granite3.2".into(), "qwen2.5".into(), "gemma3".into()],
                default_model: "granite3.2".into(),
                api_type: ApiType::OpenAI,
            },
            ProviderType::LmStudio => Self {
                name: ProviderType::LmStudio.to_string(),
                base_url: "http://127.0.0.1:1234/v1".to_string(),
                api_key: "xxx".to_string(),
                provider_type: ProviderType::Ollama,

                models: vec![
                    "damienclere/granite-3.2-2b-instruct-4bit".into(),
                    "bartowski/Tesslate_Gradience-T1-3B-preview-GGUF".into(),
                    "codestral-22b-v0.1".into(),
                    "t3q-qwen2.5-14b-v1.0-e3".into(),
                    "gonzo404/MN-GRAND-Gutenberg-Lyra4-Lyra-12B-DARKNESS-mlx-4Bit".into(),
                    "mlx-community/mistral-small-3.1-24b-instruct-2503".into(),
                    "arcee-agent".into(),
                    "nxcode-cq-7b-orpo".into(),
                    "kieron-buyskes/Josiefied-Qwen2.5-Coder-14B-Instruct-abliterated-v1-mlx-4Bit"
                        .into(),
                    "phi-4".into(),
                    "qwen2.5-coder-14b-houdini_vex_functions".into(),
                    "alexgusevski/Selene-1-Mini-Llama-3.1-8B-q3-mlx".into(), // decision maker
                    "qwen2.5-7b-instruct-1m".into(),
                    "qwq-coder-instruct-mlx".into(),
                    "fuseo1-deepseekr1-qwen2.5-coder-14b-preview-mixed_3_6".into(),
                    "alexgusevski/Selene-1-Mini-Llama-3.1-8B-q3-mlx".into(),
                    "reasoning-ties-coder-v1.1".into(),
                    "lmstudio-community/internlm3-8b-instruct".into(),
                    "auto-rag-llama-3-8b-instruct-i1".into(),
                    "calme-3.2-llamaloi-3b".into(),
                    "aya-expanse-8b".into(),
                    "fluentlylm-prinum".into(),
                    "awqward2.5-32b-instruct".into(),
                    "mradermacher/Calme-4x7B-MoE-v0.2-i1-GGUF".into(),
                ],
                default_model: "granite3.2".into(),
                api_type: ApiType::OpenAI,
            },
            // ProviderType::Llamafile => Self::provider_llamafile(),
            _ => Self::default(),
        }
    }

    // Create a ModelBuilder with the specified model
    pub fn with_model(&self, model_name: &str) -> Option<ModelBuilder> {
        // Find the model
        let custom_model = Models {
            name: model_name.to_string(),
            model: model_name.to_string(),
            params: None,
        };
        let model = self
            .models
            .iter()
            .find(|m| m.name == model_name || m.model == model_name)
            .unwrap_or(&custom_model)
            .clone();

        // Create and return the builder
        Some(ModelBuilder {
            provider: self.clone(),
            model,
            temperature: None,
            max_tokens: None,
            ctx_size: None,
            top_p: None,
            top_k: None,
            schema: None,
        })
    }

    // Create a ModelBuilder with the default model
    pub fn with_default_model(&self) -> ModelBuilder {
        ModelBuilder {
            provider: self.clone(),
            model: self.default_model.clone(),
            temperature: None,
            max_tokens: None,
            ctx_size: None,
            top_p: None,
            top_k: None,
            schema: None,
        }
    }
}

impl Default for Provider {
    fn default() -> Self {
        Provider::provider(ProviderType::OpenAI)
    }
}
