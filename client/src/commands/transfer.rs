use crate::agent::AgentCore;
use crate::error::AgentError;
use crate::mcp_client::McpClient;
use anyhow::Result;
use colored::*;
use inquire::Confirm;

pub struct TransferCommand;

impl TransferCommand {
    pub async fn execute(
        amount: &str,
        from: &str,
        to: &str,
        mcp_client: &mut McpClient,
        agent: &AgentCore,
    ) -> Result<()> {
        let from_addr = Self::resolve_address(from);
        let to_addr = Self::resolve_address(to);

        let private_key = agent
            .get_private_key_for_address(&from_addr)
            .ok_or_else(|| AgentError::private_key(&from_addr))?;

        println!(
            "{} {} ETH from {} to {}",
            "💸 Sending".bright_blue(),
            amount.bright_white().bold(),
            from_addr.bright_cyan(),
            to_addr.bright_cyan()
        );

        // Confirm the transfer
        let confirm = Confirm::new("Confirm this ETH transfer?")
            .with_default(true)
            .with_help_message("This will execute a real blockchain transaction")
            .prompt()?;

        if !confirm {
            println!("{}", "❌ Transfer cancelled".bright_red());
            return Ok(());
        }

        let result = mcp_client
            .send_transfer(&from_addr, &to_addr, amount, &private_key)
            .await?;
        println!("{}", result.bright_green());
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
