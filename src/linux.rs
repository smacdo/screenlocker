use std::path::Path;
use std::process::Command;

use crate::{Error, Result};

/// Defines a potential program that can be executed to lock the computer
/// screen.
struct LockCommand<'a> {
    /// Absolute path to the expected location of this program.
    exe: &'a str,
    /// A list of parameters to pass to the program.
    args: &'a [&'a str],
}

/// A list of potential screen locking programs to try when locking the screen
/// on Linux. Not all of these programs may exist and some might also return
/// errors so continue to try all possibilities in the list.
///
/// This list is ordered with the most popular commands (popular meaning highest
/// percentage across distros and versions) to least popular.
static DEFAULT_COMMANDS: [LockCommand; 7] = [
    LockCommand {
        exe: "/usr/bin/xdg-screensaver",
        args: &["lock"],
    },
    LockCommand {
        exe: "/usr/bin/gnome-screensaver-command",
        args: &["--lock"],
    },
    LockCommand {
        exe: "/usr/bin/qdbus",
        args: &["org.freedesktop.ScreenSaver", "/ScreenSaver", "Lock"],
    },
    LockCommand {
        exe: "/usr/bin/xscreensaver-command",
        args: &["-lock"],
    },
    LockCommand {
        exe: "/usr/bin/xlock",
        args: &["-mode", "blank"],
    },
    LockCommand {
        exe: "/usr/bin/slock",
        args: &[],
    },
    LockCommand {
        exe: "/usr/bin/hyprlock",
        args: &[],
    },
];

