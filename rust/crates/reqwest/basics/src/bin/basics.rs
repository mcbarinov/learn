use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct IpResponse {
    origin: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let res: HashMap<String, String> = reqwest::get("https://httpbin.org/ip").await?.json().await?;
    println!("{:?}", res);

    let res: Value = reqwest::get("https://httpbin.org/get?q=1").await?.json().await?;
    println!("{:?}", res.pointer("/args/q").unwrap());

    let res: IpResponse = reqwest::get("https://httpbin.org/ip").await?.json().await?;
    println!("{:?}", res);

    Ok(())
}
