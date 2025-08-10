use anyhow::Result;
use rmcp::{
    handler::server::{router::tool::ToolRouter, tool::Parameters},
    model::*,
    service::RequestContext,
    tool, tool_handler, tool_router, ErrorData as McpError, RoleServer, ServerHandler,
};
use std::{env, future::Future};

use crate::provider::EthereumProvider;
use crate::tools::{self, SwapParams};

#[derive(Clone)]
pub struct EthereumToolRouter {
    provider: EthereumProvider,
    tool_router: ToolRouter<EthereumToolRouter>,
}

#[tool_router]
impl EthereumToolRouter {
    pub fn new() -> Result<Self> {
        let rpc_url = env::var("RPC_URL").unwrap_or_else(|_| "http://127.0.0.1:8545".to_string());
        let provider = EthereumProvider::new(&rpc_url)?;

        Ok(Self {
            provider,
            tool_router: Self::tool_router(),
        })
    }

    #[tool(description = "Get the balance of an Ethereum address for ETH or ERC20 tokens")]
    async fn get_balance(
        &self,
        Parameters(params): Parameters<serde_json::Value>,
    ) -> Result<CallToolResult, McpError> {
        let address = params
            .get("address")
            .and_then(|v| v.as_str())
            .ok_or_else(|| McpError::invalid_params("Missing required parameter: address", None))?
            .to_string();

        let token = params
            .get("token")
            .and_then(|v| v.as_str())
            .unwrap_or("ETH")
            .to_string();

        match tools::get_token_balance(&self.provider, address, token).await {
            Ok(result) => Ok(CallToolResult::success(vec![Content::text(result)])),
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }

    #[tool(description = "Send ETH from one address to another")]
    async fn send_transfer(
        &self,
        Parameters(params): Parameters<serde_json::Value>,
    ) -> Result<CallToolResult, McpError> {
        let from_address = params
            .get("from_address")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                McpError::invalid_params("Missing required parameter: from_address", None)
            })?
            .to_string();

        let to_address = params
            .get("to_address")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                McpError::invalid_params("Missing required parameter: to_address", None)
            })?
            .to_string();

        let amount_eth = params
            .get("amount_eth")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                McpError::invalid_params("Missing required parameter: amount_eth", None)
            })?
            .to_string();

        let private_key = params
            .get("private_key")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                McpError::invalid_params("Missing required parameter: private_key", None)
            })?
            .to_string();

        match tools::send_eth_transfer(
            &self.provider,
            from_address,
            to_address,
            amount_eth,
            private_key,
        )
        .await
        {
            Ok(result) => Ok(CallToolResult::success(vec![Content::text(result)])),
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }

    #[tool(description = "Check if an address has deployed contract code")]
    async fn check_contract(
        &self,
        Parameters(params): Parameters<serde_json::Value>,
    ) -> Result<CallToolResult, McpError> {
        let address = params
            .get("address")
            .and_then(|v| v.as_str())
            .ok_or_else(|| McpError::invalid_params("Missing required parameter: address", None))?
            .to_string();

        match tools::check_contract_deployed(&self.provider, address).await {
            Ok(result) => Ok(CallToolResult::success(vec![Content::text(result)])),
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }

    #[tool(description = "Execute a token swap using Uniswap V2 Router")]
    async fn execute_swap(
        &self,
        Parameters(params): Parameters<serde_json::Value>,
    ) -> Result<CallToolResult, McpError> {
        let from_address = params
            .get("from_address")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                McpError::invalid_params("Missing required parameter: from_address", None)
            })?
            .to_string();

        let token_in = params
            .get("token_in")
            .and_then(|v| v.as_str())
            .ok_or_else(|| McpError::invalid_params("Missing required parameter: token_in", None))?
            .to_string();

        let token_out = params
            .get("token_out")
            .and_then(|v| v.as_str())
            .ok_or_else(|| McpError::invalid_params("Missing required parameter: token_out", None))?
            .to_string();

        let amount_in = params
            .get("amount_in")
            .and_then(|v| v.as_str())
            .ok_or_else(|| McpError::invalid_params("Missing required parameter: amount_in", None))?
            .to_string();

        let private_key = params
            .get("private_key")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                McpError::invalid_params("Missing required parameter: private_key", None)
            })?
            .to_string();

        let slippage_bps = params
            .get("slippage_bps")
            .and_then(|v| v.as_u64())
            .map(|v| v as u16)
            .unwrap_or(200);

        let swap_params = SwapParams {
            from_address,
            token_in,
            token_out,
            amount_in,
            slippage_bps,
            private_key,
        };

        match tools::execute_swap(&self.provider, swap_params).await {
            Ok(result) => Ok(CallToolResult::success(vec![Content::text(result)])),
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }
}

#[tool_handler]
impl ServerHandler for EthereumToolRouter {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("Ethereum blockchain interaction server providing balance queries, transfers, contract checks, and token swaps.".to_string()),
        }
    }

    async fn initialize(
        &self,
        _request: InitializeRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        Ok(self.get_info())
    }
}
