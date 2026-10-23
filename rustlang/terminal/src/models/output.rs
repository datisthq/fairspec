use serde_json::Value;

/// A command's result: what `--json` prints, and the exit code (1 for an invalid report).
#[derive(Debug, Clone, PartialEq)]
pub struct CommandOutput {
    pub data: Value,
    pub code: i32,
}

impl CommandOutput {
    pub fn new(data: Value) -> Self {
        CommandOutput { data, code: 0 }
    }
}

/// A command that ended early with this exit code; its output is already written.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Exit(pub i32);
