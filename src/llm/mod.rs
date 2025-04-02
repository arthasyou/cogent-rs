mod provider;

use crate::error::Result;
use crate::types::Message;
use provider::{LLMProvider, OllamaProvider};
use serde_json::Value;
use std::sync::{Arc, Mutex};

// Core Config
pub struct LLMConfig {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: usize,
    pub max_input_tokens: Option<usize>,
    pub api_type: String,
    pub api_version: Option<String>,
}

// Token tracker
#[derive(Default)]
pub struct TokenUsage {
    pub input: usize,
    pub completion: usize,
}

// LLM Trait

// Central LLM struct managing singleton instances
pub struct LLM {
    pub config: LLMConfig,
    pub client: Arc<dyn LLMProvider>,
    pub token_usage: Arc<Mutex<TokenUsage>>,
}

impl LLM {
    pub fn new(config: LLMConfig, provider: Arc<dyn LLMProvider>) -> Self {
        Self {
            config,
            client: provider,
            token_usage: Arc::new(Mutex::new(TokenUsage::default())),
        }
    }

    pub fn default() -> Self {
        let config = LLMConfig {
            api_key: "".into(),
            base_url: "http://localhost:11434".into(),
            model: "llama3:instruct".into(),
            temperature: 0.7,
            max_tokens: 1024,
            max_input_tokens: Some(4096),
            api_type: "ollama".into(),
            api_version: None,
        };

        let provider = Arc::new(OllamaProvider::default());
        LLM::new(config, provider)
    }

    //     let dummy = Arc::new(crate::llm::DummyProvider {});
    //     LLM::new(config, dummy)
    // }

    // Entry points
    pub async fn ask(
        &self,
        messages: Vec<Message>,
        temperature: Option<f32>,
        stream: bool,
    ) -> Result<String> {
        self.client.ask(messages, temperature, stream).await
    }

    pub async fn ask_with_images(
        &self,
        messages: Vec<Message>,
        images: Vec<Value>,
        temperature: Option<f32>,
        stream: bool,
    ) -> Result<String> {
        self.client
            .ask_with_images(messages, images, temperature, stream)
            .await
    }

    pub async fn ask_tool(
        &self,
        messages: Vec<Message>,
        tools: Vec<Value>,
        tool_choice: Option<String>,
        temperature: Option<f32>,
    ) -> Result<Message> {
        self.client
            .ask_tool(messages, tools, tool_choice, temperature)
            .await
    }
}

// You should implement `LLMProvider` trait for each backend (OpenAI, Azure, Bedrock, etc).
// Each provider can have its own logic and tokenizer handling.
