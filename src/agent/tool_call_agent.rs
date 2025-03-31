use async_trait::async_trait;

use super::{Agent, AgentState, Memory, Message, Role};
use crate::llm::LLM;
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
        let response = self.llm.ask(&self.memory.messages).await;
        self.memory.add_message(Message {
            role: Role::Assistant,
            content: response.clone(),
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
                content: input,
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
