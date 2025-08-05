mod provider;
mod tools;

use provider::EthereumProvider;
use axum::{extract::Json, http::StatusCode, response::Json as ResponseJson, routing::post, Router};
use serde_json::{json, Value};
use std::env;
use tracing::{info, error};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    
    let rpc_url = env::var("RPC_URL").unwrap_or_else(|_| "http://127.0.0.1:8545".to_string());
    info!("Starting HTTP server with RPC URL: {}", rpc_url);
    
    let app = Router::new()
        .route("/balance", post(balance_handler))
        .route("/transfer", post(transfer_handler))
        .route("/contract_check", post(contract_check_handler));
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    info!("Server running on http://127.0.0.1:3000");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn balance_handler(Json(params): Json<Value>) -> Result<ResponseJson<Value>, StatusCode> {
    let address = params.get("address")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    let in_ether = params.get("in_ether")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    
    let provider = EthereumProvider::new("http://127.0.0.1:8545")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    match tools::get_balance(&provider, address.to_string(), Some(in_ether)).await {
        Ok(result) => Ok(ResponseJson(json!({"result": result}))),
        Err(e) => {
            error!("Balance error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn transfer_handler(Json(params): Json<Value>) -> Result<ResponseJson<Value>, StatusCode> {
    let from_address = params.get("from_address")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    let to_address = params.get("to_address")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    let amount_eth = params.get("amount_eth")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    let private_key = params.get("private_key")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    let provider = EthereumProvider::new("http://127.0.0.1:8545")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    match tools::send_eth_transfer(&provider, from_address.to_string(), to_address.to_string(), amount_eth.to_string(), private_key.to_string()).await {
        Ok(result) => Ok(ResponseJson(json!({"result": result}))),
        Err(e) => {
            error!("Transfer error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn contract_check_handler(Json(params): Json<Value>) -> Result<ResponseJson<Value>, StatusCode> {
    let address = params.get("address")
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