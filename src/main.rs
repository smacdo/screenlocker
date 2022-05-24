use std::fmt;

// Link to platform specific APIs to initiate the screen lock.
#[cfg(target_os = "macos")]
#[link(name = "login", kind = "framework")]
extern "C" {
    fn SACLockScreenImmediate();
}

type Result<T> = std::result::Result<T, LockScreenError>;

/// Contains relevant error information when the screen could't be locked.
#[derive(Debug, Clone)]
pub struct LockScreenError {}

impl fmt::Display for LockScreenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Error reporting for Windows, Linux platforms.
        write!(f, "something went wrong")
    }
}
/// Locks the computer screen by hiding the current desktop, and requiring
/// the user to re-enter their password before continuing.
pub fn lock_screen() -> Result<()> {
    #[cfg(target_os = "macos")]
    unsafe {
        SACLockScreenImmediate();
        Ok(())
    }
}

fn main() {
    lock_screen().unwrap_or_else(|err| {
        eprintln!("error when trying to lock screen: {}", err);
        std::process::exit(1);
    });
}
