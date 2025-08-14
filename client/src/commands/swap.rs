use crate::agent::EthereumAgent;
use crate::error::AgentError;
use crate::mcp_client::McpClient;
use anyhow::Result;
use colored::*;
use inquire::Confirm;

pub struct SwapCommand;

impl SwapCommand {
    pub async fn execute(
        amount: &str,
        token_in: &str,
        token_out: &str,
        from: &str,
        slippage: f64,
        mcp_client: &mut McpClient,
        agent: &EthereumAgent,
    ) -> Result<()> {
        let from_addr = Self::resolve_address(from);

        let private_key = agent
            .get_private_key_for_address(&from_addr)
            .ok_or_else(|| AgentError::private_key(&from_addr))?;

        let slippage_bps = (slippage * 100.0) as u16;

        println!(
            "{} {} {} for {} (slippage: {}%)",
            "🔄 Swapping".bright_blue(),
            amount.bright_white().bold(),
            token_in.to_uppercase().bright_cyan(),
            token_out.to_uppercase().bright_cyan(),
            slippage
        );

        // Confirm the swap
        let confirm = Confirm::new("Confirm this token swap?")
            .with_default(true)
            .with_help_message("This will execute a real Uniswap transaction")
            .prompt()?;

        if !confirm {
            println!("{}", "❌ Swap cancelled".bright_red());
            return Ok(());
        }

        let result = mcp_client
            .execute_swap(
                &from_addr,
                token_in,
                token_out,
                amount,
                slippage_bps,
                &private_key,
            )
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
