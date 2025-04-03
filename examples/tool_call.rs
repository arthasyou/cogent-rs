use cogent_rs::{
    agent::{Agent, AgentState, Memory, tool_call_agent::ToolCallAgent},
    llm::LLM,
    tool::{ToolCollection, builtin::echo_tool::EchoTool},
};

#[tokio::main]
async fn main() {
    let llm = LLM::default();
    let memory = Memory::default();

    let mut tools = ToolCollection::new();
    tools.add_tool(EchoTool {});

    let mut agent = ToolCallAgent {
        name: "TestAgent".to_string(),
        memory,
        state: AgentState::Idle,
        tools,
        llm,
    };

    let output = agent
        .run(Some("What is the weather today?".to_string()))
        .await;
    println!("{}", output);
}
