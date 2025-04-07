use super::NodeBehavior;
use crate::workflow::node::NodeCommonFields;

#[derive(Clone, Debug)]
pub struct LoopNodeData {
    pub common: NodeCommonFields,
    pub start_node_id: Option<String>,
}

impl NodeBehavior for LoopNodeData {
    fn common(&self) -> &NodeCommonFields {
        &self.common
    }
}
