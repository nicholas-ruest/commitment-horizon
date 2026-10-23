use ch_errors::{HorizonError, Result};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CommitmentId(String);
impl CommitmentId {
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();
        if value.is_empty()
            || value.len() > 64
            || !value
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-')
        {
            return Err(HorizonError::Invalid(
                "commitment_id must be 1-64 URL-safe characters".into(),
            ));
        }
        Ok(Self(value))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl fmt::Display for CommitmentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct EvidencePoint {
    pub sequence: u64,
    pub observed: f64,
    pub timestamp_ms: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RiskBand {
    Healthy,
    Watch,
    AtRisk,
    Breached,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Analogue {
    pub commitment_id: String,
    pub distance: f64,
    pub outcome: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct EvaluationResult {
    pub commitment_id: CommitmentId,
    pub observed_mean: f64,
    pub lower_bound: f64,
    pub e_value: f64,
    pub breach_probability: f64,
    pub risk_band: RiskBand,
    pub analogues: Vec<Analogue>,
    pub receipt: String,
    pub correlation_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "record_type", rename_all = "snake_case")]
pub enum StoredRecord {
    Commitment {
        id: CommitmentId,
        name: String,
        owner: String,
        target: f64,
        window_minutes: u32,
        active: bool,
        receipt: String,
    },
    Evidence {
        commitment_id: CommitmentId,
        points: Vec<EvidencePoint>,
        source: String,
        receipt: String,
    },
    Evaluation {
        result: EvaluationResult,
    },
    Exception {
        commitment_id: CommitmentId,
        reason: String,
        duration_minutes: u32,
        approved: bool,
        receipt: String,
    },
    Audit {
        receipt: String,
        commitment_id: CommitmentId,
        event: String,
        correlation_id: String,
        payload_hash: String,
    },
}
