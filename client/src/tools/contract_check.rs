use super::Tool;
use crate::json_schema;
use crate::mcp_client::McpClient;
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractCheckParams {
    pub address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractCheckResult {
    pub address: String,
    pub is_contract: bool,
    pub code_size: Option<u64>,
    pub contract_type: Option<String>,
}

pub struct ContractCheckTool;

#[async_trait]
impl Tool for ContractCheckTool {
    type Input = ContractCheckParams;
    type Output = ContractCheckResult;

    fn name(&self) -> &'static str {
        "check_contract"
    }

    fn description(&self) -> &'static str {
        "Check if an address has deployed contract code"
    }

    fn input_schema(&self) -> serde_json::Value {
        json_schema!(ContractCheckParams, {
            address: "string", true, "The contract address to check",
        })
    }

    async fn execute(&self, input: Self::Input) -> Result<Self::Output> {
        // Mock result for demonstration
        Ok(ContractCheckResult {
            address: input.address.clone(),
            is_contract: true,
            code_size: Some(1024),
            contract_type: Some("ERC20".to_string()),
        })
    }
}

impl ContractCheckTool {
    /// Integration point with existing MCP client
    #[allow(dead_code)]
    pub async fn execute_with_mcp(
        &self,
        input: ContractCheckParams,
        mcp_client: &mut McpClient,
    ) -> Result<ContractCheckResult> {
        let result = mcp_client.check_contract(&input.address).await?;

        // Parse the result string into structured data
        let is_contract = result.contains("true") || result.contains("contract");

        Ok(ContractCheckResult {
            address: input.address,
            is_contract,
            code_size: None,     // Would need to be extracted from result
            contract_type: None, // Would need analysis to determine
        })
    }
}
