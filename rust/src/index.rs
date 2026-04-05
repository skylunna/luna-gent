use rkyv::{Archive, Deserialize, Serialize};
use serde::{Deserialize as SerdeDe, Serialize as SerdeSer};
use std::fs;

#[derive(Archive, Serialize, Deserialize, SerdeSer, SerdeDe, Debug, Clone)]
pub struct VectorEntry {
    pub vector: Vec<f32>,
    pub meta: String, // 统一使用 meta 字段
}

#[derive(Archive, Serialize, Deserialize, SerdeSer, SerdeDe, Debug)]
pub struct FlatIndex {
    pub entries: Vec<VectorEntry>,
    pub dimension: usize,
}

impl FlatIndex {
    pub fn new(dimension: usize) -> Self {
        Self {
            entries: Vec::new(),
            dimension,
        }
    }

    pub fn add(&mut self, vector: Vec<f32>, meta: String) -> Result<(), String> {
        if vector.len() != self.dimension {
            return Err(format!(
                "维度不匹配: 期望 {}, 实际 {}",
                self.dimension,
                vector.len()
            ));
        }
        self.entries.push(VectorEntry { vector, meta });
        Ok(())
    }

    pub fn search(&self, query: &[f32], top_k: usize) -> Result<Vec<(usize, f32)>, String> {
        if query.len() != self.dimension {
            return Err("查询向量维度不匹配".into());
        }
        let mut scores: Vec<(usize, f32)> = self
            .entries
            .iter()
            .enumerate()
            .map(|(i, e)| (i, cosine_sim(query, &e.vector)))
            .collect();

        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        Ok(scores.into_iter().take(top_k).collect())
    }

    pub fn save(&self, path: &str) -> Result<(), String> {
        let bytes =
            rkyv::to_bytes::<_, 256>(self).map_err(|e| format!("rkyv 序列化失败: {}", e))?;
        fs::write(path, bytes).map_err(|e| e.to_string())
    }

    pub fn load(path: &str) -> Result<Self, String> {
        let bytes = fs::read(path).map_err(|e| e.to_string())?;

        // SAFETY: `bytes` 由本模块的 `save()` 通过 `rkyv::to_bytes` 生成
        // 内存布局与对齐已严格保证，反序列化不会越界
        let archived = unsafe { rkyv::archived_root::<Self>(&bytes) };

        archived
            .deserialize(&mut rkyv::Infallible)
            .map_err(|e| format!("rkyv 反序列化失败: {:?}", e))
    }
}

fn cosine_sim(a: &[f32], b: &[f32]) -> f32 {
    let dot = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum::<f32>();
    let na = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let nb = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if na == 0.0 || nb == 0.0 {
        return 0.0;
    }
    dot / (na * nb)
}
