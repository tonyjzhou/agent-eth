use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use tracing::{debug, info};

pub mod address_resolver;
use address_resolver::{AddressResolver, Network};

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractAddress {
    pub name: String,
    pub address: String,
    pub network: String,
}

pub struct ApiClient {
    client: Client,
    brave_api_key: Option<String>,
    address_resolver: AddressResolver,
}

impl ApiClient {
    pub fn new() -> Result<Self> {
        let client = Client::new();
        let brave_api_key = env::var("BRAVE_API_KEY").ok();
        let address_resolver = AddressResolver::new()?;

        if brave_api_key.is_none() {
            info!("BRAVE_API_KEY not set - search functionality will be limited");
        }

        Ok(Self {
            client,
            brave_api_key,
            address_resolver,
        })
    }

    pub async fn search_contract_address(&self, query: &str) -> Result<Vec<ContractAddress>> {
        if let Some(api_key) = &self.brave_api_key {
            self.brave_search(query, api_key).await
        } else {
            self.fallback_contract_search(query).await
        }
    }

    async fn brave_search(&self, query: &str, api_key: &str) -> Result<Vec<ContractAddress>> {
        debug!("Searching for contract with Brave API: {}", query);

        let response = self
            .client
            .get("https://api.search.brave.com/res/v1/web/search")
            .header("X-Subscription-Token", api_key)
            .query(&[("q", format!("{query} ethereum contract address"))])
            .send()
            .await?;

        let search_results: Value = response.json().await?;

        let mut addresses = Vec::new();

        if let Some(web_results) = search_results["web"]["results"].as_array() {
            for result in web_results {
                if let Some(snippet) = result["description"].as_str() {
                    if let Some(address) = self.extract_ethereum_address(snippet) {
                        addresses.push(ContractAddress {
                            name: query.to_string(),
                            address,
                            network: "ethereum".to_string(),
                        });
                    }
                }
            }
        }

        Ok(addresses)
    }

    async fn fallback_contract_search(&self, query: &str) -> Result<Vec<ContractAddress>> {
        debug!("Using fallback contract addresses for: {}", query);

        let addresses = match query.to_lowercase().as_str() {
            "uniswap v2 router" | "uniswap router" => {
                // Try to get from address resolver first
                if let Ok(address) = self
                    .address_resolver
                    .get_uniswap_v2_router(Network::Ethereum)
                    .await
                {
                    vec![ContractAddress {
                        name: "Uniswap V2 Router".to_string(),
                        address: address.to_string(),
                        network: "ethereum".to_string(),
                    }]
                } else {
                    // Fall back to hardcoded
                    vec![ContractAddress {
                        name: "Uniswap V2 Router".to_string(),
                        address: "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D".to_string(),
                        network: "ethereum".to_string(),
                    }]
                }
            }
            "weth" => {
                // Try to get from address resolver first
                if let Ok(address) = self
                    .address_resolver
                    .get_weth_address(Network::Ethereum)
                    .await
                {
                    vec![ContractAddress {
                        name: "Wrapped Ether".to_string(),
                        address: address.to_string(),
                        network: "ethereum".to_string(),
                    }]
                } else {
                    // Fall back to hardcoded
                    vec![ContractAddress {
                        name: "Wrapped Ether".to_string(),
                        address: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".to_string(),
                        network: "ethereum".to_string(),
                    }]
                }
            }
            "usdc" => vec![ContractAddress {
                name: "USD Coin".to_string(),
                address: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".to_string(),
                network: "ethereum".to_string(),
            }],
            _ => vec![],
        };

        Ok(addresses)
    }

    fn extract_ethereum_address(&self, text: &str) -> Option<String> {
        let ethereum_address_regex = regex::Regex::new(r"0x[a-fA-F0-9]{40}").ok()?;
        ethereum_address_regex
            .find(text)
            .map(|m| m.as_str().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_ethereum_address() {
        let client = ApiClient::new().unwrap();
        let text = "The Uniswap V2 Router contract is at 0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D on Ethereum";
        let address = client.extract_ethereum_address(text);
        assert_eq!(
            address,
            Some("0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D".to_string())
        );
    }

    #[test]
    fn test_fallback_contract_search() {
        let client = ApiClient::new().unwrap();
        let result = tokio_test::block_on(client.fallback_contract_search("uniswap v2 router"));
        assert!(result.is_ok());
        let addresses = result.unwrap();
        assert!(!addresses.is_empty());
        assert_eq!(
            addresses[0].address,
            "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D"
        );
    }
}
