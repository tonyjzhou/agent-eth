use anyhow::Result;
use serde_json::{json, Value};
use std::env;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tracing::{error, info};

use crate::provider::EthereumProvider;
use crate::tools;

pub struct EthereumMcpServer {
    provider: EthereumProvider,
}

impl EthereumMcpServer {
    pub fn new() -> Result<Self> {
        let rpc_url = env::var("RPC_URL").unwrap_or_else(|_| "http://127.0.0.1:8545".to_string());
        let provider = EthereumProvider::new(&rpc_url)?;

        Ok(Self { provider })
    }

    pub async fn run(self) -> Result<()> {
        info!("Starting MCP server on stdio...");

        let stdin = tokio::io::stdin();
        let _stdout = tokio::io::stdout();
        let mut reader = BufReader::new(stdin);
        let mut line = String::new();

        loop {
            line.clear();
            match reader.read_line(&mut line).await {
                Ok(0) => break, // EOF
                Ok(_) => {
                    if let Err(e) = self.handle_message(&line, &mut tokio::io::stdout()).await {
                        error!("Error handling message: {}", e);
                    }
                }
                Err(e) => {
                    error!("Error reading from stdin: {}", e);
                    break;
                }
            }
        }

        Ok(())
    }

    async fn handle_message(&self, message: &str, stdout: &mut tokio::io::Stdout) -> Result<()> {
        let message = message.trim();
        if message.is_empty() {
            return Ok(());
        }

        let request: Value = serde_json::from_str(message)?;
        let response = self.process_request(request).await;

        let response_str = format!("{}\n", serde_json::to_string(&response)?);
        stdout.write_all(response_str.as_bytes()).await?;
        stdout.flush().await?;

        Ok(())
    }

