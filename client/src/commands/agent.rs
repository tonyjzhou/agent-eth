use crate::agent::AgentCore;
use crate::mcp_client::McpClient;
use anyhow::Result;
use colored::*;
use inquire::Confirm;

pub struct AgentCommand;

impl AgentCommand {
    pub async fn execute(
        command: &str,
        agent_core: &mut AgentCore,
        mcp_client: &mut McpClient,
    ) -> Result<()> {
        println!(
            "{} {}",
            "🤖 AI Agent analyzing:".bright_blue(),
            command.italic()
        );

        // Plan the execution
        let plan = agent_core.plan_execution(command).await?;

        // Show the plan to user
        println!("{}", "📋 Execution Plan:".bright_green().bold());
        println!("💭 {}", plan.reasoning.bright_white());
        println!();

        for (i, step) in plan.steps.iter().enumerate() {
            println!(
                "{}. {} ({})",
                (i + 1).to_string().bright_cyan(),
                step.description.bright_white(),
                step.tool_name.bright_black()
            );
        }
        println!();

        // Ask for confirmation if needed
        if plan.requires_confirmation {
            println!(
                "{}",
                "⚠️ This operation requires confirmation."
                    .bright_yellow()
                    .bold()
            );

            let confirm = Confirm::new("Continue with this operation?")
                .with_default(false)
                .with_help_message("This operation may involve real blockchain transactions")
                .prompt()?;

            if !confirm {
                println!("{}", "❌ Operation cancelled".bright_red());
                return Ok(());
            }
        }

        // Execute the plan
        println!("{}", "⚡ Executing plan...".bright_blue().bold());
        let results = agent_core.execute_plan(plan, mcp_client).await?;

        // Show results
        println!("{}", "🎉 Results:".bright_green().bold());
        for result in results {
            println!("  {result}");
        }
        println!();

        Ok(())
    }
}
