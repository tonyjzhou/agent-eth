use thiserror::Error;

/// Structured error types for the agent-eth server
#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum ServerError {
    #[error("Blockchain operation failed: {message}")]
    BlockchainError { message: String },

    #[error("Address validation failed: {address}")]
    InvalidAddress { address: String },

    #[error("Insufficient balance: required {required}, available {available}")]
    InsufficientBalance { required: String, available: String },

    #[error("Contract interaction failed: {message}")]
    ContractError { message: String },

    #[error("Token not found: {token}")]
    TokenNotFound { token: String },

    #[error("Swap operation failed: {message}")]
    SwapError { message: String },

    #[error("API request failed: {service} - {message}")]
    ApiError { service: String, message: String },

    #[error("Provider connection failed: {message}")]
    ProviderError { message: String },

    #[error("Transaction failed: {hash} - {reason}")]
    TransactionError { hash: String, reason: String },

    #[error("Slippage exceeded: expected {expected}, got {actual}")]
    SlippageError { expected: String, actual: String },
}

#[allow(dead_code)]
impl ServerError {
    /// Create a new blockchain error
    pub fn blockchain(message: impl Into<String>) -> Self {
        Self::BlockchainError {
            message: message.into(),
        }
    }

    /// Create a new invalid address error
    pub fn invalid_address(address: impl Into<String>) -> Self {
        Self::InvalidAddress {
            address: address.into(),
        }
    }

    /// Create a new insufficient balance error
    pub fn insufficient_balance(required: impl Into<String>, available: impl Into<String>) -> Self {
        Self::InsufficientBalance {
            required: required.into(),
            available: available.into(),
        }
    }

    /// Create a new contract error
    pub fn contract(message: impl Into<String>) -> Self {
        Self::ContractError {
            message: message.into(),
        }
    }

    /// Create a new token not found error
    pub fn token_not_found(token: impl Into<String>) -> Self {
        Self::TokenNotFound {
            token: token.into(),
        }
    }

    /// Create a new swap error
    pub fn swap(message: impl Into<String>) -> Self {
        Self::SwapError {
            message: message.into(),
        }
    }

    /// Create a new API error
    pub fn api(service: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ApiError {
            service: service.into(),
            message: message.into(),
        }
    }

    /// Create a new provider error
    pub fn provider(message: impl Into<String>) -> Self {
        Self::ProviderError {
            message: message.into(),
        }
    }

    /// Create a new transaction error
    pub fn transaction(hash: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::TransactionError {
            hash: hash.into(),
            reason: reason.into(),
        }
    }

    /// Create a new slippage error
    pub fn slippage(expected: impl Into<String>, actual: impl Into<String>) -> Self {
        Self::SlippageError {
            expected: expected.into(),
            actual: actual.into(),
        }
    }
}

/// Convenient type alias for Results using ServerError
#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, ServerError>;