    async fn process_request(&self, request: Value) -> Value {
        let method = request.get("method").and_then(|m| m.as_str());
        let id = request.get("id");

        match method {
            Some("initialize") => {
                let response = json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "protocolVersion": "2025-06-18",
                        "capabilities": {
                            "tools": {
                                "listChanged": false
                            }
                        },
                        "serverInfo": {
                            "name": "Agent-ETH MCP Server",
                            "version": "0.1.0"
                        },
                        "instructions": "Ethereum blockchain interaction server"
                    }
                });
                response
            }
            Some("notifications/initialized") => {
                // Just acknowledge the notification, no response needed for notifications
                json!({})
            }
            Some("tools/list") => {
                let response = json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "tools": [
                            {
                                "name": "get_balance",
                                "description": "Get the balance of an Ethereum address for ETH or ERC20 tokens",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "address": {
                                            "type": "string",
                                            "description": "The Ethereum address to check"
                                        },
                                        "token": {
                                            "type": "string",
                                            "description": "The token symbol (ETH for Ethereum, or ERC20 token symbol)",
                                            "default": "ETH"
                                        }
                                    },
                                    "required": ["address"]
                                }
                            },
                            {
                                "name": "send_transfer",
                                "description": "Send ETH from one address to another",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "from_address": {
                                            "type": "string",
                                            "description": "The sender's Ethereum address"
                                        },
                                        "to_address": {
                                            "type": "string",
                                            "description": "The recipient's Ethereum address"
                                        },
                                        "amount_eth": {
                                            "type": "string",
                                            "description": "The amount of ETH to send"
                                        },
                                        "private_key": {
                                            "type": "string",
                                            "description": "The sender's private key"
                                        }
                                    },
                                    "required": ["from_address", "to_address", "amount_eth", "private_key"]
                                }
                            },
                            {
                                "name": "check_contract",
                                "description": "Check if an address has deployed contract code",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "address": {
                                            "type": "string",
                                            "description": "The Ethereum address to check"
                                        }
                                    },
                                    "required": ["address"]
                                }
                            },
                            {
                                "name": "execute_swap",
                                "description": "Execute a token swap using Uniswap V2 Router",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "from_address": {
                                            "type": "string",
                                            "description": "The trader's Ethereum address"
                                        },
                                        "token_in": {
                                            "type": "string",
                                            "description": "Input token symbol or address"
                                        },
                                        "token_out": {
                                            "type": "string",
                                            "description": "Output token symbol or address"
                                        },
                                        "amount_in": {
                                            "type": "string",
                                            "description": "Amount of input tokens to swap"
                                        },
                                        "slippage_bps": {
                                            "type": "number",
                                            "description": "Slippage tolerance in basis points (default: 200)",
                                            "default": 200
                                        },
                                        "private_key": {
                                            "type": "string",
                                            "description": "The trader's private key"
                                        }
                                    },
                                    "required": ["from_address", "token_in", "token_out", "amount_in", "private_key"]
                                }
                            }
                        ]
                    }
                });
                response
            }
            Some("tools/call") => self.handle_tool_call(&request, id).await,
            _ => {
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "error": {
                        "code": -32601,
                        "message": format!("Method not found: {}", method.unwrap_or("unknown"))
                    }
                })
            }
        }
    }

    async fn handle_tool_call(&self, request: &Value, id: Option<&Value>) -> Value {
        let params = request.get("params");
        let tool_name = params.and_then(|p| p.get("name")).and_then(|n| n.as_str());
        let empty_args = json!({});
        let arguments = params
            .and_then(|p| p.get("arguments"))
            .unwrap_or(&empty_args);

        let result = match tool_name {
            Some("get_balance") => self.handle_get_balance(arguments).await,
            Some("send_transfer") => self.handle_send_transfer(arguments).await,
            Some("check_contract") => self.handle_check_contract(arguments).await,
            Some("execute_swap") => self.handle_execute_swap(arguments).await,
            _ => Err(anyhow::anyhow!("Unknown tool: {:?}", tool_name)),
        };

        match result {
            Ok(content) => json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "content": [
                        {
                            "type": "text",
                            "text": content
                        }
                    ]
                }
            }),
            Err(e) => json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": {
                    "code": -32000,
                    "message": e.to_string()
                }
            }),
        }
    }

    async fn handle_get_balance(&self, arguments: &Value) -> Result<String> {
        let address = arguments
            .get("address")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: address"))?;

        let token = arguments
            .get("token")
            .and_then(|v| v.as_str())
            .unwrap_or("ETH");

        tools::get_token_balance(&self.provider, address.to_string(), token.to_string()).await
    }

    async fn handle_send_transfer(&self, arguments: &Value) -> Result<String> {
        let from_address = arguments
            .get("from_address")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: from_address"))?;

        let to_address = arguments
            .get("to_address")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: to_address"))?;

        let amount_eth = arguments
            .get("amount_eth")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: amount_eth"))?;

        let private_key = arguments
            .get("private_key")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: private_key"))?;

        tools::send_eth_transfer(
            &self.provider,
            from_address.to_string(),
            to_address.to_string(),
            amount_eth.to_string(),
            private_key.to_string(),
        )
        .await
    }

    async fn handle_check_contract(&self, arguments: &Value) -> Result<String> {
        let address = arguments
            .get("address")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: address"))?;

        tools::check_contract_deployed(&self.provider, address.to_string()).await
    }

    async fn handle_execute_swap(&self, arguments: &Value) -> Result<String> {
        let from_address = arguments
            .get("from_address")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: from_address"))?;

        let token_in = arguments
            .get("token_in")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: token_in"))?;

        let token_out = arguments
            .get("token_out")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: token_out"))?;

        let amount_in = arguments
            .get("amount_in")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: amount_in"))?;

        let slippage_bps = arguments
            .get("slippage_bps")
            .and_then(|v| v.as_u64())
            .unwrap_or(200) as u16;

        let private_key = arguments
            .get("private_key")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: private_key"))?;

        let swap_params = tools::SwapParams {
            from_address: from_address.to_string(),
            token_in: token_in.to_string(),
            token_out: token_out.to_string(),
            amount_in: amount_in.to_string(),
            slippage_bps,
            private_key: private_key.to_string(),
        };

        tools::execute_swap(&self.provider, swap_params).await
    }
}
