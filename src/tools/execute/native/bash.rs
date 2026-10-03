use serde::Deserialize;

use crate::tools::define::ToolDefinition;
use crate::tools::error::ToolError;
use crate::tools::execute::ExecutableTool;
use async_trait::async_trait;
use std::process::Command;

#[allow(dead_code)]
pub struct BashTool;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct BashArgs {
    pub command: String,
}

#[async_trait]
impl ExecutableTool for BashTool {
    fn name(&self) -> &str {
        "Bash"
    }

    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: self.name().to_string(),
            description: "Execute a shell command".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "command": {
                        "type": "string",
                        "description": "The command to execute"
                    },
                },
                "required": ["command"]
            }),
        }
    }

    async fn execute(&self, arguments: &str) -> Result<String, ToolError> {
        let args: BashArgs = serde_json::from_str(arguments)?;
        let output = Command::new("bash").arg("-c").arg(args.command).output()?;

        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }
}
