use alloy::network::EthereumWallet;
use alloy::primitives::{Address, U256};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::signers::local::PrivateKeySigner;
use anyhow::Result;
use std::str::FromStr;

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
            // For ENS names, manually resolve to address first
            // TODO: Implement ENS resolution or return error for now
            return Err(anyhow::anyhow!(
                "ENS resolution not yet implemented in Alloy migration"
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
            // TODO: Implement ENS resolution or return error for now
            return Err(anyhow::anyhow!(
                "ENS resolution not yet implemented in Alloy migration"
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

        Ok(format!("{tx_hash:#x}"))
    }
}
