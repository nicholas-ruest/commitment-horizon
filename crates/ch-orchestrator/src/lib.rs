use ch_audit::audit;
use ch_commitment::Commitment;
use ch_errors::{HorizonError, Result};
use ch_evidence::EvidenceBatch;
use ch_repository::Repository;
use ch_ruvector::AnalogueIndex;
use ch_types::{CommitmentId, EvaluationResult, StoredRecord};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegisterCommitment {
    pub id: String,
    pub name: String,
    pub owner: String,
    pub target: f64,
    pub window_minutes: u32,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExceptionRequest {
    pub reason: String,
    pub duration_minutes: u32,
}
#[derive(Clone)]
pub struct HorizonService {
    repository: Repository,
    index: Arc<Mutex<AnalogueIndex>>,
}
impl HorizonService {
    pub fn new(repository: Repository) -> Result<Self> {
        let mut index = AnalogueIndex::new()?;
        for record in repository.records()? {
            if let StoredRecord::Evidence {
                commitment_id,
                points,
                ..
            } = record
            {
                let vector = ch_embedding::embed(&points, 0.99);
                let _ = index.insert(commitment_id.as_str(), vector, "historical");
            }
        }
        Ok(Self {
            repository,
            index: Arc::new(Mutex::new(index)),
        })
    }
    pub fn register(
        &self,
        command: RegisterCommitment,
        correlation_id: &str,
    ) -> Result<StoredRecord> {
        let aggregate = Commitment::register(
            CommitmentId::new(command.id)?,
            command.name,
            command.owner,
            command.target,
            command.window_minutes,
        )?;
        if self.repository.commitment(&aggregate.id).is_ok() {
            return Err(HorizonError::Conflict("commitment already exists".into()));
        }
        let payload =
            serde_json::to_vec(&aggregate).map_err(|e| HorizonError::Invalid(e.to_string()))?;
        let receipt = self.repository.append(&audit(
            aggregate.id.clone(),
            "commitment.registered",
            correlation_id,
            &"0".repeat(64),
            &payload,
        )?)?;
        let record = StoredRecord::Commitment {
            id: aggregate.id,
            name: aggregate.name,
            owner: aggregate.owner,
            target: aggregate.target,
            window_minutes: aggregate.window_minutes,
            active: aggregate.active,
            receipt,
        };
        self.repository.append(&record)?;
        Ok(record)
    }
    pub fn list(&self) -> Result<Vec<StoredRecord>> {
        self.repository.commitments()
    }
    pub fn get(&self, id: &str) -> Result<StoredRecord> {
        self.repository.commitment(&CommitmentId::new(id)?)
    }
    pub fn evaluate(&self, batch: EvidenceBatch, correlation_id: &str) -> Result<EvaluationResult> {
        batch.validate()?;
        let id = CommitmentId::new(batch.commitment_id.clone())?;
        let commitment = self.repository.commitment(&id)?;
        let target = match commitment {
            StoredRecord::Commitment { target, .. } => target,
            _ => return Err(HorizonError::NotFound(id.to_string())),
        };
        let values: Vec<f64> = batch.points.iter().map(|p| p.observed).collect();
        let lower = ch_calibration::adaptive_lower_bound(&values, 0.1)?;
        let e = ch_eprocess::anytime_e_value(&values, target)?;
        let probability = ch_forecast::breach_probability(&values, target)?;
        let vector = ch_embedding::embed(&batch.points, target);
        let analogues = {
            let index = self
                .index
                .lock()
                .map_err(|_| HorizonError::Dependency("index lock poisoned".into()))?;
            ch_recall::recall(&index, &vector)?
        };
        let band = ch_risk::classify(lower, target, probability, e);
        let evidence = StoredRecord::Evidence {
            commitment_id: id.clone(),
            points: batch.points,
            source: batch.source,
            receipt: "pending".into(),
        };
        let receipt = self.repository.append(&evidence)?;
        let result = EvaluationResult {
            commitment_id: id.clone(),
            observed_mean: values.iter().sum::<f64>() / values.len() as f64,
            lower_bound: lower,
            e_value: e,
            breach_probability: probability,
            risk_band: band.clone(),
            analogues,
            receipt,
            correlation_id: correlation_id.into(),
        };
        self.repository.append(&StoredRecord::Evaluation {
            result: result.clone(),
        })?;
        self.index
            .lock()
            .map_err(|_| HorizonError::Dependency("index lock poisoned".into()))?
            .insert(id.as_str(), vector, &format!("{band:?}"))?;
        Ok(result)
    }
    pub fn exception(
        &self,
        id: &str,
        request: ExceptionRequest,
        role: &str,
        correlation_id: &str,
    ) -> Result<StoredRecord> {
        let id = CommitmentId::new(id)?;
        self.repository.commitment(&id)?;
        ch_policy::authorize_exception(&request.reason, request.duration_minutes, role)?;
        let payload =
            serde_json::to_vec(&request).map_err(|e| HorizonError::Invalid(e.to_string()))?;
        let receipt = self.repository.append(&audit(
            id.clone(),
            "exception.approved",
            correlation_id,
            &"0".repeat(64),
            &payload,
        )?)?;
        let record = StoredRecord::Exception {
            commitment_id: id,
            reason: request.reason,
            duration_minutes: request.duration_minutes,
            approved: true,
            receipt,
        };
        self.repository.append(&record)?;
        Ok(record)
    }
    pub fn audit(&self, id: &str) -> Result<Vec<StoredRecord>> {
        let id = CommitmentId::new(id)?;
        Ok(self
            .repository
            .records()?
            .into_iter()
            .filter(|record| match record {
                StoredRecord::Commitment { id: found, .. }
                | StoredRecord::Evidence {
                    commitment_id: found,
                    ..
                }
                | StoredRecord::Exception {
                    commitment_id: found,
                    ..
                }
                | StoredRecord::Audit {
                    commitment_id: found,
                    ..
                } => found == &id,
                StoredRecord::Evaluation { result } => result.commitment_id == id,
            })
            .collect())
    }
}
