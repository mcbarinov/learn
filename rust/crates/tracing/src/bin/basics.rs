use tracing::Level;

#[tracing::instrument]
fn trace_me(a: u32, b: u32) -> u32 {
    tracing::debug!("zzz");
    a + b
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(Level::DEBUG).init();

    tracing::info!("star");
    tracing::debug!("bla bla bla");
    let _res = trace_me(1, 3);

    tracing::info!("finish");
}
