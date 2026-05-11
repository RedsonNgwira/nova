use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Serialize, Deserialize)]
pub enum DocumentType {
    Writer,
    Calc,
    Impress,
    Draw,
    Base,
    None,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Selection {
    pub start: String,
    pub end: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SkillLevel {
    Beginner,
    Intermediate,
    Expert,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfile {
    pub skill_level: SkillLevel,
    pub preferred_language: String,
    pub common_tasks: Vec<String>,
    pub formula_knowledge: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    pub timestamp: u64,
    pub action_type: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentContext {
    pub active_document: DocumentType,
    pub active_sheet: Option<String>,
    pub selection: Option<Selection>,
    pub cursor_position: Option<Position>,
    pub document_summary: String,
    pub recent_actions: VecDeque<Action>,
    pub user_profile: UserProfile,
}

pub struct ContextEngine {
    pub current_context: DocumentContext,
}

impl ContextEngine {
    pub fn new() -> Self {
        Self {
            current_context: DocumentContext {
                active_document: DocumentType::None,
                active_sheet: None,
                selection: None,
                cursor_position: None,
                document_summary: String::new(),
                recent_actions: VecDeque::with_capacity(20),
                user_profile: UserProfile {
                    skill_level: SkillLevel::Beginner,
                    preferred_language: String::from("en"),
                    common_tasks: vec![],
                    formula_knowledge: false,
                },
            },
        }
    }

    pub fn update_context(&mut self, new_context: DocumentContext) {
        self.current_context = new_context;
    }
}
