#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
mod linux;

use std::fmt;

pub type Win32ErrorCode = u32;
type Result<T> = std::result::Result<T, Error>;

/// Error information explaining why the screen couldn't be locked.
#[derive(Debug, Clone)]
pub enum Error {
    /// A Win32 API function reported an error code.
    Win32(Win32ErrorCode),
    /// The current OS platform is not supported (yet) by screenlocker. Please
    /// send a pull request or file an issue if you would like to add support!
    UnsupportedPlatform,
    /// An error occurred when trying to run a caller specified executable.
    ExeIoError {
        /// String path of the command that was to be executed, or `None` if the
        /// command was invalid UTF8.
        cmd: Option<String>,
        /// The error kind reported by `std::io::Error` when trying to run this
        /// command.
        kind: std::io::ErrorKind,
        /// The error message reported by `std::io::Error` when trying to run
        /// this command.
        msg: String,
    },
    /// A user specified exe returned a non-zero exit code when executed.
    NonZeroExit {
        /// String path of the command that was to be executed, or `None` if the
        /// command was invalid UTF8.
        cmd: Option<String>,
        /// Exit code returned by the user program after termination. This value
        /// is `None` if the program was terminated by a signal instead of
        /// normal termination.
        exit_code: Option<i32>,
    },
    /// None of the provided screenlocking programs could be found.
    NoExeFound,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Win32(ec) => write!(f, "GetLastResult() returned {}", ec),
            Error::UnsupportedPlatform => {
                write!(f, "This platform is not supported - please file a bug")
            }
            Error::ExeIoError { cmd, kind, msg } => {
                let cmd_name = match cmd {
                    Some(cmd) => cmd.clone(),
                    None => "with invalid utf8 name".to_string(),
                };

                write!(
                    f,
                    "io error {:?} ({}) when running user program {}",
                    kind, msg, cmd_name
                )
            }
            Error::NonZeroExit { cmd, exit_code } => {
                let reason = match exit_code {
                    Some(exit_code) => {
                        format!("non-succesful exit-code {}", exit_code)
                    }
                    None => {
                        format!("signal termination")
                    }
                };

                let cmd_name = match cmd {
                    Some(cmd) => cmd.clone(),
                    None => "with invalid utf8 name".to_string(),
                };

                write!(f, "{} when running user program {}", reason, cmd_name)
            }
            Error::NoExeFound => {
                write!(f, "none of the provided user programs were found")
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

    #[cfg(target_os = "linux")]
    return crate::linux::lock_screen_linux();

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    return Err(Error::UnsupportedPlatform);
}
