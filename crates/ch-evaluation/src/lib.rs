use ch_errors::Result;

#[derive(Clone, Debug)]
pub struct Baselines {
    pub combined: f64,
    pub without_calibration: f64,
    pub without_eprocess: f64,
    pub ruvnet_only: f64,
    pub enterprise_only: f64,
}

pub fn ablations(values: &[f64], target: f64) -> Result<Baselines> {
    let lower = ch_calibration::adaptive_lower_bound(values, 0.1)?;
    let e = ch_eprocess::anytime_e_value(values, target)?;
    let p = ch_forecast::breach_probability(values, target)?;
    // RuVector-only can provide analogue coverage, but cannot emit a calibrated
    // breach decision without the statistical and policy components.
    let ruvnet_only = (values.len() as f64 / 16.0).min(1.0);
    Ok(Baselines {
        combined: p + (target - lower).max(0.0) + e.ln() / 10.0,
        without_calibration: p + e.ln() / 10.0,
        without_eprocess: p + (target - lower).max(0.0),
        ruvnet_only,
        enterprise_only: p,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposes_all_required_baselines() {
        let result = ablations(&[0.99, 0.96, 0.88, 0.81], 0.95).expect("ablation");
        assert!(result.combined > result.enterprise_only);
        assert_eq!(result.ruvnet_only, 0.25);
    }
}
