use serde_json::Value;

#[derive(Clone, Debug)]
pub enum DefaultValueType {
    String,
    Number,
    Object,
    ArrayNumber,
    ArrayString,
    ArrayObject,
    ArrayFiles,
}

#[derive(Clone, Debug)]
pub enum DefaultValue {
    String { key: String, value: String },
    Number { key: String, value: f64 },
    Object { key: String, value: Value },
    ArrayString { key: String, value: Vec<String> },
    ArrayNumber { key: String, value: Vec<f64> },
    ArrayObject { key: String, value: Vec<Value> },
    ArrayFiles { key: String, value: Vec<String> },
}

#[derive(Clone, Debug, Default)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub retry_interval: u32, // milliseconds
    pub retry_enabled: bool,
}

impl RetryConfig {
    pub fn retry_interval_seconds(&self) -> f64 {
        self.retry_interval as f64 / 1000.0
    }
}

#[derive(Clone, Debug)]
pub struct NodeCommonFields {
    pub title: String,
    pub desc: Option<String>,
    pub error_strategy: Option<String>,
    pub default_value: Option<Vec<DefaultValue>>,
    pub version: String,
    pub retry_config: RetryConfig,
}
