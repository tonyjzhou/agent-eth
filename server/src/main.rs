mod api;
mod contracts;
mod provider;
mod tools;

use axum::{
    extract::Json, http::StatusCode, response::Json as ResponseJson, routing::post, Router,
};
use provider::EthereumProvider;
use serde_json::{json, Value};
use std::env;
use tracing::{error, info};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let rpc_url = env::var("RPC_URL").unwrap_or_else(|_| "http://127.0.0.1:8545".to_string());
    info!("Starting HTTP server with RPC URL: {}", rpc_url);

    let app = Router::new()
        .route("/balance", post(balance_handler))
        .route("/transfer", post(transfer_handler))
        .route("/contract_check", post(contract_check_handler))
        .route("/swap", post(swap_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    info!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await?;

    Ok(())
}

async fn balance_handler(Json(params): Json<Value>) -> Result<ResponseJson<Value>, StatusCode> {
    let address = params
        .get("address")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;

    let token = params
        .get("token")
        .and_then(|v| v.as_str())
        .unwrap_or("ETH");

    let provider = EthereumProvider::new("http://127.0.0.1:8545")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Use token balance handler for both ETH and ERC20 tokens
    match tools::get_token_balance(&provider, address.to_string(), token.to_string()).await {
        Ok(result) => Ok(ResponseJson(json!({"result": result}))),
        Err(e) => {
            error!("Balance error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn transfer_handler(Json(params): Json<Value>) -> Result<ResponseJson<Value>, StatusCode> {
    let from_address = params
        .get("from_address")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;

    let to_address = params
        .get("to_address")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;

    let amount_eth = params
        .get("amount_eth")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;

    let private_key = params
        .get("private_key")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;

    let provider = EthereumProvider::new("http://127.0.0.1:8545")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match tools::send_eth_transfer(
        &provider,
        from_address.to_string(),
        to_address.to_string(),
        amount_eth.to_string(),
        private_key.to_string(),
    )
    .await
    {
        Ok(result) => Ok(ResponseJson(json!({"result": result}))),
        Err(e) => {
            error!("Transfer error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn contract_check_handler(
    Json(params): Json<Value>,
) -> Result<ResponseJson<Value>, StatusCode> {
    let address = params
        .get("address")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;

    let provider = EthereumProvider::new("http://127.0.0.1:8545")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match tools::check_contract_deployed(&provider, address.to_string()).await {
        Ok(result) => Ok(ResponseJson(json!({"result": result}))),
        Err(e) => {
            error!("Contract check error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn swap_handler(Json(params): Json<Value>) -> Result<ResponseJson<Value>, StatusCode> {
    let from_address = params
        .get("from_address")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;

    let token_in = params
        .get("token_in")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;

    let token_out = params
        .get("token_out")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;

    let amount_in = params
        .get("amount_in")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;

    let slippage_bps = params
        .get("slippage_bps")
        .and_then(|v| v.as_u64())
        .unwrap_or(200) as u16;

    let private_key = params
        .get("private_key")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;

    let provider = EthereumProvider::new("http://127.0.0.1:8545")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let swap_params = tools::SwapParams {
        from_address: from_address.to_string(),
        token_in: token_in.to_string(),
        token_out: token_out.to_string(),
        amount_in: amount_in.to_string(),
        slippage_bps,
        private_key: private_key.to_string(),
    };

    match tools::execute_swap(&provider, swap_params).await {
        Ok(result) => Ok(ResponseJson(json!({"result": result}))),
        Err(e) => {
            error!("Swap error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
