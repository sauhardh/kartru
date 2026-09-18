use serde::Deserialize;

use crate::tools::error::ToolError;
use crate::tools::execute::ExecutableTool;
use std::fs::write;

#[allow(dead_code)]
pub struct WriteTool;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct WriteArgs {
    pub file_path: String,
    pub content: String,
}

impl ExecutableTool for WriteTool {
    fn name(&self) -> &'static str {
        "write"
    }

    fn execute(&self, arguments: &str) -> Result<String, ToolError> {
        let args: WriteArgs = serde_json::from_str(arguments)?;
        write(args.file_path, args.content)?;

        Ok("File written successfully.".to_string())
    }
}
