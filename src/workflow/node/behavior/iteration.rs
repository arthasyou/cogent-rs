use super::NodeBehavior;
use crate::workflow::node::NodeCommonFields;

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
