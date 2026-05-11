pub mod agent;
pub mod models;
pub mod tools;
pub mod bridge;

pub struct LumenAgent {
    // Core components will be added here
}

impl LumenAgent {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn process_query(&self, query: &str) -> anyhow::Result<String> {
        // This will be the main entry point for queries
        Ok(format!("Lumen received: {}", query))
    }
}
