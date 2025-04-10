use std::collections::HashMap;

use serde_json::Value;

#[derive(Debug, Clone)]
pub enum WorkflowNodeExecutionStatus {
    Success,
    Failed,
    Running,
}

#[derive(Debug, Clone)]
pub struct LlmUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Clone)]
pub struct NodeRunResult {
    pub status: WorkflowNodeExecutionStatus,

    pub inputs: Option<HashMap<String, Value>>,
    pub process_data: Option<HashMap<String, Value>>,
    pub outputs: Option<HashMap<String, Value>>,
    pub metadata: Option<HashMap<String, Value>>,

    pub llm_usage: Option<LlmUsage>,
    pub edge_source_handle: Option<String>,

    pub error: Option<String>,
    pub error_type: Option<String>,

    pub retry_index: u32,
}

impl NodeRunResult {
    pub fn success(outputs: HashMap<String, Value>) -> Self {
        Self {
            status: WorkflowNodeExecutionStatus::Success,
            inputs: None,
            process_data: None,
            outputs: Some(outputs),
            metadata: None,
            llm_usage: None,
            edge_source_handle: None,
            error: None,
            error_type: None,
            retry_index: 0,
        }
    }

    pub fn failed(msg: &str) -> Self {
        Self {
            status: WorkflowNodeExecutionStatus::Failed,
            inputs: None,
            process_data: None,
            outputs: None,
            metadata: None,
            llm_usage: None,
            edge_source_handle: None,
            error: Some(msg.to_string()),
            error_type: Some("WorkflowNodeError".to_string()),
            retry_index: 0,
        }
    }

    pub fn running() -> Self {
        Self {
            status: WorkflowNodeExecutionStatus::Running,
            inputs: None,
            process_data: None,
            outputs: None,
            metadata: None,
            llm_usage: None,
            edge_source_handle: None,
            error: None,
            error_type: None,
            retry_index: 0,
        }
    }

    pub fn with_outputs(mut self, outputs: HashMap<String, Value>) -> Self {
        self.outputs = Some(outputs);
        self
    }

    pub fn with_metadata(mut self, metadata: HashMap<String, Value>) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn with_error(mut self, error_msg: &str, error_type: Option<&str>) -> Self {
        self.error = Some(error_msg.to_string());
        self.error_type = Some(error_type.unwrap_or("WorkflowNodeError").to_string());
        self.status = WorkflowNodeExecutionStatus::Failed;
        self
    }
}

#[derive(Debug, Clone)]
pub enum NodeEvent {
    RunCompleted(NodeRunResult),
    // 可扩展更多类型
}
