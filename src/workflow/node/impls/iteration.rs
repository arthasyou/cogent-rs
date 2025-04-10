use std::collections::HashMap;

use serde_json::Value;

use crate::workflow::node::{NodeBehavior, NodeCommonFields};

#[derive(Debug, Clone)]
pub struct MetaData {
    // 目前无字段，占位可扩展
}

#[derive(Debug, Clone)]
pub struct IterationState {
    pub iteration_node_id: String,
    pub index: usize,
    pub inputs: HashMap<String, Value>,
    pub metadata: MetaData,
}

#[derive(Clone, Debug)]
pub struct IterationNodeData {
    pub common: NodeCommonFields,
    pub start_node_id: Option<String>,
}

impl NodeBehavior for IterationNodeData {
    fn common(&self) -> &NodeCommonFields {
        &self.common
    }
}
