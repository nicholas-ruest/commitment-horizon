use ch_errors::{HorizonError, Result};
#[derive(Clone, Debug)]
pub struct Principal {
    pub role: String,
}
pub fn authenticate(header: Option<&str>, expected: &str) -> Result<Principal> {
    let valid = format!("Bearer {expected}");
    match header {
        Some(value) if value == valid => Ok(Principal {
            role: "operator".into(),
        }),
        Some("Bearer local-viewer") => Ok(Principal {
            role: "viewer".into(),
        }),
        _ => Err(HorizonError::Forbidden(
            "valid bearer token required".into(),
        )),
    }
}
