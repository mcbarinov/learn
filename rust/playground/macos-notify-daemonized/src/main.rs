extern crate daemonize;
use std::fs::File;
use std::thread::sleep;
use std::time::Duration;

use daemonize::Daemonize;
use mac_notification_sys::{Notification, send_notification};

// fn main() {
//     println!("z1");
//     spawn(|| {
//         send_notification(
//             "Notify from daemonized",
//             None,
//             "it works",
//             Some(Notification::new().sound("Blow")),
//         )
//         .unwrap();
//         println!("z2");
//     });
//
//     sleep(Duration::from_secs(5));
// }

fn main() {
    let stdout = File::create("/tmp/daemon.out").unwrap();
    let stderr = File::create("/tmp/daemon.err").unwrap();

    println!("z1");
    let daemonize = Daemonize::new()
        .pid_file("/tmp/test.pid") // Every method except `new` and `start`
        .chown_pid_file(true) // is optional, see `Daemonize` documentation
        .working_directory("/tmp") // for default behaviour.
        .stdout(stdout) // Redirect stdout to `/tmp/daemon.out`.
        .stderr(stderr) // Redirect stderr to `/tmp/daemon.err`.
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => {
            println!("z2");
            notify2();
            println!("z5");
            sleep(Duration::from_secs(5));
        }
        Err(e) => eprintln!("Error, {}", e),
    }
}

fn notify1() {
    send_notification(
        "Notify from daemonized",
        None,
        "it works",
        Some(Notification::new().sound("Blow")),
    )
    .unwrap();
}

fn notify2() {
    println!("z3");
    let res = notify_rust::Notification::new()
        .summary("Firefox News")
        .body("This will almost look like a real firefox notification.")
        .icon("firefox")
        .show();
    println!("z4");
    println!("{:?}", res);
}
