mod api;
mod contracts;
mod mcp_server;
mod provider;
mod tools;

use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    info!("Starting MCP server...");

    let server = mcp_server::EthereumMcpServer::new()?;
    server.run().await?;

    Ok(())
}
