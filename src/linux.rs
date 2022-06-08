use std::path::Path;
use std::process::Command;

use crate::{Error, Result};

pub fn lock_screen_linux() -> Result<()> {
    // TODO(smacdo): Finish implementation with multiple well known locking
    //               programs from major distributions.
    todo!()
}

/// Search through a list of programs and execute the first one that is found on
/// the system. Return the result of running said command - either success if
/// the commmand returned 0, or an error code indicating what went wrong.
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
