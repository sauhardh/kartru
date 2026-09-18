use serde::Deserialize;

use crate::tools::error::ToolError;
use crate::tools::execute::ExecutableTool;
use std::process::Command;

#[allow(dead_code)]
pub struct BashTool;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct BashArgs {
    pub command: String,
}

impl ExecutableTool for BashTool {
    fn name(&self) -> &'static str {
        "bash"
    }

    fn execute(&self, arguments: &str) -> Result<String, ToolError> {
        let args: BashArgs = serde_json::from_str(arguments)?;
        let output = Command::new("bash").arg("-c").arg(args.command).output()?;

        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }
}
