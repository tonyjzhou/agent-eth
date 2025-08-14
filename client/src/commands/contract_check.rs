use crate::mcp_client::McpClient;
use anyhow::Result;
use colored::*;

pub struct ContractCheckCommand;

impl ContractCheckCommand {
    pub async fn execute(address: &str, mcp_client: &mut McpClient) -> Result<()> {
        println!("{} {}", "🔍 Checking contract at:".bright_yellow(), address);

        let result = mcp_client.check_contract(address).await?;
        println!("{result}");
        println!();

        Ok(())
    }
}
