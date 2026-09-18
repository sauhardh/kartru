use serde::{Deserialize, Serialize};

use crate::llm::ToolCalls;

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AgentMessage {
    pub role: String,
    pub content: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCalls>>,
}

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct Agent {
    pub messages: Vec<AgentMessage>,
}

#[allow(dead_code)]
impl Agent {
    pub fn insert_new(
        &mut self,
        role: String,
        content: Option<String>,
        tool_call_id: Option<String>,
        tool_calls: Option<Vec<ToolCalls>>,
    ) -> &Self {
        let message = AgentMessage {
            role,
            content,
            tool_call_id,
            tool_calls,
        };
        self.messages.push(message);

        self
    }

    pub fn insert_msg(&mut self, message: AgentMessage) {
        self.messages.push(message)
    }

    pub fn get_messages(self) -> Vec<AgentMessage> {
        self.messages
    }

    /// This is user prompt. Tool calling are not proceeded by the request of user (prompter).
    /// So, `tool_call_id`: `None` and `tool_calls`: `None`. Hence, they are not in parameters also.
    pub fn insert_user(&mut self, _content: String) {
        let message = AgentMessage {
            role: "user".to_string(),
            content: Some(_content),
            tool_call_id: None,
            tool_calls: None,
        };

        self.messages.push(message);
    }

    /// Only the tool that has been executed on request by AI.
    /// It itself doesnot request for tools. So, `tool_calls` is `None` and not in parameters.
    pub fn insert_tool(&mut self, _content: String, _tool_call_id: String) {
        let message = AgentMessage {
            role: "tool".to_string(),
            content: Some(_content),
            tool_call_id: Some(_tool_call_id),
            tool_calls: None,
        };

        self.messages.push(message);
    }

    /// Message sent by an AI.
    /// AI doesnot execute tools, it only requests. So, `tool_call_id`: `None` and not in
    /// parameters.
    /// However, there is `tool_calls` because it's the tools that are requested by AI to run on
    /// computer.
    pub fn insert_assistant(
        &mut self,
        content: Option<String>,
        tool_calls: Option<Vec<ToolCalls>>,
    ) {
        let message = AgentMessage {
            role: "assistant".to_string(),
            content,
            tool_call_id: None,
            tool_calls,
        };

        self.messages.push(message);
    }
}
