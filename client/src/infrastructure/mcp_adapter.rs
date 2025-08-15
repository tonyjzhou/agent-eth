use crate::mcp_client::McpClient;
use anyhow::Result;

pub struct McpAdapter {
    client: McpClient,
}

impl McpAdapter {
    pub fn new(client: McpClient) -> Self {
        Self { client }
    }

    pub async fn get_balance(&mut self, address: &str, token: Option<&str>) -> Result<String> {
        self.client.get_balance(address, token).await
    }

    pub async fn send_transfer(
        &mut self,
        from_address: &str,
        to_address: &str,
        amount_eth: &str,
        private_key: &str,
    ) -> Result<String> {
        self.client
            .send_transfer(from_address, to_address, amount_eth, private_key)
            .await
    }

    pub async fn execute_swap(
        &mut self,
        from_address: &str,
        token_in: &str,
        token_out: &str,
        amount_in: &str,
        slippage_bps: u16,
        private_key: &str,
    ) -> Result<String> {
        self.client
            .execute_swap(
                from_address,
                token_in,
                token_out,
                amount_in,
                slippage_bps,
                private_key,
            )
            .await
    }

    pub async fn check_contract(&mut self, address: &str) -> Result<String> {
        self.client.check_contract(address).await
    }
}
