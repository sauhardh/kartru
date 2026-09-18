use serde::Serialize;
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct ToolDefinition {
    pub r#type: &'static str,
    pub function: FunctionDefinition,
}

#[derive(Debug, Serialize)]
pub struct FunctionDefinition {
    pub name: &'static str,
    pub description: &'static str,
    pub parameters: ParametersDefinition,
}

#[derive(Debug, Serialize)]
pub struct ParametersDefinition {
    pub r#type: &'static str,
    pub properties: HashMap<&'static str, PropertyDefinition>,
    pub required: Vec<&'static str>,
}

#[derive(Debug, Serialize)]
pub struct PropertyDefinition {
    pub r#type: &'static str,
    pub description: &'static str,
}

#[allow(dead_code)]
impl ToolDefinition {
    pub fn function(function: FunctionDefinition) -> Self {
        Self {
            r#type: "function",
            function,
        }
    }
}

#[allow(dead_code)]
impl FunctionDefinition {
    pub fn new(
        name: &'static str,
        description: &'static str,
        parameters: ParametersDefinition,
    ) -> Self {
        Self {
            name,
            description,
            parameters,
        }
    }
}

#[allow(dead_code)]
impl ParametersDefinition {
    pub fn object() -> Self {
        Self {
            r#type: "object",
            properties: HashMap::new(),
            required: Vec::new(),
        }
    }

    pub fn property(mut self, name: &'static str, property: PropertyDefinition) -> Self {
        self.properties.insert(name, property);
        self
    }

    pub fn required(mut self, name: &'static str) -> Self {
        self.required.push(name);
        self
    }
}

#[allow(dead_code)]
impl PropertyDefinition {
    pub fn string(description: &'static str) -> Self {
        Self {
            r#type: "string",
            description,
        }
    }
}
