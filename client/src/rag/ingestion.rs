use crate::rag::{DocumentMetadata, RagSystem};
use anyhow::Result;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub struct DocumentIngestion;

impl DocumentIngestion {
    pub fn new() -> Self {
        Self
    }

    pub async fn ingest_directory(&self, dir_path: &str, rag: &mut RagSystem) -> Result<usize> {
        let mut ingested_count = 0;

        for entry in WalkDir::new(dir_path) {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some("md" | "txt" | "sol" | "rs") = path.extension().and_then(|s| s.to_str())
                {
                    if let Ok(document) = self.process_file(path).await {
                        rag.add_document(document).await?;
                        ingested_count += 1;
                    }
                }
            }
        }

        Ok(ingested_count)
    }

    async fn process_file(&self, path: &Path) -> Result<DocumentMetadata> {
        let content = fs::read_to_string(path)?;
        let file_name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        let doc_type = match path.extension().and_then(|s| s.to_str()) {
            Some("md") => "documentation",
            Some("sol") => "contract",
            Some("rs") => "source",
            Some("txt") => "text",
            _ => "unknown",
        }
        .to_string();

        // Create chunks for large documents
        let chunks = self.chunk_text(&content, 1000);

        // For now, we'll use the first chunk or full content if small
        let content_to_embed = if chunks.len() > 1 {
            chunks[0].clone()
        } else {
            content.clone()
        };

        let document = DocumentMetadata {
            id: format!("{}_{}", file_name, chrono::Utc::now().timestamp()),
            title: self.extract_title(&content, &file_name),
            source: path.to_string_lossy().to_string(),
            doc_type,
            content: content_to_embed,
        };

        Ok(document)
    }

    fn chunk_text(&self, text: &str, max_chars: usize) -> Vec<String> {
        if text.len() <= max_chars {
            return vec![text.to_string()];
        }

        let mut chunks = Vec::new();
        let mut current_chunk = String::new();

        for line in text.lines() {
            if current_chunk.len() + line.len() + 1 > max_chars && !current_chunk.is_empty() {
                chunks.push(current_chunk.trim().to_string());
                current_chunk = String::new();
            }
            current_chunk.push_str(line);
            current_chunk.push('\n');
        }

        if !current_chunk.trim().is_empty() {
            chunks.push(current_chunk.trim().to_string());
        }

        chunks
    }

    fn extract_title(&self, content: &str, file_name: &str) -> String {
        // Try to extract title from markdown headers
        for line in content.lines().take(10) {
            let line = line.trim();
            if let Some(stripped) = line.strip_prefix("# ") {
                return stripped.trim().to_string();
            }
        }

        // Fallback to filename without extension
        Path::new(file_name)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(file_name)
            .to_string()
    }
}
