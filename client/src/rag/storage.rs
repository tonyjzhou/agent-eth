use crate::rag::{DocumentMetadata, SearchResult};
use anyhow::Result;
use rusqlite::{params, Connection};

#[derive(Debug)]
pub struct VectorStorage {
    conn: Connection,
}

impl VectorStorage {
    pub async fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;

        // Create tables
        conn.execute(
            "CREATE TABLE IF NOT EXISTS documents (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                source TEXT NOT NULL,
                doc_type TEXT NOT NULL,
                content TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS embeddings (
                document_id TEXT PRIMARY KEY,
                embedding BLOB NOT NULL,
                FOREIGN KEY(document_id) REFERENCES documents(id)
            )",
            [],
        )?;

        Ok(Self { conn })
    }

    pub async fn insert_document(
        &mut self,
        document: DocumentMetadata,
        embedding: Vec<f32>,
    ) -> Result<()> {
        // Insert document metadata
        self.conn.execute(
            "INSERT OR REPLACE INTO documents (id, title, source, doc_type, content) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![document.id, document.title, document.source, document.doc_type, document.content],
        )?;

        // Serialize embedding as bytes
        let embedding_bytes: Vec<u8> = embedding.iter().flat_map(|f| f.to_le_bytes()).collect();

        // Insert embedding
        self.conn.execute(
            "INSERT OR REPLACE INTO embeddings (document_id, embedding) VALUES (?1, ?2)",
            params![document.id, embedding_bytes],
        )?;

        Ok(())
    }

    pub async fn similarity_search(
        &self,
        query_embedding: &[f32],
        limit: usize,
    ) -> Result<Vec<SearchResult>> {
        let mut stmt = self.conn.prepare(
            "SELECT d.id, d.title, d.source, d.doc_type, d.content, e.embedding 
             FROM documents d 
             JOIN embeddings e ON d.id = e.document_id",
        )?;

        let document_iter = stmt.query_map([], |row| {
            let id: String = row.get(0)?;
            let title: String = row.get(1)?;
            let source: String = row.get(2)?;
            let doc_type: String = row.get(3)?;
            let content: String = row.get(4)?;
            let embedding_bytes: Vec<u8> = row.get(5)?;

            Ok((
                DocumentMetadata {
                    id,
                    title,
                    source,
                    doc_type,
                    content,
                },
                embedding_bytes,
            ))
        })?;

        let mut scored_results = Vec::new();

        for document_result in document_iter {
            let (document, embedding_bytes) = document_result?;

            // Deserialize embedding
            let embedding: Vec<f32> = embedding_bytes
                .chunks(4)
                .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
                .collect();

            // Calculate cosine similarity
            let score = self.cosine_similarity(query_embedding, &embedding);
            scored_results.push((document, score));
        }

        // Sort by score (descending) and take top results
        // Handle NaN scores by treating them as lowest priority
        scored_results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Less));
        scored_results.truncate(limit);

        let search_results = scored_results
            .into_iter()
            .map(|(document, _score)| {
                // For chunks, use the entire content since they're already reasonably sized
                // Add score information to help with debugging
                let relevant_chunk = if document.content.len() > 2000 {
                    // If somehow we have a very large chunk, truncate intelligently
                    let truncated = &document.content[..2000];
                    // Try to end at a sentence or paragraph boundary
                    if let Some(last_period) = truncated.rfind(". ") {
                        format!("{}.", &truncated[..last_period])
                    } else if let Some(last_newline) = truncated.rfind("\n\n") {
                        truncated[..last_newline].to_string()
                    } else {
                        format!("{truncated}...")
                    }
                } else {
                    // Use full content for reasonably sized chunks
                    document.content.clone()
                };

                SearchResult {
                    document,
                    relevant_chunk,
                }
            })
            .collect();

        Ok(search_results)
    }

    pub async fn clear_all_documents(&mut self) -> Result<()> {
        self.conn.execute("DELETE FROM embeddings", [])?;
        self.conn.execute("DELETE FROM documents", [])?;
        Ok(())
    }

    fn cosine_similarity(&self, a: &[f32], b: &[f32]) -> f32 {
        let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            0.0
        } else {
            dot_product / (norm_a * norm_b)
        }
    }
}
