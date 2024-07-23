use std::error::Error;

use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::builder().build()?;
    let res = client.get("https://crates.io/api/v1/crates/url").header("User-Agent", "zorro").send().await?.text().await?;
    dbg!(res);
    Ok(())
}
