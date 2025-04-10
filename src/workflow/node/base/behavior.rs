use std::collections::HashMap;

use super::{DefaultValue, NodeCommonFields};

pub trait NodeBehavior {
    fn common(&self) -> &NodeCommonFields;

    fn default_value_dict(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        if let Some(defaults) = &self.common().default_value {
            for item in defaults.iter() {
                if let DefaultValue::String { key, value } = item {
                    map.insert(key.clone(), value.clone());
                }
            }
        }
        map
    }
}
