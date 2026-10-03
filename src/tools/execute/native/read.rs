use serde::{Deserialize, Serialize};

use crate::tools::define::ToolDefinition;
use crate::tools::execute::ExecutableTool;
use crate::tools::execute::ToolError;
use async_trait::async_trait;

#[allow(dead_code)]
pub struct ReadTool;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct ReadArgs {
    pub file_path: String,
}

#[async_trait]
impl ExecutableTool for ReadTool {
    fn name(&self) -> &str {
        "Read"
    }

    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: self.name().to_string(),
            description: "Read and return the contents of a file".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "file_path": {
                        "type": "string",
                        "description": "The path to the file to read"
                    }
                },
                "required": ["file_path"]
            }),
        }
    }

    async fn execute(&self, arguments: &str) -> Result<String, ToolError> {
        let args: ReadArgs = serde_json::from_str(arguments)?;

        let content = tokio::fs::read_to_string(args.file_path).await?;

        Ok(content)
    }
}
