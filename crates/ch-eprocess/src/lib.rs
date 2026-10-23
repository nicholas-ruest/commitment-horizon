use ch_errors::{HorizonError, Result};
pub fn anytime_e_value(values: &[f64], target: f64) -> Result<f64> {
    if values.is_empty() || !(0.0..=1.0).contains(&target) {
        return Err(HorizonError::Invalid("invalid e-process input".into()));
    }
    Ok(values
        .iter()
        .fold(1.0, |wealth, value| {
            wealth * (1.0 + (target - value).max(0.0) * 2.0)
        })
        .max(1.0))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn persistent_shortfall_grows_evidence() {
        assert!(
            anytime_e_value(&[0.7, 0.6, 0.5], 0.9).unwrap()
                > anytime_e_value(&[0.9, 0.9, 0.9], 0.9).unwrap()
        );
    }
}
