use async_trait::async_trait;

use super::{Agent, AgentState, Memory};
use crate::{
    llm::LLM,
    tool::ToolCollection,
    types::{Message, Role},
};

pub struct ToolCallAgent {
    pub name: String,
    pub memory: Memory,
    pub llm: LLM,
    pub state: AgentState,
    pub tools: ToolCollection,
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
            base64_image: None,
            audio_file: None,
            asr_text: None,
            name: None,
            tool_call_id: None,
            tool_calls: None,
        });

        response.contains("use tool")
    }

    async fn act(&mut self) -> String {
        // TODO: 解析 tool_call 请求，调用 self.tools.execute(name, args)
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

        for i in 0 .. 10 {
            let res = self.step().await;
            results.push(format!("Step {}: {}", i + 1, res));
            if res.contains("Executed") {
                break;
            }
        }

        results.join("\n")
    }
}
