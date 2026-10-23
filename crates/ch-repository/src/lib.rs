use ch_errors::{HorizonError, Result};
use ch_rvf::RvfArchive;
use ch_types::{CommitmentId, StoredRecord};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct Repository {
    path: PathBuf,
}

impl Repository {
    pub fn open(root: &Path) -> Result<Self> {
        std::fs::create_dir_all(root).map_err(|e| HorizonError::Persistence(e.to_string()))?;
        Ok(Self {
            path: root.join("commitment-horizon.rvf"),
        })
    }

    pub fn append(&self, record: &StoredRecord) -> Result<String> {
        let bytes =
            serde_json::to_vec(record).map_err(|e| HorizonError::Persistence(e.to_string()))?;
        let records = RvfArchive::read_all(&self.path)?;
        let id = records.last().map_or(1, |(id, _)| id + 1);
        RvfArchive::append(&self.path, id, &bytes)?;
        Ok(hex::encode(Sha256::digest(bytes)))
    }

    pub fn records(&self) -> Result<Vec<StoredRecord>> {
        RvfArchive::read_all(&self.path)?
            .into_iter()
            .map(|(_, bytes)| {
                serde_json::from_slice(&bytes).map_err(|e| HorizonError::Persistence(e.to_string()))
            })
            .collect()
    }

    pub fn commitment(&self, id: &CommitmentId) -> Result<StoredRecord> {
        self.records()?.into_iter().rev().find(|record| matches!(record, StoredRecord::Commitment { id: found, .. } if found == id)).ok_or_else(|| HorizonError::NotFound(id.to_string()))
    }

    pub fn commitments(&self) -> Result<Vec<StoredRecord>> {
        Ok(self
            .records()?
            .into_iter()
            .filter(|record| matches!(record, StoredRecord::Commitment { .. }))
            .collect())
    }
}
