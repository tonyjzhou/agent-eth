use crate::domain::{
    models::{Account, Address},
    services::AccountService,
};
use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;

#[derive(Debug)]
pub struct DefaultAccountService {
    accounts: HashMap<String, String>, // address -> private_key
    aliases: HashMap<String, String>,  // alias -> address
}

impl Default for DefaultAccountService {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultAccountService {
    pub fn new() -> Self {
        let mut accounts = HashMap::new();
        let mut aliases = HashMap::new();

        // Hardcoded test accounts (for Anvil testing only)
        let test_accounts = vec![
            (
                "alice",
                "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
                "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
            ),
            (
                "bob",
                "0x70997970C51812dc3A010C7d01b50e0d17dc79C8",
                "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d",
            ),
            (
                "carol",
                "0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC",
                "0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a",
            ),
        ];

        for (alias, address, private_key) in test_accounts {
            accounts.insert(address.to_string(), private_key.to_string());
            aliases.insert(alias.to_string(), address.to_string());
        }

        Self { accounts, aliases }
    }
}

#[async_trait]
impl AccountService for DefaultAccountService {
    fn get_private_key(&self, address: &Address) -> Result<String> {
        self.accounts
            .get(address.as_str())
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("No private key available for address: {}", address))
    }

    fn resolve_address(&self, input: &str) -> Result<Address> {
        // First try alias resolution
        if let Some(address_str) = self.aliases.get(&input.to_lowercase()) {
            return Address::new(address_str.clone());
        }

        // Then try direct address parsing
        Address::new(input.to_string())
    }

    fn get_all_accounts(&self) -> Vec<Account> {
        self.aliases
            .iter()
            .filter_map(|(alias, address_str)| {
                Address::new(address_str.clone())
                    .ok()
                    .map(|address| Account::new(address, Some(alias.clone())))
            })
            .collect()
    }

    fn is_known_account(&self, address: &Address) -> bool {
        self.accounts.contains_key(address.as_str())
    }
}
