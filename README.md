# Agent-ETH: AI Agent for Ethereum Blockchain

An AI-powered agent system that allows users to interact with Ethereum blockchain using natural language commands. Features a CLI client with Anthropic Claude integration and an RMCP server exposing Ethereum tools.

## Architecture

```
             ┌─────────────────┐  RMCP over stdio   ┌──────────────────┐
             │   CLI Client    │◄──────────────────►│   RMCP Server    │
             │                 │                    │                  │
             ├─────────────────┤                    ├──────────────────┤
User   ◄───► │ • Interactive   │                    │ • Balance Tool   │
Claude ◄───► │   REPL          │                    │ • Transfer Tool  │
             │ • Claude API    │                    │ • Contract Tool  │
             │ • RAG System    │                    │ • Swap Tool      │
             │ • Account       │                    │ • Alloy v1.0     │
             │   Aliases       │                    │ • External APIs  │
             │ • RMCP Client   │                    │ • RMCP v0.5.0    │
             └─────────────────┘                    └──────────────────┘
                      │                                       │
                      │                                       │
                      └───────────────┐           ┌───────────┘
                                      │           │
                                 ┌────▼───────────▼──────┐
                                 │   Forked Ethereum     │
                                 │     Test Network      │
                                 │      (Anvil)          │
                                 └───────────────────────┘
```

## Features

- **Natural Language Processing**: Interact with Ethereum using plain English
- **Account Aliases**: Use friendly names (Alice, Bob, Carol) instead of hex addresses
- **Balance Queries**: Check ETH balances for any address
- **ETH Transfers**: Send ETH between accounts with automatic transaction handling
- **Contract Verification**: Check if contracts are deployed at specific addresses
- **Token Swaps**: Swap tokens using Uniswap V2 Router with slippage protection
- **RAG Documentation Search**: Query ingested documentation using natural language
- **CLI REPL Interface**: Interactive command-line interface with colored output

## Prerequisites

1. **Install Foundry**:
   ```bash
   curl -L https://foundry.sh | bash
   foundryup
   ```

2. **Set up Anthropic API Key**:
   ```bash
   export ANTHROPIC_API_KEY="your-api-key-here"
   ```

3. **Start Local Ethereum Fork**:
   ```bash
   anvil --fork-url https://eth-mainnet.g.alchemy.com/v2/4UjEl1ULr2lQYsGR5n7gGKd3pzgAzxKs
   ```

## Installation

1. Clone and build the project:
   ```bash
   git clone <repository-url>
   cd agent-eth
   cargo build --release
   ```

2. The build will create two binaries:
   - `./target/release/agent-eth-server` (RMCP Server)
   - `./target/release/agent-eth-client` (CLI Client)

## Usage

1. **Start the Anvil fork** (in a separate terminal):
   ```bash
   anvil --fork-url https://eth-mainnet.g.alchemy.com/v2/4UjEl1ULr2lQYsGR5n7gGKd3pzgAzxKs
   ```

2. **Run the client** (from the workspace root):
   ```bash
   ANTHROPIC_API_KEY="your-key" cargo run --bin agent-eth-client
   ```

   **Note**: The client automatically spawns the RMCP server as a subprocess, so no separate server process is needed.

3. **Example Commands**:
   ```
   eth> send 1 ETH from Alice to Bob
   eth> How much ETH does Alice have?
   eth> Is Uniswap V2 Router deployed at 0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D?
   eth> What's Bob's balance?
   eth> Transfer 0.5 ETH from Carol to Alice
   eth> Use Uniswap V2 Router to swap 10 ETH for USDC on Alice's account
   ```

## Test Accounts

The system comes pre-configured with test accounts from Anvil:

| Alias | Address | Private Key |
|-------|---------|-------------|
| Alice | `0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266` | `0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80` |
| Bob   | `0x70997970C51812dc3A010C7d01b50e0d17dc79C8` | `0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d` |
| Carol | `0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC` | `0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a` |

Each account starts with 10,000 ETH on the local fork.

## Available Commands

### Balance Check
- `"How much ETH does Alice have?"`
- `"What's the balance of 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266?"`
- `"Check Alice's balance"`

### ETH Transfer
- `"Send 1 ETH from Alice to Bob"`
- `"Transfer 0.5 ETH from Carol to Alice"`
- `"Move 2.5 ETH from Bob to Carol"`

### Contract Verification
- `"Is Uniswap V2 Router deployed at 0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D?"`
- `"Check if there's a contract at 0x1234..."`

### Token Swaps
- `"Use Uniswap V2 Router to swap 10 ETH for USDC on Alice's account"`
- `"Swap 1000 USDC for WETH from Bob's account"`
- `"Swap 500 USDC for ETH using Alice's account"`

