use crate::mcp_client::McpClient;
use crate::rag::RagSystem;
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
    rag: Option<RagSystem>,
}

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
}

#[derive(Debug, Clone)]
pub struct ToolInfo {
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
            rag: None,
        })
    }

    pub async fn initialize_rag(&mut self, db_path: &str) -> Result<()> {
        let rag_system = RagSystem::new(db_path).await?;
        self.rag = Some(rag_system);
        Ok(())
    }

    pub async fn search_docs(&self, query: &str) -> Result<Vec<crate::rag::SearchResult>> {
        if let Some(rag) = &self.rag {
            // Increase limit since we now have more focused chunks
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

    pub async fn is_documentation_query(&self, input: &str) -> bool {
        let input_lower = input.to_lowercase();

        // First check for clear documentation indicators - if present, it's a doc query
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

        // Then check if this looks like a blockchain operation - if so, it's NOT a doc query
        let blockchain_patterns = [
            // Balance queries
            "much eth",
            "much usdc",
            "much token",
            "eth does",
            "usdc does",
            "token does",
            "balance of",
            "balance for",
            // Transfer operations
            "send",
            "transfer",
            "from alice",
            "from bob",
            "from carol",
            "to alice",
            "to bob",
            "to carol",
            // Swap operations with specific patterns that indicate actual swaps
            "swap 10",
            "swap 1 ",
            "swap 0.", // for decimal amounts
            "use uniswap",
            "uniswap v2 router",
            "for usdc",
            "for eth",
            "for weth",
            // Contract operations
            "deployed at",
            "contract at",
            "check contract",
        ];

        if blockchain_patterns
            .iter()
            .any(|&pattern| input_lower.contains(pattern))
        {
            return false;
        }

        // Default to false for anything else (neither clear doc query nor blockchain operation)
        false
    }

    pub async fn parse_command(&self, user_input: &str) -> Result<ParsedCommand> {
        // Check if this is a documentation query and get RAG context
        let mut additional_context = String::new();
        if self.is_documentation_query(user_input).await {
            if let Ok(search_results) = self.search_docs(user_input).await {
                if !search_results.is_empty() {
                    additional_context = "\n\nRelevant Documentation Context:\n".to_string();
                    for result in search_results.iter().take(5) {
                        // Show more context since chunks are now better organized
                        let chunk_preview = if result.relevant_chunk.len() > 800 {
                            format!(
                                "{}...",
                                result.relevant_chunk.chars().take(800).collect::<String>()
                            )
                        } else {
                            result.relevant_chunk.clone()
                        };
                        additional_context.push_str(&format!(
                            "Document: {} ({})\n{}\n\n",
                            result.document.title, result.document.source, chunk_preview
                        ));
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

For token balance queries:
{{
  "action": "balance", 
  "address": "0x...",
  "token": "USDC"
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

For token swaps (including Uniswap commands):
{{
  "action": "swap",
  "from_address": "0x...",
  "token_in": "ETH",
  "token_out": "USDC", 
  "amount_in": "10.0",
  "slippage_bps": 200
}}

Examples of swap commands to recognize:
- "Use Uniswap V2 Router to swap 10 ETH for USDC on Alice's account"
- "Swap 5 ETH for USDC from Bob's account"  
- "Convert 1000 USDC to ETH using Alice"

Account aliases (resolve these to hex addresses):
- Alice: 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
- Bob: 0x70997970C51812dc3A010C7d01b50e0d17dc79C8
- Carol: 0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC

IMPORTANT: Always use "action" as the field name, never "command". Response must be valid JSON only."#
        );

        let messages = json!([
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
                "system": system_prompt,
                "messages": messages
            }))
            .send()
            .await?;

        let response_text = response.text().await?;
        let response_json: serde_json::Value = serde_json::from_str(&response_text)?;

        // Check for API errors first
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
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Invalid response format - expected content[0].text in API response"
                )
            })?;

        // Parse as generic JSON first to normalize field names
        let mut json_value: serde_json::Value = serde_json::from_str(content).map_err(|e| {
            anyhow::anyhow!(
                "Failed to parse Claude response as JSON: {}\nResponse was: {}",
                e,
                content
            )
        })?;

        // Normalize "command" to "action" for backward compatibility
        if let Some(obj) = json_value.as_object_mut() {
            if obj.contains_key("command") && !obj.contains_key("action") {
                if let Some(command_value) = obj.remove("command") {
                    obj.insert("action".to_string(), command_value);
                }
            }
        }

        let parsed: ParsedCommand = serde_json::from_value(json_value.clone()).map_err(|e| {
            anyhow::anyhow!(
                "Failed to parse normalized JSON as ParsedCommand: {}\nOriginal response: {}\nNormalized JSON: {}",
                e,
                content,
                serde_json::to_string_pretty(&json_value).unwrap_or_default()
            )
        })?;

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

    pub async fn answer_documentation_query(&self, query: &str) -> Result<String> {
        let search_results = self.search_docs(query).await?;

        if search_results.is_empty() {
            return Ok(
                "I don't have specific documentation for that query in my knowledge base."
                    .to_string(),
            );
        }

        // Build context from search results - use more results since chunks are better organized
        let mut context = String::new();
        for result in search_results.iter().take(8) {
            context.push_str(&format!(
                "Source: {} ({})\n{}\n\n---\n\n",
                result.document.title, result.document.source, result.relevant_chunk
            ));
        }

        let system_prompt = format!(
            "You are an expert on Ethereum and Uniswap. Answer the user's question based on the provided documentation context. Be accurate and specific.\n\nContext:\n{context}"
        );

        let messages = json!([
            {
                "role": "user",
                "content": query
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

        let content = response_json["content"][0]["text"]
            .as_str()
            .unwrap_or("Unable to generate response");

        Ok(content.to_string())
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

// Implementation for the new AgentCore
impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolRegistry {
    pub fn new() -> Self {
        let mut tools = HashMap::new();

        // Register available blockchain tools
        tools.insert(
            "get_balance".to_string(),
            ToolInfo {
                name: "get_balance".to_string(),
                description: "Get the balance of an Ethereum address for ETH or ERC20 tokens"
                    .to_string(),
                parameters: vec![
                    ToolParameter {
                        name: "address".to_string(),
                        param_type: "string".to_string(),
                        required: true,
                        description: "The Ethereum address to check".to_string(),
                    },
                    ToolParameter {
                        name: "token".to_string(),
                        param_type: "string".to_string(),
                        required: false,
                        description: "Token symbol (defaults to ETH)".to_string(),
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
                        description: "The sender's address".to_string(),
                    },
                    ToolParameter {
                        name: "to_address".to_string(),
                        param_type: "string".to_string(),
                        required: true,
                        description: "The recipient's address".to_string(),
                    },
                    ToolParameter {
                        name: "amount_eth".to_string(),
                        param_type: "string".to_string(),
                        required: true,
                        description: "Amount of ETH to send".to_string(),
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
                    description: "The contract address to check".to_string(),
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
                        description: "The address executing the swap".to_string(),
                    },
                    ToolParameter {
                        name: "token_in".to_string(),
                        param_type: "string".to_string(),
                        required: true,
                        description: "Input token symbol".to_string(),
                    },
                    ToolParameter {
                        name: "token_out".to_string(),
                        param_type: "string".to_string(),
                        required: true,
                        description: "Output token symbol".to_string(),
                    },
                    ToolParameter {
                        name: "amount_in".to_string(),
                        param_type: "string".to_string(),
                        required: true,
                        description: "Amount of input tokens".to_string(),
                    },
                    ToolParameter {
                        name: "slippage_bps".to_string(),
                        param_type: "number".to_string(),
                        required: false,
                        description: "Slippage tolerance in basis points (default 200)".to_string(),
                    },
                ],
            },
        );

        Self { tools }
    }

    pub fn get_tools_description(&self) -> String {
        let mut description = String::new();
        description.push_str("Available blockchain tools:\n");

        for tool in self.tools.values() {
            description.push_str(&format!("\n- {}: {}\n", tool.name, tool.description));
            description.push_str("  Parameters:\n");
            for param in &tool.parameters {
                let required_str = if param.required {
                    "(required)"
                } else {
                    "(optional)"
                };
                description.push_str(&format!(
                    "    - {} {}: {} {}\n",
                    param.name, param.param_type, param.description, required_str
                ));
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
        })
    }

    pub async fn initialize_rag(&mut self, db_path: &str) -> Result<()> {
        let rag_system = RagSystem::new(db_path).await?;
        self.rag = Some(rag_system);
        Ok(())
    }

    pub async fn plan_execution(&mut self, user_input: &str) -> Result<AgentPlan> {
        // Build context for the AI agent
        let mut context_info = String::new();

        // Add tool registry info
        context_info.push_str(&self.tool_registry.get_tools_description());

        // Add account aliases
        context_info.push_str("\nAvailable test accounts:\n");
        for (alias, address) in &self.account_aliases {
            context_info.push_str(&format!("- {alias}: {address}\n"));
        }

        // Add session memory if any
        if !self.context.session_memory.is_empty() {
            context_info.push_str("\nRecent operations in this session:\n");
            for memory in self.context.session_memory.iter().rev().take(5) {
                context_info.push_str(&format!("- {memory}\n"));
            }
        }

        // Search RAG for relevant documentation if available
        if let Some(rag) = &self.rag {
            if let Ok(search_results) = rag.search(user_input, 3).await {
                if !search_results.is_empty() {
                    context_info.push_str("\nRelevant documentation:\n");
                    for result in search_results.iter().take(2) {
                        let preview = if result.relevant_chunk.len() > 200 {
                            format!(
                                "{}...",
                                result.relevant_chunk.chars().take(200).collect::<String>()
                            )
                        } else {
                            result.relevant_chunk.clone()
                        };
                        context_info
                            .push_str(&format!("- {}: {}\n", result.document.title, preview));
                    }
                }
            }
        }

        let system_prompt = format!(
            r#"You are an intelligent Ethereum blockchain agent that creates execution plans for user requests.

Your task is to analyze the user's request and create a detailed execution plan using the available tools.

{context_info}

Guidelines:
1. Always think step by step and explain your reasoning
2. Break complex operations into smaller steps  
3. Consider dependencies between steps
4. Resolve account aliases (alice, bob, carol) to actual addresses
5. For dangerous operations (large transfers, irreversible swaps), set requires_confirmation: true
6. Include parameter validation and error handling considerations
7. For queries like balance checks, documentation searches, set requires_confirmation: false

Response format (JSON only):
{{
  "reasoning": "Detailed explanation of your analysis and approach",
  "steps": [
    {{
      "step_id": "step_1",
      "description": "Human readable description", 
      "tool_name": "get_balance",
      "parameters": {{"address": "0x...", "token": "ETH"}},
      "depends_on": null
    }}
  ],
  "requires_confirmation": false
}}

IMPORTANT: Only use tools that exist in the tool registry. Always validate addresses and parameters."#
        );

        let messages = json!([
            {
                "role": "user",
                "content": format!("Create an execution plan for: {}", user_input)
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
                "max_tokens": 2048,
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

        let content = response_json["content"][0]["text"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid response format from Claude API"))?;

        let mut plan: AgentPlan = serde_json::from_str(content).map_err(|e| {
            anyhow::anyhow!(
                "Failed to parse agent plan: {}\nResponse was: {}",
                e,
                content
            )
        })?;

        // Resolve account aliases in parameters
        for step in &mut plan.steps {
            if let Some(obj) = step.parameters.as_object_mut() {
                for (_key, value) in obj.iter_mut() {
                    if let Some(addr_str) = value.as_str() {
                        if let Some(real_addr) = self.account_aliases.get(&addr_str.to_lowercase())
                        {
                            *value = serde_json::Value::String(real_addr.clone());
                        }
                    }
                }
            }
        }

        Ok(plan)
    }

    pub async fn execute_plan(
        &mut self,
        plan: AgentPlan,
        mcp_client: &mut McpClient,
    ) -> Result<Vec<String>> {
        let mut results = Vec::new();
        let mut executed_steps = HashMap::new();

        // Add this operation to session memory
        self.context
            .session_memory
            .push(format!("Executed plan: {}", plan.reasoning));

        for step in &plan.steps {
            // Check dependencies
            if let Some(depends_on) = &step.depends_on {
                if !executed_steps.contains_key(depends_on) {
                    return Err(anyhow::anyhow!(
                        "Step {} depends on {}, but that step hasn't been executed",
                        step.step_id,
                        depends_on
                    ));
                }
            }

            // Execute the step
            let result = match step.tool_name.as_str() {
                "get_balance" => {
                    let address = step.parameters["address"]
                        .as_str()
                        .ok_or_else(|| anyhow::anyhow!("Missing address parameter"))?;
                    let token = step.parameters.get("token").and_then(|t| t.as_str());
                    mcp_client.get_balance(address, token).await?
                }
                "send_transfer" => {
                    let from_address = step.parameters["from_address"]
                        .as_str()
                        .ok_or_else(|| anyhow::anyhow!("Missing from_address parameter"))?;
                    let to_address = step.parameters["to_address"]
                        .as_str()
                        .ok_or_else(|| anyhow::anyhow!("Missing to_address parameter"))?;
                    let amount_eth = step.parameters["amount_eth"]
                        .as_str()
                        .ok_or_else(|| anyhow::anyhow!("Missing amount_eth parameter"))?;

                    // Get private key
                    let private_key =
                        self.get_private_key_for_address(from_address)
                            .ok_or_else(|| {
                                anyhow::anyhow!(
                                    "No private key available for address: {}",
                                    from_address
                                )
                            })?;

                    mcp_client
                        .send_transfer(from_address, to_address, amount_eth, &private_key)
                        .await?
                }
                "check_contract" => {
                    let address = step.parameters["address"]
                        .as_str()
                        .ok_or_else(|| anyhow::anyhow!("Missing address parameter"))?;
                    mcp_client.check_contract(address).await?
                }
                "execute_swap" => {
                    let from_address = step.parameters["from_address"]
                        .as_str()
                        .ok_or_else(|| anyhow::anyhow!("Missing from_address parameter"))?;
                    let token_in = step.parameters["token_in"]
                        .as_str()
                        .ok_or_else(|| anyhow::anyhow!("Missing token_in parameter"))?;
                    let token_out = step.parameters["token_out"]
                        .as_str()
                        .ok_or_else(|| anyhow::anyhow!("Missing token_out parameter"))?;
                    let amount_in = step.parameters["amount_in"]
                        .as_str()
                        .ok_or_else(|| anyhow::anyhow!("Missing amount_in parameter"))?;
                    let slippage_bps = step
                        .parameters
                        .get("slippage_bps")
                        .and_then(|s| s.as_u64())
                        .map(|s| s as u16)
                        .unwrap_or(200);

                    // Get private key
                    let private_key =
                        self.get_private_key_for_address(from_address)
                            .ok_or_else(|| {
                                anyhow::anyhow!(
                                    "No private key available for address: {}",
                                    from_address
                                )
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
                _ => {
                    return Err(anyhow::anyhow!("Unknown tool: {}", step.tool_name));
                }
            };

            // Store result for potential use by dependent steps
            executed_steps.insert(
                step.step_id.clone(),
                serde_json::Value::String(result.clone()),
            );
            self.context.step_results.insert(
                step.step_id.clone(),
                serde_json::Value::String(result.clone()),
            );

            results.push(format!("✅ {}: {}", step.description, result));
        }

        Ok(results)
    }

    pub fn get_private_key_for_address(&self, address: &str) -> Option<String> {
        // Reuse the same logic from EthereumAgent
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

    // Delegation methods for RAG functionality
    pub async fn search_docs(&self, query: &str) -> Result<Vec<crate::rag::SearchResult>> {
        if let Some(rag) = &self.rag {
            rag.search(query, 15).await
        } else {
            Ok(vec![])
        }
    }

    pub async fn answer_documentation_query(&self, query: &str) -> Result<String> {
        // Reuse the same logic from EthereumAgent but simplified
        let search_results = self.search_docs(query).await?;

        if search_results.is_empty() {
            return Ok(
                "I don't have specific documentation for that query in my knowledge base."
                    .to_string(),
            );
        }

        let mut context = String::new();
        for result in search_results.iter().take(5) {
            context.push_str(&format!(
                "Source: {} ({})\n{}\n\n---\n\n",
                result.document.title, result.document.source, result.relevant_chunk
            ));
        }

        let system_prompt = format!(
            "You are an expert on Ethereum and Uniswap. Answer the user's question based on the provided documentation context. Be accurate and specific.\n\nContext:\n{context}"
        );

        let messages = json!([
            {
                "role": "user",
                "content": query
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

        let content = response_json["content"][0]["text"]
            .as_str()
            .unwrap_or("Unable to generate response");

        Ok(content.to_string())
    }
}
