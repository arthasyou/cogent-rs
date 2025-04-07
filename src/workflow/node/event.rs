#[derive(Debug, Clone)]
pub enum WorkflowNodeExecutionStatus {
    Success,
    Failed,
}

#[derive(Debug, Clone)]
pub struct NodeRunResult {
    pub status: WorkflowNodeExecutionStatus,
    pub error: Option<String>,
    pub error_type: Option<String>,
}

impl NodeRunResult {
    pub fn failed(msg: &str) -> Self {
        Self {
            status: WorkflowNodeExecutionStatus::Failed,
            error: Some(msg.to_string()),
            error_type: Some("WorkflowNodeError".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum NodeEvent {
    RunCompleted(NodeRunResult),
    // 可扩展更多类型
}
