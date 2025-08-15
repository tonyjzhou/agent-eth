use super::Tool;
use crate::json_schema;
use crate::mcp_client::McpClient;
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BalanceParams {
    pub address: String,
    pub token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BalanceResult {
    pub address: String,
    pub token: String,
    pub balance: String,
    pub formatted_balance: String,
}

pub struct BalanceTool;

#[async_trait]
impl Tool for BalanceTool {
    type Input = BalanceParams;
    type Output = BalanceResult;

    fn name(&self) -> &'static str {
        "get_balance"
    }

    fn description(&self) -> &'static str {
        "Get the balance of an Ethereum address for ETH or ERC20 tokens"
    }

    fn input_schema(&self) -> serde_json::Value {
        json_schema!(BalanceParams, {
            address: "string", true, "The Ethereum address to check",
            token: "string", false, "Token symbol (defaults to ETH)",
        })
    }

    async fn execute(&self, input: Self::Input) -> Result<Self::Output> {
        // This would integrate with MCP client
        // For now, return a mock result to demonstrate the structure
        Ok(BalanceResult {
            address: input.address.clone(),
            token: input.token.unwrap_or_else(|| "ETH".to_string()),
            balance: "1000000000000000000".to_string(), // 1 ETH in wei
            formatted_balance: "1.0 ETH".to_string(),
        })
    }
}

impl BalanceTool {
    /// Integration point with existing MCP client
    #[allow(dead_code)]
    pub async fn execute_with_mcp(
        &self,
        input: BalanceParams,
        mcp_client: &mut McpClient,
    ) -> Result<BalanceResult> {
        let result = mcp_client
            .get_balance(&input.address, input.token.as_deref())
            .await?;

        // Parse the result string into structured data
        Ok(BalanceResult {
            address: input.address,
            token: input.token.unwrap_or_else(|| "ETH".to_string()),
            balance: result.clone(),
            formatted_balance: result,
        })
    }
}
