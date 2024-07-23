async fn run() {
    println!("it works");
}

fn main() {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async { run().await })
}
