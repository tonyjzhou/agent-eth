use crate::api::address_resolver::Network;
use crate::api::ApiClient;
use crate::contracts::ContractInteraction;
use crate::provider::EthereumProvider;
use alloy::primitives::{Address, U256};
use anyhow::Result;
use std::str::FromStr;
use tracing::{debug, info};

#[derive(Debug)]
pub struct SwapParams {
    pub from_address: String,
    pub token_in: String,
    pub token_out: String,
    pub amount_in: String,
    pub slippage_bps: u16,
    pub private_key: String,
}

/// Executes a token swap using Uniswap V2 Router.
///
/// # Errors
///
/// Returns an error if:
/// - Contract interaction setup fails
/// - Token amount parsing fails
/// - Address parsing fails
/// - Token approval fails
/// - Swap transaction fails
/// - RPC provider requests fail
pub async fn execute_swap(provider: &EthereumProvider, params: SwapParams) -> Result<String> {
    info!(
        "Executing swap: {} {} -> {}",
        params.amount_in, params.token_in, params.token_out
    );

    let api_client = ApiClient::new()?;
    let contracts = ContractInteraction::new_with_resolver(Network::Ethereum).await?;

    let _from_address = Address::from_str(&params.from_address)?;

    if params.token_in.to_lowercase() == "eth" {
        execute_eth_to_token_swap(provider, &api_client, &contracts, params).await
    } else if params.token_out.to_lowercase() == "eth" {
        execute_token_to_eth_swap(provider, &api_client, &contracts, params).await
    } else {
        execute_token_to_token_swap(provider, &api_client, &contracts, params).await
    }
}

async fn execute_eth_to_token_swap(
    provider: &EthereumProvider,
    api_client: &ApiClient,
    contracts: &ContractInteraction,
    params: SwapParams,
) -> Result<String> {
    debug!("Executing ETH to token swap");

    let token_out_addresses = api_client
        .search_contract_address(&params.token_out)
        .await?;

    let token_out_address = token_out_addresses
        .first()
        .ok_or_else(|| anyhow::anyhow!("Token {} not found", params.token_out))?;

    let token_out_addr = Address::from_str(&token_out_address.address)?;

    let amount_in_f64: f64 = params.amount_in.parse()?;
    let amount_in_wei = U256::from((amount_in_f64 * 1e18) as u128);

    // Use Uniswap's getAmountsOut for accurate pricing instead of external API
    let path = contracts.get_swap_path_eth_to_token(token_out_addr);
    let get_amounts_out_calldata = contracts.encode_get_amounts_out(amount_in_wei, path.clone())?;

    let amounts_result = provider
        .call_contract(
            &contracts.router_address.to_string(),
            &hex::encode(get_amounts_out_calldata),
        )
        .await?;

    // Decode the amounts array from the result
    let amounts_bytes = hex::decode(amounts_result)?;
    // ABI encoding: [offset][length][amounts[0]][amounts[1]]
    // For ETH->Token: amounts[0]=inputETH, amounts[1]=expectedTokens
    // We want amounts[1] which is at bytes 96-127
    if amounts_bytes.len() < 128 {
        return Err(anyhow::anyhow!("Invalid getAmountsOut response"));
    }
    let expected_tokens_out_wei = U256::from_be_slice(&amounts_bytes[96..128]);
    let min_tokens_out =
        contracts.calculate_min_amount_out(expected_tokens_out_wei, params.slippage_bps);

    let deadline = contracts.calculate_deadline();

    let calldata = contracts.encode_swap_exact_eth_for_tokens(
        min_tokens_out,
        path,
        Address::from_str(&params.from_address)?,
        deadline,
    )?;

    let tx_hash = provider
        .send_contract_transaction(
            &params.from_address,
            &contracts.router_address.to_string(),
            &amount_in_wei.to_string(),
            &hex::encode(calldata),
            &params.private_key,
        )
        .await?;

    // Convert expected output to display format (handle USDC 6 decimals)
    let expected_out_display = if params.token_out.to_lowercase() == "usdc" {
        expected_tokens_out_wei
            .to_string()
            .parse::<f64>()
            .unwrap_or(0.0)
            / 1e6
    } else {
        expected_tokens_out_wei
            .to_string()
            .parse::<f64>()
            .unwrap_or(0.0)
            / 1e18
    };

    Ok(format!(
        "ETH to {} swap executed!\nTransaction: {}\nExpected output: ~{:.4} {}\nSlippage protection: {}%",
        params.token_out.to_uppercase(),
        tx_hash,
        expected_out_display,
        params.token_out.to_uppercase(),
        params.slippage_bps as f64 / 100.0
    ))
}

