use agent_eth_server::provider::EthereumProvider;
use agent_eth_server::tools::get_token_balance;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let provider = EthereumProvider::new("http://127.0.0.1:8545")?;
    let address = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".to_string(); // Alice
    let token = "ETH".to_string();

    println!("Testing balance query...");
    println!("Address: {address}");
    println!("Token: {token}");

    match get_token_balance(&provider, address, token).await {
        Ok(result) => {
            println!("SUCCESS: {result}");
        }
        Err(e) => {
            println!("ERROR: {e}");
        }
    }

    Ok(())
}
