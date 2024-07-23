use reqwest::{Client, Proxy};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let proxy = std::env::var("PROXY").unwrap();
    dbg!(&proxy);
    let client = Client::builder().proxy(Proxy::all(proxy)?).build()?;
    let res = client.get("https://httpbin.org/ip").send().await?.text().await?;
    println!("{res}");
    Ok(())
}
