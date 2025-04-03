use std::collections::HashMap;

use serde_json::Value;

use super::{Tool, ToolParam, ToolResult, tool_failure};

pub struct ToolCollection {
    tools: Vec<Box<dyn Tool>>,
    tool_map: HashMap<String, usize>,
}

impl ToolCollection {
    pub fn new() -> Self {
        Self {
            tools: vec![],
            tool_map: HashMap::new(),
        }
    }

    pub fn add_tool<T: Tool + 'static>(&mut self, tool: T) {
        let name = tool.name().to_string();
        self.tool_map.insert(name.clone(), self.tools.len());
        self.tools.push(Box::new(tool));
    }

    pub fn get_tool(&self, name: &str) -> Option<&dyn Tool> {
        self.tool_map.get(name).map(|&i| self.tools[i].as_ref())
    }

    pub async fn execute(&self, name: &str, input: Value) -> ToolResult {
        match self.get_tool(name) {
            Some(tool) => tool.execute(input).await,
            None => tool_failure(format!("Tool {} is invalid", name)),
        }
    }

    pub async fn execute_all(&self) -> Vec<ToolResult> {
        let mut results = vec![];
        for tool in &self.tools {
            results.push(tool.execute(serde_json::json!({})).await);
        }
        results
    }

    pub fn to_params(&self) -> Vec<ToolParam> {
        self.tools.iter().map(|t| t.to_param()).collect()
    }
}
