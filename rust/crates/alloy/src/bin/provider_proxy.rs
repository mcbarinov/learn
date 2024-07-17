use std::env;
use std::time::Duration;

use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::client::RpcClient;
use alloy::transports::http::Http;
use dotenv::dotenv;
use eyre::Result;
use reqwest::Proxy;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let rpc_url = "https://eth.merkle.io".parse()?;
    let proxy = Proxy::all(env::var("PROXY").expect("PROXY env must be set"))?;

    let http_client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .proxy(proxy)
        .build()?;

    let client = Http::with_client(http_client, rpc_url);
    let provider = ProviderBuilder::new().on_client(RpcClient::new(client, false));

    let latest_block = provider.get_block_number().await?;
    println!("Latest block number: {latest_block}");

    Ok(())
}
