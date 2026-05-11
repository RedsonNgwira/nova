use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnoOp {
    pub command: String,
    pub args: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlanStep {
    pub description: String,
    pub uno_operations: Vec<UnoOp>,
    pub can_fail_safely: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ImpactLevel {
    Minor,
    Moderate,
    Major,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionPlan {
    pub id: Uuid,
    pub user_intent: String,
    pub interpreted_goal: String,
    pub steps: Vec<PlanStep>,
    pub estimated_impact: ImpactLevel,
    pub reversible: bool,
}

pub struct Planner {
}

impl Planner {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_plan(&self, intent: &str, _context: &super::context::DocumentContext) -> ActionPlan {
        // In a full implementation, this would be an AI call to generate the steps.
        // For Phase 3, we define the structure that the AI will fill.
        ActionPlan {
            id: Uuid::new_v4(),
            user_intent: intent.to_string(),
            interpreted_goal: intent.to_string(),
            steps: vec![], // To be filled by AI reasoning
            estimated_impact: ImpactLevel::Minor,
            reversible: true,
        }
    }
}
