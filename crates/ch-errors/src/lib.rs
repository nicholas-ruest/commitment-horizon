use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HorizonError {
    #[error("invalid input: {0}")]
    Invalid(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("forbidden: {0}")]
    Forbidden(String),
    #[error("conflict: {0}")]
    Conflict(String),
    #[error("dependency unavailable: {0}")]
    Dependency(String),
    #[error("persistence failure: {0}")]
    Persistence(String),
}

pub type Result<T> = std::result::Result<T, HorizonError>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Problem {
    pub kind: String,
    pub title: String,
    pub status: u16,
    pub detail: String,
    pub correlation_id: String,
}

impl HorizonError {
    pub fn problem(&self, correlation_id: impl Into<String>) -> Problem {
        let (kind, title, status) = match self {
            Self::Invalid(_) => ("validation", "Request rejected", 400),
            Self::NotFound(_) => ("not-found", "Record not found", 404),
            Self::Forbidden(_) => ("forbidden", "Policy denied", 403),
            Self::Conflict(_) => ("conflict", "Request conflicts", 409),
            Self::Dependency(_) => ("dependency", "Dependency unavailable", 503),
            Self::Persistence(_) => ("persistence", "Persistence failed", 503),
        };
        Problem {
            kind: format!("https://commitment-horizon.dev/problems/{kind}"),
            title: title.into(),
            status,
            detail: self.to_string(),
            correlation_id: correlation_id.into(),
        }
    }
}
