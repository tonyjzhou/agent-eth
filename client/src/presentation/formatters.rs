use crate::domain::models::*;
use colored::*;

pub struct BalanceFormatter;

impl BalanceFormatter {
    pub fn format(balance: &Balance) -> String {
        format!(
            "{} {}: {} {}",
            "💰".bright_green(),
            "Balance".bright_green().bold(),
            balance.amount.value.bright_cyan(),
            balance.token.symbol.bright_white()
        )
    }
}

pub struct TransferFormatter;

impl TransferFormatter {
    pub fn format(tx_hash: &TransactionHash) -> String {
        format!(
            "{} {}: {}",
            "✅".bright_green(),
            "Transaction sent".bright_green().bold(),
            tx_hash.as_str().bright_cyan()
        )
    }
}

pub struct ContractFormatter;

impl ContractFormatter {
    pub fn format(contract_info: &ContractInfo) -> String {
        let status = if contract_info.is_contract {
            format!("{} Contract detected", "✅".bright_green())
        } else {
            format!("{} No contract found", "❌".bright_red())
        };

        format!(
            "{} at {}",
            status,
            contract_info.address.as_str().bright_cyan()
        )
    }
}

pub struct ErrorFormatter;

impl ErrorFormatter {
    pub fn format(error: &anyhow::Error) -> String {
        format!(
            "{} {}: {}",
            "❌".bright_red(),
            "Error".bright_red().bold(),
            error.to_string().bright_white()
        )
    }
}
