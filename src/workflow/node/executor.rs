use std::collections::HashMap;

use super::{
    behavior::NodeBehavior,
    event::{NodeEvent, NodeRunResult},
};

pub struct BaseNode<T: NodeBehavior> {
    pub id: String,
    pub node_id: String,
    pub node_data: T,
    pub node_type: String,
}

impl<T: NodeBehavior> BaseNode<T> {
    pub fn run_internal(&mut self) -> Result<NodeRunResult, String> {
        Err("Not implemented".to_string())
    }

    pub fn run(&mut self) -> Vec<NodeEvent> {
        match self.run_internal() {
            Ok(result) => vec![NodeEvent::RunCompleted(result)],
            Err(e) => vec![NodeEvent::RunCompleted(NodeRunResult::failed(&e))],
        }
    }

    pub fn should_continue_on_error(&self, continue_types: &[&str]) -> bool {
        self.node_data.common().error_strategy.is_some()
            && continue_types.contains(&self.node_type.as_str())
    }

    pub fn should_retry(&self, retry_types: &[&str]) -> bool {
        self.node_data.common().retry_config.retry_enabled
            && retry_types.contains(&self.node_type.as_str())
    }

    pub fn extract_variable_selector_to_variable_mapping(
        _config: &HashMap<String, String>,
    ) -> HashMap<String, Vec<String>> {
        HashMap::new()
    }

    pub fn get_default_config(
        _filters: Option<&HashMap<String, String>>,
    ) -> HashMap<String, String> {
        HashMap::new()
    }
}
