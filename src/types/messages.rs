use serde::{Deserialize, Serialize};

use super::Role;

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

impl Message {
    pub fn user_message(content: String, base64_image: Option<String>) -> Self {
        Self {
            role: Role::User,
            content: Some(content),
            base64_image,
            audio_file: None,
            asr_text: None,
            name: None,
            tool_call_id: None,
            tool_calls: None,
        }
    }

    pub fn system_message(content: String) -> Self {
        Self {
            role: Role::System,
            content: Some(content),
            base64_image: None,
            audio_file: None,
            asr_text: None,
            name: None,
            tool_call_id: None,
            tool_calls: None,
        }
    }

    pub fn assistant_message(content: Option<String>, base64_image: Option<String>) -> Self {
        Self {
            role: Role::Assistant,
            content,
            base64_image,
            audio_file: None,
            asr_text: None,
            name: None,
            tool_call_id: None,
            tool_calls: None,
        }
    }

    pub fn tool_message(
        content: String,
        name: String,
        tool_call_id: String,
        base64_image: Option<String>,
    ) -> Self {
        Self {
            role: Role::Tool,
            content: Some(content),
            base64_image,
            audio_file: None,
            asr_text: None,
            name: Some(name),
            tool_call_id: Some(tool_call_id),
            tool_calls: None,
        }
    }

    pub fn from_tool_calls(
        tool_calls: Vec<ToolCall>,
        content: Option<String>,
        base64_image: Option<String>,
    ) -> Self {
        Self {
            role: Role::Assistant,
            content,
            base64_image,
            audio_file: None,
            asr_text: None,
            name: None,
            tool_call_id: None,
            tool_calls: Some(tool_calls),
        }
    }
}
