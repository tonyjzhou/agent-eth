use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecutionPlan {
    pub reasoning: String,
    pub steps: Vec<ExecutionStep>,
    pub requires_confirmation: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecutionStep {
    pub step_id: String,
    pub description: String,
    pub tool_name: String,
    pub parameters: serde_json::Value,
    pub depends_on: Option<String>,
}

#[derive(Debug)]
pub struct ExecutionResult {
    pub success: bool,
    pub results: Vec<String>,
    pub errors: Vec<String>,
}

#[async_trait]
pub trait ToolAgent {
    async fn plan_execution(&mut self, request: &str) -> Result<ExecutionPlan>;
    async fn execute_plan(&mut self, plan: ExecutionPlan) -> Result<ExecutionResult>;
    async fn execute_request(&mut self, request: &str) -> Result<ExecutionResult> {
        let plan = self.plan_execution(request).await?;
        self.execute_plan(plan).await
    }
}
