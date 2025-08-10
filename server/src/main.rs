mod api;
mod contracts;
mod mcp_server;
mod provider;
mod tools;

use rmcp::{transport::stdio, ServiceExt};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .init();

    info!("Starting RMCP server...");

    let tool_router = mcp_server::EthereumToolRouter::new()?;

    // Serve using RMCP SDK with stdio transport
    let server = tool_router.serve(stdio()).await?;
    server.waiting().await?;

    Ok(())
}
