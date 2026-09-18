mod define;

pub use define::{FunctionDefinition, ParametersDefinition, PropertyDefinition, ToolDefinition};

pub fn read_tools() -> ToolDefinition {
    ToolDefinition::function(FunctionDefinition::new(
        "Read",
        "Read and return the contents of a file",
        ParametersDefinition::object()
            .property(
                "file_path",
                PropertyDefinition::string("The path to the file to read"),
            )
            .required("file_path"),
    ))
}

pub fn write_tools() -> ToolDefinition {
    ToolDefinition::function(FunctionDefinition::new(
        "Write",
        "Write content to a file",
        ParametersDefinition::object()
            .property(
                "file_path",
                PropertyDefinition::string("The path of the file to write to"),
            )
            .property(
                "content",
                PropertyDefinition::string("The content to write to the file"),
            )
            .required("file_path")
            .required("content"),
    ))
}

pub fn bash_tools() -> ToolDefinition {
    ToolDefinition::function(FunctionDefinition::new(
        "Bash",
        "Execute a shell command",
        ParametersDefinition::object()
            .property(
                "command",
                PropertyDefinition::string("The command to execute"),
            )
            .required("command"),
    ))
}
