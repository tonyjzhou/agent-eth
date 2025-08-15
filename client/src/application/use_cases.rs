use crate::domain::{
    models::*,
    services::{AccountService, EthereumService, ValidationService},
};
use anyhow::Result;
use std::sync::Arc;

pub struct GetBalanceUseCase<E: EthereumService, A: AccountService> {
    ethereum_service: Arc<E>,
    #[allow(dead_code)]
    account_service: Arc<A>,
    validator: (),
}

impl<E: EthereumService, A: AccountService> GetBalanceUseCase<E, A> {
    pub fn new(ethereum_service: Arc<E>, account_service: Arc<A>) -> Self {
        Self {
            ethereum_service,
            account_service,
            validator: (),
        }
    }

    pub async fn execute(
        &self,
        address_input: &str,
        token_symbol: Option<&str>,
    ) -> Result<Balance> {
        // Validate and resolve address
        let address = self.validator.validate_address(address_input)?;

        // Create token if specified
        let token = if let Some(symbol) = token_symbol {
            if symbol.to_uppercase() == "ETH" {
                None
            } else {
                // In a real implementation, we'd resolve token addresses
                Some(Token::erc20(
                    symbol.to_uppercase(),
                    Address::new("0x0000000000000000000000000000000000000000".to_string())?,
                    18,
                ))
            }
        } else {
            None
        };

        // Get balance
        self.ethereum_service
            .get_balance(&address, token.as_ref())
            .await
    }
}

pub struct ExecuteTransferUseCase<E: EthereumService, A: AccountService> {
    ethereum_service: Arc<E>,
    account_service: Arc<A>,
    validator: (),
}

impl<E: EthereumService, A: AccountService> ExecuteTransferUseCase<E, A> {
    pub fn new(ethereum_service: Arc<E>, account_service: Arc<A>) -> Self {
        Self {
            ethereum_service,
            account_service,
            validator: (),
        }
    }

    pub async fn execute(
        &self,
        from_input: &str,
        to_input: &str,
        amount_str: &str,
    ) -> Result<TransactionHash> {
        // Validate and resolve addresses
        let from_address = self.validator.validate_address(from_input)?;
        let to_address = self.validator.validate_address(to_input)?;

        // Validate amount
        let token = Token::eth();
        let amount = self.validator.validate_amount(amount_str, &token)?;

        // Get private key
        let private_key = self.account_service.get_private_key(&from_address)?;

        // Create transfer
        let transfer = Transfer::new(from_address, to_address, amount, token);

        // Execute transfer
        self.ethereum_service
            .send_transfer(&transfer, &private_key)
            .await
    }
}

pub struct ExecuteSwapUseCase<E: EthereumService, A: AccountService> {
    ethereum_service: Arc<E>,
    account_service: Arc<A>,
    validator: (),
}

impl<E: EthereumService, A: AccountService> ExecuteSwapUseCase<E, A> {
    pub fn new(ethereum_service: Arc<E>, account_service: Arc<A>) -> Self {
        Self {
            ethereum_service,
            account_service,
            validator: (),
        }
    }

    pub async fn execute(
        &self,
        from_input: &str,
        token_in_symbol: &str,
        token_out_symbol: &str,
        amount_str: &str,
        slippage_bps: Option<u16>,
    ) -> Result<TransactionHash> {
        // Validate and resolve address
        let from_address = self.validator.validate_address(from_input)?;

        // Create tokens
        let token_in = if token_in_symbol.to_uppercase() == "ETH" {
            Token::eth()
        } else {
            Token::erc20(
                token_in_symbol.to_uppercase(),
                Address::new("0x0000000000000000000000000000000000000000".to_string())?,
                18,
            )
        };

        let token_out = if token_out_symbol.to_uppercase() == "ETH" {
            Token::eth()
        } else {
            Token::erc20(
                token_out_symbol.to_uppercase(),
                Address::new("0x0000000000000000000000000000000000000000".to_string())?,
                18,
            )
        };

        // Validate amount and slippage
        let amount_in = self.validator.validate_amount(amount_str, &token_in)?;
        let slippage = slippage_bps.unwrap_or(200);
        self.validator.validate_slippage(slippage)?;

        // Get private key
        let private_key = self.account_service.get_private_key(&from_address)?;

        // Create swap
        let swap = Swap::new(from_address, token_in, token_out, amount_in, slippage);

        // Execute swap
        self.ethereum_service
            .execute_swap(&swap, &private_key)
            .await
    }
}

pub struct CheckContractUseCase<E: EthereumService> {
    ethereum_service: Arc<E>,
    validator: (),
}

impl<E: EthereumService> CheckContractUseCase<E> {
    pub fn new(ethereum_service: Arc<E>) -> Self {
        Self {
            ethereum_service,
            validator: (),
        }
    }

    pub async fn execute(&self, address_input: &str) -> Result<ContractInfo> {
        // Validate address
        let address = self.validator.validate_address(address_input)?;

        // Check contract
        self.ethereum_service.check_contract(&address).await
    }
}
