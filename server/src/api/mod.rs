use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use tracing::{debug, info};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPrice {
    pub symbol: String,
    pub price_usd: f64,
    pub last_updated: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractAddress {
    pub name: String,
    pub address: String,
    pub network: String,
}

pub struct ApiClient {
    client: Client,
    brave_api_key: Option<String>,
}

impl ApiClient {
    pub fn new() -> Result<Self> {
        let client = Client::new();
        let brave_api_key = env::var("BRAVE_API_KEY").ok();

        if brave_api_key.is_none() {
            info!("BRAVE_API_KEY not set - search functionality will be limited");
        }

        Ok(Self {
            client,
            brave_api_key,
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
            "uniswap v2 router" | "uniswap router" => vec![ContractAddress {
                name: "Uniswap V2 Router".to_string(),
                address: "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D".to_string(),
                network: "ethereum".to_string(),
            }],
            "usdc" => vec![ContractAddress {
                name: "USD Coin".to_string(),
                address: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".to_string(),
                network: "ethereum".to_string(),
            }],
            "weth" => vec![ContractAddress {
                name: "Wrapped Ether".to_string(),
                address: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".to_string(),
                network: "ethereum".to_string(),
            }],
            _ => vec![],
        };

        Ok(addresses)
    }

    pub async fn get_token_price(&self, token_symbol: &str) -> Result<TokenPrice> {
        debug!("Getting price for token: {}", token_symbol);

        // Try external API first, then fallback to hardcoded prices
        if let Ok(price) = self.try_external_price(token_symbol).await {
            return Ok(price);
        }

        // Fallback to hardcoded prices for testing
        self.get_fallback_price(token_symbol)
    }

    async fn try_external_price(&self, token_symbol: &str) -> Result<TokenPrice> {
        let symbol_map = match token_symbol.to_lowercase().as_str() {
            "usdc" => "usd-coin",
            "weth" => "weth",
            "ethereum" | "eth" => "ethereum",
            _ => token_symbol,
        };

        let url = format!(
            "https://api.coingecko.com/api/v3/simple/price?ids={symbol_map}&vs_currencies=usd"
        );

        let response = self.client.get(&url).send().await?;

        let price_data: Value = response.json().await?;

        if let Some(token_data) = price_data.get(symbol_map) {
            if let Some(price) = token_data.get("usd").and_then(|p| p.as_f64()) {
                return Ok(TokenPrice {
                    symbol: token_symbol.to_uppercase(),
                    price_usd: price,
                    last_updated: Some(
                        std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs()
                            .to_string(),
                    ),
                });
            }
        }

        Err(anyhow::anyhow!(
            "External price API failed for {}",
            token_symbol
        ))
    }

    fn get_fallback_price(&self, token_symbol: &str) -> Result<TokenPrice> {
        debug!("Using fallback prices for: {}", token_symbol);

        let (price, symbol) = match token_symbol.to_lowercase().as_str() {
            "usdc" | "usd-coin" => (1.0, "USDC"),
            "weth" | "ethereum" | "eth" => (3200.0, "ETH"), // Approximate ETH price
            "wbtc" | "bitcoin" => (95000.0, "WBTC"),
            "dai" => (1.0, "DAI"),
            _ => {
                return Err(anyhow::anyhow!(
                    "No fallback price available for {}",
                    token_symbol
                ))
            }
        };

        Ok(TokenPrice {
            symbol: symbol.to_string(),
            price_usd: price,
            last_updated: Some("fallback".to_string()),
        })
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
