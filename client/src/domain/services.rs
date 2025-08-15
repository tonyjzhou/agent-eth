use super::models::*;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait EthereumService {
    async fn get_balance(&self, address: &Address, token: Option<&Token>) -> Result<Balance>;
    async fn send_transfer(
        &self,
        transfer: &Transfer,
        private_key: &str,
    ) -> Result<TransactionHash>;
    async fn execute_swap(&self, swap: &Swap, private_key: &str) -> Result<TransactionHash>;
    async fn check_contract(&self, address: &Address) -> Result<ContractInfo>;
}

#[async_trait]
pub trait AccountService {
    fn get_private_key(&self, address: &Address) -> Result<String>;
    fn resolve_address(&self, input: &str) -> Result<Address>;
    fn get_all_accounts(&self) -> Vec<Account>;
    fn is_known_account(&self, address: &Address) -> bool;
}

#[async_trait]
pub trait PricingService {
    async fn get_token_price(&self, token: &Token) -> Result<f64>;
    async fn estimate_swap_output(
        &self,
        token_in: &Token,
        token_out: &Token,
        amount_in: &Amount,
    ) -> Result<Amount>;
}

pub trait ValidationService {
    fn validate_address(&self, address: &str) -> Result<Address>;
    fn validate_amount(&self, amount: &str, token: &Token) -> Result<Amount>;
    fn validate_slippage(&self, slippage_bps: u16) -> Result<()>;
}

impl ValidationService for () {
    fn validate_address(&self, address: &str) -> Result<Address> {
        Address::from_alias(address)
            .or_else(|| Address::new(address.to_string()).ok())
            .ok_or_else(|| anyhow::anyhow!("Invalid address: {}", address))
    }

    fn validate_amount(&self, amount: &str, token: &Token) -> Result<Amount> {
        amount
            .parse::<f64>()
            .map_err(|_| anyhow::anyhow!("Invalid amount format: {}", amount))?;

        Ok(Amount::new(amount.to_string(), token.decimals))
    }

    fn validate_slippage(&self, slippage_bps: u16) -> Result<()> {
        if slippage_bps > 5000 {
            return Err(anyhow::anyhow!(
                "Slippage too high: {}bps (max 5000bps)",
                slippage_bps
            ));
        }
        Ok(())
    }
}
