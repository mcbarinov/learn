use std::fs::File;
use std::process::Command;

use daemonize::Daemonize;

fn main() {
    println!("start in main process");

    let daemonize = Daemonize::new()
        .pid_file("/tmp/my-daemon.pid")
        .stdout(File::create("/tmp/my-daemon.out").unwrap());

    daemonize.start().unwrap();
    println!("daemonized!");

    let output = Command::new("osascript")
        .arg("-e")
        .arg("display notification \"This is a test notification\" with title \"Test Title\"")
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("Notification sent successfully");
    } else {
        eprintln!(
            "Failed to send notification: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    println!("done!");
}