### Documentation Commands
- `ingest <directory_path>` - Ingest documentation from a directory for RAG search
- `clear` or `clear docs` - Clear all ingested documents from RAG system
- `"How do I create a pool in Uniswap V3?"` - Ask questions about ingested documentation

### System Commands
- `help` - Show available commands
- `exit` or `quit` - Exit the program

## Development

### Project Structure

```
agent-eth/
├── Cargo.toml              # Workspace configuration
├── client/                 # CLI Agent Client
│   ├── Cargo.toml
│   ├── docs/               # Uniswap documentation for RAG
│   └── src/
│       ├── main.rs         # CLI REPL interface
│       ├── lib.rs          # Library exports
│       ├── agent.rs        # Claude API integration
│       ├── mcp_client.rs   # MCP client for server communication
│       ├── bin/
│       │   └── test_client.rs # Test client for debugging
│       └── rag/            # RAG system implementation
│           ├── mod.rs      # RAG system core
│           ├── embeddings.rs # Embedding service
│           ├── ingestion.rs  # Document ingestion
│           └── storage.rs  # Vector storage
├── server/                 # MCP Server
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs         # MCP server with stdio transport
│       ├── lib.rs          # Library exports
│       ├── mcp_server.rs   # MCP protocol implementation
│       ├── provider.rs     # Ethereum provider (Alloy)
│       ├── bin/
│       │   └── test_balance.rs # Test balance functionality
│       └── tools/          # Blockchain tools
│           ├── mod.rs        # Module exports
│           ├── token_balance.rs # Balance checking (ETH & ERC20)
│           ├── transfer.rs   # ETH transfers
│           ├── contract_check.rs # Contract verification
│           └── swap.rs       # Token swaps via Uniswap
└── README.md
```

### Running in Development

1. **Terminal 1** - Start Anvil:
   ```bash
   anvil --fork-url https://eth-mainnet.g.alchemy.com/v2/4UjEl1ULr2lQYsGR5n7gGKd3pzgAzxKs
   ```

2. **Terminal 2** - Run the client (from workspace root):
   ```bash
   ANTHROPIC_API_KEY="your-key" cargo run --bin agent-eth-client
   ```

   The client will automatically spawn and communicate with the MCP server.

### Environment Variables

- `ANTHROPIC_API_KEY` - Required. Your Anthropic API key
- `RPC_URL` - Optional. Ethereum RPC URL (defaults to `http://127.0.0.1:8545`)
- `BRAVE_API_KEY` - Optional. Brave Search API key for contract address discovery
- `COINGECKO_API_KEY` - Optional. CoinGecko API key for protocol address resolution
- `OPENAI_API_KEY` - Optional. OpenAI API key for enhanced embeddings (falls back to basic embeddings)

## Technical Details

### Components

1. **CLI Client**:
   - Interactive REPL using rustyline with colored output
   - Direct Anthropic Claude API integration for natural language parsing
   - Account alias resolution (Alice, Bob, Carol → hex addresses)
   - RMCP client for communicating with server via stdio transport
   - RAG system for documentation search with vector embeddings using SQLite

2. **RMCP Server**:
   - RMCP (Rust Model Context Protocol) v0.5.0 compliant server exposing Ethereum tools
   - Uses Alloy v1.0 for Ethereum blockchain interactions (migrated from deprecated ethers-rs)
   - Four main tools: `get_balance`, `send_transfer`, `check_contract`, `execute_swap`
   - External API integration (Brave Search, CoinGecko) for contract discovery
   - On-chain pricing via Uniswap V2 Router's `getAmountsOut` for accurate token pricing
   - Connects to local Anvil fork on http://127.0.0.1:8545

3. **RAG System**:
   - Vector database for document storage and similarity search
   - OpenAI embeddings API for enhanced text understanding
   - Document ingestion from markdown files
   - Natural language querying of technical documentation

4. **Communication**:
   - Client and server communicate via RMCP (Rust Model Context Protocol) using stdio transport
   - Server runs as subprocess spawned by client
   - Async/await throughout for non-blocking operations
   - Error handling with RMCP-compliant error responses using McpError types

### External API Integration

**Contract Discovery:**
- **Brave Search API**: Searches for Ethereum contract addresses by token name using web search
  - Endpoint: `https://api.search.brave.com/res/v1/web/search`
  - Extracts addresses using regex pattern matching from search results
  - Fallback to hardcoded addresses for common tokens (USDC, WETH, Uniswap Router)

- **CoinGecko API**: Protocol address resolution for DeFi protocols
  - Endpoint: `https://api.coingecko.com/api/v3/exchanges/uniswap_v2`
  - Currently falls back to hardcoded addresses but framework is in place
  - Supports free tier with rate limits or Pro tier with API key

**Token Pricing:**
- **Uniswap V2 Router On-Chain Calls**: Real-time, accurate pricing using `getAmountsOut`
  - Calls Uniswap V2 Router contract directly for swap price calculations
  - More accurate than external price APIs as it uses actual AMM pool ratios
  - Supports ETH↔Token and Token↔Token pricing through WETH intermediary

