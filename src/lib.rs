extern crate nix;
use nix::pty::PtyMaster;
use nix::{Result, libc, Error};

use std::os::unix::prelude::*;
use std::ffi::CStr;

#[cfg(any(target_os = "android", target_os = "linux"))]
pub use nix::pty::ptsname_r;

/// (emulated on macOS) Get the name of the slave pseudoterminal (see
/// [ptsname(3)](http://man7.org/linux/man-pages/man3/ptsname.3.html))
///
/// returns the name of the slave pseudoterminal device corresponding to the master
/// referred to by `fd`. This is the threadsafe version of `ptsname()`, but it is not part of the
/// POSIX standard and is instead a Linux-specific extension.
///
/// This value is useful for opening the slave ptty once the master has already been opened with
/// `posix_openpt()`.
///
/// As `ptsname_r()` is Linux-specific, this implementation emulates `ptsname_r()` through
/// the `TIOCPTYGNAME` syscall on macOS.
#[cfg(any(target_os = "macos", target_os = "ios"))]
#[inline]
pub fn ptsname_r(fd: &PtyMaster) -> Result<String> {
    // This is based on
    // https://blog.tarq.io/ptsname-on-osx-with-rust/
    // and its derivative
    // https://github.com/philippkeller/rexpect/blob/a71dd02/src/process.rs#L67
    use libc::{c_ulong, ioctl, TIOCPTYGNAME};

    // the buffer size on OSX is 128, defined by sys/ttycom.h
    let buf: [i8; 128] = [0; 128];

    unsafe {
        match ioctl(fd.as_raw_fd(), TIOCPTYGNAME as c_ulong, &buf) {
            0 => {
                let res = CStr::from_ptr(buf.as_ptr())
                    .to_string_lossy()
                    .into_owned();
                Ok(res)
            }
            _ => Err(Error::last()),
        }
    }
}

#[cfg(test)]
mod tests {
    use nix::fcntl::O_RDWR;
    use nix::pty::posix_openpt;
    use std::os::unix::prelude::*;
    use super::ptsname_r;

    /// Test data copying of `ptsname_r`
    #[test]
    #[cfg(any(target_os = "android", target_os = "linux", target_os = "macos",
                target_os = "ios"))]
    fn test_ptsname_r_copy() {
        // Open a new PTTY master
        let master_fd = posix_openpt(O_RDWR).unwrap();
        assert!(master_fd.as_raw_fd() > 0);

        // Get the name of the slave
        let slave_name1 = ptsname_r(&master_fd).unwrap();
        let slave_name2 = ptsname_r(&master_fd).unwrap();
        assert!(slave_name1 == slave_name2);
        assert!(slave_name1.as_ptr() != slave_name2.as_ptr());
    }
}
