use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Write;

pub mod balance;
pub mod contract_check;
pub mod swap;
pub mod transfer;

/// Core trait for all blockchain tools
#[async_trait]
pub trait Tool: Send + Sync {
    /// Input parameters for the tool
    type Input: for<'de> Deserialize<'de> + Send + Sync;
    /// Output result from the tool
    type Output: Serialize + Send + Sync;

    /// Unique tool identifier
    fn name(&self) -> &'static str;
    /// Human-readable description for AI agent
    fn description(&self) -> &'static str;
    /// JSON schema for input parameters
    fn input_schema(&self) -> serde_json::Value;

    /// Execute the tool with typed parameters
    async fn execute(&self, input: Self::Input) -> Result<Self::Output>;
}

/// Tool registry with strongly typed tool definitions
pub struct TypedToolRegistry {
    tools: HashMap<String, Box<dyn Tool<Input = serde_json::Value, Output = serde_json::Value>>>,
}

impl Default for TypedToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for TypedToolRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypedToolRegistry")
            .field("tools", &self.tools.keys().collect::<Vec<_>>())
            .finish()
    }
}

impl TypedToolRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            tools: HashMap::new(),
        };

        // Register all blockchain tools
        registry.register(Box::new(balance::BalanceTool));
        registry.register(Box::new(transfer::TransferTool));
        registry.register(Box::new(contract_check::ContractCheckTool));
        registry.register(Box::new(swap::SwapTool));

        registry
    }

    /// Register a new tool
    pub fn register<T>(&mut self, tool: Box<T>)
    where
        T: Tool + 'static,
        T::Input: for<'de> Deserialize<'de> + Send + Sync + 'static,
        T::Output: Serialize + Send + Sync + 'static,
    {
        let name = tool.name().to_string();

        // Type-erased wrapper to store different tool types
        let wrapped_tool = TypeErasedTool { inner: tool };

        self.tools.insert(name, Box::new(wrapped_tool));
    }

    /// Get tool by name
    #[allow(dead_code)]
    pub fn get_tool(
        &self,
        name: &str,
    ) -> Option<&dyn Tool<Input = serde_json::Value, Output = serde_json::Value>> {
        self.tools.get(name).map(|tool| tool.as_ref())
    }

    /// Get all tool names
    #[allow(dead_code)]
    pub fn tool_names(&self) -> Vec<String> {
        self.tools.keys().cloned().collect()
    }

    /// Get tools description for AI agent
    #[allow(dead_code)]
    pub fn get_tools_description(&self) -> String {
        let mut description = String::new();
        description.push_str("Available blockchain tools:\n");

        for tool in self.tools.values() {
            write!(description, "\n- {}: {}\n", tool.name(), tool.description())
                .expect("String formatting should not fail");
            writeln!(description, "  Schema: {}", tool.input_schema())
                .expect("String formatting should not fail");
        }

        description
    }

    /// Execute tool with JSON parameters
    #[allow(dead_code)]
    pub async fn execute_tool(
        &self,
        name: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let tool = self
            .get_tool(name)
            .ok_or_else(|| anyhow::anyhow!("Tool '{}' not found", name))?;

        tool.execute(params).await
    }
}

/// Type-erased wrapper to store different tool types in the same collection
struct TypeErasedTool<T: Tool> {
    inner: Box<T>,
}

#[async_trait]
impl<T> Tool for TypeErasedTool<T>
where
    T: Tool + Send + Sync,
    T::Input: for<'de> Deserialize<'de> + Send + Sync,
    T::Output: Serialize + Send + Sync,
{
    type Input = serde_json::Value;
    type Output = serde_json::Value;

    fn name(&self) -> &'static str {
        self.inner.name()
    }

    fn description(&self) -> &'static str {
        self.inner.description()
    }

    fn input_schema(&self) -> serde_json::Value {
        self.inner.input_schema()
    }

    async fn execute(&self, input: Self::Input) -> Result<Self::Output> {
        // Deserialize JSON input to the concrete type
        let typed_input: T::Input = serde_json::from_value(input)?;

        // Execute with typed parameters
        let output = self.inner.execute(typed_input).await?;

        // Serialize output back to JSON
        Ok(serde_json::to_value(output)?)
    }
}

/// Helper macro to generate JSON schema for structs
#[macro_export]
macro_rules! json_schema {
    ($struct_name:ty, { $($field:ident: $field_type:expr, $required:expr, $description:expr),* $(,)? }) => {
        {
            let mut required_fields = Vec::new();
            $(
                if $required {
                    required_fields.push(stringify!($field));
                }
            )*

            serde_json::json!({
                "type": "object",
                "properties": {
                    $(
                        stringify!($field): {
                            "type": $field_type,
                            "description": $description
                        }
                    ),*
                },
                "required": required_fields
            })
        }
    };
}
