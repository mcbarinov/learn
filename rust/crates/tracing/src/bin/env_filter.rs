use tracing::{debug, error, info};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter, Layer};

use learn::sum;

/// RUST_LOG="env_filter=warn,learn=debug" cargo r --bin env_filter
#[tokio::main]
async fn main() {
    let filter = EnvFilter::from_default_env();
    tracing_subscriber::registry()
        .with(fmt::layer().with_file(true).pretty().with_filter(filter))
        .init();

    debug!("debug?");
    info!("info?");
    error!("error?");

    sum(1, 33);

    debug!("debug!");
}
