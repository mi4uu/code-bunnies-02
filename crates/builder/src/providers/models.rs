use serde::{Deserialize, Serialize};
use std::string::ToString;

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
pub struct Models {
    pub name: String,
    pub model: String,
    pub params: Option<ModelParams>,
}
impl From<&str> for Models {
    fn from(model: &str) -> Self {
        Self {
            name: model.to_string(),
            model: model.to_string(),
            params: None,
        }
    }
}
impl From<String> for Models {
    fn from(model: String) -> Self {
        Self {
            name: model.clone(),
            model,
            params: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
pub struct ModelParams {
    pub ctx: Option<i32>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub top_k: Option<i32>,
    pub max_tokens: Option<i32>,
    pub stop: Option<String>,
    pub presence_penalty: Option<f32>,
    pub frequency_penalty: Option<f32>,
    pub n: Option<i32>,
    pub stream: Option<bool>,
}
