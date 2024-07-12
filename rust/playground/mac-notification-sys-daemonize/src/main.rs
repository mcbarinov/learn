use std::fs::File;
use std::thread::sleep;
use std::time::Duration;

use daemonize::Daemonize;

fn main() {
    println!("start");

    let daemonize = Daemonize::new()
        .pid_file("/tmp/my-daemon.pid") // Every method except `new` and `start`
        .stdout(File::create("/tmp/my-daemon.out").unwrap());

    match daemonize.start() {
        Ok(_) => {
            println!("start daemonize");
            sleep(Duration::from_secs(5));
        }
        Err(e) => eprintln!("Error, {}", e),
    }

    println!("finish");
}
