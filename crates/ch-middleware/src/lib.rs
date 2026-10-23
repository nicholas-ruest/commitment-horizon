use ch_errors::{HorizonError, Result};
pub fn validate_correlation(value: Option<&str>) -> Result<String> {
    let value = value.unwrap_or("missing-correlation");
    if value.len() > 128 || value.bytes().any(|b| b.is_ascii_control()) {
        return Err(HorizonError::Invalid("invalid correlation id".into()));
    }
    Ok(value.to_owned())
}
pub fn validate_idempotency(value: Option<&str>) -> Result<String> {
    let value = value.ok_or_else(|| HorizonError::Invalid("idempotency-key required".into()))?;
    if value.len() < 8 || value.len() > 128 {
        return Err(HorizonError::Invalid(
            "idempotency-key must be 8-128 chars".into(),
        ));
    }
    Ok(value.to_owned())
}
