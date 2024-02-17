use std::{
    fmt, fs, io,
    os::fd::RawFd,
    path::PathBuf,
    process::{self, Command},
};

use nix::{errno::Errno, fcntl, sys::stat::Mode};

use serde::Deserialize;
use serde_yaml as yaml;

enum QuartzError {
    OpenKmsg(Errno),
}

enum UnitError {
    ReadConfig(io::Error),
    ParseConfig(yaml::Error),
}

#[derive(Deserialize)]
struct UnitConfig {
    dependencies: Vec<String>
}

#[derive(Default)]
struct Quartz {
    kmsg: RawFd,
}

impl Quartz {
    fn new() -> Result<Self, QuartzError> {
        let kmsg = fcntl::open("/dev/kmsg", fcntl::OFlag::O_RDWR, Mode::empty())
            .map_err(|e| QuartzError::OpenKmsg(e))?;

        Ok(Self {
            kmsg,
            ..Default::default()
        })
    }

    fn run_unit(&self, name: PathBuf) -> Result<(), UnitError> {
        let cfg: UnitConfig = yaml::from_str(
            &fs::read_to_string(name.join("unit.yml")).map_err(|e| UnitError::ReadConfig(e))?,
        )
        .map_err(|e| UnitError::ParseConfig(e))?;

        for dep in cfg.dependencies {
            println!("This unit depends on {}", dep);
        }

        Ok(())
    }
}

fn main() -> ! {
    if process::id() != 1 {
        panic!("Quartz can only be run as the init process")
    }

    let inst = match Quartz::new() {
        Ok(inst) => inst,
        Err(err) => match err {
            _ => panic!("Failed to initialize Quartz!"),
        },
    };

    println!("Successfully initialized Quartz. Welcome to Oxide Linux!");

    loop {
        match inst.run_unit("/system".into()).err() {
            Some(err) => match err {
                _ => println!("An unexpected error occurred while loading the system unit!"),
            },
            None => {
                println!("The system unit exited! This shouldn't happen normally, but does not necessarily indicate an error.");
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
