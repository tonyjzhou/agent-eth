use crate::agent::EthereumAgent;
use anyhow::Result;
use colored::*;

pub struct DocsCommand;

impl DocsCommand {
    pub async fn search(query: &str, agent: &EthereumAgent) -> Result<()> {
        println!(
            "{} {}",
            "🔍 Searching docs for:".bright_blue(),
            query.italic()
        );

        let answer = agent.answer_documentation_query(query).await?;
        println!("{} {}", "📖".bright_blue(), answer);
        println!();

        Ok(())
    }

    pub async fn ingest(directory: &str, agent: &mut EthereumAgent) -> Result<()> {
        println!(
            "{} {}",
            "📚 Ingesting documents from:".bright_blue(),
            directory
        );

        let count = agent.ingest_documents(directory).await?;
        println!("{} Ingested {} documents", "✅".bright_green(), count);
        println!();

        Ok(())
    }

    pub async fn clear(agent: &mut EthereumAgent) -> Result<()> {
        println!(
            "{}",
            "🗑️  Clearing all ingested documents...".bright_yellow()
        );

        agent.clear_documents().await?;
        println!(
            "{} All documents cleared from RAG system",
            "✅".bright_green()
        );
        println!();

        Ok(())
    }
}
