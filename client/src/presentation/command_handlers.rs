use crate::application::use_cases::*;
use crate::domain::services::*;
use crate::presentation::formatters::*;
use anyhow::Result;
use colored::*;
use std::sync::Arc;

/// Clean command handler that only deals with presentation logic
pub struct BalanceCommandHandler<E: EthereumService, A: AccountService> {
    use_case: GetBalanceUseCase<E, A>,
}

impl<E: EthereumService, A: AccountService> BalanceCommandHandler<E, A> {
    pub fn new(ethereum_service: Arc<E>, account_service: Arc<A>) -> Self {
        Self {
            use_case: GetBalanceUseCase::new(ethereum_service, account_service),
        }
    }

    pub async fn handle(&self, address: &str, token: Option<&str>) -> Result<()> {
        println!("{}", "🔍 Checking balance...".bright_yellow());

        match self.use_case.execute(address, token).await {
            Ok(balance) => {
                println!("{}", BalanceFormatter::format(&balance));
            }
            Err(e) => {
                println!("{}", ErrorFormatter::format(&e));
            }
        }

        Ok(())
    }
}

pub struct TransferCommandHandler<E: EthereumService, A: AccountService> {
    use_case: ExecuteTransferUseCase<E, A>,
}

impl<E: EthereumService, A: AccountService> TransferCommandHandler<E, A> {
    pub fn new(ethereum_service: Arc<E>, account_service: Arc<A>) -> Self {
        Self {
            use_case: ExecuteTransferUseCase::new(ethereum_service, account_service),
        }
    }

    pub async fn handle(&self, from: &str, to: &str, amount: &str) -> Result<()> {
        println!("{}", "🚀 Executing transfer...".bright_yellow());

        match self.use_case.execute(from, to, amount).await {
            Ok(tx_hash) => {
                println!("{}", TransferFormatter::format(&tx_hash));
            }
            Err(e) => {
                println!("{}", ErrorFormatter::format(&e));
            }
        }

        Ok(())
    }
}

pub struct SwapCommandHandler<E: EthereumService, A: AccountService> {
    use_case: ExecuteSwapUseCase<E, A>,
}

impl<E: EthereumService, A: AccountService> SwapCommandHandler<E, A> {
    pub fn new(ethereum_service: Arc<E>, account_service: Arc<A>) -> Self {
        Self {
            use_case: ExecuteSwapUseCase::new(ethereum_service, account_service),
        }
    }

    pub async fn handle(
        &self,
        from: &str,
        token_in: &str,
        token_out: &str,
        amount: &str,
        slippage_bps: Option<u16>,
    ) -> Result<()> {
        println!("{}", "🔄 Executing swap...".bright_yellow());

        match self
            .use_case
            .execute(from, token_in, token_out, amount, slippage_bps)
            .await
        {
            Ok(tx_hash) => {
                println!("{}", TransferFormatter::format(&tx_hash));
            }
            Err(e) => {
                println!("{}", ErrorFormatter::format(&e));
            }
        }

        Ok(())
    }
}

pub struct ContractCheckCommandHandler<E: EthereumService> {
    use_case: CheckContractUseCase<E>,
}

impl<E: EthereumService> ContractCheckCommandHandler<E> {
    pub fn new(ethereum_service: Arc<E>) -> Self {
        Self {
            use_case: CheckContractUseCase::new(ethereum_service),
        }
    }

    pub async fn handle(&self, address: &str) -> Result<()> {
        println!("{}", "🔍 Checking contract...".bright_yellow());

        match self.use_case.execute(address).await {
            Ok(contract_info) => {
                println!("{}", ContractFormatter::format(&contract_info));
            }
            Err(e) => {
                println!("{}", ErrorFormatter::format(&e));
            }
        }

        Ok(())
    }
}
