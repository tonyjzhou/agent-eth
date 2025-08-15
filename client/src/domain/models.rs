use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Address(String);

impl Address {
    pub fn new(address: String) -> Result<Self> {
        if address.starts_with("0x") && address.len() == 42 {
            Ok(Self(address.to_lowercase()))
        } else {
            Err(anyhow::anyhow!(
                "Invalid Ethereum address format: {}",
                address
            ))
        }
    }

    pub fn from_alias(alias: &str) -> Option<Self> {
        let address = match alias.to_lowercase().as_str() {
            "alice" => "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
            "bob" => "0x70997970C51812dc3A010C7d01b50e0d17dc79C8",
            "carol" => "0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC",
            _ => return None,
        };
        Some(Self(address.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Amount {
    pub value: String,
    pub decimals: u8,
}

impl Amount {
    pub fn new(value: String, decimals: u8) -> Self {
        Self { value, decimals }
    }

    pub fn eth(value: String) -> Self {
        Self::new(value, 18)
    }

    pub fn token(value: String, decimals: u8) -> Self {
        Self::new(value, decimals)
    }
}

impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub symbol: String,
    pub address: Option<Address>,
    pub decimals: u8,
}

impl Token {
    pub fn new(symbol: String, address: Option<Address>, decimals: u8) -> Self {
        Self {
            symbol,
            address,
            decimals,
        }
    }

    pub fn eth() -> Self {
        Self {
            symbol: "ETH".to_string(),
            address: None,
            decimals: 18,
        }
    }

    pub fn erc20(symbol: String, address: Address, decimals: u8) -> Self {
        Self {
            symbol,
            address: Some(address),
            decimals,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol)
    }
}

#[derive(Debug, Clone)]
pub struct Balance {
    pub address: Address,
    pub token: Token,
    pub amount: Amount,
}

impl Balance {
    pub fn new(address: Address, token: Token, amount: Amount) -> Self {
        Self {
            address,
            token,
            amount,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Transfer {
    pub from: Address,
    pub to: Address,
    pub amount: Amount,
    pub token: Token,
}

impl Transfer {
    pub fn new(from: Address, to: Address, amount: Amount, token: Token) -> Self {
        Self {
            from,
            to,
            amount,
            token,
        }
    }

    pub fn eth_transfer(from: Address, to: Address, amount_eth: String) -> Self {
        Self {
            from,
            to,
            amount: Amount::eth(amount_eth),
            token: Token::eth(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Swap {
    pub from_address: Address,
    pub token_in: Token,
    pub token_out: Token,
    pub amount_in: Amount,
    pub slippage_bps: u16,
}

impl Swap {
    pub fn new(
        from_address: Address,
        token_in: Token,
        token_out: Token,
        amount_in: Amount,
        slippage_bps: u16,
    ) -> Self {
        Self {
            from_address,
            token_in,
            token_out,
            amount_in,
            slippage_bps,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Account {
    pub address: Address,
    pub alias: Option<String>,
}

impl Account {
    pub fn new(address: Address, alias: Option<String>) -> Self {
        Self { address, alias }
    }

    pub fn from_alias(alias: &str) -> Option<Self> {
        Address::from_alias(alias).map(|addr| Self {
            address: addr,
            alias: Some(alias.to_string()),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionHash(String);

impl TransactionHash {
    pub fn new(hash: String) -> Self {
        Self(hash)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TransactionHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct ContractInfo {
    pub address: Address,
    pub is_contract: bool,
    pub bytecode_size: Option<usize>,
}

impl ContractInfo {
    pub fn new(address: Address, is_contract: bool, bytecode_size: Option<usize>) -> Self {
        Self {
            address,
            is_contract,
            bytecode_size,
        }
    }
}
