use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Response {
    pub choices: Vec<Choice>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Choice {
    pub index: usize,
    pub message: Message,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Message {
    pub role: String,
    pub content: Option<String>,

    #[serde(default)]
    pub tool_calls: Vec<ToolCalls>,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolCalls {
    pub id: String,
    pub r#type: String,
    pub function: Function,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub arguments: String,
}

#[allow(dead_code)]
impl Response {
    pub fn first_choice(&self) -> Option<&Choice> {
        self.choices.first()
    }
    pub fn select_choice(&self, idx: usize) -> Option<&Choice> {
        self.choices.get(idx)
    }
}

#[allow(dead_code)]
impl Message {
    pub fn first_tool(&self) -> Option<&ToolCalls> {
        self.tool_calls.first()
    }

    pub fn remove_tools(&mut self) {
        self.tool_calls.clear();
    }

    pub fn all_tools_name(&self) -> Vec<String> {
        self.tool_calls
            .iter()
            .map(|tool| tool.function.name.clone())
            .collect::<Vec<String>>()
    }
}

#[cfg(test)]
mod tests {
    use crate::llm::Response;

    #[test]
    fn response_test() {
        let res = Response::default();
        let choice = res.select_choice(0);
    }
}
