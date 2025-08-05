# Agent-ETH: AI Agent for Ethereum Blockchain

An AI-powered agent system that allows users to interact with Ethereum blockchain using natural language commands. Built with RIG framework for AI agents and Model Context Protocol (MCP) for tool integration.

## Architecture

```
             ┌─────────────────┐    MCP Protocol    ┌──────────────────┐
             │   RIG Agent     │◄──────────────────►│   MCP Server     │
             │   (Client)      │                    │                  │
             ├─────────────────┤                    ├──────────────────┤
User   ◄───► │ • CLI REPL      │                    │ • Foundry - Cast │
Claude ◄───► │ • LLM API Key   │                    │ • Tx Generation  │
             │ • User Input    │                    │ • State Fork     │
             │ • Response      │                    │ • Anthropic SDK  │
             └─────────────────┘                    └──────────────────┘
                      │                                       │
                      │                                       │
                      └───────────────┐           ┌───────────┘
                                      │           │
                                 ┌────▼───────────▼──────┐
                                 │   Forked Ethereum     │
                                 │     Test Network      │
                                 │   (via Foundry)       │
                                 └───────────────────────┘
```

## Features

- **Natural Language Processing**: Interact with Ethereum using plain English
- **Account Aliases**: Use friendly names (Alice, Bob, Carol) instead of hex addresses
- **Balance Queries**: Check ETH balances for any address
- **ETH Transfers**: Send ETH between accounts with automatic transaction handling
- **Contract Verification**: Check if contracts are deployed at specific addresses
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
   - `./target/release/agent-eth-server` (MCP Server)
   - `./target/release/agent-eth-client` (CLI Client)

## Usage

1. **Start the Anvil fork** (in a separate terminal):
   ```bash
   anvil --fork-url https://eth-mainnet.g.alchemy.com/v2/4UjEl1ULr2lQYsGR5n7gGKd3pzgAzxKs
   ```

2. **Run the client**:
   ```bash
   ./target/release/agent-eth-client
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

### System Commands
- `help` - Show available commands
- `exit` or `quit` - Exit the program

## Development

### Project Structure

```
agent-eth/
├── Cargo.toml              # Workspace configuration
├── client/                 # RIG Agent Client
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs         # CLI REPL interface
│       ├── agent.rs        # RIG agent implementation
│       └── mcp_client.rs   # MCP client
├── server/                 # MCP Server
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs         # MCP server main
│       ├── provider.rs     # Ethereum provider
│       └── tools/          # MCP tools
│           ├── balance.rs
│           ├── transfer.rs
│           └── contract_check.rs
└── README.md
```

### Running in Development

1. **Terminal 1** - Start Anvil:
   ```bash
   anvil --fork-url https://eth-mainnet.g.alchemy.com/v2/4UjEl1ULr2lQYsGR5n7gGKd3pzgAzxKs
   ```

2. **Terminal 2** - Run the client:
   ```bash
   cd client
   ANTHROPIC_API_KEY="your-key" cargo run
   ```

### Environment Variables

- `ANTHROPIC_API_KEY` - Required. Your Anthropic API key
- `RPC_URL` - Optional. Ethereum RPC URL (defaults to `http://127.0.0.1:8545`)

## Technical Details

### Components

1. **RIG Agent (Client)**:
   - Uses RIG framework for AI agent functionality
   - Integrates with Anthropic Claude API
   - Parses natural language into structured commands
   - Handles account aliases and address resolution

2. **MCP Server**:
   - Implements Model Context Protocol for tool exposure
   - Uses ethers-rs for Ethereum interactions
   - Provides tools for balance, transfer, and contract checking
   - Connects to local Anvil fork

3. **Communication**:
   - Client and server communicate via MCP over stdio
   - JSON-RPC protocol for tool calls
   - Async/await throughout for non-blocking operations

### Security Notes

- Private keys are hardcoded for test accounts only
- Never use these private keys on mainnet
- The system is designed for local development and testing

## Troubleshooting

1. **"Connection refused"**: Make sure Anvil is running on `127.0.0.1:8545`
2. **"Invalid API key"**: Check your `ANTHROPIC_API_KEY` environment variable
3. **Build errors**: Ensure Rust 1.70+ is installed

## Future Enhancements

- Token balance queries (ERC-20)
- Uniswap swap integration
- Multi-chain support
- RAG system for documentation queries
- Web search integration for contract addresses