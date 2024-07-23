use std::io::stdout;

use tracing::info;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use learn::sum;

#[tokio::main]
async fn main() {
    // let file_appender = tracing_appender::rolling::never(stdout());
    //
    let (non_blocking, _guard) = tracing_appender::non_blocking(stdout());
    let file_layer = fmt::layer().with_writer(non_blocking).pretty();
    tracing_subscriber::registry().with(file_layer).init();

    info!("start");
    sum(2, 3);
    info!("finish");
}
