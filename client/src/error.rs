use thiserror::Error;

/// Structured error types for the agent-eth client
#[derive(Debug, Error)]
#[allow(dead_code, clippy::enum_variant_names)]
pub enum AgentError {
    #[error("Failed to parse LLM response: {message}")]
    LlmParsingError { message: String },

    #[error("Tool execution failed: {tool_name}: {reason}")]
    ToolExecutionError { tool_name: String, reason: String },

    #[error("MCP communication error: {message}")]
    McpError { message: String },

    #[error("RAG system error: {message}")]
    RagError { message: String },

    #[error("Configuration error: {message}")]
    ConfigError { message: String },

    #[error("Address resolution failed: {address}")]
    AddressResolutionError { address: String },

    #[error("Private key not found for address: {address}")]
    PrivateKeyError { address: String },

    #[error("Command parsing failed: {message}")]
    CommandParsingError { message: String },
}

#[allow(dead_code)]
impl AgentError {
    /// Create a new LLM parsing error with context
    pub fn llm_parsing(message: impl Into<String>) -> Self {
        Self::LlmParsingError {
            message: message.into(),
        }
    }

    /// Create a new tool execution error
    pub fn tool_execution(tool_name: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::ToolExecutionError {
            tool_name: tool_name.into(),
            reason: reason.into(),
        }
    }

    /// Create a new MCP communication error
    pub fn mcp(message: impl Into<String>) -> Self {
        Self::McpError {
            message: message.into(),
        }
    }

    /// Create a new RAG system error
    pub fn rag(message: impl Into<String>) -> Self {
        Self::RagError {
            message: message.into(),
        }
    }

    /// Create a new configuration error
    pub fn config(message: impl Into<String>) -> Self {
        Self::ConfigError {
            message: message.into(),
        }
    }

    /// Create a new address resolution error
    pub fn address_resolution(address: impl Into<String>) -> Self {
        Self::AddressResolutionError {
            address: address.into(),
        }
    }

    /// Create a new private key error
    pub fn private_key(address: impl Into<String>) -> Self {
        Self::PrivateKeyError {
            address: address.into(),
        }
    }

    /// Create a new command parsing error
    pub fn command_parsing(message: impl Into<String>) -> Self {
        Self::CommandParsingError {
            message: message.into(),
        }
    }
}

/// Convenient type alias for Results using AgentError
#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, AgentError>;
