// src/models.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct OllamaOptions {
    pub temperature: f32,
}

#[derive(Serialize)]
pub struct OllamaRequest {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
    pub keep_alive: i32,
    pub options: OllamaOptions,
}

#[derive(Deserialize)]
pub struct OllamaResponse {
    pub response: String,
}
