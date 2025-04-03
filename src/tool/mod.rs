pub mod builtin;
pub mod tool_collection;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
pub use tool_collection::ToolCollection;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolParam {
    pub name: String,
    pub description: String,
    pub parameters: Option<Value>, // 结构化参数，可为 JSON Schema 等
}

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn parameters(&self) -> Option<Value> {
        None
    }

    fn to_param(&self) -> ToolParam {
        ToolParam {
            name: self.name().to_string(),
            description: self.description().to_string(),
            parameters: self.parameters(),
        }
    }

    async fn execute(&self, kwargs: Value) -> ToolResult;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub output: Option<String>,
    pub error: Option<String>,
    pub base64_image: Option<String>,
    pub system: Option<String>,
}

impl ToolResult {
    pub fn is_success(&self) -> bool {
        self.error.is_none()
    }

    pub fn merge(&self, other: &ToolResult) -> ToolResult {
        ToolResult {
            output: self.output.clone().or(other.output.clone()),
            error: self.error.clone().or(other.error.clone()),
            base64_image: self.base64_image.clone().or(other.base64_image.clone()),
            system: self.system.clone().or(other.system.clone()),
        }
    }

    pub fn replace(&self, update: ToolResult) -> ToolResult {
        ToolResult {
            output: update.output.or_else(|| self.output.clone()),
            error: update.error.or_else(|| self.error.clone()),
            base64_image: update.base64_image.or_else(|| self.base64_image.clone()),
            system: update.system.or_else(|| self.system.clone()),
        }
    }
}

impl std::fmt::Display for ToolResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(err) = &self.error {
            write!(f, "Error: {}", err)
        } else if let Some(out) = &self.output {
            write!(f, "{}", out)
        } else {
            write!(f, "")
        }
    }
}

pub type CLIResult = ToolResult;

pub fn tool_failure(message: impl Into<String>) -> ToolResult {
    ToolResult {
        output: None,
        error: Some(message.into()),
        base64_image: None,
        system: None,
    }
}
