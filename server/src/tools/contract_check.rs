use crate::error::ServerError;
use crate::provider::EthereumProvider;
use anyhow::Result;

pub async fn check_contract_deployed(
    provider: &EthereumProvider,
    address: String,
) -> Result<String> {
    match provider.get_code(&address).await {
        Ok(code) => {
            let is_deployed = !code.is_empty() && code.len() > 2; // More than just "0x"
            let result = if is_deployed {
                format!("Contract is deployed at {address}")
            } else {
                format!("No contract found at {address}")
            };
            Ok(result)
        }
        Err(e) => {
            Err(ServerError::contract(format!("Failed to check contract at {address}: {e}")).into())
        }
    }
}
