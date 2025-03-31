use async_trait::async_trait;

pub mod tool_call_agent;

#[derive(Debug, Clone, PartialEq)]
pub enum AgentState {
    Idle,
    Running,
    Finished,
    Error,
}

#[derive(Debug, Clone)]
pub enum Role {
    User,
    Assistant,
    System,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Default)]
pub struct Memory {
    pub messages: Vec<Message>,
}

impl Memory {
    pub fn add_message(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    pub fn last_content(&self) -> Option<String> {
        self.messages.last().map(|m| m.content.clone())
    }
}

#[async_trait]
pub trait Agent {
    async fn think(&mut self) -> bool;
    async fn act(&mut self) -> String;

    async fn step(&mut self) -> String {
        if self.think().await {
            self.act().await
        } else {
            "No action needed.".to_string()
        }
    }

    async fn run(&mut self, request: Option<String>) -> String;
}
