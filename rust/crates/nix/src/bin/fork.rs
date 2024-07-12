use std::process::exit;
use std::thread::sleep;

use nix::unistd::{fork, ForkResult};

fn main() {
    println!("start");
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            println!(
                "Continuing execution in parent process, new child has pid: {}",
                child
            );
            // waitpid(child, None).unwrap();
            println!("parent done");
            exit(0);
        }
        Ok(ForkResult::Child) => {
            // Unsafe to use `println!` (or `unwrap`) here. See Safety.
            println!("child process");
            for i in 0..10 {
                println!("child process {}", i);
                sleep(std::time::Duration::from_secs(1));
            }
        }
        Err(_) => println!("Fork failed"),
    }
    println!("finish");
}
