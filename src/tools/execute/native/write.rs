use serde::Deserialize;

use crate::tools::define::ToolDefinition;
use crate::tools::error::ToolError;
use crate::tools::execute::ExecutableTool;
use async_trait::async_trait;

#[allow(dead_code)]
pub struct WriteTool;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct WriteArgs {
    pub file_path: String,
    pub content: String,
}

#[async_trait]
impl ExecutableTool for WriteTool {
    fn name(&self) -> &'static str {
        "Write"
    }

    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: self.name().to_string(),
            description: "Write content to a file".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "file_path": {
                        "type": "string",
                        "description": "The path to the file to write to"
                    },
                    "content":{
                        "type":"string",
                        "description": "The content to write to the file"
                    },
                },
                "required": ["file_path", "content"]
            }),
        }
    }

    async fn execute(&self, arguments: &str) -> Result<String, ToolError> {
        let args: WriteArgs = serde_json::from_str(arguments)?;
        tokio::fs::write(args.file_path, args.content).await?;

        Ok("File written successfully.".to_string())
    }
}
