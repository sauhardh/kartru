use async_openai::{Client, config::OpenAIConfig};
use core::result::Result;
use serde_json::json;
use std::env;

use crate::agent::AgentMessage;
use crate::llm::response::Response;
use crate::tools::advertise::{bash_tools, read_tools, write_tools};
use crate::{DEFAULT_BASE_URL, MODEL};

pub fn create_client() -> Result<Client<OpenAIConfig>, Box<dyn std::error::Error>> {
    let base_url = env::var("OPENROUTER_BASE_URL").unwrap_or_else(|_| DEFAULT_BASE_URL.to_string());
    let api_key = env::var("OPENROUTER_API_KEY").map_err(|_| "OPENROUTER_API_KEY is not set")?;

    let config = OpenAIConfig::new()
        .with_api_base(base_url)
        .with_api_key(api_key);

    Ok(Client::with_config(config))
}

pub async fn call_ai(
    client: &Client<OpenAIConfig>,
    messages: &[AgentMessage],
) -> Result<Response, Box<dyn std::error::Error>> {
    let response: Response = client
        .chat()
        .create_byot(json!({
            "messages": messages,            "model": MODEL,
            "max_tokens": 4096,
            "tools": [read_tools(), write_tools(), bash_tools()]

        }))
        .await?;

    Ok(response)
}
