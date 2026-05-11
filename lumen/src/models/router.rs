use super::cerebras::CerebrasProvider;
use super::gemini::GeminiProvider;

pub enum ModelProvider {
    Cerebras(CerebrasProvider),
    Gemini(GeminiProvider),
}

pub enum TaskComplexity {
    Simple,
    Medium,
    Complex,
}

pub struct ModelRouter {
    pub default_provider: ModelProvider,
}

impl ModelRouter {
    pub fn new(cerebras_key: String) -> Self {
        Self {
            default_provider: ModelProvider::Cerebras(CerebrasProvider::new(cerebras_key)),
        }
    }

    pub fn route(&self, complexity: TaskComplexity) -> &ModelProvider {
        match complexity {
            TaskComplexity::Simple => &self.default_provider,
            TaskComplexity::Medium => &self.default_provider,
            TaskComplexity::Complex => &self.default_provider, // Fallback for now
        }
    }
}
