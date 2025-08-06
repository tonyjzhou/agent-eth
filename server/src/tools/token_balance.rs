use crate::api::address_resolver::Network;
use crate::api::ApiClient;
use crate::contracts::ContractInteraction;
use crate::provider::EthereumProvider;
use alloy::primitives::{Address, U256};
use anyhow::Result;
use std::str::FromStr;
use tracing::debug;

pub async fn get_token_balance(
    provider: &EthereumProvider,
    user_address: String,
    token_symbol: String,
) -> Result<String> {
    debug!(
        "Getting {} balance for address: {}",
        token_symbol, user_address
    );

    // Handle ETH specially
    if token_symbol.to_uppercase() == "ETH" {
        let balance = provider.get_balance(&user_address).await?;
        let balance_f64 = balance.to_string().parse::<f64>().unwrap_or(0.0) / 1e18;
        return Ok(format!("{balance_f64:.6} ETH"));
    }

    // For ERC20 tokens, we need to get the token contract address
    let api_client = ApiClient::new()?;
    let contracts = ContractInteraction::new_with_resolver(Network::Ethereum).await?;

    let token_addresses = api_client.search_contract_address(&token_symbol).await?;

    let token_address = token_addresses
        .first()
        .ok_or_else(|| anyhow::anyhow!("Token contract not found for {}", token_symbol))?;

    let _token_addr = Address::from_str(&token_address.address)?;
    let user_addr = Address::from_str(&user_address)?;

    // Create the balanceOf call
    let calldata = contracts.encode_erc20_balance_of(user_addr)?;

    // Make a static call to get the balance
    let balance_result = provider
        .call_contract(&token_address.address, &hex::encode(calldata))
        .await?;

    // Decode the balance
    let balance_bytes = hex::decode(balance_result)?;
    let balance = U256::from_be_slice(&balance_bytes);

    // Use correct decimals for each token
    let (decimals, precision) = match token_symbol.to_lowercase().as_str() {
        "usdc" | "usdt" => (1e6, 2), // 6 decimals, show 2 decimal places
        _ => (1e18, 6),              // Most tokens have 18 decimals, show 6 decimal places
    };

    let balance_f64 = balance.to_string().parse::<f64>().unwrap_or(0.0) / decimals;

    Ok(format!(
        "{:.prec$} {}",
        balance_f64,
        token_symbol.to_uppercase(),
        prec = precision
    ))
}
