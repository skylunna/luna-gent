use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Debug)]
pub struct Chunk {
    pub id: usize,
    pub content: String,
    pub meta: HashMap<String, String>,
}

/// MVP 分块策略：按段落切割，累积超过 chunk_size 则截断为新块
pub fn parse_and_chunk(path: &str, chunk_size: usize) -> Result<Vec<Chunk>, String> {
    let content =
        std::fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;

    let paragraphs: Vec<&str> = content.split("\n\n").collect();
    let mut chunks = Vec::new();
    let mut current_text = String::new();
    let mut id = 0;

    for para in paragraphs {
        let trimmed = para.trim();
        if trimmed.is_empty() {
            continue;
        }

        if current_text.len() + trimmed.len() > chunk_size && !current_text.is_empty() {
            chunks.push(Chunk {
                id,
                content: current_text.clone(),
                meta: HashMap::from([("source".to_string(), path.to_string())]),
            });
            id += 1;
            current_text = String::new();
        }
        if !current_text.is_empty() {
            current_text.push_str("\n\n");
        }
        current_text.push_str(trimmed);
    }
    if !current_text.is_empty() {
        chunks.push(Chunk {
            id,
            content: current_text,
            meta: HashMap::from([("source".to_string(), path.to_string())]),
        });
    }

    Ok(chunks)
}
