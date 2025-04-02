use cogent_rs::{
    agent::{Agent, AgentState, Memory, tool_call_agent::ToolCallAgent},
    llm::LLM,
};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let llm = LLM::default();
    let memory = Memory::default();

    let mut agent = ToolCallAgent {
        name: "TestAgent".to_string(),
        memory,
        state: AgentState::Idle,
        tools: HashMap::new(),
        llm,
    };

    let output = agent
        .run(Some("What is the weather today?".to_string()))
        .await;
    println!("{}", output);
}
