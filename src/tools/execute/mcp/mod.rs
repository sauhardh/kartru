mod client;
mod tool;

use std::collections::HashMap;

#[derive(Debug, serde::Deserialize)]
struct MCPConfigFile {
    #[serde(rename = "mcpServers")]
    mcp_servers: HashMap<String, McpServerConfig>,
}

#[derive(Debug, serde::Deserialize)]
struct McpServerConfig {
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub env: HashMap<String, String>,
}

use crate::tools::execute::ExecutableTool;
use client::McpClient;
use std::path::Path;
use std::sync::Arc;
use tool::McpTool;

#[allow(dead_code)]
pub struct McpManager {
    // Keep clients in memory so background child processes aren't dropped
    pub clients: Vec<Arc<McpClient>>,
}

impl McpManager {
    pub async fn load_from_config(
        config_path: &Path,
    ) -> Result<(Self, Vec<Box<dyn ExecutableTool>>), Box<dyn std::error::Error>> {
        let config_data = tokio::fs::read_to_string(config_path).await?;
        let config: MCPConfigFile = serde_json::from_str(&config_data)?;

        let mut clients = Vec::new();
        let mut tools: Vec<Box<dyn ExecutableTool>> = Vec::new();

        for (_server_name, server_cfg) in config.mcp_servers {
            let client = Arc::new(
                McpClient::connect(&server_cfg.command, &server_cfg.args, server_cfg.env).await?,
            );

            // Fetch the list of tools from this server
            let server_tools = client
                .service()
                .peer()
                .list_all_tools()
                .await?;

            for mcp_tool_def in server_tools {
                let tool = McpTool {
                    name: mcp_tool_def.name.to_string(),
                    description: mcp_tool_def.description.to_string(),
                    input_schema: serde_json::to_value(mcp_tool_def.input_schema)?,
                    client: Arc::clone(&client),
                };

                tools.push(Box::new(tool));
            }

            clients.push(client);
        }

        Ok((McpManager { clients }, tools))
    }
}
