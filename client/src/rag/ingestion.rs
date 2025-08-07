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
        // First pass: collect all documents and chunks
        let mut all_documents = Vec::new();
        let mut all_chunk_contents = Vec::new();

        for entry in WalkDir::new(dir_path) {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some("md" | "txt" | "sol" | "rs") = path.extension().and_then(|s| s.to_str())
                {
                    if let Ok(documents) = self.prepare_file_documents(path).await {
                        for doc in documents {
                            all_chunk_contents.push(doc.content.clone());
                            all_documents.push(doc);
                        }
                    }
                }
            }
        }

        if all_documents.is_empty() {
            return Ok(0);
        }

        let unique_files: std::collections::HashSet<&String> =
            all_documents.iter().map(|d| &d.source).collect();
        println!(
            "📊 Prepared {} chunks from {} files. Generating embeddings in batches...",
            all_documents.len(),
            unique_files.len()
        );

        // Second pass: generate embeddings in batches
        let embeddings = rag.generate_embeddings_batch(&all_chunk_contents).await?;

        // Third pass: store everything in database
        let total_docs = all_documents.len();
        println!("💾 Storing {total_docs} documents in database...");
        let mut ingested_count = 0;
        for (document, embedding) in all_documents.into_iter().zip(embeddings.into_iter()) {
            rag.store_document_with_embedding(document, embedding)
                .await?;
            ingested_count += 1;

            if ingested_count % 50 == 0 {
                print!("✅ Stored {ingested_count}/{total_docs} documents\r");
                std::io::Write::flush(&mut std::io::stdout()).unwrap();
            }
        }
        println!("✅ Stored {ingested_count} documents");

        Ok(ingested_count)
    }

    async fn prepare_file_documents(&self, path: &Path) -> Result<Vec<DocumentMetadata>> {
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

        let title = self.extract_title(&content, &file_name);
        let source_path = path.to_string_lossy().to_string();

        // Create chunks for documents
        let chunks = self.chunk_text(&content, 1000);
        let mut documents = Vec::new();

        // Process ALL chunks, not just the first one
        for (chunk_index, chunk_content) in chunks.iter().enumerate() {
            let document = DocumentMetadata {
                id: format!(
                    "{}_chunk_{}__{}",
                    file_name,
                    chunk_index,
                    chrono::Utc::now().timestamp()
                ),
                title: if chunks.len() == 1 {
                    title.clone()
                } else {
                    format!("{} (part {}/{})", title, chunk_index + 1, chunks.len())
                },
                source: source_path.clone(),
                doc_type: doc_type.clone(),
                content: chunk_content.clone(),
            };

            documents.push(document);
        }

        Ok(documents)
    }

    fn chunk_text(&self, text: &str, max_chars: usize) -> Vec<String> {
        if text.len() <= max_chars {
            return vec![text.to_string()];
        }

        // First, try to split on markdown sections (## headers)
        let section_chunks = self.split_by_sections(text, max_chars);
        if section_chunks.len() > 1 {
            return section_chunks;
        }

        // If no sections, split by paragraphs (double newlines)
        let paragraph_chunks = self.split_by_paragraphs(text, max_chars);
        if paragraph_chunks.len() > 1 {
            return paragraph_chunks;
        }

        // Fallback to line-based chunking
        self.split_by_lines(text, max_chars)
    }

    fn split_by_sections(&self, text: &str, max_chars: usize) -> Vec<String> {
        let mut chunks = Vec::new();
        let mut current_chunk = String::new();

        for line in text.lines() {
            let line_with_newline = format!("{line}\n");

            // Check if this is a section header (## or ###, but not #)
            let is_section_header =
                line.trim_start().starts_with("## ") || line.trim_start().starts_with("### ");

            // If we hit a section header and current chunk would be too big, start new chunk
            if is_section_header
                && current_chunk.len() + line_with_newline.len() > max_chars
                && !current_chunk.trim().is_empty()
            {
                chunks.push(current_chunk.trim().to_string());
                current_chunk = String::new();
            }

            current_chunk.push_str(&line_with_newline);

            // If current chunk is getting too big, try to finish it at next section
            if current_chunk.len() > max_chars * 2 {
                chunks.push(current_chunk.trim().to_string());
                current_chunk = String::new();
            }
        }

        if !current_chunk.trim().is_empty() {
            chunks.push(current_chunk.trim().to_string());
        }

        chunks
    }

    fn split_by_paragraphs(&self, text: &str, max_chars: usize) -> Vec<String> {
        let paragraphs: Vec<&str> = text.split("\n\n").collect();
        let mut chunks = Vec::new();
        let mut current_chunk = String::new();

        for paragraph in paragraphs {
            let paragraph_with_breaks = format!("{paragraph}\n\n");

            if current_chunk.len() + paragraph_with_breaks.len() > max_chars
                && !current_chunk.trim().is_empty()
            {
                chunks.push(current_chunk.trim().to_string());
                current_chunk = String::new();
            }

            current_chunk.push_str(&paragraph_with_breaks);
        }

        if !current_chunk.trim().is_empty() {
            chunks.push(current_chunk.trim().to_string());
        }

        chunks
    }

    fn split_by_lines(&self, text: &str, max_chars: usize) -> Vec<String> {
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