/// Trigger a Linux screen lock by trying a hardcoded list of potential screen
/// locking programs.
pub fn lock_screen_linux() -> Result<()> {
    // TODO(smacdo): Add user customization via config file.
    run_first_found_exe(&DEFAULT_COMMANDS).map(|_| ())
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
///
/// ````notest
/// let cmds: [LockCommand; 3] = [
///     LockCommand {
///         exe: "does_not_exist_1",
///         args: &[],
///     },
///     LockCommand {
///         exe: cmd_path.to_str().unwrap(),
///         args: &[],
///     },
/// ];
/// let result = run_first_found_exe(&cmds);
/// ```
fn run_first_found_exe(possible_cmds: &[LockCommand]) -> Result<usize> {
    fn io_error(cmd: &Command, e: &std::io::Error) -> Error {
        Error::ExeIoError {
            cmd: cmd.get_program().to_str().map(|s| s.to_string()),
            kind: e.kind(),
            msg: e.to_string(),
        }
    }

    let mut first_error = None;

    // Iterate over all possible lock commands. Return an error if no command works
    // on the user's system.
    for (cmd_pos, cmd) in possible_cmds.iter().enumerate() {
        // Check if the command exists on the system, if not continue.
        if !Path::is_file(Path::new(cmd.exe)) {
            continue;
        }

        // Try to execute the command.
        let mut runnable = Command::new(cmd.exe);
        runnable.args(cmd.args.clone());

        match runnable.output() {
            Ok(output) => {
                // Only return success if the command executed successfully and
                // returned an error code of zero.
                if output.status.success() {
                    return Ok(cmd_pos);
                } else {
                    if first_error != None {
                        let err = Error::NonZeroExit {
                            cmd: Some(cmd.exe.to_string()),
                            exit_code: output.status.code(),
                        };
                        eprintln!("{err:?}");
                    } else {
                        first_error = Some(Error::NonZeroExit {
                            cmd: Some(cmd.exe.to_string()),
                            exit_code: output.status.code(),
                        });
                    }
                }
            }
            Err(e) => {
                if first_error != None {
                    let err = io_error(&runnable, &e);
                    eprintln!("{err:?}");
                } else {
                    first_error = Some(io_error(&runnable, &e));
                }
            },
        };
    };

    Err(first_error.unwrap_or_else(|| {
        // Replace this with an appropriate variant from your Error enum.
        Error::NoExeFound
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn path_to_test_cmd(script_name: &str) -> PathBuf {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("tests/");
        d.push(script_name);
        d
    }

    #[test]
    fn one_runnable_exe_exit_zero_returns_ok() {
        let cmd_path = path_to_test_cmd("run_success.sh");
        let cmds: [LockCommand; 1] = [LockCommand {
            exe: cmd_path.to_str().unwrap(),
            args: &[],
        }];
        let result = run_first_found_exe(&cmds);
        assert_eq!(Ok(0), result);
    }

    #[test]
    fn one_runnable_exe_does_not_exist_returns_err() {
        let cmds: [LockCommand; 1] = [LockCommand {
            exe: "does_not_exist",
            args: &[],
        }];
        let result = run_first_found_exe(&cmds);
        assert_eq!(Err(crate::Error::NoExeFound), result);
    }

    #[test]
    fn multiple_runnable_exe_none_exist_returns_err() {
        let cmds: [LockCommand; 3] = [
            LockCommand {
                exe: "does_not_exist_1",
                args: &[],
            },
            LockCommand {
                exe: "does_not_exist_2",
                args: &[],
            },
            LockCommand {
                exe: "does_not_exist_3",
                args: &[],
            },
        ];
        let result = run_first_found_exe(&cmds);
        assert_eq!(Err(crate::Error::NoExeFound), result);
    }

    #[test]
    fn multiple_runnable_exe_one_exist_returns_ok_with_index() {
        let cmd_path = path_to_test_cmd("run_success.sh");

        let cmds: [LockCommand; 3] = [
            LockCommand {
                exe: "does_not_exist_1",
                args: &[],
            },
            LockCommand {
                exe: cmd_path.to_str().unwrap(),
                args: &[],
            },
            LockCommand {
                exe: "does_not_exist_3",
                args: &[],
            },
        ];
        let result = run_first_found_exe(&cmds);
        assert_eq!(Ok(1), result);
    }

    #[test]
    fn runnable_exe_exit_non_zero_returns_err_with_exit_code() {
        let cmd_path = path_to_test_cmd("run_error_27.sh");
        let cmds: [LockCommand; 1] = [LockCommand {
            exe: cmd_path.to_str().unwrap(),
            args: &[],
        }];
        let result = run_first_found_exe(&cmds);
        assert_eq!(
            Err(Error::NonZeroExit {
                cmd: Some(cmd_path.to_str().unwrap().to_string()),
                exit_code: Some(27)
            }),
            result
        );
    }

    #[test]
    fn one_runnable_exe_without_execute_returns_err() {
        let cmd_path = path_to_test_cmd("run_no_exec.sh");
        let cmds: [LockCommand; 1] = [LockCommand {
            exe: cmd_path.to_str().unwrap(),
            args: &[],
        }];
        let result = run_first_found_exe(&cmds);
        assert_eq!(
            Err(Error::ExeIoError {
                cmd: Some(cmd_path.to_str().unwrap().to_string()),
                kind: std::io::ErrorKind::PermissionDenied,
                msg: "Permission denied (os error 13)".to_string(),
            }),
            result
        );
    }

    #[test]
    fn second_runnable_exe_is_used_if_first_error() {
        let cmd_path1 = path_to_test_cmd("run_error_27.sh");
        let cmd_path2 = path_to_test_cmd("run_success.sh");

        let cmds: [LockCommand; 2] = [
            LockCommand {
                exe: cmd_path1.to_str().unwrap(),
                args: &[],
            },
            LockCommand {
                exe: cmd_path2.to_str().unwrap(),
                args: &[],
            },
        ];
        let result = run_first_found_exe(&cmds);
        assert_eq!(Ok(1), result);
    }

    // TODO(smacdo): Write integrations tests for `lock_screen_linux`.
    //  - Write comprehensive integration tests including scenarios:
    //    - command that exists and returns non-zero with bad utf name.
    //    - command that exists and terminates via signal.
    //    - above but with bad utf name.
}
