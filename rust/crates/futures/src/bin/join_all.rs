use futures::future::join_all;
use tokio::task;

async fn process_element(data: i32) -> i32 {
    // Simulate some asynchronous work
    println!("Processing element: {}", data);
    data * 2
}

#[tokio::main]
async fn main() {
    let data = vec![1, 2, 3, 4, 5];

    // Create a collection of tasks to run in parallel
    let tasks: Vec<_> = data.into_iter().map(|element| task::spawn(async move { process_element(element).await })).collect();

    // Wait for all tasks to complete
    let results: Vec<_> = join_all(tasks).await.into_iter().map(|res| res.expect("Task failed")).collect();

    println!("Results: {:?}", results);
}
