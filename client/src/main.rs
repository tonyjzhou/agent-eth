mod agent;
mod mcp_client;

use agent::EthereumAgent;
use mcp_client::McpClient;
use anyhow::Result;
use clap::Parser;
use colored::*;
use rustyline::Editor;
use std::env;

#[derive(Parser)]
#[command(name = "agent-eth-client")]
#[command(about = "An AI agent for interacting with Ethereum blockchain")]
struct Cli {
    #[arg(long, default_value = "http://127.0.0.1:3000")]
    server_url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    let args = Cli::parse();
    
    let anthropic_api_key = env::var("ANTHROPIC_API_KEY")
        .expect("ANTHROPIC_API_KEY environment variable must be set");
    
    println!("{}", "🚀 Starting Ethereum AI Agent...".bright_blue().bold());
    
    // Initialize components
    let agent = EthereumAgent::new(&anthropic_api_key)?;
    let mcp_client = McpClient::new(&args.server_url);
    
    println!("{}", "✅ Connected to MCP server".bright_green());
    println!("{}", "💡 Type 'help' for available commands or 'exit' to quit".bright_yellow());
    println!();
    
    // REPL loop
    let mut rl = Editor::<(), rustyline::history::FileHistory>::new()?;
    loop {
        let readline = rl.readline(&format!("{} ", "eth>".bright_cyan().bold()));
        
        match readline {
            Ok(line) => {
                let input = line.trim();
                
                if input.is_empty() {
                    continue;
                }
                
                let _ = rl.add_history_entry(input);
                
                match input {
                    "exit" | "quit" => {
                        println!("{}", "👋 Goodbye!".bright_blue());
                        break;
                    }
                    "help" => {
                        print_help();
                        continue;
                    }
                    _ => {
                        if let Err(e) = handle_command(&agent, &mcp_client, input).await {
                            println!("{} {}", "❌ Error:".bright_red().bold(), e);
                        }
                    }
                }
            }
            Err(rustyline::error::ReadlineError::Interrupted) => {
                println!("{}", "^C".bright_yellow());
                continue;
            }
            Err(rustyline::error::ReadlineError::Eof) => {
                println!("{}", "👋 Goodbye!".bright_blue());
                break;
            }
            Err(err) => {
                println!("{} {}", "❌ Error:".bright_red().bold(), err);
                break;
            }
        }
    }
    
    Ok(())
}

async fn handle_command(
    agent: &EthereumAgent,
    mcp_client: &McpClient,
    input: &str,
) -> Result<()> {
    println!("{} {}", "🤖 Processing:".bright_blue(), input.italic());
    
    let parsed = agent.parse_command(input).await?;
    
    match parsed.action.as_str() {
        "balance" => {
            if let Some(address) = parsed.address {
                println!("{} {}", "🔍 Checking balance for:".bright_yellow(), address);
                let result = mcp_client.get_balance(&address, true).await?;
                println!("{} {}", "💰 Balance:".bright_green().bold(), result);
            } else {
                println!("{}", "❌ No address specified for balance check".bright_red());
            }
        }
        "transfer" => {
            if let (Some(from), Some(to), Some(amount)) = 
                (parsed.from_address.as_ref(), parsed.to_address.as_ref(), parsed.amount.as_ref()) {
                
                let from_addr = from.as_str();
                let private_key = agent.get_private_key_for_address(from_addr)
                    .ok_or_else(|| anyhow::anyhow!("No private key available for address: {}", from_addr))?;
                
                println!("{} {} ETH from {} to {}", 
                    "💸 Sending".bright_blue(), 
                    amount.bright_white().bold(),
                    from_addr.bright_cyan(),
                    to.bright_cyan());
                
                let result = mcp_client.send_transfer(from_addr, to, amount, &private_key).await?;
                println!("{}", result.bright_green());
            } else {
                println!("{}", "❌ Missing parameters for transfer (need from, to, amount)".bright_red());
            }
        }
        "contract_check" => {
            if let Some(address) = parsed.address {
                println!("{} {}", "🔍 Checking contract at:".bright_yellow(), address);
                let result = mcp_client.check_contract(&address).await?;
                println!("{}", result);
            } else {
                println!("{}", "❌ No address specified for contract check".bright_red());
            }
        }
        _ => {
            println!("{} {}", "❓ Unknown action:".bright_red(), parsed.action);
        }
    }
    
    println!();
    Ok(())
}

fn print_help() {
    println!("{}", "📖 Available Commands:".bright_blue().bold());
    println!();
    println!("  {} - Get ETH balance", "balance <address>".bright_cyan());
    println!("    Example: {}", "How much ETH does Alice have?".italic());
    println!();
    println!("  {} - Send ETH transfer", "transfer <amount> from <sender> to <receiver>".bright_cyan());
    println!("    Example: {}", "send 1 ETH from Alice to Bob".italic());
    println!();
    println!("  {} - Check contract deployment", "contract_check <address>".bright_cyan());
    println!("    Example: {}", "Is Uniswap V2 Router deployed at 0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D?".italic());
    println!();
    println!("  {} - Account aliases:", "Available accounts:".bright_yellow().bold());
    println!("    • Alice: 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
    println!("    • Bob:   0x70997970C51812dc3A010C7d01b50e0d17dc79C8");
    println!("    • Carol: 0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC");
    println!();
    println!("  {} - Show this help", "help".bright_cyan());
    println!("  {} - Exit the program", "exit".bright_cyan());
    println!();
}