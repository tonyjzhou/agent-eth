use agent_eth_client::{
    application::agent_service::ToolAgent,
    infrastructure::{
        ai::IntelligentAgent, DefaultAccountService, DefaultEthereumService, McpAdapter,
    },
    mcp_client::McpClient,
    presentation::BalanceCommandHandler,
};
use anyhow::Result;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🏗️  Agent-ETH New Architecture Demo");
    println!("====================================\n");

    // 1. Initialize Infrastructure Layer
    println!("1. 🔧 Setting up infrastructure services...");

    let account_service = Arc::new(DefaultAccountService::new());
    println!("   ✅ Account service initialized");

    // Note: In real implementation, we'd properly initialize MCP client
    // For demo purposes, we'll show the structure
    println!("   ⚠️  MCP client would be initialized here");

    // Simulate MCP client for demo
    let server_command = "cargo run --bin agent-eth-server";
    let mcp_client = McpClient::new(server_command, vec![]).await?;
    let mcp_adapter = McpAdapter::new(mcp_client);
    let ethereum_service = Arc::new(DefaultEthereumService::new(mcp_adapter));
    println!("   ✅ Ethereum service initialized");

    // 2. Initialize Application Layer
    println!("\n2. 🧠 Setting up application layer...");

    let api_key = std::env::var("ANTHROPIC_API_KEY").unwrap_or_else(|_| "demo-key".to_string());

    let mut intelligent_agent =
        IntelligentAgent::new(api_key, ethereum_service.clone(), account_service.clone());
    println!("   ✅ Intelligent agent initialized");

    // 3. Initialize Presentation Layer
    println!("\n3. 🎨 Setting up presentation layer...");

    let balance_handler =
        BalanceCommandHandler::new(ethereum_service.clone(), account_service.clone());
    println!("   ✅ Command handlers initialized");

    // 4. Demonstrate Clean Architecture Usage
    println!("\n4. 🚀 Demonstrating layered architecture...");

    // Example 1: Direct use case execution through presentation layer
    println!("\n   📊 Example 1: Balance check through presentation layer");
    if let Err(e) = balance_handler.handle("alice", None).await {
        println!("   ⚠️  Demo error (expected): {e}");
    }

    // Example 2: AI agent planning
    println!("\n   🤖 Example 2: AI agent planning");
    match intelligent_agent
        .plan_execution("Check Alice's balance")
        .await
    {
        Ok(plan) => {
            println!("   ✅ Plan created:");
            println!("      💭 Reasoning: {}", plan.reasoning);
            println!("      📋 Steps: {} planned", plan.steps.len());
            println!(
                "      ⚠️  Requires confirmation: {}",
                plan.requires_confirmation
            );
        }
        Err(e) => {
            println!("   ⚠️  Planning error (expected in demo): {e}");
        }
    }

    // 5. Show Architecture Benefits
    println!("\n5. ✨ Architecture Benefits Demonstrated:");
    println!("   🔄 Separation of Concerns:");
    println!("      • Domain: Pure business logic (Balance, Transfer, Account models)");
    println!("      • Application: Use cases and agent orchestration");
    println!("      • Infrastructure: External services (MCP, Claude API, Accounts)");
    println!("      • Presentation: CLI formatting and user interaction");

    println!("\n   🧪 Testability:");
    println!("      • Each layer can be tested in isolation");
    println!("      • Services can be mocked/stubbed easily");
    println!("      • Domain logic is pure and deterministic");

    println!("\n   🔧 Maintainability:");
    println!("      • Clear responsibilities and minimal coupling");
    println!("      • Easy to modify or replace implementations");
    println!("      • No more business logic mixed with CLI or AI code");

    println!("\n🎉 Architecture demonstration complete!");
    println!("💡 This structure makes the codebase more professional and maintainable.");

    Ok(())
}
