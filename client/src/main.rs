mod agent;
mod mcp_client;
mod rag;

use agent::EthereumAgent;
use anyhow::Result;
use clap::Parser;
use colored::*;
use mcp_client::McpClient;
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

    let anthropic_api_key =
        env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY environment variable must be set");

    println!(
        "{}",
        "🚀 Starting Ethereum AI Agent...".bright_blue().bold()
    );

    // Initialize components
    let mut agent = EthereumAgent::new(&anthropic_api_key)?;
    let mcp_client = McpClient::new(&args.server_url);

    // Initialize RAG system
    let rag_db_path = "client_rag.db";
    if let Err(e) = agent.initialize_rag(rag_db_path).await {
        println!(
            "{} Failed to initialize RAG system: {}",
            "⚠️".bright_yellow(),
            e
        );
        println!(
            "{}",
            "Continuing without documentation search...".bright_yellow()
        );
    } else {
        println!("{}", "✅ RAG system initialized".bright_green());
    }

    println!("{}", "✅ Connected to MCP server".bright_green());
    println!(
        "{}",
        "💡 Type 'help' for available commands or 'exit' to quit".bright_yellow()
    );
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
                    line if line.starts_with("test swap") => {
                        // Manual test without Claude API
                        println!("🔄 Testing swap functionality manually...");
                        let test_command = agent::ParsedCommand {
                            action: "swap".to_string(),
                            from_address: Some(
                                "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".to_string(),
                            ),
                            to_address: None,
                            amount: None,
                            address: None,
                            token_in: Some("ETH".to_string()),
                            token_out: Some("USDC".to_string()),
                            amount_in: Some("0.1".to_string()),
                            slippage_bps: Some(200),
                            token: None,
                        };

                        if let Err(e) = handle_test_swap(&mcp_client, test_command).await {
                            println!("{} {}", "❌ Test Error:".bright_red().bold(), e);
                        }
                        continue;
                    }
                    line if line.starts_with("docs ") || line.starts_with("explain ") => {
                        // Handle documentation queries
                        let query = if let Some(stripped) = line.strip_prefix("docs ") {
                            stripped
                        } else {
                            &line[8..]
                        };

                        println!(
                            "{} {}",
                            "🔍 Searching docs for:".bright_blue(),
                            query.italic()
                        );

                        if let Ok(answer) = agent.answer_documentation_query(query).await {
                            println!("{} {}", "📖".bright_blue(), answer);
                        } else {
                            println!("{}", "❌ Failed to search documentation".bright_red());
                        }
                        continue;
                    }
                    _ => {
                        // Check for CLI commands first before documentation queries
                        if let Some(dir_path) = input.strip_prefix("ingest ") {
                            println!(
                                "{} {}",
                                "📚 Ingesting documents from:".bright_blue(),
                                dir_path
                            );

                            match agent.ingest_documents(dir_path).await {
                                Ok(count) => {
                                    println!("{} Ingested {} documents", "✅".bright_green(), count)
                                }
                                Err(e) => println!("{} Failed to ingest: {}", "❌".bright_red(), e),
                            }
                            continue;
                        }

                        // Check if this might be a documentation query
                        if agent.is_documentation_query(input).await {
                            println!(
                                "{} {}",
                                "🔍 Searching docs for:".bright_blue(),
                                input.italic()
                            );

                            if let Ok(answer) = agent.answer_documentation_query(input).await {
                                println!("{} {}", "📖".bright_blue(), answer);
                                continue;
                            }
                        }

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

async fn handle_command(agent: &EthereumAgent, mcp_client: &McpClient, input: &str) -> Result<()> {
    println!("{} {}", "🤖 Processing:".bright_blue(), input.italic());

    let parsed = agent.parse_command(input).await?;

    match parsed.action.as_str() {
        "balance" => {
            if let Some(address) = parsed.address {
                let token = parsed.token.as_deref();
                let token_display = token.unwrap_or("ETH");
                println!(
                    "{} {} {}",
                    "🔍 Checking".bright_yellow(),
                    token_display,
                    "balance for:".bright_yellow()
                );
                println!("  Address: {}", address.bright_cyan());
                let result = mcp_client.get_balance(&address, token).await?;
                println!("{} {}", "💰 Balance:".bright_green().bold(), result);
            } else {
                println!(
                    "{}",
                    "❌ No address specified for balance check".bright_red()
                );
            }
        }
        "transfer" => {
            if let (Some(from), Some(to), Some(amount)) = (
                parsed.from_address.as_ref(),
                parsed.to_address.as_ref(),
                parsed.amount.as_ref(),
            ) {
                let from_addr = from.as_str();
                let private_key =
                    agent
                        .get_private_key_for_address(from_addr)
                        .ok_or_else(|| {
                            anyhow::anyhow!("No private key available for address: {}", from_addr)
                        })?;

                println!(
                    "{} {} ETH from {} to {}",
                    "💸 Sending".bright_blue(),
                    amount.bright_white().bold(),
                    from_addr.bright_cyan(),
                    to.bright_cyan()
                );

                let result = mcp_client
                    .send_transfer(from_addr, to, amount, &private_key)
                    .await?;
                println!("{}", result.bright_green());
            } else {
                println!(
                    "{}",
                    "❌ Missing parameters for transfer (need from, to, amount)".bright_red()
                );
            }
        }
        "contract_check" => {
            if let Some(address) = parsed.address {
                println!("{} {}", "🔍 Checking contract at:".bright_yellow(), address);
                let result = mcp_client.check_contract(&address).await?;
                println!("{result}");
            } else {
                println!(
                    "{}",
                    "❌ No address specified for contract check".bright_red()
                );
            }
        }
        "swap" => {
            if let (Some(from), Some(token_in), Some(token_out), Some(amount_in)) = (
                parsed.from_address.as_ref(),
                parsed.token_in.as_ref(),
                parsed.token_out.as_ref(),
                parsed.amount_in.as_ref(),
            ) {
                let from_addr = from.as_str();
                let private_key =
                    agent
                        .get_private_key_for_address(from_addr)
                        .ok_or_else(|| {
                            anyhow::anyhow!("No private key available for address: {}", from_addr)
                        })?;

                let slippage_bps = parsed.slippage_bps.unwrap_or(200);

                println!(
                    "{} {} {} for {} (slippage: {}%)",
                    "🔄 Swapping".bright_blue(),
                    amount_in.bright_white().bold(),
                    token_in.to_uppercase().bright_cyan(),
                    token_out.to_uppercase().bright_cyan(),
                    slippage_bps as f64 / 100.0
                );

                let result = mcp_client
                    .execute_swap(
                        from_addr,
                        token_in,
                        token_out,
                        amount_in,
                        slippage_bps,
                        &private_key,
                    )
                    .await?;
                println!("{}", result.bright_green());
            } else {
                println!(
                    "{}",
                    "❌ Missing parameters for swap (need from_address, token_in, token_out, amount_in)".bright_red()
                );
            }
        }
        _ => {
            println!("{} {}", "❓ Unknown action:".bright_red(), parsed.action);
        }
    }

    println!();
    Ok(())
}

async fn handle_test_swap(mcp_client: &McpClient, parsed: agent::ParsedCommand) -> Result<()> {
    println!("🔍 Debug: Test parsed action = '{}'", parsed.action);

    if let (Some(from), Some(token_in), Some(token_out), Some(amount_in)) = (
        parsed.from_address.as_ref(),
        parsed.token_in.as_ref(),
        parsed.token_out.as_ref(),
        parsed.amount_in.as_ref(),
    ) {
        let private_key = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"; // Alice's test key
        let slippage_bps = parsed.slippage_bps.unwrap_or(200);

        println!(
            "{} {} {} for {} (slippage: {}%)",
            "🔄 Testing Swap:".bright_blue(),
            amount_in.bright_white().bold(),
            token_in.to_uppercase().bright_cyan(),
            token_out.to_uppercase().bright_cyan(),
            slippage_bps as f64 / 100.0
        );

        let result = mcp_client
            .execute_swap(
                from,
                token_in,
                token_out,
                amount_in,
                slippage_bps,
                private_key,
            )
            .await?;
        println!("{}", result.bright_green());
    } else {
        println!(
            "{}",
            "❌ Test command missing required parameters".bright_red()
        );
    }

    Ok(())
}

fn print_help() {
    println!("{}", "📖 Available Commands:".bright_blue().bold());
    println!();
    println!("  {} - Get ETH balance", "balance <address>".bright_cyan());
    println!("    Example: {}", "How much ETH does Alice have?".italic());
    println!();
    println!(
        "  {} - Send ETH transfer",
        "transfer <amount> from <sender> to <receiver>".bright_cyan()
    );
    println!("    Example: {}", "send 1 ETH from Alice to Bob".italic());
    println!();
    println!(
        "  {} - Check contract deployment",
        "contract_check <address>".bright_cyan()
    );
    println!(
        "    Example: {}",
        "Is Uniswap V2 Router deployed at 0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D?".italic()
    );
    println!();
    println!(
        "  {} - Swap tokens using Uniswap",
        "swap <amount> <token_in> for <token_out> from <address>".bright_cyan()
    );
    println!(
        "    Example: {}",
        "Use Uniswap V2 Router to swap 10 ETH for USDC on Alice's account".italic()
    );
    println!();
    println!(
        "  {} - Manual test swap (bypasses Claude API)",
        "test swap".bright_cyan()
    );
    println!();
    println!("  {}:", "Available accounts".bright_yellow().bold());
    println!("    • Alice: 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
    println!("    • Bob:   0x70997970C51812dc3A010C7d01b50e0d17dc79C8");
    println!("    • Carol: 0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC");
    println!();
    println!();
    println!(
        "  {} - Search documentation",
        "docs <query> or explain <query>".bright_cyan()
    );
    println!(
        "    Example: {}",
        "docs How do I calculate slippage?".italic()
    );
    println!();
    println!(
        "  {} - Ingest documents into RAG system",
        "ingest <directory_path>".bright_cyan()
    );
    println!("    Example: {}", "ingest client/docs".italic());
    println!();
    println!("  {} - Show this help", "help".bright_cyan());
    println!("  {} - Exit the program", "exit".bright_cyan());
    println!();
}
