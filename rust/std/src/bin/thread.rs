use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 0..9 {
            println!("count: {i}");
            thread::sleep(Duration::from_secs(1));
        }
    });

    println!("wait...");
    handle.join().unwrap();
}