use std::collections::HashMap;

use serde_json::Value;

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
