pub mod embeddings;
pub mod ingestion;
pub mod storage;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub id: String,
    pub title: String,
    pub source: String,
    pub doc_type: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub document: DocumentMetadata,
    pub relevant_chunk: String,
}

#[derive(Debug)]
pub struct RagSystem {
    storage: storage::VectorStorage,
    embeddings: embeddings::EmbeddingService,
}

impl Clone for RagSystem {
    fn clone(&self) -> Self {
        // For simplicity, we'll create a new instance
        // This is not ideal for production but works for testing
        panic!("RagSystem cannot be cloned - this is intentional to prevent multiple mutable references")
    }
}

impl RagSystem {
    pub async fn new(db_path: &str) -> Result<Self> {
        let storage = storage::VectorStorage::new(db_path).await?;
        let embeddings = embeddings::EmbeddingService::new().await?;

        Ok(Self {
            storage,
            embeddings,
        })
    }

    pub async fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let query_embedding = self.embeddings.embed_text(query).await?;
        let results = self
            .storage
            .similarity_search(&query_embedding, limit)
            .await?;
        Ok(results)
    }

    pub async fn add_document(&mut self, document: DocumentMetadata) -> Result<()> {
        let embedding = self.embeddings.embed_text(&document.content).await?;
        self.storage.insert_document(document, embedding).await?;
        Ok(())
    }

    pub async fn ingest_directory(&mut self, dir_path: &str) -> Result<usize> {
        let ingestion = ingestion::DocumentIngestion::new();
        ingestion.ingest_directory(dir_path, self).await
    }

    pub async fn clear_documents(&mut self) -> Result<()> {
        self.storage.clear_all_documents().await
    }
}
