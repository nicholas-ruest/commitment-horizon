use ch_embedding::DIMENSION;
use ch_errors::{HorizonError, Result};
use ch_types::Analogue;
use ruvector_core::types::{DbOptions, DistanceMetric, SearchQuery, VectorEntry};
use ruvector_core::VectorDB;
use std::collections::HashMap;
pub struct AnalogueIndex {
    db: VectorDB,
    len: usize,
}
impl AnalogueIndex {
    pub fn new() -> Result<Self> {
        let db = VectorDB::new(DbOptions {
            dimensions: DIMENSION,
            distance_metric: DistanceMetric::Euclidean,
            storage_path: "memory://commitment-horizon".into(),
            hnsw_config: None,
            quantization: None,
        })
        .map_err(|e| HorizonError::Dependency(format!("RuVector create: {e}")))?;
        Ok(Self { db, len: 0 })
    }
    pub fn insert(&mut self, id: &str, vector: Vec<f32>, outcome: &str) -> Result<()> {
        let mut metadata = HashMap::new();
        metadata.insert("outcome".into(), serde_json::json!(outcome));
        self.db
            .insert(VectorEntry {
                id: Some(id.into()),
                vector,
                metadata: Some(metadata),
            })
            .map_err(|e| HorizonError::Dependency(format!("RuVector insert: {e}")))?;
        self.len += 1;
        Ok(())
    }
    pub fn search(&self, vector: &[f32], k: usize) -> Result<Vec<Analogue>> {
        if self.len == 0 {
            return Ok(Vec::new());
        }
        let hits = self
            .db
            .search(SearchQuery {
                vector: vector.to_vec(),
                k: k.min(self.len),
                filter: None,
                ef_search: None,
            })
            .map_err(|e| HorizonError::Dependency(format!("RuVector search: {e}")))?;
        Ok(hits
            .into_iter()
            .map(|hit| Analogue {
                commitment_id: hit.id,
                distance: f64::from(hit.score),
                outcome: hit
                    .metadata
                    .and_then(|m| m.get("outcome").and_then(|v| v.as_str()).map(str::to_owned))
                    .unwrap_or_default(),
            })
            .collect())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn real_index_returns_nearest() {
        let mut index = AnalogueIndex::new().unwrap();
        index
            .insert("near", vec![0.0; DIMENSION], "healthy")
            .unwrap();
        index
            .insert("far", vec![1.0; DIMENSION], "breached")
            .unwrap();
        assert_eq!(
            index.search(&[0.1; DIMENSION], 1).unwrap()[0].commitment_id,
            "near"
        );
    }
}
