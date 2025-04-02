use super::LLMProvider;
use crate::error::{Error, Result};
use crate::types::Message;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::{Value, json};

pub struct OllamaProvider {
    client: Client,
}

impl Default for OllamaProvider {
    fn default() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

#[async_trait]
impl LLMProvider for OllamaProvider {
    async fn ask(
        &self,
        messages: Vec<Message>,
        temperature: Option<f32>,
        _stream: bool,
    ) -> Result<String> {
        let prompt = messages
            .iter()
            .filter_map(|m| m.content.as_ref())
            .filter_map(|v| Some(v.as_str()))
            .collect::<Vec<_>>()
            .join("\n");

        let body = json!({
            "model": "llama3.2",
            "prompt": prompt,
            "temperature": temperature.unwrap_or(0.7),
            "stream": false
        });

        let res = self
            .client
            .post("http://localhost:11434/api/generate")
            .json(&body)
            .send()
            .await?;

        let json: Value = res.json().await?;
        Ok(json["response"].as_str().unwrap_or("").to_string())
    }

    async fn ask_with_images(
        &self,
        _messages: Vec<Message>,
        _images: Vec<Value>,
        _temperature: Option<f32>,
        _stream: bool,
    ) -> Result<String> {
        Err(Error::ErrorMessage(
            "Ollama does not support image input".to_owned(),
        ))
    }

    async fn ask_tool(
        &self,
        _messages: Vec<Message>,
        _tools: Vec<Value>,
        _tool_choice: Option<String>,
        _temperature: Option<f32>,
    ) -> Result<Message> {
        Err(Error::ErrorMessage(
            "Tool calling not implemented in OllamaProvider".to_owned(),
        ))
    }

    fn count_tokens(&self, text: &str) -> usize {
        text.split_whitespace().count()
    }

    fn count_message_tokens(&self, messages: &[Message]) -> usize {
        messages
            .iter()
            .map(|m| {
                self.count_tokens(
                    m.content
                        .as_ref()
                        .and_then(|v| Some(v.as_str()))
                        .unwrap_or(""),
                )
            })
            .sum()
    }

    fn check_token_limit(&self, _input_tokens: usize) -> bool {
        true
    }

    fn get_limit_error_message(&self, _input_tokens: usize) -> String {
        "Token limit exceeded".to_string()
    }

    fn format_messages(
        &self,
        messages: Vec<Message>,
        _supports_images: bool,
    ) -> Result<Vec<Message>> {
        Ok(messages)
    }

    fn update_token_count(&self, _input: usize, _completion: usize) {}
}
