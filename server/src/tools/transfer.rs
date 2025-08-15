use crate::provider::EthereumProvider;
use anyhow::Result;

/// Sends an ETH transfer transaction.
///
/// # Errors
///
/// Returns an error if:
/// - Address parsing fails
/// - Amount parsing fails
/// - Private key validation fails
/// - Transaction submission fails
/// - Provider communication fails
pub async fn send_eth_transfer(
    provider: &EthereumProvider,
    from_address: String,
    to_address: String,
    amount_eth: String,
    private_key: String,
) -> Result<String> {
    // Execute actual transaction on blockchain
    let tx_hash = provider
        .send_transaction(&from_address, &to_address, &amount_eth, &private_key)
        .await?;

    Ok(format!("Transaction sent! Hash: {tx_hash}"))
}
