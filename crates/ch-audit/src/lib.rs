use ch_errors::{HorizonError, Result};
use ch_types::{CommitmentId, StoredRecord};
use sha2::{Digest, Sha256};

pub fn receipt(previous: &str, event: &str, payload: &[u8]) -> Result<String> {
    if event.is_empty() || event.len() > 80 {
        return Err(HorizonError::Invalid("audit event".into()));
    }
    let mut hash = Sha256::new();
    hash.update(previous.as_bytes());
    hash.update(event.as_bytes());
    hash.update(payload);
    Ok(hex::encode(hash.finalize()))
}

pub fn audit(
    commitment_id: CommitmentId,
    event: &str,
    correlation_id: &str,
    previous: &str,
    payload: &[u8],
) -> Result<StoredRecord> {
    let payload_hash = hex::encode(Sha256::digest(payload));
    Ok(StoredRecord::Audit {
        receipt: receipt(previous, event, payload)?,
        commitment_id,
        event: event.into(),
        correlation_id: correlation_id.into(),
        payload_hash,
    })
}
