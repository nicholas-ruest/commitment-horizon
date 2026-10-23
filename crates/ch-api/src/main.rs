use ch_config::RuntimeConfig;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let config = RuntimeConfig::load().unwrap_or_else(|error| {
        eprintln!("{error}");
        std::process::exit(2)
    });
    if let Err(error) = ch_api::serve(config).await {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
