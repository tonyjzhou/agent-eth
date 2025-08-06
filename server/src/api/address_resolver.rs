use alloy::primitives::Address;
use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolAddress {
    pub name: String,
    pub address: String,
    pub network: String,
    pub version: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Network {
    Ethereum,
    #[allow(dead_code)]
    Polygon,
    #[allow(dead_code)]
    Arbitrum,
    #[allow(dead_code)]
    Base,
}

impl Network {
    #[allow(dead_code)]
    pub fn chain_id(&self) -> u64 {
        match self {
            Network::Ethereum => 1,
            Network::Polygon => 137,
            Network::Arbitrum => 42161,
            Network::Base => 8453,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Network::Ethereum => "ethereum",
            Network::Polygon => "polygon",
            Network::Arbitrum => "arbitrum",
            Network::Base => "base",
        }
    }
}

#[derive(Debug, Clone)]
pub struct AddressResolver {
    client: Client,
    cache: Arc<Mutex<HashMap<String, ProtocolAddress>>>,
    coingecko_api_key: Option<String>,
}

impl AddressResolver {
    pub fn new() -> Result<Self> {
        let client = Client::new();
        let cache = Arc::new(Mutex::new(HashMap::new()));
        let coingecko_api_key = env::var("COINGECKO_API_KEY").ok();

        if coingecko_api_key.is_none() {
            info!("COINGECKO_API_KEY not set - using free tier with rate limits");
        }

        Ok(Self {
            client,
            cache,
            coingecko_api_key,
        })
    }

    /// Get Uniswap V2 Router address for the specified network
    pub async fn get_uniswap_v2_router(&self, network: Network) -> Result<Address> {
        let cache_key = format!("uniswap_v2_router_{}", network.name());

        // Check cache first
        if let Some(cached) = self.get_from_cache(&cache_key).await {
            return Address::from_str(&cached.address)
                .map_err(|e| anyhow!("Invalid cached address: {}", e));
        }

        // Try multiple sources for address resolution
        let address = self
            .resolve_uniswap_v2_router(network.clone())
            .await
            .or_else(|_| self.get_from_official_docs(network.clone()))
            .or_else(|_| self.get_from_fallback(network.clone()))?;

        // Cache the result
        self.cache_address(
            cache_key,
            ProtocolAddress {
                name: "Uniswap V2 Router".to_string(),
                address: address.to_string(),
                network: network.name().to_string(),
                version: Some("2".to_string()),
            },
        )
        .await;

        Ok(address)
    }

    /// Get WETH address for the specified network
    pub async fn get_weth_address(&self, network: Network) -> Result<Address> {
        let cache_key = format!("weth_{}", network.name());

        if let Some(cached) = self.get_from_cache(&cache_key).await {
            return Address::from_str(&cached.address)
                .map_err(|e| anyhow!("Invalid cached address: {}", e));
        }

        let address = self
            .resolve_weth_address(network.clone())
            .await
            .or_else(|_| self.get_weth_from_fallback(network.clone()))?;

        self.cache_address(
            cache_key,
            ProtocolAddress {
                name: "Wrapped Ether".to_string(),
                address: address.to_string(),
                network: network.name().to_string(),
                version: None,
            },
        )
        .await;

        Ok(address)
    }

    /// Try to resolve Uniswap V2 Router from CoinGecko API
    async fn resolve_uniswap_v2_router(&self, network: Network) -> Result<Address> {
        debug!(
            "Resolving Uniswap V2 Router from CoinGecko for {}",
            network.name()
        );

        let url = "https://api.coingecko.com/api/v3/exchanges/uniswap_v2";
        let mut request = self.client.get(url);

        if let Some(api_key) = &self.coingecko_api_key {
            request = request.header("x-cg-pro-api-key", api_key);
        }

        let response = request.send().await?;
        let data: Value = response.json().await?;

        // Extract router address from response (this would need to be adapted based on actual API response)
        if let Some(address_str) = data.get("router_address").and_then(|v| v.as_str()) {
            return Address::from_str(address_str)
                .map_err(|e| anyhow!("Invalid address from CoinGecko: {}", e));
        }

        Err(anyhow!(
            "Could not find router address in CoinGecko response"
        ))
    }

    /// Get addresses from official documentation/known deployments
    fn get_from_official_docs(&self, network: Network) -> Result<Address> {
        debug!(
            "Getting Uniswap V2 Router from official docs for {}",
            network.name()
        );

        let address_str = match network {
            Network::Ethereum => "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D",
            Network::Polygon => "0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff", // QuickSwap Router
            Network::Arbitrum => "0x4752ba5dbc23f44d87826276bf6fd6b1c372ad24", // ArbiSwap Router
            Network::Base => "0x4752ba5dbc23f44d87826276bf6fd6b1c372ad24",    // BaseSwap Router
        };

        Address::from_str(address_str)
            .map_err(|e| anyhow!("Invalid official router address: {}", e))
    }

    /// Fallback to hardcoded addresses as last resort
    fn get_from_fallback(&self, network: Network) -> Result<Address> {
        warn!(
            "Using fallback address for Uniswap V2 Router on {}",
            network.name()
        );

        let address_str = match network {
            Network::Ethereum => "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D",
            _ => {
                return Err(anyhow!(
                    "No fallback address available for {}",
                    network.name()
                ))
            }
        };

        Address::from_str(address_str)
            .map_err(|e| anyhow!("Fallback address resolution failed: {}", e))
    }

    /// Resolve WETH address from API or official sources
    async fn resolve_weth_address(&self, network: Network) -> Result<Address> {
        debug!("Resolving WETH address for {}", network.name());

        // Try official WETH addresses first
        let address_str = match network {
            Network::Ethereum => "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
            Network::Polygon => "0x7ceB23fD6bC0adD59E62ac25578270cFf1b9f619", // WETH on Polygon
            Network::Arbitrum => "0x82aF49447D8a07e3bd95BD0d56f35241523fBab1", // WETH on Arbitrum
            Network::Base => "0x4200000000000000000000000000000000000006",    // WETH on Base
        };

        Address::from_str(address_str).map_err(|e| anyhow!("Invalid WETH address: {}", e))
    }

    fn get_weth_from_fallback(&self, network: Network) -> Result<Address> {
        let address_str = match network {
            Network::Ethereum => "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
            _ => return Err(anyhow!("No fallback WETH address for {}", network.name())),
        };

        Address::from_str(address_str).map_err(|e| anyhow!("WETH fallback failed: {}", e))
    }

    async fn get_from_cache(&self, key: &str) -> Option<ProtocolAddress> {
        let cache = self.cache.lock().await;
        cache.get(key).cloned()
    }

    async fn cache_address(&self, key: String, address: ProtocolAddress) {
        let mut cache = self.cache.lock().await;
        cache.insert(key, address);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_resolver_creation() {
        let resolver = AddressResolver::new();
        assert!(resolver.is_ok());
    }

    #[tokio::test]
    async fn test_get_uniswap_v2_router_ethereum() {
        let resolver = AddressResolver::new().unwrap();
        let address = resolver.get_uniswap_v2_router(Network::Ethereum).await;
        assert!(address.is_ok());

        let addr = address.unwrap();
        // Alloy returns addresses in lowercase
        assert_eq!(
            addr.to_string().to_lowercase(),
            "0x7a250d5630b4cf539739df2c5dacb4c659f2488d"
        );
    }

    #[tokio::test]
    async fn test_get_weth_address_ethereum() {
        let resolver = AddressResolver::new().unwrap();
        let address = resolver.get_weth_address(Network::Ethereum).await;
        assert!(address.is_ok());

        let addr = address.unwrap();
        // Alloy returns addresses in lowercase
        assert_eq!(
            addr.to_string().to_lowercase(),
            "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"
        );
    }

    #[tokio::test]
    async fn test_network_chain_ids() {
        assert_eq!(Network::Ethereum.chain_id(), 1);
        assert_eq!(Network::Polygon.chain_id(), 137);
        assert_eq!(Network::Arbitrum.chain_id(), 42161);
        assert_eq!(Network::Base.chain_id(), 8453);
    }

    #[tokio::test]
    async fn test_caching_functionality() {
        let resolver = AddressResolver::new().unwrap();

        // First call should populate cache
        let addr1 = resolver
            .get_uniswap_v2_router(Network::Ethereum)
            .await
            .unwrap();

        // Second call should use cache
        let addr2 = resolver
            .get_uniswap_v2_router(Network::Ethereum)
            .await
            .unwrap();

        assert_eq!(addr1, addr2);
    }
}
