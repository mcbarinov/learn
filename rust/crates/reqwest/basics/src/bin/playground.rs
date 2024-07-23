extern crate core;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let res = reqwest::get("https://crates.io/api/v1/crates/url").await?.text().await.unwrap();
    dbg!(res);
    Ok(())
}
