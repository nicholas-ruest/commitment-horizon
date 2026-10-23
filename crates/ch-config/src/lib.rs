use ch_errors::{HorizonError, Result};
use std::{env, net::SocketAddr, path::PathBuf, time::Duration};

#[derive(Clone, Debug)]
pub struct RuntimeConfig {
    pub bind: SocketAddr,
    pub data_root: PathBuf,
    pub namespace: String,
    pub dev_token: String,
    pub request_timeout: Duration,
}

impl RuntimeConfig {
    pub fn load() -> Result<Self> {
        let bind = env::var("HORIZON_BIND")
            .unwrap_or_else(|_| "127.0.0.1:8080".into())
            .parse()
            .map_err(|_| HorizonError::Invalid("HORIZON_BIND".into()))?;
        let data_root =
            PathBuf::from(env::var("HORIZON_DATA_ROOT").unwrap_or_else(|_| "./data".into()));
        let namespace = env::var("HORIZON_NAMESPACE").unwrap_or_else(|_| "demo".into());
        if namespace.is_empty()
            || namespace.len() > 48
            || !namespace
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-')
        {
            return Err(HorizonError::Invalid("HORIZON_NAMESPACE".into()));
        }
        let dev_token = env::var("HORIZON_DEV_TOKEN").unwrap_or_else(|_| "local-operator".into());
        if dev_token.len() < 8 {
            return Err(HorizonError::Invalid("HORIZON_DEV_TOKEN".into()));
        }
        Ok(Self {
            bind,
            data_root,
            namespace,
            dev_token,
            request_timeout: Duration::from_secs(10),
        })
    }
    pub fn namespace_dir(&self) -> PathBuf {
        self.data_root.join(&self.namespace)
    }
}
