use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize)]
struct EmbeddingBatchRequest {
    input: Vec<String>,
    model: String,
    encoding_format: String,
}

#[derive(Debug, Deserialize)]
struct EmbeddingResponse {
    data: Vec<EmbeddingData>,
}

#[derive(Debug, Deserialize)]
struct EmbeddingData {
    embedding: Vec<f32>,
}

#[derive(Debug)]
pub struct EmbeddingService {
    client: Client,
    api_key: String,
    model: String,
}

impl EmbeddingService {
    pub async fn new() -> Result<Self> {
        let api_key = env::var("OPENAI_API_KEY").context(
            "OPENAI_API_KEY environment variable is required for embeddings. Please set it to use the RAG system.",
        )?;

        let client = Client::new();
        let model = "text-embedding-3-small".to_string();

        Ok(Self {
            client,
            api_key,
            model,
        })
    }

    pub async fn embed_text(&self, text: &str) -> Result<Vec<f32>> {
        let embeddings = self.embed_batch(&[text.to_string()]).await?;
        embeddings
            .into_iter()
            .next()
            .context("No embedding returned from batch request")
    }

    pub async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        if texts.is_empty() {
            return Ok(vec![]);
        }

        // OpenAI supports up to 2048 inputs per request for embeddings
        const BATCH_SIZE: usize = 100; // Conservative batch size
        let mut all_embeddings = Vec::new();

        let total_batches = texts.len().div_ceil(BATCH_SIZE);
        println!(
            "🔄 Processing {} texts in {} batches of up to {} items each...",
            texts.len(),
            total_batches,
            BATCH_SIZE
        );

        for (batch_idx, chunk) in texts.chunks(BATCH_SIZE).enumerate() {
            print!(
                "📦 Batch {}/{} ({} texts)... ",
                batch_idx + 1,
                total_batches,
                chunk.len()
            );
            // Ignore stdout flush errors for non-critical logging
            let _ = std::io::Write::flush(&mut std::io::stdout());
            let request = EmbeddingBatchRequest {
                input: chunk.to_vec(),
                model: self.model.clone(),
                encoding_format: "float".to_string(),
            };

            let response = self
                .client
                .post("https://api.openai.com/v1/embeddings")
                .header("Authorization", format!("Bearer {}", self.api_key))
                .header("Content-Type", "application/json")
                .json(&request)
                .send()
                .await
                .context("Failed to send batch embedding request to OpenAI")?;

            if !response.status().is_success() {
                let status = response.status();
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                return Err(anyhow::anyhow!(
                    "OpenAI API request failed with status {}: {}",
                    status,
                    error_text
                ));
            }

            let embedding_response: EmbeddingResponse = response
                .json()
                .await
                .context("Failed to parse embedding response from OpenAI")?;

            for data in embedding_response.data {
                all_embeddings.push(data.embedding);
            }

            println!("✅ Done");

            // Small delay to avoid rate limiting
            if batch_idx + 1 < total_batches {
                tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
            }
        }

        Ok(all_embeddings)
    }
}
