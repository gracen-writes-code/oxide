use std::{
    fmt, fs, io,
    os::fd::RawFd,
    path::PathBuf,
    process::{self, Command},
};

use nix::{errno::Errno, fcntl, sys::stat::Mode};
use rhai::{Engine, ImmutableString};

enum LoadUnitError {
    FailedToOpenInit(io::Error),
}

enum QuartzError {
    KmsgOpen(Errno),
    SystemUnitLoad(LoadUnitError),
}

impl fmt::Debug for QuartzError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuartzError::KmsgOpen(e) => {
                write!(f, "couldn't open /dev/kmsg: {e:?}")
            }
            QuartzError::SystemUnitLoad(e) => {
                write!(f, "failed to load system unit")
            }
        }
    }
}

#[derive(Default)]
struct Quartz {
    kmsg: RawFd,

    system_unit: Option<String>,
}

#[derive(Clone)]
struct Service {
    name: ImmutableString,
    program: ImmutableString,
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

    fn prepare_start(&mut self) -> Result<&Self, QuartzError> {
        self.system_unit = Some(
            self.load_unit("/system".into())
                .map_err(|e| QuartzError::SystemUnitLoad(e))?,
        );

        Ok(self)
    }

    fn start(&self) -> ! {
        loop {
            self.run_unit(self.system_unit.as_ref().unwrap().clone())
        }
    }

    fn load_unit(&self, path: PathBuf) -> Result<String, LoadUnitError> {
        let init = fs::read_to_string(path.join("unit.rhai"))
            .map_err(|e| LoadUnitError::FailedToOpenInit(e))?;

        let engine = Engine::new();

        engine.run(&init);

        Ok(String::new())
    }

    fn run_unit(&self, id: String) {
        println!("We would normally run a unit named {id} here. We're testing, so we won't.")
    }
}

fn main() -> ! {
    if process::id() != 1 {
        panic!("quartz can only be run as the init process!")
    }

    let mut inst = Quartz::new().expect("failed to initialize quartz");

    loop {
        match inst.prepare_start() {
            Ok(inst) => inst.start(),
            Err(e) => eprintln!("{e:?}: dropping into recovery shell..."),
        }

        let mut handle = Command::new("/bin/sh")
            .spawn()
            .expect("shell failed to start");

        handle.wait().expect("failed to exit shell properly");
    }
}
