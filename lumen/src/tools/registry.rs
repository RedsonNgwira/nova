use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ToolType {
    Writer,
    Calc,
    Impress,
}

pub trait OfficeTool {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn execute(&self, args: serde_json::Value) -> anyhow::Result<serde_json::Value>;
}

pub struct ToolRegistry {
    pub tools: Vec<Box<dyn OfficeTool + Send + Sync>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: vec![],
        }
    }

    pub fn register_tool(&mut self, tool: Box<dyn OfficeTool + Send + Sync>) {
        self.tools.push(tool);
    }

    pub fn get_tool(&self, name: &str) -> Option<&dyn OfficeTool> {
        self.tools.iter().find(|t| t.name() == name).map(|t| t.as_ref())
    }
}
