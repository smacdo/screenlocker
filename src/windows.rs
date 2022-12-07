use crate::{Error, Result, Win32ErrorCode};

#[link(name = "user32")]
extern "system" {
    fn LockWorkStation() -> i32;
}

#[link(name = "Kernel32")]
extern "system" {
    fn GetLastError() -> Win32ErrorCode;
}

/// Trigger the Windows screen lock by calling into the Windows API, or return
/// an error code via `GetLastError` if the screen failed to lock.
pub fn lock_screen_windows() -> Result<()> {
    unsafe {
        if 0 == LockWorkStation() {
            Err(Error::Win32(GetLastError()))
        } else {
            Ok(())
        }
    }
}
