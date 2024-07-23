use futures::future::join_all;
use tokio::task;
use tokio::time::Instant;

async fn sync_sleep(data: i32) -> i32 {
    // Simulate some asynchronous work
    println!("Processing element: {}", data);
    std::thread::sleep(std::time::Duration::from_secs(5));
    data * 2
}

async fn async_sleep(data: i32) -> i32 {
    // Simulate some asynchronous work
    println!("Processing element: {}", data);
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    data * 2
}

#[tokio::main]
async fn main() {
    let data = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
        33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
    ];

    measure_async_sleep(data.clone()).await; // measure_async_sleep: 5.003878417s
    measure_sync_sleep(data).await; // measure_sync_sleep: 15.01313825s
}

async fn measure_async_sleep(data: Vec<i32>) {
    let start = Instant::now();

    let tasks: Vec<_> = data.into_iter().map(|element| task::spawn(async move { async_sleep(element).await })).collect();

    let results: Vec<_> = join_all(tasks).await.into_iter().map(|res| res.expect("Task failed")).collect();
    println!("Results: {:?}", results);

    let duration = start.elapsed();
    println!("measure_async_sleep: {:?}", duration);
}

async fn measure_sync_sleep(data: Vec<i32>) {
    let start = Instant::now();

    let tasks: Vec<_> = data.into_iter().map(|element| task::spawn(async move { sync_sleep(element).await })).collect();

    let results: Vec<_> = join_all(tasks).await.into_iter().map(|res| res.expect("Task failed")).collect();
    println!("Results: {:?}", results);

    let duration = start.elapsed();
    println!("measure_sync_sleep: {:?}", duration);
}
