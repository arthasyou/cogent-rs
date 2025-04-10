use std::collections::HashMap;

use serde_json::json;

use crate::workflow::node::base::{
    behavior::NodeBehavior,
    data::NodeCommonFields,
    event::{NodeEvent, NodeRunResult, WorkflowNodeExecutionStatus},
    executor::BaseNode,
};

/// 节点数据结构
#[derive(Debug, Clone)]
pub struct EchoNodeData {
    pub common: NodeCommonFields,
    pub message: String,
}

impl NodeBehavior for EchoNodeData {
    fn common(&self) -> &NodeCommonFields {
        &self.common
    }
}

/// Echo 节点执行器
pub struct EchoNode {
    pub base: BaseNode<EchoNodeData>,
}

impl EchoNode {
    pub fn new(base: BaseNode<EchoNodeData>) -> Self {
        Self { base }
    }

    pub fn run(&mut self) -> Vec<NodeEvent> {
        let msg = self.base.node_data.message.clone();

        let mut output = HashMap::new();
        output.insert("echo".to_string(), json!(msg));

        let result = NodeRunResult {
            status: WorkflowNodeExecutionStatus::Success,
            inputs: None,
            process_data: None,
            outputs: Some(output),
            metadata: None,
            llm_usage: None,
            edge_source_handle: None,
            error: None,
            error_type: None,
            retry_index: 0,
        };

        vec![NodeEvent::RunCompleted(result)]
    }
}
