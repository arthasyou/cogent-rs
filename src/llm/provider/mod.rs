pub mod ollama_provider;
pub use ollama_provider::OllamaProvider;

use async_trait::async_trait;
use serde_json::Value;

use crate::error::Result;
use crate::types::Message;

#[async_trait]
pub trait LLMProvider: Send + Sync {
    async fn ask(
        &self,
        messages: Vec<Message>,
        temperature: Option<f32>,
        stream: bool,
    ) -> Result<String>;
    async fn ask_with_images(
        &self,
        messages: Vec<Message>,
        images: Vec<Value>,
        temperature: Option<f32>,
        stream: bool,
    ) -> Result<String>;
    async fn ask_tool(
        &self,
        messages: Vec<Message>,
        tools: Vec<Value>,
        tool_choice: Option<String>,
        temperature: Option<f32>,
    ) -> Result<Message>;
    fn count_tokens(&self, text: &str) -> usize;
    fn count_message_tokens(&self, messages: &[Message]) -> usize;
    fn check_token_limit(&self, input_tokens: usize) -> bool;
    fn get_limit_error_message(&self, input_tokens: usize) -> String;
    fn format_messages(
        &self,
        messages: Vec<Message>,
        supports_images: bool,
    ) -> Result<Vec<Message>>;
    fn update_token_count(&self, input: usize, completion: usize);
}
