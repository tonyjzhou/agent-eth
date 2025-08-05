use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct EthereumAgent {
    client: Client,
    api_key: String,
    account_aliases: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParsedCommand {
    pub action: String,
    pub from_address: Option<String>,
    pub to_address: Option<String>,
    pub amount: Option<String>,
    pub address: Option<String>,
}

impl EthereumAgent {
    pub fn new(anthropic_api_key: &str) -> Result<Self> {
        let client = Client::new();

        let mut account_aliases = HashMap::new();
        account_aliases.insert(
            "alice".to_lowercase(),
            "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".to_string(),
        );
        account_aliases.insert(
            "bob".to_lowercase(),
            "0x70997970C51812dc3A010C7d01b50e0d17dc79C8".to_string(),
        );
        account_aliases.insert(
            "carol".to_lowercase(),
            "0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC".to_string(),
        );

        Ok(Self {
            client,
            api_key: anthropic_api_key.to_string(),
            account_aliases,
        })
    }

    pub async fn parse_command(&self, user_input: &str) -> Result<ParsedCommand> {
        let system_prompt = r#"You are an Ethereum blockchain assistant that helps users interact with the blockchain using natural language.

Your job is to:
1. Parse user requests into structured commands
2. Handle account aliases (Alice = 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266, Bob = 0x70997970C51812dc3A010C7d01b50e0d17dc79C8, etc.)
3. Convert amounts to proper format
4. Extract addresses and validate them

Available actions:
- balance: Get balance of an address
- transfer: Send ETH from one address to another
- contract_check: Check if a contract is deployed

Account aliases:
- Alice: 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
- Bob: 0x70997970C51812dc3A010C7d01b50e0d17dc79C8
- Carol: 0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC

For transfer commands, you need:
- from_address (default to Alice if not specified)
- to_address
- amount (in ETH)

Always respond with a JSON object describing the parsed command."#;

        let messages = json!([
            {
                "role": "system",
                "content": system_prompt
            },
            {
                "role": "user",
                "content": format!("Parse this user command into a structured format: {}\n\nRespond with JSON only.", user_input)
            }
        ]);

        let response = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&json!({
                "model": "claude-3-5-sonnet-20241022",
                "max_tokens": 1024,
                "messages": messages
            }))
            .send()
            .await?;

        let response_text = response.text().await?;
        let response_json: serde_json::Value = serde_json::from_str(&response_text)?;

        let content = response_json["content"][0]["text"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid response format"))?;

        let parsed: ParsedCommand = serde_json::from_str(content)?;

        // Resolve aliases
        let mut resolved = parsed;
        if let Some(ref addr) = resolved.from_address {
            if let Some(real_addr) = self.account_aliases.get(&addr.to_lowercase()) {
                resolved.from_address = Some(real_addr.clone());
            }
        }
        if let Some(ref addr) = resolved.to_address {
            if let Some(real_addr) = self.account_aliases.get(&addr.to_lowercase()) {
                resolved.to_address = Some(real_addr.clone());
            }
        }
        if let Some(ref addr) = resolved.address {
            if let Some(real_addr) = self.account_aliases.get(&addr.to_lowercase()) {
                resolved.address = Some(real_addr.clone());
            }
        }

        Ok(resolved)
    }

    pub fn get_private_key_for_address(&self, address: &str) -> Option<String> {
        // Predefined private keys for test accounts
        match address {
            "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266" => Some(
                "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".to_string(),
            ),
            "0x70997970C51812dc3A010C7d01b50e0d17dc79C8" => Some(
                "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d".to_string(),
            ),
            "0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC" => Some(
                "0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a".to_string(),
            ),
            _ => None,
        }
    }
}
