use alloy::primitives::{Address, U256};
use alloy::sol;
use alloy::sol_types::SolCall;
use anyhow::Result;
use std::str::FromStr;

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    interface IERC20 {
        function approve(address spender, uint256 value) external returns (bool);
        function balanceOf(address account) external view returns (uint256);
        function transfer(address to, uint256 value) external returns (bool);
        function allowance(address owner, address spender) external view returns (uint256);
    }
}

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    interface IUniswapV2Router02 {
        function swapExactETHForTokens(
            uint amountOutMin,
            address[] calldata path,
            address to,
            uint deadline
        ) external payable returns (uint[] memory amounts);

        function swapExactTokensForETH(
            uint amountIn,
            uint amountOutMin,
            address[] calldata path,
            address to,
            uint deadline
        ) external returns (uint[] memory amounts);

        function swapExactTokensForTokens(
            uint amountIn,
            uint amountOutMin,
            address[] calldata path,
            address to,
            uint deadline
        ) external returns (uint[] memory amounts);

        function getAmountsOut(uint amountIn, address[] calldata path)
            external view returns (uint[] memory amounts);
    }
}

#[derive(Debug, Clone)]
pub struct ContractInteraction {
    pub router_address: Address,
    pub weth_address: Address,
}

impl ContractInteraction {
    pub fn new() -> Result<Self> {
        let router_address = Address::from_str("0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D")?;
        let weth_address = Address::from_str("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2")?;

        Ok(Self {
            router_address,
            weth_address,
        })
    }

    pub fn encode_swap_exact_eth_for_tokens(
        &self,
        amount_out_min: U256,
        path: Vec<Address>,
        to: Address,
        deadline: U256,
    ) -> Result<Vec<u8>> {
        let call = IUniswapV2Router02::swapExactETHForTokensCall {
            amountOutMin: amount_out_min,
            path,
            to,
            deadline,
        };
        Ok(call.abi_encode())
    }

    pub fn encode_erc20_approve(&self, spender: Address, amount: U256) -> Result<Vec<u8>> {
        let call = IERC20::approveCall {
            spender,
            value: amount,
        };
        Ok(call.abi_encode())
    }

    #[allow(dead_code)]
    pub fn encode_erc20_balance_of(&self, owner: Address) -> Result<Vec<u8>> {
        let call = IERC20::balanceOfCall { account: owner };
        Ok(call.abi_encode())
    }

    pub fn get_swap_path_eth_to_token(&self, token_address: Address) -> Vec<Address> {
        vec![self.weth_address, token_address]
    }

    pub fn get_swap_path_token_to_eth(&self, token_address: Address) -> Vec<Address> {
        vec![token_address, self.weth_address]
    }

    pub fn get_swap_path_token_to_token(
        &self,
        token_in: Address,
        token_out: Address,
    ) -> Vec<Address> {
        vec![token_in, self.weth_address, token_out]
    }

    pub fn calculate_deadline(&self) -> U256 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        U256::from(now + 1800)
    }

    pub fn calculate_min_amount_out(&self, amount_in: U256, slippage_bps: u16) -> U256 {
        let slippage_factor = U256::from(10000 - slippage_bps);
        (amount_in * slippage_factor) / U256::from(10000)
    }

    pub fn encode_swap_exact_tokens_for_eth(
        &self,
        amount_in: U256,
        amount_out_min: U256,
        path: Vec<Address>,
        to: Address,
        deadline: U256,
    ) -> Result<Vec<u8>> {
        let call = IUniswapV2Router02::swapExactTokensForETHCall {
            amountIn: amount_in,
            amountOutMin: amount_out_min,
            path,
            to,
            deadline,
        };
        Ok(call.abi_encode())
    }

    pub fn encode_swap_exact_tokens_for_tokens(
        &self,
        amount_in: U256,
        amount_out_min: U256,
        path: Vec<Address>,
        to: Address,
        deadline: U256,
    ) -> Result<Vec<u8>> {
        let call = IUniswapV2Router02::swapExactTokensForTokensCall {
            amountIn: amount_in,
            amountOutMin: amount_out_min,
            path,
            to,
            deadline,
        };
        Ok(call.abi_encode())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contract_interaction_creation() {
        let contracts = ContractInteraction::new();
        assert!(contracts.is_ok());
    }

    #[test]
    fn test_swap_path_creation() {
        let contracts = ContractInteraction::new().unwrap();
        let usdc_address = Address::from_str("0xA0b86a33E6441Fd1fd8e60dd20f4c50Ff1eeF0A8").unwrap();

        let path = contracts.get_swap_path_eth_to_token(usdc_address);
        assert_eq!(path.len(), 2);
        assert_eq!(path[0], contracts.weth_address);
        assert_eq!(path[1], usdc_address);
    }

    #[test]
    fn test_deadline_calculation() {
        let contracts = ContractInteraction::new().unwrap();
        let deadline = contracts.calculate_deadline();

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let expected_min = U256::from(now + 1700);
        let expected_max = U256::from(now + 1900);

        assert!(deadline >= expected_min && deadline <= expected_max);
    }

    #[test]
    fn test_slippage_calculation() {
        let contracts = ContractInteraction::new().unwrap();
        let amount_in = U256::from(1000);
        let slippage_bps = 200;

        let min_out = contracts.calculate_min_amount_out(amount_in, slippage_bps);

        assert_eq!(min_out, U256::from(980));
    }
}
