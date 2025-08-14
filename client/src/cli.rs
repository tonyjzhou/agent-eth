use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "agent-eth-client")]
#[command(about = "An AI agent for interacting with Ethereum blockchain")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[arg(long, default_value = "./target/debug/agent-eth-server")]
    pub server_command: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Check ETH or token balance for an address
    Balance {
        /// Address to check balance for (hex address or Alice/Bob/Carol)
        address: String,
        /// Token symbol to check (default: ETH)
        #[arg(short, long)]
        token: Option<String>,
    },

    /// Send ETH transfer between accounts
    Transfer {
        /// Amount to transfer (in ETH)
        amount: String,
        /// Sender address (hex address or Alice/Bob/Carol)
        #[arg(short, long)]
        from: String,
        /// Recipient address (hex address or Alice/Bob/Carol)
        #[arg(short, long)]
        to: String,
    },

    /// Check if a contract is deployed at an address
    #[command(name = "contract-check")]
    ContractCheck {
        /// Contract address to check
        address: String,
    },

    /// Swap tokens using Uniswap V2 Router
    Swap {
        /// Amount of input token to swap
        amount: String,
        /// Input token symbol (e.g., ETH, USDC)
        token_in: String,
        /// Output token symbol (e.g., USDC, WETH)
        token_out: String,
        /// Account to execute swap from (hex address or Alice/Bob/Carol)
        #[arg(short, long)]
        from: String,
        /// Slippage tolerance in percentage (default: 2.0)
        #[arg(short, long, default_value = "2.0")]
        slippage: f64,
    },

    /// Search documentation using RAG system
    Docs {
        /// Query to search for in documentation
        query: String,
    },

    /// Ingest documentation into RAG system
    Ingest {
        /// Directory path containing documentation
        directory: String,
    },

    /// Clear all ingested documents from RAG system
    Clear,

    /// Start interactive REPL mode (legacy interface)
    Interactive,

    /// Execute AI agent command with natural language
    Agent {
        /// Natural language command for the AI agent
        command: String,
    },
}
