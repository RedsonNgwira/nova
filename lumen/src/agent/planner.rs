use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanStep {
    pub description: String,
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
    // Planner state
}

impl Planner {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_plan(&self, intent: &str, _context: &super::context::DocumentContext) -> ActionPlan {
        // Mock plan creation for now
        ActionPlan {
            id: Uuid::new_v4(),
            user_intent: intent.to_string(),
            interpreted_goal: format!("Execute: {}", intent),
            steps: vec![
                PlanStep {
                    description: "Initial analysis".to_string(),
                    can_fail_safely: false,
                }
            ],
            estimated_impact: ImpactLevel::Minor,
            reversible: true,
        }
    }
}
