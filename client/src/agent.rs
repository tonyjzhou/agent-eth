use crate::mcp_client::McpClient;
use crate::rag::RagSystem;
use crate::tools::TypedToolRegistry;
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::fmt::Write;

#[derive(Debug, Serialize, Deserialize)]
pub struct ParsedCommand {
    pub action: String,
    pub from_address: Option<String>,
    pub to_address: Option<String>,
    pub amount: Option<String>,
    pub address: Option<String>,
    pub token_in: Option<String>,
    pub token_out: Option<String>,
    pub amount_in: Option<String>,
    pub slippage_bps: Option<u16>,
    pub token: Option<String>,
}

// Modern AI Agent Core
#[derive(Debug, Serialize, Deserialize)]
pub struct AgentPlan {
    pub reasoning: String,
    pub steps: Vec<AgentStep>,
    pub requires_confirmation: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentStep {
    pub step_id: String,
    pub description: String,
    pub tool_name: String,
    pub parameters: serde_json::Value,
    pub depends_on: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecutionContext {
    pub step_results: HashMap<String, serde_json::Value>,
    pub session_memory: Vec<String>,
    pub user_preferences: HashMap<String, String>,
}

#[derive(Debug)]
pub struct AgentCore {
    client: Client,
    api_key: String,
    account_aliases: HashMap<String, String>,
    rag: Option<RagSystem>,
    context: ExecutionContext,
    tool_registry: ToolRegistry,
    #[allow(dead_code)]
    typed_tools: TypedToolRegistry,
}

#[derive(Debug, Clone)]
pub struct ToolInfo {
    #[allow(dead_code)]
    pub name: String,
    pub description: String,
    pub parameters: Vec<ToolParameter>,
}

#[derive(Debug, Clone)]
pub struct ToolParameter {
    pub name: String,
    pub param_type: String,
    pub required: bool,
    pub description: String,
}

#[derive(Debug)]
pub struct ToolRegistry {
    pub tools: HashMap<String, ToolInfo>,
}

// Helper function to extract JSON from markdown code blocks
fn extract_json_from_response(content: &str) -> String {
    // Check if the response is wrapped in markdown code blocks
    if content.trim().starts_with("```json") && content.trim().ends_with("```") {
        // Extract the JSON content between the code block markers
        let lines: Vec<&str> = content.trim().lines().collect();
        if lines.len() > 2 {
            // Skip first line (```json) and last line (```)
            return lines[1..lines.len() - 1].join("\n");
        }
    } else if content.trim().starts_with("```") && content.trim().ends_with("```") {
        // Generic code block - try to extract content
        let lines: Vec<&str> = content.trim().lines().collect();
        if lines.len() > 2 {
            return lines[1..lines.len() - 1].join("\n");
        }
    }

    // Return original content if no code blocks found
    content.to_string()
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolRegistry {
    pub fn new() -> Self {
        let mut tools = HashMap::new();

        // Define available tools that the AI agent can use
        tools.insert(
            "get_balance".to_string(),
            ToolInfo {
                name: "get_balance".to_string(),
                description: "Get ETH or token balance for an Ethereum address".to_string(),
                parameters: vec![
                    ToolParameter {
                        name: "address".to_string(),
                        param_type: "string".to_string(),
                        required: true,
                        description: "Ethereum address to check balance for".to_string(),
                    },
                    ToolParameter {
                        name: "token".to_string(),
                        param_type: "string".to_string(),
                        required: false,
                        description: "Token symbol (e.g., 'ETH', 'USDC')".to_string(),
                    },
                ],
            },
        );

        tools.insert(
            "send_transfer".to_string(),
            ToolInfo {
                name: "send_transfer".to_string(),
                description: "Send ETH from one address to another".to_string(),
                parameters: vec![
                    ToolParameter {
                        name: "from_address".to_string(),
                        param_type: "string".to_string(),
                        required: true,
                        description: "Address to send ETH from".to_string(),
                    },
                    ToolParameter {
                        name: "to_address".to_string(),
                        param_type: "string".to_string(),
                        required: true,
                        description: "Address to send ETH to".to_string(),
                    },
                    ToolParameter {
                        name: "amount_eth".to_string(),
                        param_type: "string".to_string(),
                        required: true,
                        description: "Amount of ETH to send".to_string(),
                    },
                    ToolParameter {
                        name: "private_key".to_string(),
                        param_type: "string".to_string(),
                        required: true,
                        description: "Private key of the sender".to_string(),
                    },
                ],
            },
        );

        tools.insert(
            "check_contract".to_string(),
            ToolInfo {
                name: "check_contract".to_string(),
                description: "Check if an address has deployed contract code".to_string(),
                parameters: vec![ToolParameter {
                    name: "address".to_string(),
                    param_type: "string".to_string(),
                    required: true,
                    description: "Contract address to check".to_string(),
                }],
            },
        );

        tools.insert(
            "execute_swap".to_string(),
            ToolInfo {
                name: "execute_swap".to_string(),
                description: "Execute a token swap using Uniswap V2 Router".to_string(),
                parameters: vec![
                    ToolParameter {
                        name: "from_address".to_string(),
                        param_type: "string".to_string(),
                        required: true,
                        description: "Address executing the swap".to_string(),
                    },
                    ToolParameter {
                        name: "token_in".to_string(),
                        param_type: "string".to_string(),
                        required: true,
                        description: "Input token symbol (e.g., 'ETH', 'USDC')".to_string(),
                    },
                    ToolParameter {
                        name: "token_out".to_string(),
                        param_type: "string".to_string(),
                        required: true,
                        description: "Output token symbol (e.g., 'USDC', 'ETH')".to_string(),
                    },
                    ToolParameter {
                        name: "amount_in".to_string(),
                        param_type: "string".to_string(),
                        required: true,
                        description: "Amount of input token to swap".to_string(),
                    },
                    ToolParameter {
                        name: "slippage_bps".to_string(),
                        param_type: "number".to_string(),
                        required: false,
                        description: "Slippage tolerance in basis points (default: 200 = 2%)"
                            .to_string(),
                    },
                    ToolParameter {
                        name: "private_key".to_string(),
                        param_type: "string".to_string(),
                        required: true,
                        description: "Private key of the sender".to_string(),
                    },
                ],
            },
        );

        Self { tools }
    }

    pub fn get_tools_description(&self) -> String {
        let mut description = String::with_capacity(2048);
        description.push_str("Available blockchain tools:\n");

        for (name, tool) in &self.tools {
            write!(description, "\n{}: {}\n", name, tool.description)
                .expect("String formatting should not fail");
            description.push_str("Parameters:\n");
            for param in &tool.parameters {
                let required_str = if param.required {
                    "required"
                } else {
                    "optional"
                };
                write!(
                    description,
                    "    - {} {}: {} {}",
                    param.name, param.param_type, param.description, required_str
                )
                .expect("String formatting should not fail");
            }
        }

        description
    }
}

impl AgentCore {
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

        let context = ExecutionContext {
            step_results: HashMap::new(),
            session_memory: Vec::new(),
            user_preferences: HashMap::new(),
        };

        Ok(Self {
            client,
            api_key: anthropic_api_key.to_string(),
            account_aliases,
            rag: None,
            context,
            tool_registry: ToolRegistry::new(),
            typed_tools: TypedToolRegistry::new(),
        })
    }

    pub async fn initialize_rag(&mut self, db_path: &str) -> Result<()> {
        let rag_system = RagSystem::new(db_path).await?;
        self.rag = Some(rag_system);
        Ok(())
    }

    // RAG system delegation methods
    pub async fn search_docs(&self, query: &str) -> Result<Vec<crate::rag::SearchResult>> {
        if let Some(rag) = &self.rag {
            rag.search(query, 15).await
        } else {
            Ok(vec![])
        }
    }

    pub async fn ingest_documents(&mut self, dir_path: &str) -> Result<usize> {
        if let Some(rag) = &mut self.rag {
            rag.ingest_directory(dir_path).await
        } else {
            Err(anyhow::anyhow!("RAG system not initialized"))
        }
    }

    pub async fn clear_documents(&mut self) -> Result<()> {
        if let Some(rag) = &mut self.rag {
            rag.clear_documents().await
        } else {
            Err(anyhow::anyhow!("RAG system not initialized"))
        }
    }

    // Legacy parsing methods for backward compatibility
    pub async fn is_documentation_query(&self, input: &str) -> bool {
        let input_lower = input.to_lowercase();

        let doc_keywords = [
            "explain how",
            "what is the difference",
            "how do i",
            "how does",
            "documentation",
            "docs ",
            "what are the steps",
            "how to calculate",
            "what does this mean",
            "difference between",
        ];

        if doc_keywords
            .iter()
            .any(|&keyword| input_lower.contains(keyword))
        {
            return true;
        }

        let blockchain_patterns = [
            "much eth",
            "much usdc",
            "much token",
            "eth does",
            "usdc does",
            "token does",
            "balance of",
            "balance for",
            "send",
            "transfer",
            "from alice",
            "from bob",
            "from carol",
            "to alice",
            "to bob",
            "to carol",
            "swap",
            "uniswap",
            "exchange",
        ];

        if blockchain_patterns
            .iter()
            .any(|&pattern| input_lower.contains(pattern))
        {
            return false;
        }

        false
    }

    pub async fn parse_command(&self, user_input: &str) -> Result<ParsedCommand> {
        let mut additional_context = String::new();
        if self.is_documentation_query(user_input).await {
            if let Ok(search_results) = self.search_docs(user_input).await {
                if !search_results.is_empty() {
                    additional_context = "\n\nRelevant Documentation Context:\n".to_string();
                    for result in search_results.iter().take(5) {
                        let chunk_preview = if result.relevant_chunk.len() > 800 {
                            format!(
                                "{}...",
                                result.relevant_chunk.chars().take(800).collect::<String>()
                            )
                        } else {
                            result.relevant_chunk.clone()
                        };
                        write!(
                            additional_context,
                            "Document: {} ({})\n{}\n\n",
                            result.document.title, result.document.source, chunk_preview
                        )
                        .expect("String formatting should not fail");
                    }
                }
            }
        }

        let system_prompt = format!(
            r#"You are an Ethereum blockchain assistant that helps users interact with the blockchain using natural language.{additional_context}

Parse user requests into JSON with EXACTLY this structure. Use "action" (not "command") as the field name:

For balance queries:
{{
  "action": "balance",
  "address": "0x...",
  "token": "ETH"
}}

For transfers:
{{
  "action": "transfer", 
  "from_address": "0x...",
  "to_address": "0x...",
  "amount": "1.0"
}}

For contract checks:
{{
  "action": "contract_check",
  "address": "0x..."
}}

For token swaps:
{{
  "action": "swap",
  "from_address": "0x...",
  "token_in": "ETH",
  "token_out": "USDC", 
  "amount_in": "10.0",
  "slippage_bps": 200
}}

Account aliases:
- Alice = 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
- Bob = 0x70997970C51812dc3A010C7d01b50e0d17dc79C8  
- Carol = 0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC

Convert all account names (alice, bob, carol) to their hex addresses.
For slippage, use 200 (2%) as default if not specified.

Return ONLY the JSON object, no explanations."#
        );

        let messages = json!([{"role": "user", "content": user_input}]);

        let response = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&json!({
                "model": "claude-sonnet-4-0",
                "max_tokens": 1024,
                "system": system_prompt,
                "messages": messages
            }))
            .send()
            .await?;

        let response_text = response.text().await?;
        let response_json: serde_json::Value = serde_json::from_str(&response_text)?;

        if let Some(error) = response_json.get("error") {
            return Err(anyhow::anyhow!(
                "Claude API error: {}",
                error
                    .get("message")
                    .and_then(|m| m.as_str())
                    .unwrap_or("Unknown error")
            ));
        }

        let content = response_json["content"][0]["text"].as_str().unwrap_or("{}");
        let cleaned_content = extract_json_from_response(content);

        let parsed_command: ParsedCommand = match serde_json::from_str(&cleaned_content) {
            Ok(cmd) => cmd,
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Failed to parse Claude response as JSON: {}\nOriginal response: {}\nCleaned response: {}",
                    e, content, cleaned_content
                ))
            }
        };

        Ok(parsed_command)
    }

    #[allow(dead_code)]
    pub fn resolve_address(&self, address: &str) -> String {
        let lowercased = address.to_lowercase();
        self.account_aliases
            .get(&lowercased)
            .cloned()
            .unwrap_or_else(|| address.to_string())
    }

    pub async fn answer_documentation_query(&self, query: &str) -> Result<String> {
        let search_results = self.search_docs(query).await?;

        if search_results.is_empty() {
            return Ok("I don't have specific documentation about that topic. Please try a more specific query or ingest relevant documentation first.".to_string());
        }

        let mut context = String::with_capacity(8192);
        for result in search_results.iter().take(8) {
            write!(
                context,
                "Source: {} ({})\n{}\n\n---\n\n",
                result.document.title, result.document.source, result.relevant_chunk
            )
            .expect("String formatting should not fail");
        }

        let system_prompt = format!(
            r#"You are an expert at Ethereum development and DeFi protocols. Use the provided documentation to answer the user's question comprehensively.

Documentation context:
{context}

Instructions:
- Answer based primarily on the provided documentation
- If the documentation doesn't contain enough information, say so clearly
- Provide practical examples when possible
- Be accurate and don't make up information not in the context
- Structure your response clearly with proper formatting"#
        );

        let response = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&json!({
                "model": "claude-sonnet-4-0",
                "max_tokens": 1024,
                "system": system_prompt,
                "messages": [{"role": "user", "content": query}]
            }))
            .send()
            .await?;

        let response_text = response.text().await?;
        let response_json: serde_json::Value = serde_json::from_str(&response_text)?;

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
            .unwrap_or("Unable to generate response");
        Ok(content.to_string())
    }

    // Modern AI agent methods
    pub async fn plan_execution(&mut self, user_input: &str) -> Result<AgentPlan> {
        let mut context_info = String::with_capacity(2048);
        context_info.push_str(&self.tool_registry.get_tools_description());
        context_info.push_str("\nAvailable test accounts:\n");

        for (alias, address) in &self.account_aliases {
            writeln!(context_info, "- {alias}: {address}")
                .expect("String formatting should not fail");
        }

        if let Ok(search_results) = self.search_docs(user_input).await {
            if !search_results.is_empty() {
                context_info.push_str("\n\nRelevant Documentation:\n");
                for result in search_results.iter().take(3) {
                    writeln!(
                        context_info,
                        "- {}: {}",
                        result.document.title,
                        result.relevant_chunk.chars().take(200).collect::<String>()
                    )
                    .expect("String formatting should not fail");
                }
            }
        }

        let system_prompt = format!(
            r#"You are an intelligent Ethereum blockchain agent that creates execution plans for user requests.

Available context:
{context_info}

Create a detailed execution plan for the user's request. Consider:
1. What blockchain operations are needed?
2. Are there dependencies between steps?
3. Does this require user confirmation (dangerous operations)?
4. What parameters are needed for each step?

Respond with JSON in this exact format:
{{
  "reasoning": "Explain your approach and why these steps are needed",
  "steps": [
    {{
      "step_id": "step_1",
      "description": "Human readable description of what this step does",
      "tool_name": "get_balance",
      "parameters": {{ "address": "0x...", "token": "ETH" }},
      "depends_on": null
    }}
  ],
  "requires_confirmation": false
}}

Available tools: get_balance, send_transfer, check_contract, execute_swap"#
        );

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
                "messages": [{"role": "user", "content": user_input}]
            }))
            .send()
            .await?;

        let response_text = response.text().await?;
        let response_json: serde_json::Value = serde_json::from_str(&response_text)?;

        if let Some(error) = response_json.get("error") {
            return Err(anyhow::anyhow!(
                "Claude API error: {}",
                error
                    .get("message")
                    .and_then(|m| m.as_str())
                    .unwrap_or("Unknown error")
            ));
        }

        let content = response_json["content"][0]["text"].as_str().unwrap_or("{}");
        let cleaned_content = extract_json_from_response(content);

        let plan: AgentPlan = serde_json::from_str(&cleaned_content)
            .map_err(|e| anyhow::anyhow!("Failed to parse execution plan: {}", e))?;

        Ok(plan)
    }

    pub async fn execute_plan(
        &mut self,
        plan: AgentPlan,
        mcp_client: &mut McpClient,
    ) -> Result<Vec<String>> {
        let mut results = Vec::new();

        for step in plan.steps {
            let result = match step.tool_name.as_str() {
                "get_balance" => {
                    let address = step.parameters["address"].as_str().unwrap_or("");
                    let token = step.parameters["token"].as_str().unwrap_or("ETH");
                    mcp_client.get_balance(address, Some(token)).await?
                }
                "send_transfer" => {
                    let from_address = step.parameters["from_address"].as_str().unwrap_or("");
                    let to_address = step.parameters["to_address"].as_str().unwrap_or("");
                    let amount_eth = step.parameters["amount_eth"].as_str().unwrap_or("");
                    let private_key =
                        self.get_private_key_for_address(from_address)
                            .ok_or_else(|| {
                                anyhow::anyhow!("No private key found for address {}", from_address)
                            })?;

                    mcp_client
                        .send_transfer(from_address, to_address, amount_eth, &private_key)
                        .await?
                }
                "check_contract" => {
                    let address = step.parameters["address"].as_str().unwrap_or("");
                    mcp_client.check_contract(address).await?
                }
                "execute_swap" => {
                    let from_address = step.parameters["from_address"].as_str().unwrap_or("");
                    let token_in = step.parameters["token_in"].as_str().unwrap_or("");
                    let token_out = step.parameters["token_out"].as_str().unwrap_or("");
                    let amount_in = step.parameters["amount_in"].as_str().unwrap_or("");
                    let slippage_bps =
                        step.parameters["slippage_bps"].as_u64().unwrap_or(200) as u16;
                    let private_key =
                        self.get_private_key_for_address(from_address)
                            .ok_or_else(|| {
                                anyhow::anyhow!("No private key found for address {}", from_address)
                            })?;

                    mcp_client
                        .execute_swap(
                            from_address,
                            token_in,
                            token_out,
                            amount_in,
                            slippage_bps,
                            &private_key,
                        )
                        .await?
                }
                _ => return Err(anyhow::anyhow!("Unknown tool: {}", step.tool_name)),
            };

            self.context.step_results.insert(
                step.step_id.clone(),
                serde_json::Value::String(result.clone()),
            );

            results.push(format!("✅ {}: {}", step.description, result));
        }

        Ok(results)
    }

    pub fn get_private_key_for_address(&self, address: &str) -> Option<String> {
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
