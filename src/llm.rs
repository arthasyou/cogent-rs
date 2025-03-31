use crate::agent::Message;

pub struct LLM;

impl LLM {
    pub fn new() -> Self {
        Self
    }

    pub async fn ask(&self, messages: &[Message]) -> String {
        // 模拟逻辑：如果用户输入中有 "add"，就返回调用 add 工具
        if messages.iter().any(|m| m.content.contains("add")) {
            "I want to use the 'add' tool.".to_string()
        } else {
            "No action needed.".to_string()
        }
    }
}
