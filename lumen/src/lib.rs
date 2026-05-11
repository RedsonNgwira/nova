pub mod agent;
pub mod models;
pub mod tools;
pub mod bridge;

use agent::context::ContextEngine;
use models::router::{ModelRouter, ModelProvider, TaskComplexity};
use models::cerebras::ChatMessage;
use tools::registry::ToolRegistry;
use tools::writer::WriteTextTool;

pub struct LumenAgent {
    pub context_engine: ContextEngine,
    pub model_router: ModelRouter,
    pub tool_registry: ToolRegistry,
}

impl LumenAgent {
    pub fn new(cerebras_key: String) -> Self {
        let mut tool_registry = ToolRegistry::new();
        tool_registry.register_tool(Box::new(WriteTextTool));

        Self {
            context_engine: ContextEngine::new(),
            model_router: ModelRouter::new(cerebras_key),
            tool_registry,
        }
    }

    pub async fn process_query(&self, query: &str) -> anyhow::Result<String> {
        let context = &self.context_engine.current_context;

        // For now, if query contains "write" or "insert", we simulate a tool suggestion
        if query.to_lowercase().contains("write") || query.to_lowercase().contains("insert") {
            return Ok(format!("Lumen suggests: use 'write_text' tool to add content. Context: {:?}", context.active_document));
        }

        let provider = self.model_router.route(TaskComplexity::Simple);

        match provider {
            ModelProvider::Cerebras(p) => {
                let messages = vec![
                    ChatMessage {
                        role: "system".to_string(),
                        content: format!("You are Lumen, the AI assistant for Nova Office. You have tools: {:?}. Context: {:?}", 
                                         self.tool_registry.tools.iter().map(|t| t.name()).collect::<Vec<_>>(),
                                         context),
                    },
                    ChatMessage {
                        role: "user".to_string(),
                        content: query.to_string(),
                    }
                ];
                p.complete(messages).await
            },
            _ => Ok("Provider not implemented".to_string()),
        }
    }
}
