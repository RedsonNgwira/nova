use super::registry::OfficeTool;
use serde_json::{json, Value};

pub struct WriteTextTool;

impl OfficeTool for WriteTextTool {
    fn name(&self) -> &str {
        "write_text"
    }

    fn description(&self) -> &str {
        "Inserts text into the current document at the current cursor position."
    }

    fn execute(&self, args: Value) -> anyhow::Result<Value> {
        let text = args.get("text")
            .and_then(|t| t.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'text' argument"))?;

        // In a real implementation, this would call back into the C++ bridge
        // to execute the UNO command.
        println!("LUMEN EXECUTE: write_text({})", text);

        Ok(json!({
            "status": "success",
            "message": format!("Text '{}' inserted successfully", text)
        }))
    }
}
