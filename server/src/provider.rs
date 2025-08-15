use alloy::network::EthereumWallet;
use alloy::primitives::{Address, U256};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::signers::local::PrivateKeySigner;
use anyhow::Result;
use std::str::FromStr;

// ENS support removed for simplicity - would require complex mainnet contract interactions

#[derive(Clone)]
pub struct EthereumProvider {
    rpc_url: String,
}

impl EthereumProvider {
    pub fn new(rpc_url: &str) -> Result<Self> {
        Ok(Self {
            rpc_url: rpc_url.to_string(),
        })
    }

    pub async fn get_balance(&self, address: &str) -> Result<U256> {
        let addr = if address.ends_with(".eth") {
            // For now, provide a helpful error message for ENS names
            // ENS resolution requires proper mainnet connection and is complex to implement
            return Err(anyhow::anyhow!(
                "ENS name resolution (*.eth) is not supported in this development version. Please use hex addresses instead."
            ));
        } else {
            Address::from_str(address)?
        };

        let provider = ProviderBuilder::new().connect_http(self.rpc_url.parse()?);
        let balance = provider.get_balance(addr).await?;
        Ok(balance)
    }

    pub async fn get_code(&self, address: &str) -> Result<Vec<u8>> {
        let addr = if address.ends_with(".eth") {
            // For now, provide a helpful error message for ENS names
            return Err(anyhow::anyhow!(
                "ENS name resolution (*.eth) is not supported in this development version. Please use hex addresses instead."
            ));
        } else {
            Address::from_str(address)?
        };

        let provider = ProviderBuilder::new().connect_http(self.rpc_url.parse()?);
        let code = provider.get_code_at(addr).await?;
        Ok(code.to_vec())
    }

    pub async fn send_transaction(
        &self,
        from_address: &str,
        to_address: &str,
        amount_eth: &str,
        private_key: &str,
    ) -> Result<String> {
        // Parse addresses
        let from = Address::from_str(from_address)?;
        let to = Address::from_str(to_address)?;

        // Parse amount (convert ETH to wei)
        let amount_f64: f64 = amount_eth.parse()?;
        let amount_wei = U256::from((amount_f64 * 1e18) as u128);

        // Create signer from private key (remove 0x prefix if present)
        let clean_private_key = private_key.strip_prefix("0x").unwrap_or(private_key);
        let signer: PrivateKeySigner = clean_private_key.parse()?;

        // Validate that the private key corresponds to the expected sender address
        let signer_address = signer.address();
        if signer_address != from {
            return Err(anyhow::anyhow!(
                "Private key mismatch: expected {}, but private key derives to {}",
                from,
                signer_address
            ));
        }

        // Create wallet and provider with signer
        let wallet = EthereumWallet::from(signer);
        let provider_with_wallet = ProviderBuilder::new()
            .wallet(wallet)
            .connect_http(self.rpc_url.parse()?);

        // Build and send transaction
        let tx_request = alloy::rpc::types::eth::TransactionRequest::default()
            .from(from)
            .to(to)
            .value(amount_wei);

        let pending_tx = provider_with_wallet.send_transaction(tx_request).await?;
        let tx_hash = *pending_tx.tx_hash();

        // Wait for transaction to be mined
        let _receipt = pending_tx.get_receipt().await?;

        Ok(format!("{tx_hash:#x}"))
    }

    pub async fn send_contract_transaction(
        &self,
        from_address: &str,
        to_address: &str,
        value_wei: &str,
        calldata: &str,
        private_key: &str,
    ) -> Result<String> {
        // Parse addresses
        let from = Address::from_str(from_address)?;
        let to = Address::from_str(to_address)?;

        // Parse value
        let value = if value_wei == "0" {
            U256::ZERO
        } else {
            U256::from_str(value_wei)?
        };

        // Parse calldata
        let data = hex::decode(calldata)?.into();

        // Create signer from private key (remove 0x prefix if present)
        let clean_private_key = private_key.strip_prefix("0x").unwrap_or(private_key);
        let signer: PrivateKeySigner = clean_private_key.parse()?;

        // Validate that the private key corresponds to the expected sender address
        let signer_address = signer.address();
        if signer_address != from {
            return Err(anyhow::anyhow!(
                "Private key mismatch: expected {}, but private key derives to {}",
                from,
                signer_address
            ));
        }

        // Create wallet and provider with signer
        let wallet = EthereumWallet::from(signer);
        let provider_with_wallet = ProviderBuilder::new()
            .wallet(wallet)
            .connect_http(self.rpc_url.parse()?);

        // Build and send transaction
        let tx_request = alloy::rpc::types::eth::TransactionRequest::default()
            .from(from)
            .to(to)
            .value(value)
            .input(data);

        let pending_tx = provider_with_wallet.send_transaction(tx_request).await?;
        let tx_hash = *pending_tx.tx_hash();

        // Wait for transaction to be mined
        let _receipt = pending_tx.get_receipt().await?;

        Ok(format!("{tx_hash:#x}"))
    }

    pub async fn call_contract(&self, contract_address: &str, calldata: &str) -> Result<String> {
        let contract_addr = Address::from_str(contract_address)?;
        let data = hex::decode(calldata)?.into();

        let provider = ProviderBuilder::new().connect_http(self.rpc_url.parse()?);

        let call_request = alloy::rpc::types::eth::TransactionRequest::default()
            .to(contract_addr)
            .input(data);

        let result = provider.call(call_request).await?;
        Ok(hex::encode(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_creation() {
        let provider = EthereumProvider::new("http://127.0.0.1:8545");
        assert!(provider.is_ok());
    }

    #[test]
    fn test_ens_name_detection() {
        let provider = EthereumProvider::new("http://127.0.0.1:8545")
            .expect("Failed to create provider for test");

        // Test that ENS names are properly detected and rejected with helpful error
        let result = tokio_test::block_on(provider.get_balance("vitalik.eth"));
        assert!(result.is_err());
        let error_msg = result
            .expect_err("Expected ENS error but got success")
            .to_string();
        assert!(error_msg.contains("ENS name resolution"));
    }

    #[test]
    fn test_invalid_address_format() {
        let provider = EthereumProvider::new("http://127.0.0.1:8545")
            .expect("Failed to create provider for test");

        // Test invalid hex address
        let result = tokio_test::block_on(provider.get_balance("invalid_address"));
        assert!(result.is_err());
    }

    #[test]
    fn test_valid_address_format() {
        // Test that valid hex addresses are parsed correctly
        let valid_address = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";
        let result = Address::from_str(valid_address);
        assert!(result.is_ok());
    }
}
