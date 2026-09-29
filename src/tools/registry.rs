use std::collections::HashMap;

use async_trait::async_trait;

use crate::tools::define::ToolDefinition;

use super::error::ToolError;
use super::execute::ExecutableTool;

#[allow(dead_code)]
pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn ExecutableTool>>,
}

#[allow(dead_code)]
impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    pub fn definitions(&self) -> Vec<ToolDefinition> {
        self.tools.values().map(|tool| tool.definition()).collect()
    }

    pub fn register<T>(&mut self, tool: T)
    where
        T: ExecutableTool + 'static,
    {
        self.tools
            .insert(tool.name().to_lowercase(), Box::new(tool));
    }

    pub fn registers(&mut self, input_tools: Vec<Box<dyn ExecutableTool>>) {
        for tool in input_tools {
            self.tools.insert(tool.name().to_lowercase(), tool);
        }
    }

    pub fn get(&self, name: &str) -> Option<&dyn ExecutableTool> {
        self.tools
            .get(&name.to_lowercase())
            .map(|tool| tool.as_ref())
    }

    pub async fn execute(&self, name: &str, arguments: &str) -> Result<String, ToolError> {
        let name = name.to_lowercase();
        let tool = self.get(&name).ok_or(ToolError::UnknownTool(name))?;

        tool.execute(arguments).await
    }
}
