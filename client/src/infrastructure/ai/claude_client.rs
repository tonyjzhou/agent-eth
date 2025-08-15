use anyhow::Result;
use reqwest::Client;
use serde_json::{json, Value};

pub struct ClaudeClient {
    client: Client,
    api_key: String,
}

impl ClaudeClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn chat_completion(&self, system_prompt: &str, user_message: &str) -> Result<String> {
        let messages = json!([
            {
                "role": "user",
                "content": user_message
            }
        ]);

        let response = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&json!({
                "model": "claude-sonnet-4-0",
                "max_tokens": 2048,
                "system": system_prompt,
                "messages": messages
            }))
            .send()
            .await?;

        let response_text = response.text().await?;
        let response_json: Value = serde_json::from_str(&response_text)?;

        if let Some(error) = response_json.get("error") {
            return Err(anyhow::anyhow!(
                "Claude API error: {}",
                error
                    .get("message")
                    .and_then(|m| m.as_str())
                    .unwrap_or("Unknown error")
            ));
        }

        let content = response_json["content"][0]["text"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid response format from Claude API"))?;

        Ok(content.to_string())
    }
}

// Helper function to extract JSON from markdown code blocks
pub fn extract_json_from_response(content: &str) -> String {
    // Check if the response is wrapped in markdown code blocks
    if content.trim().starts_with("```json") && content.trim().ends_with("```") {
        // Extract the JSON content between the code block markers
        let lines: Vec<&str> = content.trim().lines().collect();
        if lines.len() > 2 {
            // Skip first line (```json) and last line (```)
            return lines[1..lines.len() - 1].join("\n");
        }
    } else if content.trim().starts_with("```") && content.trim().ends_with("```") {
        // Handle generic code blocks
        let lines: Vec<&str> = content.trim().lines().collect();
        if lines.len() > 2 {
            return lines[1..lines.len() - 1].join("\n");
        }
    }

    // Return original content if no code blocks found
    content.to_string()
}
