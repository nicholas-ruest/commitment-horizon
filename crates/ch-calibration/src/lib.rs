use ch_errors::{HorizonError, Result};
pub fn adaptive_lower_bound(values: &[f64], alpha: f64) -> Result<f64> {
    if values.len() < 3 || !(0.0..0.5).contains(&alpha) {
        return Err(HorizonError::Invalid(
            "calibration requires >=3 values and alpha in (0,0.5)".into(),
        ));
    }
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let variance = values
        .iter()
        .map(|value| (value - mean).powi(2))
        .sum::<f64>()
        / (values.len() - 1) as f64;
    let adaptive_penalty =
        (variance.sqrt() * (1.0 / alpha).ln().sqrt()) / (values.len() as f64).sqrt();
    Ok((mean - adaptive_penalty).clamp(0.0, 1.0))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lower_bound_is_conservative() {
        let values = [0.96, 0.95, 0.94, 0.97];
        assert!(adaptive_lower_bound(&values, 0.1).unwrap() <= 0.955);
    }
}
