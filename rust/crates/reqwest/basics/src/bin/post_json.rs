use reqwest::Client;
use serde_json::json;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let data = json!({"jsonrpc": "2.0", "id": 1, "method": "getSlot"});
    let url = "https://api.mainnet-beta.solana.com";
    let res = Client::new().post(url).json(&data).send().await?.text().await?;
    println!("{}", res);
    Ok(())
}
