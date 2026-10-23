use ch_evidence::EvidenceBatch;
use ch_types::EvidencePoint;
pub fn deterministic_batch(id: &str) -> EvidenceBatch {
    EvidenceBatch {
        commitment_id: id.into(),
        source: "synthetic-seed-v1".into(),
        points: vec![
            EvidencePoint {
                sequence: 1,
                observed: 0.99,
                timestamp_ms: 1_700_000_000_000,
            },
            EvidencePoint {
                sequence: 2,
                observed: 0.96,
                timestamp_ms: 1_700_000_060_000,
            },
            EvidencePoint {
                sequence: 3,
                observed: 0.88,
                timestamp_ms: 1_700_000_120_000,
            },
            EvidencePoint {
                sequence: 4,
                observed: 0.81,
                timestamp_ms: 1_700_000_180_000,
            },
        ],
    }
}
