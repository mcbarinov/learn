use tracing::Level;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter, Layer};

#[tracing::instrument]
fn trace_me(a: u32, b: u32) -> u32 {
    tracing::error!("mega error!");
    a + b
}

#[tokio::main]
async fn main() {
    let (non_blocking, _guard) = tracing_appender::non_blocking(tracing_appender::rolling::never("tmp/", "l1.txt"));
    let file_layer = fmt::Layer::new().with_writer(non_blocking.with_max_level(Level::WARN)).json();

    let console_layer = fmt::layer()
        .with_line_number(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_filter(EnvFilter::from("debug"));

    tracing_subscriber::registry().with(file_layer).with(console_layer).init();

    tracing::info!("start");
    tracing::debug!("bla bla bla");
    let _res = trace_me(1, 3);

    tracing::warn!("uh...");

    tracing::info!("finish");
}
