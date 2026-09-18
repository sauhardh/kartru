use clap::Parser;
use core::result::Result;

mod agent;
mod constants;
mod llm;
mod tools;
mod utils;

use constants::{DEFAULT_BASE_URL, MODEL};
use llm::response::Response;
use llm::{call_ai, create_client};
use tools::execute::ReadTool;
use utils::Args;

extern crate dotenv;

use dotenv::dotenv;

use crate::agent::Agent;
use crate::tools::execute::{BashTool, WriteTool};
use crate::tools::registry::ToolRegistry;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let client = create_client()?;

    let mut registry = ToolRegistry::new();
    registry.registers(vec![
        Box::new(WriteTool),
        Box::new(ReadTool),
        Box::new(BashTool),
    ]);

    let args = Args::parse();

    let mut agent = Agent::default();
    // USER
    agent.insert_user(args.prompt.clone());

    if args.prompt == "STOP" {
        println!("Got STOP request: stopping...");
        return Ok(());
    }

    loop {
        if agent.messages.is_empty() {
            println!("Could not got your request");
            break;
        }

        let response: Response = call_ai(&client, &agent.messages).await?;
        let choice = response
            .first_choice()
            .ok_or_else(|| "Choice is empty".to_string())?;

        // ASSISTANT
        agent.insert_assistant(
            choice.message.content.clone(),
            Some(choice.message.tool_calls.clone()),
        );

        if choice.message.tool_calls.is_empty() {
            if let Some(content) = &choice.message.content {
                println!("{}", content);
            }

            // println!("No TOOL Found.");
            break;
        }

        for tool in &choice.message.tool_calls {
            let result = registry.execute(&tool.function.name, &tool.function.arguments)?;

            // TOOL
            agent.insert_tool(result, tool.id.clone());
        }
    }
    Ok(())
}
