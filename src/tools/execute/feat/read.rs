use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

use crate::tools::execute::ExecutableTool;
use crate::tools::execute::ToolError;

#[allow(dead_code)]
pub struct ReadTool;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct ReadArgs {
    pub file_path: String,
}

impl ExecutableTool for ReadTool {
    fn name(&self) -> &'static str {
        "read"
    }

    fn execute(&self, arguments: &str) -> Result<String, ToolError> {
        let args: ReadArgs = serde_json::from_str(arguments)?;

        let content = read_to_string(args.file_path)?;

        Ok(content)
    }
}
