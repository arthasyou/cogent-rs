use async_trait::async_trait;
use serde_json::Value;

use crate::tool::{Tool, ToolResult};

pub struct EchoTool;

#[async_trait]
impl Tool for EchoTool {
    fn name(&self) -> &str {
        "echo"
    }

    fn description(&self) -> &str {
        "Echoes the input string"
    }

    async fn execute(&self, kwargs: Value) -> ToolResult {
        let msg = kwargs
            .get("text")
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        ToolResult {
            output: Some(format!("Echo: {}", msg)),
            error: None,
            base64_image: None,
            system: None,
        }
    }
}
