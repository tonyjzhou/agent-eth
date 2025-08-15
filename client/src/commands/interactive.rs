use crate::agent::AgentCore;
use crate::mcp_client::McpClient;
use anyhow::Result;
use colored::*;
use rustyline::Editor;
use std::io::{self, Write};

pub struct InteractiveCommand;

impl InteractiveCommand {
    pub async fn start(agent_core: &mut AgentCore, mcp_client: &mut McpClient) -> Result<()> {
        println!("{}", "🚀 Starting Interactive Mode...".bright_blue().bold());
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
                            Self::print_help();
                            continue;
                        }
                        "clear" | "clear docs" => {
                            println!(
                                "{}",
                                "🗑️  Clearing all ingested documents...".bright_yellow()
                            );

                            match agent_core.clear_documents().await {
                                Ok(()) => {
                                    println!(
                                        "{} All documents cleared from RAG system",
                                        "✅".bright_green()
                                    )
                                }
                                Err(e) => {
                                    println!(
                                        "{} Failed to clear documents: {}",
                                        "❌".bright_red(),
                                        e
                                    )
                                }
                            }
                            continue;
                        }
                        line if line.starts_with("test swap") => {
                            // Manual test without Claude API
                            println!("🔄 Testing swap functionality manually...");
                            let test_command = crate::agent::ParsedCommand {
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

                            if let Err(e) = Self::handle_test_swap(mcp_client, test_command).await {
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

                            if let Ok(answer) = agent_core.answer_documentation_query(query).await {
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

                                match agent_core.ingest_documents(dir_path).await {
                                    Ok(count) => {
                                        println!(
                                            "{} Ingested {} documents",
                                            "✅".bright_green(),
                                            count
                                        )
                                    }
                                    Err(e) => {
                                        println!("{} Failed to ingest: {}", "❌".bright_red(), e)
                                    }
                                }
                                continue;
                            }

                            // Check for intelligent agent requests (complex operations)
                            if Self::is_complex_operation(input) {
                                if let Err(e) =
                                    Self::handle_intelligent_command(agent_core, mcp_client, input)
                                        .await
                                {
                                    println!("{} {}", "❌ Agent Error:".bright_red().bold(), e);
                                    // Fallback to simple parsing on agent error
                                    println!(
                                        "{}",
                                        "🔄 Falling back to simple command parsing..."
                                            .bright_yellow()
                                    );
                                    if let Err(e2) =
                                        Self::handle_command(agent_core, mcp_client, input).await
                                    {
                                        println!(
                                            "{} {}",
                                            "❌ Fallback Error:".bright_red().bold(),
                                            e2
                                        );
                                    }
                                }
                                continue;
                            }

                            // Check if this might be a documentation query
                            if agent_core.is_documentation_query(input).await {
                                println!(
                                    "{} {}",
                                    "🔍 Searching docs for:".bright_blue(),
                                    input.italic()
                                );

                                if let Ok(answer) =
                                    agent_core.answer_documentation_query(input).await
                                {
                                    println!("{} {}", "📖".bright_blue(), answer);
                                    continue;
                                }
                            }

                            // Default to legacy command handling for simple operations
                            if let Err(e) =
                                Self::handle_command(agent_core, mcp_client, input).await
                            {
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

    // Helper functions extracted from main.rs
    async fn handle_command(
        agent: &AgentCore,
        mcp_client: &mut McpClient,
        input: &str,
    ) -> Result<()> {
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
                                anyhow::anyhow!(
                                    "No private key available for address: {}",
                                    from_addr
                                )
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
                                anyhow::anyhow!(
                                    "No private key available for address: {}",
                                    from_addr
                                )
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

    async fn handle_test_swap(
        mcp_client: &mut McpClient,
        parsed: crate::agent::ParsedCommand,
    ) -> Result<()> {
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

    // Helper function to determine if an operation is complex enough for the intelligent agent
    fn is_complex_operation(input: &str) -> bool {
        let input_lower = input.to_lowercase();

        // Multi-step operation indicators
        let multi_step_patterns = [
            "if",
            "then",
            "after",
            "before",
            "once",
            "when",
            "check if",
            "and then",
            "but only if",
            "depending on",
            "compare",
            "optimal",
            "best",
            "cheapest",
            "most efficient",
        ];

        // Complex operation types
        let complex_patterns = [
            "find the best",
            "compare prices",
            "what's the cheapest",
            "find contract address for",
            "lookup",
            "search for address",
            "check both",
            "check all",
            "multiple",
            "several",
            "various",
            "plan",
            "strategy",
            "optimize",
            "analysis",
            "analyze",
        ];

        // Conditional operations
        let conditional_patterns = [
            "if alice has more than",
            "if balance is",
            "if there's enough",
            "only if",
            "provided that",
            "assuming",
            "check first",
        ];

        multi_step_patterns
            .iter()
            .any(|&pattern| input_lower.contains(pattern))
            || complex_patterns
                .iter()
                .any(|&pattern| input_lower.contains(pattern))
            || conditional_patterns
                .iter()
                .any(|&pattern| input_lower.contains(pattern))
    }

    async fn handle_intelligent_command(
        agent_core: &mut AgentCore,
        mcp_client: &mut McpClient,
        input: &str,
    ) -> Result<()> {
        println!(
            "{} {}",
            "🤖 AI Agent analyzing:".bright_blue(),
            input.italic()
        );

        // Plan the execution
        let plan = agent_core.plan_execution(input).await?;

        // Show the plan to user
        println!("{}", "📋 Execution Plan:".bright_green().bold());
        println!("💭 {}", plan.reasoning.bright_white());
        println!();

        for (i, step) in plan.steps.iter().enumerate() {
            println!(
                "{}. {} ({})",
                (i + 1).to_string().bright_cyan(),
                step.description.bright_white(),
                step.tool_name.bright_black()
            );
        }
        println!();

        // Ask for confirmation if needed
        if plan.requires_confirmation {
            println!(
                "{}",
                "⚠️ This operation requires confirmation."
                    .bright_yellow()
                    .bold()
            );
            print!("Continue? (y/N): ");
            io::stdout().flush()?;

            let mut response = String::new();
            io::stdin().read_line(&mut response)?;

            if !response.trim().to_lowercase().starts_with('y') {
                println!("{}", "❌ Operation cancelled".bright_red());
                return Ok(());
            }
        }

        // Execute the plan
        println!("{}", "⚡ Executing plan...".bright_blue().bold());
        let results = agent_core.execute_plan(plan, mcp_client).await?;

        // Show results
        println!("{}", "🎉 Results:".bright_green().bold());
        for result in results {
            println!("  {result}");
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
        println!(
            "  {}:",
            "🧠 Intelligent Agent Examples".bright_magenta().bold()
        );
        println!(
            "    • {}",
            "Check Alice's balance, and if she has more than 5 ETH, swap 2 ETH for USDC".italic()
        );
        println!(
            "    • {}",
            "Find the best swap rate between USDC and WETH".italic()
        );
        println!(
            "    • {}",
            "Compare token balances across all accounts".italic()
        );
        println!("    • {}", "Plan an optimal arbitrage strategy".italic());
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
        println!(
            "  {} - Clear all ingested documents",
            "clear or clear docs".bright_cyan()
        );
        println!("    Example: {}", "clear docs".italic());
        println!();
        println!("  {} - Show this help", "help".bright_cyan());
        println!("  {} - Exit the program", "exit".bright_cyan());
        println!();
    }
}
