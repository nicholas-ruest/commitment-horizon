use ch_errors::{HorizonError, Result};
use rvf_runtime::{RvfOptions, RvfStore};
use std::path::Path;

pub const RECORD_DIMENSION: usize = 8192;

fn encode(bytes: &[u8]) -> Result<Vec<f32>> {
    if bytes.is_empty() || bytes.len() >= RECORD_DIMENSION {
        return Err(HorizonError::Persistence(
            "RVF record size out of range".into(),
        ));
    }
    let mut vector = vec![0.0; RECORD_DIMENSION];
    vector[0] = bytes.len() as f32;
    for (index, byte) in bytes.iter().enumerate() {
        vector[index + 1] = f32::from(*byte);
    }
    Ok(vector)
}

fn decode(vector: &[f32]) -> Result<Vec<u8>> {
    let len = vector.first().copied().unwrap_or_default() as usize;
    if len == 0 || len + 1 > vector.len() {
        return Err(HorizonError::Persistence("invalid RVF record".into()));
    }
    vector[1..=len]
        .iter()
        .map(|value| {
            let byte = *value as u8;
            if (f32::from(byte) - *value).abs() > f32::EPSILON {
                Err(HorizonError::Persistence("corrupt RVF byte".into()))
            } else {
                Ok(byte)
            }
        })
        .collect()
}

pub struct RvfArchive;

impl RvfArchive {
    pub fn append(path: &Path, id: u64, bytes: &[u8]) -> Result<()> {
        let vector = encode(bytes)?;
        let refs = [vector.as_slice()];
        let ids = [id];
        let mut store = if path.exists() {
            RvfStore::open(path)
        } else {
            RvfStore::create(
                path,
                RvfOptions {
                    dimension: RECORD_DIMENSION as u16,
                    ..Default::default()
                },
            )
        }
        .map_err(|e| HorizonError::Persistence(format!("RVF open: {e}")))?;
        let result = store
            .ingest_batch(&refs, &ids, None)
            .map_err(|e| HorizonError::Persistence(format!("RVF ingest: {e}")))?;
        if result.accepted != 1 {
            return Err(HorizonError::Persistence("RVF rejected record".into()));
        }
        store
            .close()
            .map_err(|e| HorizonError::Persistence(format!("RVF close: {e}")))
    }

    pub fn read_all(path: &Path) -> Result<Vec<(u64, Vec<u8>)>> {
        if !path.exists() {
            return Ok(Vec::new());
        }
        let store = RvfStore::open_readonly(path)
            .map_err(|e| HorizonError::Persistence(format!("RVF reopen: {e}")))?;
        store
            .read_all_vectors()
            .into_iter()
            .map(|(id, vector)| decode(&vector).map(|bytes| (id, bytes)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn round_trip_uses_rvf() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("archive.rvf");
        RvfArchive::append(&path, 1, br#"{"ok":true}"#).unwrap();
        assert_eq!(RvfArchive::read_all(&path).unwrap()[0].1, br#"{"ok":true}"#);
    }
}
