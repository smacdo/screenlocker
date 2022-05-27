use std::fmt;

//============================================================================//
// External platform specific API functions.                                  //
//============================================================================//
// MacOS.
#[cfg(target_os = "macos")]
#[link(name = "login", kind = "framework")]
extern "C" {
    fn SACLockScreenImmediate();
}

// Windows.
pub type Win32ErrorCode = u32;

#[cfg(target_os = "windows")]
#[link(name = "user32")]
extern "system" {
    fn LockWorkStation() -> i32;
}

#[cfg(target_os = "windows")]
#[link(name = "Kernel32")]
extern "system" {
    fn GetLastError() -> Win32ErrorCode;
}

//============================================================================//
// Lock screen crate implementation.                                          //
//============================================================================//
type Result<T> = std::result::Result<T, Error>;

/// Error information explaining why the screen couldn't be locked.
#[derive(Debug, Clone)]
pub enum Error {
    /// A Win32 API function reported an error code.
    Win32(Win32ErrorCode),
    /// The current OS platform is not supported (yet) by screenlocker.
    UnsupportedPlatform,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Win32(ec) => write!(f, "GetLastResult() returned {}", ec),
            Error::UnsupportedPlatform => {
                write!(f, "This platform is not supported - please file a bug")
            }
        }
    }
}

/// Locks the computer screen by hiding the current desktop, and requiring
/// the user to re-enter their password before continuing.
pub fn lock_screen() -> Result<()> {
    // TODO(scott): Linux lock screen - write code to invoke a list of programs.

    #[cfg(target_os = "macos")]
    unsafe {
        SACLockScreenImmediate();
        Ok(())
    }

    #[cfg(target_os = "windows")]
    unsafe {
        if 0 == LockWorkStation() {
            Err(Error::Win32(GetLastError()))
        } else {
            Ok(())
        }
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    Err(Error::UnsupportedPlatform)
}
