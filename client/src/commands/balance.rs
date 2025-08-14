use crate::mcp_client::McpClient;
use anyhow::Result;
use colored::*;

pub struct BalanceCommand;

impl BalanceCommand {
    pub async fn execute(
        address: &str,
        token: Option<&str>,
        mcp_client: &mut McpClient,
    ) -> Result<()> {
        let token_display = token.unwrap_or("ETH");

        println!(
            "{} {} {}",
            "🔍 Checking".bright_yellow(),
            token_display,
            "balance for:".bright_yellow()
        );
        println!("  Address: {}", address.bright_cyan());

        let result = mcp_client.get_balance(address, token).await?;
        println!("{} {}", "💰 Balance:".bright_green().bold(), result);
        println!();

        Ok(())
    }

    /// Resolve account aliases (Alice/Bob/Carol) to hex addresses
    pub fn resolve_address(input: &str) -> String {
        match input.to_lowercase().as_str() {
            "alice" => "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".to_string(),
            "bob" => "0x70997970C51812dc3A010C7d01b50e0d17dc79C8".to_string(),
            "carol" => "0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC".to_string(),
            _ => input.to_string(),
        }
    }
}
