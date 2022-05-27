use std::fmt;
// TODO(scott): Move this code to a library.
// TODO(scott): Move library code to library specific crate called `screenlock`.

// Link to platform specific APIs to initiate the screen lock.
#[cfg(target_os = "macos")]
#[link(name = "login", kind = "framework")]
extern "C" {
    fn SACLockScreenImmediate();
}

#[cfg(target_os = "windows")]
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

type Result<T> = std::result::Result<T, ErrorDetails>;

#[derive(Debug, Clone)]
pub enum ErrorType {
    Win32(Win32ErrorCode),
    UnsupportedPlatform
}

/// Contains relevant error information when the screen could't be locked.
#[derive(Debug, Clone)]
pub struct ErrorDetails {
    pub error_type: ErrorType,
}

impl ErrorDetails {
    pub fn new_win32_error(code: Win32ErrorCode) -> Self {
        ErrorDetails { error_type: ErrorType::Win32(code) }
    }
}

impl fmt::Display for ErrorDetails {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.error_type {
            ErrorType::Win32(ec) => write!(f, "GetLastResult() returned {}", ec),
            ErrorType::UnsupportedPlatform => write!(f, "This platform is not supported - please file a bug")
        }
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

    #[cfg(target_os = "windows")]
    unsafe {
        if 0 == LockWorkStation() {
            Err(ErrorDetails::new_win32_error(GetLastError()))
        } else {
            Ok(())
        }        
    }

    // TODO(scott): Linux lock screen - write code to invoke a list of programs.

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    panic!("Platform not supported -- please file a bug report!")
}

fn main() {
    lock_screen().unwrap_or_else(|err| {
        eprintln!("error when trying to lock screen: {}", err);
        std::process::exit(1);
    });
}
