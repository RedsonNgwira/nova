pub mod agent;
pub mod models;
pub mod tools;
pub mod bridge;

use agent::context::ContextEngine;
use models::router::{ModelRouter, ModelProvider, TaskComplexity};
use models::cerebras::ChatMessage;

pub struct LumenAgent {
    pub context_engine: ContextEngine,
    pub model_router: ModelRouter,
}

impl LumenAgent {
    pub fn new(cerebras_key: String) -> Self {
        Self {
            context_engine: ContextEngine::new(),
            model_router: ModelRouter::new(cerebras_key),
        }
    }

    pub async fn process_query(&self, query: &str) -> anyhow::Result<String> {
        // 1. Perception (handled by context engine)
        let context = &self.context_engine.current_context;

        // 2. Planning (mocked for now)
        // 3. Model Routing
        let provider = self.model_router.route(TaskComplexity::Simple);

        // 4. Execution (via model completion)
        match provider {
            ModelProvider::Cerebras(p) => {
                let messages = vec![
                    ChatMessage {
                        role: "system".to_string(),
                        content: format!("You are Lumen, the AI assistant for Nova Office. Context: {:?}", context),
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
