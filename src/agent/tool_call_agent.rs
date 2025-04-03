use async_trait::async_trait;

use super::{Agent, AgentState, Memory};
use crate::{
    llm::LLM,
    types::{Message, Role},
};
use std::collections::HashMap;

pub struct ToolCallAgent {
    pub name: String,
    pub memory: Memory,
    pub llm: LLM,
    pub state: AgentState,
    pub tools: HashMap<String, fn(HashMap<String, String>) -> String>,
}

#[async_trait]
impl Agent for ToolCallAgent {
    async fn think(&mut self) -> bool {
        let response = self
            .llm
            .ask(self.memory.messages.clone(), None, false)
            .await
            .unwrap_or_else(|_| "Error".into());
        self.memory.add_message(Message {
            role: Role::Assistant,
            content: Some(response.clone()),
            // TODO: Handle base64_image if response includes an image
            base64_image: None,
            // TODO: Handle audio_file if response includes audio
            audio_file: None,
            // TODO: Handle asr_text if response includes ASR text
            asr_text: None,
            // TODO: Handle name if response includes a name
            name: None,
            // TODO: Handle tool_call_id if response includes a tool call ID
            tool_call_id: None,
            // TODO: Handle tool_calls if response includes tool calls
            tool_calls: None,
        });

        response.contains("use tool")
    }

    async fn act(&mut self) -> String {
        // tool 调用逻辑
        "Executed tool".into()
    }

    async fn run(&mut self, request: Option<String>) -> String {
        if let Some(input) = request {
            self.memory.add_message(Message {
                role: Role::User,
                content: Some(input),
                base64_image: None,
                audio_file: None,
                asr_text: None,
                name: None,
                tool_call_id: None,
                tool_calls: None,
            });
        }

        let mut results = vec![];

        for i in 0..10 {
            let res = self.step().await;
            results.push(format!("Step {}: {}", i + 1, res));
            if res.contains("Executed") {
                break;
            }
        }

        results.join("\n")
    }
}
