use ch_errors::{HorizonError, Result};
use ch_types::CommitmentId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Commitment {
    pub id: CommitmentId,
    pub name: String,
    pub owner: String,
    pub target: f64,
    pub window_minutes: u32,
    pub active: bool,
}

impl Commitment {
    pub fn register(
        id: CommitmentId,
        name: String,
        owner: String,
        target: f64,
        window_minutes: u32,
    ) -> Result<Self> {
        if name.trim().len() < 3 || name.len() > 120 {
            return Err(HorizonError::Invalid(
                "name must be 3-120 characters".into(),
            ));
        }
        if owner.trim().is_empty() || owner.len() > 80 {
            return Err(HorizonError::Invalid(
                "owner must be 1-80 characters".into(),
            ));
        }
        if !(0.5..=1.0).contains(&target) {
            return Err(HorizonError::Invalid(
                "target must be between 0.5 and 1.0".into(),
            ));
        }
        if !(5..=43_200).contains(&window_minutes) {
            return Err(HorizonError::Invalid("window_minutes out of range".into()));
        }
        Ok(Self {
            id,
            name,
            owner,
            target,
            window_minutes,
            active: true,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn target_is_bounded() {
        assert!(Commitment::register(
            CommitmentId::new("x").unwrap(),
            "API availability".into(),
            "platform".into(),
            1.1,
            60
        )
        .is_err());
    }
}