async fn execute_token_to_eth_swap(
    provider: &EthereumProvider,
    api_client: &ApiClient,
    contracts: &ContractInteraction,
    params: SwapParams,
) -> Result<String> {
    debug!("Executing token to ETH swap");

    let token_in_addresses = api_client.search_contract_address(&params.token_in).await?;

    let token_in_address = token_in_addresses
        .first()
        .ok_or_else(|| anyhow::anyhow!("Token {} not found", params.token_in))?;

    let token_in_addr = Address::from_str(&token_in_address.address)?;
    let from_address = Address::from_str(&params.from_address)?;

    let amount_in_f64: f64 = params.amount_in.parse()?;
    let amount_in_tokens = U256::from((amount_in_f64 * 1e18) as u128);

    // Use Uniswap's getAmountsOut for accurate pricing instead of external API
    let path = contracts.get_swap_path_token_to_eth(token_in_addr);
    let get_amounts_out_calldata =
        contracts.encode_get_amounts_out(amount_in_tokens, path.clone())?;

    let amounts_result = provider
        .call_contract(
            &contracts.router_address.to_string(),
            &hex::encode(get_amounts_out_calldata),
        )
        .await?;

    // Decode the amounts array from the result
    let amounts_bytes = hex::decode(amounts_result)?;
    // ABI encoding: [offset][length][amounts[0]][amounts[1]]
    // For Token->ETH: amounts[0]=inputTokens, amounts[1]=expectedETH
    // We want amounts[1] which is at bytes 96-127
    if amounts_bytes.len() < 128 {
        return Err(anyhow::anyhow!("Invalid getAmountsOut response"));
    }
    let expected_eth_out_wei = U256::from_be_slice(&amounts_bytes[96..128]);
    let min_eth_out = contracts.calculate_min_amount_out(expected_eth_out_wei, params.slippage_bps);

    debug!("Approving token spend");
    let approve_calldata =
        contracts.encode_erc20_approve(contracts.router_address, amount_in_tokens)?;

    let approve_tx_hash = provider
        .send_contract_transaction(
            &params.from_address,
            &token_in_address.address,
            "0",
            &hex::encode(approve_calldata),
            &params.private_key,
        )
        .await?;

    info!("Token approval transaction: {}", approve_tx_hash);

    let deadline = contracts.calculate_deadline();

    let swap_calldata = contracts.encode_swap_exact_tokens_for_eth(
        amount_in_tokens,
        min_eth_out,
        path,
        from_address,
        deadline,
    )?;

    let swap_tx_hash = provider
        .send_contract_transaction(
            &params.from_address,
            &contracts.router_address.to_string(),
            "0",
            &hex::encode(swap_calldata),
            &params.private_key,
        )
        .await?;

    let expected_eth_out_display = expected_eth_out_wei
        .to_string()
        .parse::<f64>()
        .unwrap_or(0.0)
        / 1e18;

    Ok(format!(
        "{} to ETH swap executed!\nApproval transaction: {}\nSwap transaction: {}\nExpected output: ~{:.4} ETH\nSlippage protection: {}%",
        params.token_in.to_uppercase(),
        approve_tx_hash,
        swap_tx_hash,
        expected_eth_out_display,
        params.slippage_bps as f64 / 100.0
    ))
}

