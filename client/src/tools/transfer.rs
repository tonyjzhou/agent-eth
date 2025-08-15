use super::Tool;
use crate::json_schema;
use crate::mcp_client::McpClient;
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferParams {
    pub from_address: String,
    pub to_address: String,
    pub amount_eth: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferResult {
    pub transaction_hash: String,
    pub from_address: String,
    pub to_address: String,
    pub amount: String,
    pub gas_used: Option<String>,
    pub status: String,
}

pub struct TransferTool;

#[async_trait]
impl Tool for TransferTool {
    type Input = TransferParams;
    type Output = TransferResult;

    fn name(&self) -> &'static str {
        "send_transfer"
    }

    fn description(&self) -> &'static str {
        "Send ETH from one address to another"
    }

    fn input_schema(&self) -> serde_json::Value {
        json_schema!(TransferParams, {
            from_address: "string", true, "The sender's address",
            to_address: "string", true, "The recipient's address",
            amount_eth: "string", true, "Amount of ETH to send",
        })
    }

    async fn execute(&self, input: Self::Input) -> Result<Self::Output> {
        // Mock result for demonstration
        Ok(TransferResult {
            transaction_hash: "0x1234567890abcdef".to_string(),
            from_address: input.from_address,
            to_address: input.to_address,
            amount: input.amount_eth,
            gas_used: Some("21000".to_string()),
            status: "success".to_string(),
        })
    }
}

impl TransferTool {
    /// Integration point with existing MCP client
    #[allow(dead_code)]
    pub async fn execute_with_mcp(
        &self,
        input: TransferParams,
        mcp_client: &mut McpClient,
        private_key: &str,
    ) -> Result<TransferResult> {
        let result = mcp_client
            .send_transfer(
                &input.from_address,
                &input.to_address,
                &input.amount_eth,
                private_key,
            )
            .await?;

        // Parse the result string into structured data
        Ok(TransferResult {
            transaction_hash: result.clone(),
            from_address: input.from_address,
            to_address: input.to_address,
            amount: input.amount_eth,
            gas_used: None, // Would need to be extracted from result
            status: "success".to_string(),
        })
    }
}
