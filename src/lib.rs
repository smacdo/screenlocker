#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod windows;

use std::fmt;
// START for linux suport
use std::path::Path;
use std::process::Command;
// END

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

/// Search through a list of programs and execute the first one that is found on
/// the system. Return the result of running said command - either success if
/// the commmand returned 0, or an error code indicating what went wrong.
///
/// Note that another approach is to keep trying to run commands from
/// `possible_cmds` until one succeeds but that causes a messy situation where
/// it is unclear what result to return to the caller. If they all fail, what
/// error code is returned? If only one fails how do we pass the error code back
/// while still indicating success? Instead it's easier to assume that the list
/// defines a list of programs that are expected to always been a screen locking
/// program, and that if it fails to run there is a larger issue that the caller
/// must handle.
///
/// Developers should accomodate users with different system setups by including
/// multiple absolute paths to well known screen locking programs covering the
/// majority of desktop environments (Gnome, KDE, XFCE etc). If a hardcoded list
/// is not sufficient then consider offering a user editable configuration file
/// for endusers to customize.
fn run_first_found_exe(possible_cmds: &mut [Command]) -> Result<()> {
    // TODO(smacdo): Consider returning index of command that was selected.
    fn io_cmd_to_string(cmd: &Command) -> Option<String> {
        cmd.get_program().to_str().map(|s| s.to_string())
    }

    fn io_error(cmd: &Command, e: &std::io::Error) -> Error {
        Error::ExeIoError {
            cmd: io_cmd_to_string(cmd),
            kind: e.kind(),
            msg: e.to_string(),
        }
    }

    // Find first command that exists on the system.
    let mut cmd = possible_cmds
        .iter_mut()
        .find(|cmd| Path::is_file(Path::new(cmd.get_program())));

    // Execute the selected command, or return an error if no command could be
    // found on the user's systme.
    match cmd.as_mut() {
        Some(cmd) => match cmd.output() {
            Ok(output) => {
                // Only return sucess if the command executed succesfully and
                // returned an error code of zero.
                if output.status.success() {
                    Ok(())
                } else {
                    Err(Error::NonZeroExit {
                        cmd: io_cmd_to_string(cmd),
                        exit_code: output.status.code(),
                    })
                }
            }
            Err(e) => Err(io_error(cmd, &e)),
        },
        None => Err(Error::NoExeFound),
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

#[cfg(test)]
mod tests {
    // TODO(smacdo): Write unit tests for `run_first_found_exe`.
    // Scenarios:
    //  - one command that exists and returns 0 -> Ok(())
    //  - one command that doe not exist -> Err(NoExeFound).
    //  - n commands, none that exist -> Err(NoExeFound).
    //  - n commands, one that exists -> Ok(())
    //  - one command that exists and returns 27 -> Err(NonZeroExit)
    //  - same as above but path is not valid utf8 -> Err(NonZeroExit)
    //  - one command that exists and terminates via signal -> Err(NonZeroExit).
    //  - one command that is not executable -> Err(io_error).
}
