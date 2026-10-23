use ch_types::RiskBand;
pub fn classify(lower_bound: f64, target: f64, breach_probability: f64, e_value: f64) -> RiskBand {
    if lower_bound < target && e_value >= 2.0 {
        RiskBand::Breached
    } else if breach_probability >= 0.7 {
        RiskBand::AtRisk
    } else if breach_probability >= 0.4 || lower_bound < target {
        RiskBand::Watch
    } else {
        RiskBand::Healthy
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn evidence_can_mark_breach() {
        assert_eq!(classify(0.7, 0.95, 0.9, 2.5), RiskBand::Breached);
    }
}
