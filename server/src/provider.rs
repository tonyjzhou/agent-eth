use ethers::providers::{Http, Middleware, Provider};
use ethers::types::{Address, U256};
use anyhow::Result;
use std::str::FromStr;

pub struct EthereumProvider {
    provider: Provider<Http>,
}

impl EthereumProvider {
    pub fn new(rpc_url: &str) -> Result<Self> {
        let provider = Provider::<Http>::try_from(rpc_url)?;
        Ok(Self { provider })
    }

    pub async fn get_balance(&self, address: &str) -> Result<U256> {
        let addr = if address.ends_with(".eth") {
            // For ENS names, resolve to address first
            self.provider.resolve_name(address).await?
        } else {
            Address::from_str(address)?
        };
        
        let balance = self.provider.get_balance(addr, None).await?;
        Ok(balance)
    }

    pub async fn get_code(&self, address: &str) -> Result<Vec<u8>> {
        let addr = if address.ends_with(".eth") {
            self.provider.resolve_name(address).await?
        } else {
            Address::from_str(address)?
        };
        
        let code = self.provider.get_code(addr, None).await?;
        Ok(code.to_vec())
    }

    pub fn provider(&self) -> &Provider<Http> {
        &self.provider
    }
}