use std::{
    fmt, fs,
    os::fd::RawFd,
    process::{self, Command},
};

use nix::{fcntl, libc::PROT_GROWSDOWN, sys::stat::Mode};
use rhai::{Engine, ImmutableString};

enum QuartzError {
    FailedToOpenKmsg,
}

impl fmt::Debug for QuartzError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FailedToOpenKmsg => write!(f, "Failed to connect to the kernel logger!"),
        }
    }
}
struct Quartz {
    kmsg: RawFd,
}

#[derive(Clone)]
struct Service {
    name: ImmutableString,
    program: ImmutableString,
}

impl Service {
    fn new(name: ImmutableString, program: ImmutableString) -> Self {
        Self {
            name,
            program
        }
    }
}

impl Quartz {
    fn new() -> Result<Self, QuartzError> {
        let kmsg = fcntl::open("/dev/kmsg", fcntl::OFlag::O_RDWR, Mode::empty())
            .map_err(|_| QuartzError::FailedToOpenKmsg)?;

        Ok(Self { kmsg })
    }

    fn prepare_start(&self) -> Result<&Self, QuartzError> {
        Ok(self)
    }

    fn start(&self) -> ! {
        loop {
            let mut rhai_engine = Engine::new();

            rhai_engine
                .register_type::<Service>()
                .register_fn("new_service", Service::new);

            self.start_service(
                rhai_engine
                    .eval_file::<Service>("/system/main.rhai".into())
                    .unwrap(),
            );
        }
    }

    fn start_service(&self, service: Service) {
        println!("Starting service [ {} ]...", service.name);

        let mut handle = Command::new(service.program.as_str())
            .spawn()
            .expect("failed to start service");

        handle.wait().expect("failed to exit service");
    }
}

fn main() -> ! {
    if process::id() != 1 {
        panic!("quartz can only be run as the init process!")
    }

    let inst = Quartz::new().expect("failed to initialize quartz");

    loop {
        match inst.prepare_start() {
            Ok(inst) => inst.start(),
            Err(err) => eprintln!("{err:?} Dropping into recovery shell..."),
        }

        let mut handle = Command::new("/sbin/sh")
            .spawn()
            .expect("shell failed to start");

        handle.wait().expect("failed to exit shell properly");
    }
}
