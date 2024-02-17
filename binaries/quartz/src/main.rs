use std::{
    fmt,
    os::fd::RawFd,
    path::PathBuf,
    process::{self, Command},
};

use nix::{errno::Errno, fcntl, sys::stat::Mode};

enum QuartzError {
    KmsgOpen(Errno),
}

enum UnitError {}

impl fmt::Debug for QuartzError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuartzError::KmsgOpen(e) => {
                write!(f, "couldn't open /dev/kmsg: {e:?}")
            }
        }
    }
}

#[derive(Default)]
struct Quartz {
    kmsg: RawFd,
}

impl Quartz {
    fn new() -> Result<Self, QuartzError> {
        let kmsg = fcntl::open("/dev/kmsg", fcntl::OFlag::O_RDWR, Mode::empty())
            .map_err(|e| QuartzError::KmsgOpen(e))?;

        Ok(Self {
            kmsg,
            ..Default::default()
        })
    }

    fn run_unit(&self, id: PathBuf) -> Result<(), UnitError> {
        println!("We would normally run a unit named {id:?} here. We're testing, so we won't.");

        Ok(())
    }
}

fn main() -> ! {
    if process::id() != 1 {
        panic!("quartz can only be run as the init process")
    }

    let inst = Quartz::new().expect("failed to initialize quartz");

    loop {
        match inst.run_unit("/system".into()).err() {
            Some(err) => match err {
                _ => println!("An unexpected error occurred!"),
            },
            None => {
                println!("The init unit exited! This shouldn't happen normally, but does not necessarily indicate an error.");
            }
        }

        println!("Dropping into recovery shell.");
        println!("Exit the shell at any time to reload the system.");

        let mut handle = Command::new("/bin/sh")
            .spawn()
            .expect("shell failed to start");

        handle.wait().expect("failed to exit shell properly");
    }
}
