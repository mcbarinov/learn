use std::fs::File;
use std::os::unix::io::IntoRawFd;
use std::process::exit;

use nix::libc;
use nix::unistd::{chdir, close, fork, ForkResult, setsid};
use notify_rust::Notification;

fn main() {
    // First fork to create a new process
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            // Exit the parent process
            println!("Forked with child PID: {}", child);
            exit(0);
        }
        Ok(ForkResult::Child) => {
            // Continue in the child process
        }
        Err(err) => {
            eprintln!("Fork failed: {}", err);
            exit(1);
        }
    }

    // Create a new session
    if let Err(err) = setsid() {
        eprintln!("setsid failed: {}", err);
        exit(1);
    }

    // Change the working directory to /
    if let Err(err) = chdir("/") {
        eprintln!("chdir failed: {}", err);
        exit(1);
    }

    // Redirect standard file descriptors
    let dev_null = File::open("/dev/null").unwrap();
    let dev_null_fd = dev_null.into_raw_fd();

    unsafe {
        close(0).unwrap(); // Close stdin
        close(1).unwrap(); // Close stdout
        close(2).unwrap(); // Close stderr

        // Redirect stdin, stdout, and stderr to /dev/null
        libc::dup2(dev_null_fd, 0);
        libc::dup2(dev_null_fd, 1);
        libc::dup2(dev_null_fd, 2);
    }

    // Daemonized process code
    Notification::new()
        .summary("Hello from Daemon")
        .body("This is a test notification from a daemonized Rust program!")
        .show()
        .unwrap();

    loop {
        // Keep the daemon running
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}