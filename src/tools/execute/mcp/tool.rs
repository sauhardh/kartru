use async_trait::async_trait;
use serde_json::Value;
use std::sync::Arc;

use super::client::McpClient;
use crate::tools::define::ToolDefinition;
use crate::tools::error::ToolError;
use crate::tools::execute::ExecutableTool;

pub struct McpTool {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
    pub client: Arc<McpClient>,
}

#[async_trait]
impl ExecutableTool for McpTool {
    fn name(&self) -> &str {
        &self.name
    }

    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: self.name.clone(),
            description: self.description.clone(),
            parameters: self.input_schema.clone(),
        }
    }

    async fn execute(&self, arguments: &str) -> Result<String, ToolError> {
        let parsed_args: Value = serde_json::from_str(arguments)?;

        // Call the MCP tool via rmcp peer
        let call_params = rmcp::model::CallToolRequestParam {
            name: self.name.clone().into(),
            arguments: parsed_args.as_object().cloned(),
        };

        let result = self
            .client
            .service()
            .peer()
            .call_tool(call_params)
            .await
            .map_err(|e| ToolError::Mcp(e.to_string()))?;

        let mut output = String::new();

        for content in result.content {
            if let Some(text_content) = content.as_text() {
                output.push_str(&text_content.text);
                output.push('\n');
            }
        }

        Ok(output)
    }
}

