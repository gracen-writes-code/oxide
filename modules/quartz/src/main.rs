use std::process::{self, Command};

fn main() -> ! {
    if process::id() != 1 {
        panic!("quartz can only be run as the init process!")
    }

    let mut handle = Command::new("/bin/sh")
        .spawn()
        .expect("Shell failed to start! Prepare for kernel panic...");

    handle
        .wait()
        .expect("Failed to exit shell properly! Prepare for kernel panic...");

    println!("Shell exited. Looping forever now...");

    loop {}
}
