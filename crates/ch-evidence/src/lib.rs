use ch_errors::{HorizonError, Result};
use ch_types::EvidencePoint;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvidenceBatch {
    pub commitment_id: String,
    pub source: String,
    pub points: Vec<EvidencePoint>,
}
impl EvidenceBatch {
    pub fn validate(&self) -> Result<()> {
        if self.source.trim().is_empty() || self.source.len() > 80 {
            return Err(HorizonError::Invalid(
                "source must be 1-80 characters".into(),
            ));
        }
        if self.points.len() < 3 || self.points.len() > 512 {
            return Err(HorizonError::Invalid(
                "evidence requires 3-512 points".into(),
            ));
        }
        let mut previous = None;
        for point in &self.points {
            if !point.observed.is_finite() || !(0.0..=1.0).contains(&point.observed) {
                return Err(HorizonError::Invalid(
                    "observed values must be finite ratios".into(),
                ));
            }
            if previous.is_some_and(|value| point.sequence <= value) {
                return Err(HorizonError::Invalid(
                    "evidence sequence must be strictly increasing".into(),
                ));
            }
            previous = Some(point.sequence);
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_reordered_points() {
        let batch = EvidenceBatch {
            commitment_id: "x".into(),
            source: "synthetic".into(),
            points: vec![
                EvidencePoint {
                    sequence: 2,
                    observed: 0.9,
                    timestamp_ms: 1,
                },
                EvidencePoint {
                    sequence: 1,
                    observed: 0.8,
                    timestamp_ms: 2,
                },
                EvidencePoint {
                    sequence: 3,
                    observed: 0.7,
                    timestamp_ms: 3,
                },
            ],
        };
        assert!(batch.validate().is_err());
    }
}
