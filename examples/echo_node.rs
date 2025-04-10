use cogent_rs::workflow::node::{
    BaseNode, NodeCommonFields, NodeEvent,
    impls::{EchoNode, EchoNodeData},
};

fn main() {
    let data = EchoNodeData {
        common: NodeCommonFields {
            title: "Echo".into(),
            desc: Some("Echoes input".into()),
            error_strategy: None,
            default_value: None,
            version: "1".into(),
            retry_config: Default::default(),
        },
        message: "hello workflow".into(),
    };

    let base_node = BaseNode {
        id: "n1".into(),
        node_id: "echo_1".into(),
        node_data: data,
        node_type: "echo".into(),
    };

    let mut echo_node = EchoNode::new(base_node);
    let result = echo_node.run();
    for event in result {
        let NodeEvent::RunCompleted(result) = event;
        println!("Node run completed: {:?}", result);
    }
}
