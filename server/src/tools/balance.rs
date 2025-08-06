use crate::provider::EthereumProvider;
use alloy::primitives::utils;
use anyhow::Result;

pub async fn get_balance(
    provider: &EthereumProvider,
    address: String,
    in_ether: Option<bool>,
) -> Result<String> {
    match provider.get_balance(&address).await {
        Ok(balance) => {
            let result = if in_ether.unwrap_or(true) {
                let ether_balance = utils::format_ether(balance);
                format!("{ether_balance} ETH")
            } else {
                format!("{balance} wei")
            };
            Ok(result)
        }
        Err(e) => Err(anyhow::anyhow!(
            "Failed to get balance for {}: {}",
            address,
            e
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balance_formatting() {
        // Test that the balance function properly handles formatting parameters
        let provider = EthereumProvider::new("http://127.0.0.1:8545").unwrap();

        // Test ENS rejection
        let result = tokio_test::block_on(get_balance(
            &provider,
            "vitalik.eth".to_string(),
            Some(true),
        ));
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("ENS name resolution"));
    }

    #[test]
    fn test_invalid_address_handling() {
        let provider = EthereumProvider::new("http://127.0.0.1:8545").unwrap();

        let result =
            tokio_test::block_on(get_balance(&provider, "invalid".to_string(), Some(true)));
        assert!(result.is_err());
    }
}
