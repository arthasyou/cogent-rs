use super::Role;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: Option<String>,           // 文本内容
    pub base64_image: Option<String>,      // base64 编码的图片
    pub audio_file: Option<String>,        // base64 编码的音频或文件路径
    pub asr_text: Option<String>,          // 语音识别结果
    pub name: Option<String>,              // 可选的名字
    pub tool_call_id: Option<String>,      // 工具响应时，关联调用 ID
    pub tool_calls: Option<Vec<ToolCall>>, // 工具调用请求（由 assistant 发起）
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type", default = "default_tool_type")]
    pub tool_type: String, // 一般是 "function"
    pub function: Function,
}

fn default_tool_type() -> String {
    "function".to_string()
}
