use ch_errors::{HorizonError, Result};
use regorus::{Engine, Value};
pub fn allow_exception(
    policy: &str,
    reason: &str,
    duration_minutes: u32,
    role: &str,
) -> Result<bool> {
    let mut engine = Engine::new();
    engine
        .add_policy("exception.rego".into(), policy.into())
        .map_err(|e| HorizonError::Dependency(format!("Regorus policy: {e}")))?;
    let input =
        serde_json::json!({"reason":reason,"duration_minutes":duration_minutes,"role":role});
    engine.set_input(
        Value::from_json_str(&input.to_string())
            .map_err(|e| HorizonError::Invalid(e.to_string()))?,
    );
    let value = engine
        .eval_rule("data.commitment_horizon.exception.allow".into())
        .map_err(|e| HorizonError::Dependency(format!("Regorus eval: {e}")))?;
    Ok(value == Value::Bool(true))
}
#[cfg(test)]
mod tests {
    use super::*;
    const P: &str = include_str!("../../../policies/exception.rego");
    #[test]
    fn denies_viewer() {
        assert!(!allow_exception(P, "maintenance", 30, "viewer").unwrap());
    }
}
