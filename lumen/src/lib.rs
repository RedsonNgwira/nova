pub mod agent;
pub mod models;
pub mod tools;
pub mod bridge;

use agent::context::{ContextEngine, DocumentContext};
use agent::planner::{Planner, ActionPlan, PlanStep, UnoOp};
use models::router::{ModelRouter, ModelProvider, TaskComplexity};
use models::cerebras::ChatMessage;
use tools::registry::ToolRegistry;
use tools::writer::WriteTextTool;

pub struct LumenAgent {
    pub context_engine: ContextEngine,
    pub model_router: ModelRouter,
    pub tool_registry: ToolRegistry,
    pub planner: Planner,
}

impl LumenAgent {
    pub fn new(cerebras_key: String) -> Self {
        let mut tool_registry = ToolRegistry::new();
        tool_registry.register_tool(Box::new(WriteTextTool));

        Self {
            context_engine: ContextEngine::new(),
            model_router: ModelRouter::new(cerebras_key),
            tool_registry,
            planner: Planner::new(),
        }
    }

    pub async fn process_query(&self, query: &str) -> anyhow::Result<String> {
        let context = &self.context_engine.current_context;

        // Phase 3: Plan-then-Act Loop
        
        // 1. Generate a plan using AI reasoning
        let plan = self.generate_plan(query, context).await?;

        // 2. Format the plan for the user to see in the UI
        let mut response = format!("### Plan: {}\n\n", plan.interpreted_goal);
        for (i, step) in plan.steps.iter().enumerate() {
            response.push_str(&format!("{}. {}\n", i + 1, step.description));
        }
        
        if plan.steps.is_empty() {
            // Fallback to simple chat if no plan steps were generated
            return self.chat_fallback(query, context).await;
        }

        response.push_str("\nShould I execute this plan?");
        Ok(response)
    }

    async fn generate_plan(&self, query: &str, context: &DocumentContext) -> anyhow::Result<ActionPlan> {
        let provider = self.model_router.route(TaskComplexity::Medium);
        
        let system_prompt = format!(
            "You are the Planner for Nova Office. Your job is to take a user request and return a JSON action plan.
            Context: {:?}
            Available Tools: {:?}
            Return ONLY a JSON object matching the ActionPlan structure.",
            context,
            self.tool_registry.tools.iter().map(|t| t.name()).collect::<Vec<_>>()
        );

        match provider {
            ModelProvider::Cerebras(p) => {
                let messages = vec![
                    ChatMessage { role: "system".to_string(), content: system_prompt },
                    ChatMessage { role: "user".to_string(), content: query.to_string() }
                ];
                let ai_res = p.complete(messages).await?;
                
                // Attempt to parse AI response as JSON plan, fallback to empty plan on failure
                match serde_json::from_str::<ActionPlan>(&ai_res) {
                    Ok(plan) => Ok(plan),
                    Err(_) => Ok(self.planner.create_plan(query, context))
                }
            },
            _ => Ok(self.planner.create_plan(query, context))
        }
    }

    async fn chat_fallback(&self, query: &str, context: &DocumentContext) -> anyhow::Result<String> {
        let provider = self.model_router.route(TaskComplexity::Simple);
        match provider {
            ModelProvider::Cerebras(p) => {
                let messages = vec![
                    ChatMessage {
                        role: "system".to_string(),
                        content: format!("You are Lumen, the AI assistant for Nova Office. Context: {:?}", context),
                    },
                    ChatMessage { role: "user".to_string(), content: query.to_string() }
                ];
                p.complete(messages).await
            },
            _ => Ok("Provider not implemented".to_string()),
        }
    }
}
