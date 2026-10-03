use clap::Parser;
use std::path::Path;
mod agent;
mod constants;
mod llm;
mod tools;
mod utils;
use dotenv::dotenv;

use constants::{DEFAULT_BASE_URL, MODEL};
use core::result::Result;
use llm::response::Response;
use llm::{call_ai, create_client};
use utils::Args;

extern crate dotenv;
use crate::agent::Agent;
use crate::tools::execute::mcp::McpManager;
use crate::tools::execute::{BashTool, ReadTool, WriteTool};
use crate::tools::registry::ToolRegistry;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Initialize structured logging without emojis
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with_target(false)
        .init();

    let client = create_client()?;

    let mut registry = ToolRegistry::new();
    registry.registers(vec![
        Box::new(WriteTool),
        Box::new(ReadTool),
        Box::new(BashTool),
    ]);

    // MCP tools
    let config_path = [
        Path::new("src/configs/mcp_servers.json"),
        Path::new("configs/mcp_servers.json"),
    ]
    .into_iter()
    .find(|p| p.exists());

    let (_mcp_manager, mcp_tools) = if let Some(config_file) = config_path {
        tracing::info!("Loading MCP servers from {:?}", config_file);
        McpManager::load_from_config(config_file).await?
    } else {
        (McpManager { clients: vec![] }, vec![])
    };

    registry.registers(mcp_tools);

    let args = Args::parse();

    let mut agent = Agent::default();
    // USER
    agent.insert_user(args.prompt.clone());

    if args.prompt == "STOP" {
        tracing::info!("Got STOP request: stopping");
        return Ok(());
    }

    let mut step = 0;
    loop {
        if agent.messages.is_empty() {
            tracing::warn!("No messages found in agent history");
            break;
        }

        step += 1;
        tracing::info!("[step {}] Calling LLM...", step);

        let response: Response = call_ai(&client, &agent.messages, &registry.definitions()).await?;
        let choice = response
            .first_choice()
            .ok_or_else(|| "Choice is empty".to_string())?;

        // ASSISTANT
        agent.insert_assistant(
            choice.message.content.clone(),
            Some(choice.message.tool_calls.clone()),
        );

        if let Some(content) = &choice.message.content {
            if !content.trim().is_empty() {
                tracing::info!("[step {}] Model reasoning: {}", step, content.trim());
            }
        }

        if choice.message.tool_calls.is_empty() {
            tracing::info!("[step {}] Task completed", step);
            if let Some(content) = &choice.message.content {
                println!("\n{}", content);
            }
            break;
        }

        for tool in &choice.message.tool_calls {
            tracing::info!(
                "[step {}] Executing tool: `{}` | args: {}",
                step,
                tool.function.name,
                tool.function.arguments
            );

            match registry
                .execute(&tool.function.name, &tool.function.arguments)
                .await
            {
                Ok(result) => {
                    let preview = if result.len() > 200 {
                        format!("{}... ({} bytes)", &result[..200].trim(), result.len())
                    } else {
                        result.trim().to_string()
                    };
                    tracing::info!(
                        "[step {}] Tool `{}` output: {}",
                        step,
                        tool.function.name,
                        preview
                    );
                    agent.insert_tool(result, tool.id.clone());
                }
                Err(err) => {
                    tracing::warn!(
                        "[step {}] Tool `{}` failed: {}",
                        step,
                        tool.function.name,
                        err
                    );
                    // Pass error back to the LLM so it can recover instead of crashing
                    agent.insert_tool(format!("Error: {}", err), tool.id.clone());
                }
            }
        }
    }
    Ok(())
}
