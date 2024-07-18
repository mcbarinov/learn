use std::future::Future;

use tokio::runtime::Runtime;

fn run_async<F, T>(f: F) -> T
where
    F: Future<Output = T>,
{
    let rt = Runtime::new().expect("Failed to create runtime");
    rt.block_on(f)
}

async fn example_async_function() -> i32 {
    // Simulate some async work
    42
}

fn main() {
    let result = run_async(example_async_function());
    println!("Result: {}", result);
}
