use super::Tool;
use crate::json_schema;
use crate::mcp_client::McpClient;
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SwapParams {
    pub from_address: String,
    pub token_in: String,
    pub token_out: String,
    pub amount_in: String,
    pub slippage_bps: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwapResult {
    pub transaction_hash: String,
    pub from_address: String,
    pub token_in: String,
    pub token_out: String,
    pub amount_in: String,
    pub amount_out: String,
    pub slippage_used: u16,
    pub gas_used: Option<String>,
    pub status: String,
}

pub struct SwapTool;

#[async_trait]
impl Tool for SwapTool {
    type Input = SwapParams;
    type Output = SwapResult;

    fn name(&self) -> &'static str {
        "execute_swap"
    }

    fn description(&self) -> &'static str {
        "Execute a token swap using Uniswap V2 Router"
    }

    fn input_schema(&self) -> serde_json::Value {
        json_schema!(SwapParams, {
            from_address: "string", true, "The address executing the swap",
            token_in: "string", true, "Input token symbol",
            token_out: "string", true, "Output token symbol",
            amount_in: "string", true, "Amount of input tokens",
            slippage_bps: "number", false, "Slippage tolerance in basis points (default 200)",
        })
    }

    async fn execute(&self, input: Self::Input) -> Result<Self::Output> {
        // Mock result for demonstration
        Ok(SwapResult {
            transaction_hash: "0xabcdef1234567890".to_string(),
            from_address: input.from_address,
            token_in: input.token_in,
            token_out: input.token_out,
            amount_in: input.amount_in,
            amount_out: "2000.0".to_string(),
            slippage_used: input.slippage_bps.unwrap_or(200),
            gas_used: Some("150000".to_string()),
            status: "success".to_string(),
        })
    }
}

impl SwapTool {
    /// Integration point with existing MCP client  
    #[allow(dead_code)]
    pub async fn execute_with_mcp(
        &self,
        input: SwapParams,
        mcp_client: &mut McpClient,
        private_key: &str,
    ) -> Result<SwapResult> {
        let result = mcp_client
            .execute_swap(
                &input.from_address,
                &input.token_in,
                &input.token_out,
                &input.amount_in,
                input.slippage_bps.unwrap_or(200),
                private_key,
            )
            .await?;

        // Parse the result string into structured data
        Ok(SwapResult {
            transaction_hash: result.clone(),
            from_address: input.from_address,
            token_in: input.token_in,
            token_out: input.token_out,
            amount_in: input.amount_in,
            amount_out: "Unknown".to_string(), // Would need to be extracted from result
            slippage_used: input.slippage_bps.unwrap_or(200),
            gas_used: None, // Would need to be extracted from result
            status: "success".to_string(),
        })
    }
}
