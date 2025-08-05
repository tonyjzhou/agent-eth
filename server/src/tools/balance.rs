use crate::provider::EthereumProvider;
use anyhow::Result;
use ethers::utils;

pub async fn get_balance(
    provider: &EthereumProvider,
    address: String,
    in_ether: Option<bool>,
) -> Result<String> {
    match provider.get_balance(&address).await {
        Ok(balance) => {
            let result = if in_ether.unwrap_or(true) {
                let ether_balance = utils::format_ether(balance);
                format!("{} ETH", ether_balance)
            } else {
                format!("{} wei", balance)
            };
            Ok(result)
        }
        Err(e) => {
            Err(anyhow::anyhow!("Failed to get balance for {}: {}", address, e))
        }
    }
}