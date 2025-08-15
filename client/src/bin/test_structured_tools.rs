use agent_eth_client::tools::{balance::BalanceParams, TypedToolRegistry};
use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🧪 Testing Structured Tool System");

    // Create the typed tool registry
    let registry = TypedToolRegistry::new();

    // List all available tools
    println!("\n📋 Available Tools:");
    for tool_name in registry.tool_names() {
        println!("  • {tool_name}");
    }

    // Show tool descriptions for AI agent
    println!("\n🤖 AI Agent Tool Descriptions:");
    println!("{}", registry.get_tools_description());

    // Test balance tool with structured parameters
    println!("\n⚖️ Testing Balance Tool:");
    let balance_params = json!({
        "address": "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
        "token": "ETH"
    });

    if let Ok(result) = registry.execute_tool("get_balance", balance_params).await {
        println!("Result: {}", serde_json::to_string_pretty(&result)?);
    }

    // Test with typed parameters (compile-time safety)
    println!("\n🔒 Testing Typed Parameters:");
    let typed_params = BalanceParams {
        address: "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".to_string(),
        token: Some("USDC".to_string()),
    };

    // This demonstrates compile-time type safety
    let _serialized = serde_json::to_value(typed_params)?;
    println!("✅ Typed parameters serialize correctly");

    // Test error handling
    println!("\n❌ Testing Error Handling:");
    if let Err(e) = registry.execute_tool("nonexistent_tool", json!({})).await {
        println!("Expected error: {e}");
    }

    println!("\n✅ Structured Tool System Test Complete!");

    Ok(())
}
