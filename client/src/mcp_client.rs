use crate::error::AgentError;
use anyhow::Result;
use serde_json::{json, Value};
use std::process::Stdio;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, Command};
use tokio::time::timeout;
use tracing::{error, info, warn};

pub struct McpClient {
    process: Child,
    request_id: u64,
}

impl McpClient {
    pub async fn new(server_command: &str, server_args: Vec<&str>) -> Result<Self> {
        info!("Starting MCP server: {} {:?}", server_command, server_args);

        let process = Command::new(server_command)
            .args(server_args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let mut client = Self {
            process,
            request_id: 1,
        };

        // Initialize the MCP connection with proper JSON-RPC 2.0
        client.initialize().await?;

        Ok(client)
    }

    async fn initialize(&mut self) -> Result<()> {
        let init_request = json!({
            "jsonrpc": "2.0",
            "id": self.request_id,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": {
                    "name": "agent-eth-client",
                    "version": "0.1.0"
                }
            }
        });

        self.send_request(init_request).await?;
        self.request_id += 1;

        // Send initialized notification (JSON-RPC 2.0 notification)
        let initialized_notification = json!({
            "jsonrpc": "2.0",
            "method": "notifications/initialized"
        });

        self.send_notification(initialized_notification).await?;

        Ok(())
    }

    async fn send_request(&mut self, request: Value) -> Result<Value> {
        let request_str = format!("{}\n", serde_json::to_string(&request)?);

        tracing::debug!("Sending MCP request: {}", request_str.trim());

        if let Some(stdin) = self.process.stdin.as_mut() {
            stdin.write_all(request_str.as_bytes()).await?;
            stdin.flush().await?;
        } else {
            return Err(AgentError::mcp("Server process has no stdin").into());
        }

        // Read response with timeout and better error handling
        if let Some(stdout) = self.process.stdout.as_mut() {
            let mut reader = BufReader::new(stdout);
            let mut line = String::new();

            // Add timeout to prevent hanging
            match timeout(Duration::from_secs(30), reader.read_line(&mut line)).await {
                Ok(Ok(0)) => {
                    error!("Server closed stdout unexpectedly");
                    Err(AgentError::mcp("Server closed stdout").into())
                }
                Ok(Ok(_)) => {
                    tracing::debug!("Raw server response: {:?}", line);

                    if !line.trim().is_empty() {
                        let response: Value = serde_json::from_str(line.trim()).map_err(|e| {
                            error!("JSON parse error: {}, raw response: '{}'", e, line.trim());
                            anyhow::anyhow!(
                                "Failed to parse server response as JSON: {}. Response was: '{}'",
                                e,
                                line.trim()
                            )
                        })?;
                        Ok(response)
                    } else {
                        warn!("Received empty response from server");
                        Err(AgentError::mcp("Empty response from server").into())
                    }
                }
                Ok(Err(e)) => {
                    error!("IO error reading from server: {}", e);
                    Err(AgentError::mcp(format!("Failed to read from server: {e}")).into())
                }
                Err(_) => {
                    error!("Timeout waiting for server response");
                    Err(AgentError::mcp("Timeout waiting for server response".to_string()).into())
                }
            }
        } else {
            Err(AgentError::mcp("Server process has no stdout").into())
        }
    }

    async fn send_notification(&mut self, notification: Value) -> Result<()> {
        let notification_str = format!("{}\n", serde_json::to_string(&notification)?);

        tracing::debug!("Sending MCP notification: {}", notification_str.trim());

        if let Some(stdin) = self.process.stdin.as_mut() {
            stdin.write_all(notification_str.as_bytes()).await?;
            stdin.flush().await?;
        }

        Ok(())
    }

    /// Check if the server process is still running
    pub fn is_healthy(&mut self) -> bool {
        match self.process.try_wait() {
            Ok(Some(_)) => {
                warn!("MCP server process has exited");
                false
            }
            Ok(None) => true, // Still running
            Err(e) => {
                error!("Error checking server process status: {}", e);
                false
            }
        }
    }

    async fn call_tool(&mut self, tool_name: &str, arguments: Value) -> Result<String> {
        // Check if server is still healthy before making calls
        if !self.is_healthy() {
            return Err(AgentError::mcp("MCP server process is not running").into());
        }
        let request = json!({
            "jsonrpc": "2.0",
            "id": self.request_id,
            "method": "tools/call",
            "params": {
                "name": tool_name,
                "arguments": arguments
            }
        });

        let response = self.send_request(request).await?;
        self.request_id += 1;

        // Debug: Print the full response for troubleshooting
        tracing::debug!(
            "MCP Response: {}",
            serde_json::to_string_pretty(&response).unwrap_or_default()
        );

        // Extract the result from the response (proper JSON-RPC 2.0 handling)
        if let Some(result) = response.get("result") {
            if let Some(content) = result.get("content") {
                if let Some(content_array) = content.as_array() {
                    if let Some(first_content) = content_array.first() {
                        if let Some(text) = first_content.get("text") {
                            return Ok(text.as_str().unwrap_or("").to_string());
                        }
                    }
                }
            }
        }

        // Handle error response (JSON-RPC 2.0 error format)
        if let Some(error) = response.get("error") {
            let error_code = error.get("code").and_then(|c| c.as_i64()).unwrap_or(-1);
            let error_msg = error
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("Unknown error");
            return Err(AgentError::mcp(format!("MCP error {error_code}: {error_msg}")).into());
        }

        // Debug: Show what we received instead
        tracing::debug!(
            "Unexpected response format: {}",
            serde_json::to_string_pretty(&response).unwrap_or_default()
        );
        Ok("No response content".to_string())
    }

    pub async fn get_balance(&mut self, address: &str, token: Option<&str>) -> Result<String> {
        let mut arguments = json!({
            "address": address
        });

        if let Some(token_symbol) = token {
            arguments["token"] = json!(token_symbol);
        }

        self.call_tool("get_balance", arguments).await
    }

    pub async fn send_transfer(
        &mut self,
        from_address: &str,
        to_address: &str,
        amount_eth: &str,
        private_key: &str,
    ) -> Result<String> {
        let arguments = json!({
            "from_address": from_address,
            "to_address": to_address,
            "amount_eth": amount_eth,
            "private_key": private_key
        });

        self.call_tool("send_transfer", arguments).await
    }

    pub async fn check_contract(&mut self, address: &str) -> Result<String> {
        let arguments = json!({
            "address": address
        });

        self.call_tool("check_contract", arguments).await
    }

    pub async fn execute_swap(
        &mut self,
        from_address: &str,
        token_in: &str,
        token_out: &str,
        amount_in: &str,
        slippage_bps: u16,
        private_key: &str,
    ) -> Result<String> {
        let arguments = json!({
            "from_address": from_address,
            "token_in": token_in,
            "token_out": token_out,
            "amount_in": amount_in,
            "slippage_bps": slippage_bps,
            "private_key": private_key
        });

        self.call_tool("execute_swap", arguments).await
    }
}

impl Drop for McpClient {
    fn drop(&mut self) {
        info!("Shutting down MCP client");

        // Graceful shutdown: close stdin to signal shutdown
        if let Some(stdin) = self.process.stdin.take() {
            drop(stdin); // Close stdin to signal the server to shutdown gracefully
        }

        // Force terminate if still running
        std::mem::drop(self.process.kill());

        // Give it a moment to cleanup
        std::thread::sleep(Duration::from_millis(100));
    }
}
