use ch_errors::{HorizonError, Result};
pub fn breach_probability(values: &[f64], target: f64) -> Result<f64> {
    if values.len() < 3 {
        return Err(HorizonError::Invalid(
            "forecast requires at least three observations".into(),
        ));
    }
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let recent = *values.last().unwrap_or(&mean);
    let trend = recent - values[0];
    let margin = mean + trend * 0.5 - target;
    Ok((1.0 / (1.0 + (8.0 * margin).exp())).clamp(0.0, 1.0))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn falling_series_is_risky() {
        assert!(breach_probability(&[0.99, 0.9, 0.7], 0.95).unwrap() > 0.5);
    }
}
