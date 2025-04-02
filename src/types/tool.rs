use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tool {
    #[serde(rename = "type")]
    pub tool_type: String, // 通常为 "function"
    pub function: FunctionSpec,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionSpec {
    pub name: String,
    pub description: String,
    pub parameters: FunctionParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionParameters {
    #[serde(rename = "type")]
    pub param_type: String, // 一般为 "object"
    pub properties: HashMap<String, FunctionProperty>,
    pub required: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionProperty {
    #[serde(rename = "type")]
    pub property_type: String, // 如 "string"、"number"、"boolean"
    // 可选：添加 description
    pub description: Option<String>,
}
