mod agent;
mod cli;
mod commands;
mod error;
mod mcp_client;
mod rag;
mod tools;

use agent::AgentCore;
use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use colored::*;
use commands::*;
use mcp_client::McpClient;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = Cli::parse();

    let anthropic_api_key =
        env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY environment variable must be set");

    // Initialize components
    let mut agent_core = AgentCore::new(&anthropic_api_key)?;
    let mut mcp_client = McpClient::new(&args.server_command, vec![]).await?;

    // Initialize RAG system
    let rag_db_path = "client_rag.db";
    if let Err(e) = agent_core.initialize_rag(rag_db_path).await {
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

    // Execute the command based on clap subcommands
    match args.command {
        Commands::Balance { address, token } => {
            println!(
                "{}",
                "🚀 Ethereum AI Agent - Balance Query".bright_blue().bold()
            );
            let resolved_address = BalanceCommand::resolve_address(&address);
            BalanceCommand::execute(&resolved_address, token.as_deref(), &mut mcp_client).await?
        }
        Commands::Transfer { amount, from, to } => {
            println!("{}", "🚀 Ethereum AI Agent - Transfer".bright_blue().bold());
            TransferCommand::execute(&amount, &from, &to, &mut mcp_client, &agent_core).await?
        }
        Commands::ContractCheck { address } => {
            println!(
                "{}",
                "🚀 Ethereum AI Agent - Contract Check".bright_blue().bold()
            );
            ContractCheckCommand::execute(&address, &mut mcp_client).await?
        }
        Commands::Swap {
            amount,
            token_in,
            token_out,
            from,
            slippage,
        } => {
            println!(
                "{}",
                "🚀 Ethereum AI Agent - Token Swap".bright_blue().bold()
            );
            SwapCommand::execute(
                &amount,
                &token_in,
                &token_out,
                &from,
                slippage,
                &mut mcp_client,
                &agent_core,
            )
            .await?
        }
        Commands::Docs { query } => {
            println!(
                "{}",
                "🚀 Ethereum AI Agent - Documentation Search"
                    .bright_blue()
                    .bold()
            );
            DocsCommand::search(&query, &agent_core).await?
        }
        Commands::Ingest { directory } => {
            println!(
                "{}",
                "🚀 Ethereum AI Agent - Document Ingestion"
                    .bright_blue()
                    .bold()
            );
            DocsCommand::ingest(&directory, &mut agent_core).await?
        }
        Commands::Clear => {
            println!(
                "{}",
                "🚀 Ethereum AI Agent - Clear Documents"
                    .bright_blue()
                    .bold()
            );
            DocsCommand::clear(&mut agent_core).await?
        }
        Commands::Interactive => {
            println!(
                "{}",
                "🚀 Ethereum AI Agent - Interactive Mode"
                    .bright_blue()
                    .bold()
            );
            InteractiveCommand::start(&mut agent_core, &mut mcp_client).await?
        }
        Commands::Agent { command } => {
            println!(
                "{}",
                "🚀 Ethereum AI Agent - Intelligent Planning"
                    .bright_blue()
                    .bold()
            );
            AgentCommand::execute(&command, &mut agent_core, &mut mcp_client).await?
        }
    }

    Ok(())
}
