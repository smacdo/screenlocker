#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod windows;

use std::fmt;

pub type Win32ErrorCode = u32;
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
    #[cfg(target_os = "macos")]
    return crate::macos::lock_screen_mac();

    #[cfg(target_os = "windows")]
    return crate::windows::lock_screen_windows();

    // TODO(scott): Add linux support.

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    return Err(Error::UnsupportedPlatform);
}
