use agent_eth_client::mcp_client::McpClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    println!("Testing MCP client communication...");

    // Create MCP client pointing to server
    let mut mcp_client = McpClient::new("./target/debug/agent-eth-server", vec![]).await?;

    println!("Connected to MCP server, testing balance query...");

    // Test get_balance call
    let result = mcp_client
        .get_balance("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266", Some("ETH"))
        .await?;

    println!("Balance result: {result}");

    Ok(())
}