### Security Notes

- Private keys are hardcoded for test accounts only
- Never use these private keys on mainnet
- The system is designed for local development and testing

## RMCP (Rust Model Context Protocol) Implementation

Agent-ETH implements RMCP v0.5.0 for client-server communication, providing a standardized way to expose Ethereum tools to AI agents.

### RMCP Architecture

**Client-Server Communication:**
- **Protocol**: RMCP over stdio transport
- **Process Model**: Client spawns server as subprocess 
- **Message Flow**: Request/response with async futures
- **Error Handling**: RMCP-compliant error codes and messages using McpError

**Tool Discovery:**
Tools are automatically registered using RMCP macros:
```rust
#[tool(description = "Get the balance of an Ethereum address for ETH or ERC20 tokens")]
async fn get_balance(
    &self,
    Parameters(params): Parameters<serde_json::Value>,
) -> Result<CallToolResult, McpError> {
    // Implementation
}
```

The `#[tool]` macro automatically generates the tool schema and registration.

### Implementation Details

**Server Path Resolution:**
- Default server path: `./target/debug/agent-eth-server` (relative to workspace root)
- Configurable via `--server-command` CLI argument
- **Important**: Client must be run from workspace root directory

**Logging Configuration:**
- Server logs to stderr to avoid stdout JSON corruption
- Debug logging available via `RUST_LOG=debug` environment variable
- Client includes debug logging for MCP request/response tracing

**Error Handling:**
- Server properly handles MCP notifications (no response sent)
- Client includes detailed JSON parsing error messages
- Graceful fallback for unexpected response formats

### Recent Major Updates

1. **RMCP Migration** (`server/src/mcp_server.rs`, `server/Cargo.toml`):
   - Migrated from basic HTTP communication to RMCP v0.5.0
   - Added `#[tool_router]` and `#[tool]` macros for automatic tool registration
   - Provides standardized MCP-compliant tool exposure with schema generation

2. **Alloy v1.0 Integration** (`server/src/provider.rs`, `server/Cargo.toml`):
   - Migrated from deprecated ethers-rs to modern Alloy v1.0
   - Added real transaction execution with wallet integration
   - Improved performance and maintainability

3. **Enhanced Logging** (`server/src/main.rs:11-14`):
   - Configured tracing to write to stderr to avoid stdio transport interference
   - Clean RMCP communication without log message corruption

### Benefits of RMCP Implementation

- **Standardized Protocol**: Well-defined specification for tool exposure
- **Macro-based Registration**: `#[tool]` and `#[tool_router]` macros for automatic schema generation
- **Type Safety**: Built-in parameter validation and error handling
- **Tool Discovery**: Automatic enumeration of available tools and their schemas
- **Rust-native**: Native Rust implementation with async/await support
- **Debugging Support**: Built-in logging and error handling with McpError types
- **Future Extensibility**: Easy to add new tools using RMCP patterns

## Troubleshooting

### Common Issues

1. **"No such file or directory (os error 2)"**:
   - **Cause**: Client can't find the RMCP server executable
   - **Solution**: Run client from workspace root directory: `cd /path/to/agent-eth && cargo run --bin agent-eth-client`
   - **Check**: Verify server exists at `./target/debug/agent-eth-server`

2. **"Connection refused"**: 
   - **Cause**: Anvil is not running or not accessible
   - **Solution**: Start Anvil in separate terminal: `anvil --fork-url https://eth-mainnet.g.alchemy.com/v2/4UjEl1ULr2lQYsGR5n7gGKd3pzgAzxKs`
   - **Check**: Anvil should be listening on `127.0.0.1:8545`

3. **"Invalid API key"**: 
   - **Cause**: Missing or incorrect Anthropic API key
   - **Solution**: Set environment variable: `export ANTHROPIC_API_KEY="your-api-key-here"`
   - **Check**: Verify key is set: `echo $ANTHROPIC_API_KEY`

4. **RMCP Communication Issues**:
   - **Cause**: RMCP communication problem between client and server
   - **Solution**: Enable debug logging: `RUST_LOG=debug cargo run --bin agent-eth-client`
   - **Check**: Look for RMCP request/response debug messages in server logs

5. **Build errors**: 
   - **Cause**: Incompatible Rust version or missing dependencies
   - **Solution**: Ensure Rust 1.70+ is installed: `rustup update`

## Future Enhancements

- Enhanced ERC-20 token support with automatic contract discovery
- Multi-chain support (Polygon, Arbitrum, Base)
- Advanced DeFi integrations (Uniswap V3, V4, Aave, Compound)
- Real-time price feeds and portfolio analytics
- Transaction simulation and MEV analysis
- Advanced RAG system with more documentation sources