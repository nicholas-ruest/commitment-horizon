use ch_errors::{HorizonError, Result};
const POLICY: &str = include_str!("../../../policies/exception.rego");
pub fn authorize_exception(reason: &str, duration_minutes: u32, role: &str) -> Result<()> {
    if ch_regorus::allow_exception(POLICY, reason, duration_minutes, role)? {
        Ok(())
    } else {
        Err(HorizonError::Forbidden("Regorus denied exception; operator role, reason >=8 chars, and duration <=120 required".into()))
    }
}
