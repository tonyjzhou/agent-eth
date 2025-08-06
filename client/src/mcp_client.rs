use anyhow::Result;
use reqwest::Client;
use serde_json::{json, Value};
use tracing::info;

pub struct McpClient {
    client: Client,
    server_url: String,
}

impl McpClient {
    pub fn new(server_url: &str) -> Self {
        info!("Connecting to MCP server at: {}", server_url);

        Self {
            client: Client::new(),
            server_url: server_url.to_string(),
        }
    }

    pub async fn get_balance(&self, address: &str, token: Option<&str>) -> Result<String> {
        let mut params = json!({
            "address": address
        });

        if let Some(token_symbol) = token {
            params["token"] = json!(token_symbol);
        }

        let response = self
            .client
            .post(format!("{}/balance", self.server_url))
            .json(&params)
            .send()
            .await?;

        let result: Value = response.json().await?;
        Ok(result["result"].as_str().unwrap_or("Unknown").to_string())
    }

    pub async fn send_transfer(
        &self,
        from_address: &str,
        to_address: &str,
        amount_eth: &str,
        private_key: &str,
    ) -> Result<String> {
        let params = json!({
            "from_address": from_address,
            "to_address": to_address,
            "amount_eth": amount_eth,
            "private_key": private_key
        });

        let response = self
            .client
            .post(format!("{}/transfer", self.server_url))
            .json(&params)
            .send()
            .await?;

        let result: Value = response.json().await?;
        Ok(result["result"].as_str().unwrap_or("Unknown").to_string())
    }

    pub async fn check_contract(&self, address: &str) -> Result<String> {
        let params = json!({
            "address": address
        });

        let response = self
            .client
            .post(format!("{}/contract_check", self.server_url))
            .json(&params)
            .send()
            .await?;

        let result: Value = response.json().await?;
        Ok(result["result"].as_str().unwrap_or("Unknown").to_string())
    }

    pub async fn execute_swap(
        &self,
        from_address: &str,
        token_in: &str,
        token_out: &str,
        amount_in: &str,
        slippage_bps: u16,
        private_key: &str,
    ) -> Result<String> {
        let params = json!({
            "from_address": from_address,
            "token_in": token_in,
            "token_out": token_out,
            "amount_in": amount_in,
            "slippage_bps": slippage_bps,
            "private_key": private_key
        });

        let response = self
            .client
            .post(format!("{}/swap", self.server_url))
            .json(&params)
            .send()
            .await?;

        let result: Value = response.json().await?;
        Ok(result["result"].as_str().unwrap_or("Unknown").to_string())
    }
}
