use std::fmt;
// TODO(scott): Move this code to a library.
// TODO(scott): Move library code to library specific crate called `screenlock`.

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
        // TODO(scott): Make error message + code for Windows, Linux platforms.
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
    // TODO(scott): Windows lock screen - use Win32 API.
    // TODO(scott): Linux lock screen - write code to invoke a list of programs.
    // TODO(scott): Return an error if none of the supported platforms are used.
}

fn main() {
    lock_screen().unwrap_or_else(|err| {
        eprintln!("error when trying to lock screen: {}", err);
        std::process::exit(1);
    });
}
