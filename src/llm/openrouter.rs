use async_openai::{Client, config::OpenAIConfig};
use core::result::Result;
use serde_json::json;
use std::env;

use crate::agent::AgentMessage;
use crate::llm::response::Response;
use crate::tools::define::ToolDefinition;
use crate::{DEFAULT_BASE_URL, MODEL};

pub fn create_client() -> Result<Client<OpenAIConfig>, Box<dyn std::error::Error>> {
    let base_url = env::var("OPENROUTER_BASE_URL").unwrap_or_else(|_| DEFAULT_BASE_URL.to_string());
    let api_key = env::var("OPENROUTER_API_KEY").map_err(|_| "OPENROUTER_API_KEY is not set")?;

    let config = OpenAIConfig::new()
        .with_api_base(base_url)
        .with_api_key(api_key);

    Ok(Client::with_config(config))
}

fn to_openai_tool(tool: &ToolDefinition) -> serde_json::Value {
    serde_json::json!({
        "type":"function",
        "function":{
            "name": tool.name,
            "description": tool.description,
            "parameters": tool.parameters
        }
    })
}

pub async fn call_ai(
    client: &Client<OpenAIConfig>,
    messages: &[AgentMessage],
    tools: &[ToolDefinition],
) -> Result<Response, Box<dyn std::error::Error>> {
    let response: Response = client
        .chat()
        .create_byot(json!({
            "messages": messages,            "model": MODEL,
            "max_tokens": 4096,
            "tools": &tools.iter().map(|tool| to_openai_tool(tool)).collect::<Vec<_>>()

        }))
        .await?;

    Ok(response)
}
