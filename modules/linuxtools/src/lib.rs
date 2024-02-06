use libc::ioctl;
use nix::{fcntl, sys::stat::Mode, unistd};

const VT_ACTIVATE: u64 = 0x5606;
// const VT_WAITACTIVE: u64 = 0x5607;

pub fn chvt(ttynum: i32) {
    let fd = fcntl::open(
        format!("/dev/tty{}", ttynum).as_str(),
        fcntl::OFlag::O_RDONLY,
        Mode::empty(),
    )
    .unwrap();

    unsafe {
        ioctl(fd, VT_ACTIVATE, ttynum);
    }

    unistd::close(fd).unwrap();
}
