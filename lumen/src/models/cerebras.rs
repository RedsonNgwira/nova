use serde::{Deserialize, Serialize};
use reqwest::Client;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ChatMessage,
}

pub struct CerebrasProvider {
    client: Client,
    api_key: String,
    model: String,
}

impl CerebrasProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            model: "llama3.1-70b".to_string(), // Typical Cerebras model
        }
    }

    pub async fn complete(&self, messages: Vec<ChatMessage>) -> Result<String> {
        let url = "https://api.cerebras.ai/v1/chat/completions";
        
        let request = ChatCompletionRequest {
            model: self.model.clone(),
            messages,
        };

        let response = self.client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("Cerebras API error: {}", error_text));
        }

        let completion: ChatCompletionResponse = response.json().await?;
        
        completion.choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| anyhow::anyhow!("No completion choices returned"))
    }
}