async fn execute_token_to_token_swap(
    provider: &EthereumProvider,
    api_client: &ApiClient,
    contracts: &ContractInteraction,
    params: SwapParams,
) -> Result<String> {
    debug!("Executing token to token swap");

    let token_in_addresses = api_client.search_contract_address(&params.token_in).await?;

    let token_out_addresses = api_client
        .search_contract_address(&params.token_out)
        .await?;

    let token_in_address = token_in_addresses
        .first()
        .ok_or_else(|| anyhow::anyhow!("Token {} not found", params.token_in))?;

    let token_out_address = token_out_addresses
        .first()
        .ok_or_else(|| anyhow::anyhow!("Token {} not found", params.token_out))?;

    let token_in_addr = Address::from_str(&token_in_address.address)?;
    let token_out_addr = Address::from_str(&token_out_address.address)?;
    let from_address = Address::from_str(&params.from_address)?;

    let amount_in_f64: f64 = params.amount_in.parse()?;
    let amount_in_tokens = U256::from((amount_in_f64 * 1e18) as u128);

    // Use Uniswap's getAmountsOut for accurate pricing instead of external API
    let path = contracts.get_swap_path_token_to_token(token_in_addr, token_out_addr);
    let get_amounts_out_calldata =
        contracts.encode_get_amounts_out(amount_in_tokens, path.clone())?;

    let amounts_result = provider
        .call_contract(
            &contracts.router_address.to_string(),
            &hex::encode(get_amounts_out_calldata),
        )
        .await?;

    // Decode the amounts array from the result - for token-to-token, we need the last element
    let amounts_bytes = hex::decode(amounts_result)?;
    // ABI encoding: [offset][length][amounts[0]][amounts[1]][amounts[2]]
    // For Token->WETH->Token: amounts[0]=inputTokens, amounts[1]=intermediateETH, amounts[2]=outputTokens
    // We want amounts[2] which is at bytes 128-159
    if amounts_bytes.len() < 160 {
        // Need at least 3 amounts for token->WETH->token path
        return Err(anyhow::anyhow!(
            "Invalid getAmountsOut response for token-to-token swap"
        ));
    }
    let expected_tokens_out_wei = U256::from_be_slice(&amounts_bytes[128..160]); // Third element (index 2)
    let min_tokens_out =
        contracts.calculate_min_amount_out(expected_tokens_out_wei, params.slippage_bps);

    debug!("Approving token spend");
    let approve_calldata =
        contracts.encode_erc20_approve(contracts.router_address, amount_in_tokens)?;

    let approve_tx_hash = provider
        .send_contract_transaction(
            &params.from_address,
            &token_in_address.address,
            "0",
            &hex::encode(approve_calldata),
            &params.private_key,
        )
        .await?;

    info!("Token approval transaction: {}", approve_tx_hash);

    let deadline = contracts.calculate_deadline();

    let swap_calldata = contracts.encode_swap_exact_tokens_for_tokens(
        amount_in_tokens,
        min_tokens_out,
        path,
        from_address,
        deadline,
    )?;

    let swap_tx_hash = provider
        .send_contract_transaction(
            &params.from_address,
            &contracts.router_address.to_string(),
            "0",
            &hex::encode(swap_calldata),
            &params.private_key,
        )
        .await?;

    // Convert expected output to display format (handle different token decimals)
    let expected_tokens_out_display =
        if params.token_out.to_lowercase() == "usdc" || params.token_out.to_lowercase() == "usdt" {
            expected_tokens_out_wei
                .to_string()
                .parse::<f64>()
                .unwrap_or(0.0)
                / 1e6
        } else {
            expected_tokens_out_wei
                .to_string()
                .parse::<f64>()
                .unwrap_or(0.0)
                / 1e18
        };

    Ok(format!(
        "{} to {} swap executed!\nApproval transaction: {}\nSwap transaction: {}\nExpected output: ~{:.4} {}\nSlippage protection: {}%",
        params.token_in.to_uppercase(),
        params.token_out.to_uppercase(),
        approve_tx_hash,
        swap_tx_hash,
        expected_tokens_out_display,
        params.token_out.to_uppercase(),
        params.slippage_bps as f64 / 100.0
    ))
}
