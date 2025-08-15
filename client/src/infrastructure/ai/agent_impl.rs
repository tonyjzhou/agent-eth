use crate::application::{
    agent_service::{ExecutionPlan, ExecutionResult, ExecutionStep, ToolAgent},
    use_cases::*,
};
use crate::domain::services::*;
use crate::infrastructure::ai::claude_client::{extract_json_from_response, ClaudeClient};
use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

pub struct IntelligentAgent<E: EthereumService, A: AccountService> {
    claude_client: ClaudeClient,
    ethereum_service: Arc<E>,
    account_service: Arc<A>,
    session_memory: Vec<String>,
}

impl<E: EthereumService, A: AccountService> IntelligentAgent<E, A> {
    pub fn new(api_key: String, ethereum_service: Arc<E>, account_service: Arc<A>) -> Self {
        Self {
            claude_client: ClaudeClient::new(api_key),
            ethereum_service,
            account_service,
            session_memory: Vec::new(),
        }
    }

    fn build_context(&self) -> String {
        let mut context = String::new();

        // Add tool information
        context.push_str("Available blockchain tools:\n");
        context.push_str("- get_balance: Check ETH or token balance for an address\n");
        context.push_str("- send_transfer: Send ETH from one address to another\n");
        context.push_str("- execute_swap: Swap tokens using Uniswap\n");
        context.push_str("- check_contract: Check if an address is a contract\n\n");

        // Add account aliases
        context.push_str("Available test accounts:\n");
        for account in self.account_service.get_all_accounts() {
            if let Some(alias) = &account.alias {
                context.push_str(&format!("- {}: {}\n", alias, account.address));
            }
        }

        // Add session memory
        if !self.session_memory.is_empty() {
            context.push_str("\nRecent operations:\n");
            for memory in self.session_memory.iter().rev().take(5) {
                context.push_str(&format!("- {memory}\n"));
            }
        }

        context
    }

    fn get_rag_context(&self, _query: &str) -> String {
        // RAG context would be handled separately to avoid Send/Sync issues
        String::new()
    }
}

#[async_trait]
impl<E: EthereumService + Send + Sync, A: AccountService + Send + Sync> ToolAgent
    for IntelligentAgent<E, A>
{
    async fn plan_execution(&mut self, request: &str) -> Result<ExecutionPlan> {
        let context = self.build_context();
        let rag_context = self.get_rag_context(request);

        let system_prompt = format!(
            r#"You are an intelligent Ethereum blockchain agent that creates execution plans for user requests.

Your task is to analyze the user's request and create a detailed execution plan using the available tools.

{context}{rag_context}

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

        let response = self
            .claude_client
            .chat_completion(
                &system_prompt,
                &format!("Create an execution plan for: {request}"),
            )
            .await?;

        // Extract JSON from potential markdown code blocks
        let cleaned_content = extract_json_from_response(&response);

        let plan: ExecutionPlan = serde_json::from_str(&cleaned_content).map_err(|e| {
            anyhow::anyhow!(
                "Failed to parse agent plan: {}\nOriginal response: {}\nCleaned content: {}",
                e,
                response,
                cleaned_content
            )
        })?;

        Ok(plan)
    }

    async fn execute_plan(&mut self, plan: ExecutionPlan) -> Result<ExecutionResult> {
        let mut results = Vec::new();
        let mut errors = Vec::new();
        let mut executed_steps = HashMap::new();

        // Add this operation to session memory
        self.session_memory
            .push(format!("Executed plan: {}", plan.reasoning));

        for step in &plan.steps {
            // Check dependencies
            if let Some(depends_on) = &step.depends_on {
                if !executed_steps.contains_key(depends_on) {
                    let error_msg = format!(
                        "Step {} depends on {}, but that step hasn't been executed",
                        step.step_id, depends_on
                    );
                    errors.push(error_msg);
                    continue;
                }
            }

            // Execute the step based on tool name
            let result = match step.tool_name.as_str() {
                "get_balance" => self.execute_balance_step(step).await,
                "send_transfer" => self.execute_transfer_step(step).await,
                "execute_swap" => self.execute_swap_step(step).await,
                "check_contract" => self.execute_contract_check_step(step).await,
                _ => Err(anyhow::anyhow!("Unknown tool: {}", step.tool_name)),
            };

            match result {
                Ok(step_result) => {
                    executed_steps.insert(
                        step.step_id.clone(),
                        serde_json::Value::String(step_result.clone()),
                    );
                    results.push(format!("✅ {}: {}", step.description, step_result));
                }
                Err(e) => {
                    errors.push(format!("❌ {}: {}", step.description, e));
                }
            }
        }

        Ok(ExecutionResult {
            success: errors.is_empty(),
            results,
            errors,
        })
    }
}

impl<E: EthereumService, A: AccountService> IntelligentAgent<E, A> {
    async fn execute_balance_step(&self, step: &ExecutionStep) -> Result<String> {
        let address_str = step.parameters["address"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing address parameter"))?;

        let token_symbol = step.parameters.get("token").and_then(|t| t.as_str());

        let use_case =
            GetBalanceUseCase::new(self.ethereum_service.clone(), self.account_service.clone());

        let balance = use_case.execute(address_str, token_symbol).await?;
        Ok(format!("{} {}", balance.amount, balance.token.symbol))
    }

    async fn execute_transfer_step(&self, step: &ExecutionStep) -> Result<String> {
        let from_address = step.parameters["from_address"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing from_address parameter"))?;
        let to_address = step.parameters["to_address"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing to_address parameter"))?;
        let amount_eth = step.parameters["amount_eth"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing amount_eth parameter"))?;

        let use_case = ExecuteTransferUseCase::new(
            self.ethereum_service.clone(),
            self.account_service.clone(),
        );

        let tx_hash = use_case
            .execute(from_address, to_address, amount_eth)
            .await?;
        Ok(format!("Transaction sent: {tx_hash}"))
    }

    async fn execute_swap_step(&self, step: &ExecutionStep) -> Result<String> {
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
            .map(|s| s as u16);

        let use_case =
            ExecuteSwapUseCase::new(self.ethereum_service.clone(), self.account_service.clone());

        let tx_hash = use_case
            .execute(from_address, token_in, token_out, amount_in, slippage_bps)
            .await?;
        Ok(format!("Swap executed: {tx_hash}"))
    }

    async fn execute_contract_check_step(&self, step: &ExecutionStep) -> Result<String> {
        let address = step.parameters["address"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing address parameter"))?;

        let use_case = CheckContractUseCase::new(self.ethereum_service.clone());

        let contract_info = use_case.execute(address).await?;
        if contract_info.is_contract {
            Ok(format!("Contract detected at {}", contract_info.address))
        } else {
            Ok(format!("No contract at {}", contract_info.address))
        }
    }
}
