# Agent-ETH: AI Agent for Ethereum Blockchain

An AI-powered agent system that allows users to interact with Ethereum blockchain using natural language commands. Features a CLI client with Anthropic Claude integration and an HTTP server exposing Ethereum tools.

## Architecture

```
             ┌─────────────────┐   JSON/HTTP API   ┌──────────────────┐
             │   CLI Client    │◄──────────────────►│   HTTP Server    │
             │                 │                    │                  │
             ├─────────────────┤                    ├──────────────────┤
User   ◄───► │ • Interactive   │                    │ • Balance Tool   │
Claude ◄───► │   REPL          │                    │ • Transfer Tool  │
             │ • Claude API    │                    │ • Contract Tool  │
             │ • RAG System    │                    │ • Swap Tool      │
             │ • Account       │                    │ • Alloy Provider │
             │   Aliases       │                    │ • External APIs  │
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
   - `./target/release/agent-eth-server` (HTTP Server)
   - `./target/release/agent-eth-client` (CLI Client)

## Usage

1. **Start the Anvil fork** (in a separate terminal):
   ```bash
   anvil --fork-url https://eth-mainnet.g.alchemy.com/v2/4UjEl1ULr2lQYsGR5n7gGKd3pzgAzxKs
   ```

2. **Start the HTTP server** (in another terminal):
   ```bash
   cd server && cargo run
   ```

3. **Run the client**:
   ```bash
   cd client && ANTHROPIC_API_KEY="your-key" cargo run
   ```

3. **Example Commands**:
   ```
   eth> send 1 ETH from Alice to Bob
   eth> How much ETH does Alice have?
   eth> Is Uniswap V2 Router deployed at 0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D?
   eth> What's Bob's balance?
   eth> Transfer 0.5 ETH from Carol to Alice
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
│       ├── agent.rs        # Claude API integration
│       ├── mcp_client.rs   # HTTP client for server communication
│       └── rag/            # RAG system implementation
│           ├── mod.rs      # RAG system core
│           ├── embeddings.rs # Embedding service
│           ├── ingestion.rs  # Document ingestion
│           └── storage.rs  # Vector storage
├── server/                 # HTTP Server (Axum-based)
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs         # HTTP server with endpoints
│       ├── provider.rs     # Ethereum provider (Alloy)
│       └── tools/          # Blockchain tools
│           ├── balance.rs    # Balance checking
│           ├── transfer.rs   # ETH transfers
│           ├── contract_check.rs # Contract verification
│           └── swap.rs       # Token swaps
└── README.md
```

### Running in Development

1. **Terminal 1** - Start Anvil:
   ```bash
   anvil --fork-url https://eth-mainnet.g.alchemy.com/v2/4UjEl1ULr2lQYsGR5n7gGKd3pzgAzxKs
   ```

2. **Terminal 2** - Start the HTTP server:
   ```bash
   cd server
   cargo run
   ```

3. **Terminal 3** - Run the client:
   ```bash
   cd client
   ANTHROPIC_API_KEY="your-key" cargo run
   ```

### Environment Variables

- `ANTHROPIC_API_KEY` - Required. Your Anthropic API key
- `RPC_URL` - Optional. Ethereum RPC URL (defaults to `http://127.0.0.1:8545`)
- `OPENAI_API_KEY` - Optional. OpenAI API key for enhanced embeddings (falls back to basic embeddings)

## Technical Details

### Components

1. **CLI Client**:
   - Interactive REPL using rustyline with colored output
   - Direct Anthropic Claude API integration for natural language parsing
   - Account alias resolution (Alice, Bob, Carol → hex addresses)
   - HTTP client for communicating with server
   - RAG system for documentation search with vector embeddings

2. **HTTP Server**:
   - Axum-based HTTP server exposing Ethereum tools
   - Uses alloy for Ethereum blockchain interactions
   - Four main endpoints: /balance, /transfer, /contract_check, /swap
   - External API integration for contract discovery and token pricing
   - Connects to local Anvil fork on http://127.0.0.1:8545

3. **RAG System**:
   - Vector database for document storage and similarity search
   - OpenAI embeddings API for enhanced text understanding
   - Document ingestion from markdown files
   - Natural language querying of technical documentation

4. **Communication**:
   - Client and server communicate via JSON over HTTP
   - RESTful endpoints: GET /balance, POST /transfer, POST /contract_check, POST /swap
   - Async/await throughout for non-blocking operations
   - Error handling with HTTP status codes and JSON error responses

### Security Notes

- Private keys are hardcoded for test accounts only
- Never use these private keys on mainnet
- The system is designed for local development and testing

## Communication Protocol Analysis

Agent-ETH uses JSON over HTTP for client-server communication instead of the Model Context Protocol (MCP). Here's a detailed analysis of both approaches:

### Current Approach: JSON over HTTP

**Advantages:**
- **Simplicity**: Direct HTTP endpoints are easier to understand and debug
- **Language Agnostic**: Any HTTP client can interact with the server
- **Familiar Patterns**: Standard REST-like API that most developers know
- **Debugging**: Easy to test with curl, Postman, or browser dev tools
- **Lightweight**: No additional protocol overhead or complexity
- **Custom Control**: Full control over request/response format and error handling

**Disadvantages:**
- **No Standardization**: Custom protocol means reinventing conventions
- **Limited Discoverability**: No built-in way for clients to discover available tools
- **Manual Schema Management**: Need to manually keep client/server schemas in sync
- **No Metadata**: Missing rich tool descriptions, parameter validation, etc.
- **Scaling Complexity**: Adding new tools requires manual updates across multiple files

### Alternative: Model Context Protocol (MCP)

**Advantages:**
- **Standardized**: Well-defined protocol with established patterns
- **Tool Discovery**: Automatic discovery of available tools and their schemas
- **Rich Metadata**: Built-in support for tool descriptions, parameter validation
- **Ecosystem**: Can leverage existing MCP tooling and libraries
- **Type Safety**: Better schema validation and error handling
- **Bidirectional Communication**: Support for notifications and streaming

**Disadvantages:**
- **Complexity**: More complex to implement and understand
- **Learning Curve**: Developers need to learn MCP-specific concepts
- **Overhead**: Additional protocol layer adds complexity
- **Debugging**: Harder to debug without MCP-specific tools
- **Dependencies**: Requires MCP libraries and tooling

### Design Decision Context

For Agent-ETH's use case (simple blockchain operations with 4 endpoints), the HTTP approach makes sense for:
- **Rapid Prototyping**: Quick to implement and iterate
- **Educational Purposes**: Easy to understand the communication flow
- **Minimal Dependencies**: Fewer external libraries required
- **Development Speed**: Faster initial development cycle

However, as the system scales with more tools and features, MCP's standardization and discoverability benefits would become increasingly valuable for maintainability and extensibility.

## Troubleshooting

1. **"Connection refused"**: Make sure Anvil is running on `127.0.0.1:8545`
2. **"Invalid API key"**: Check your `ANTHROPIC_API_KEY` environment variable
3. **Build errors**: Ensure Rust 1.70+ is installed

## Future Enhancements

- Token balance queries (ERC-20)
- Multi-chain support
- Advanced DeFi integrations (Uniswap V3, V4)
- Real-time price feeds and analytics
- Transaction simulation and analysis