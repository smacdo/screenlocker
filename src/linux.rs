use std::path::Path;
use std::process::Command;

use crate::{Error, Result};

struct LockCommand<'a> {
    exe: &'a str,
    args: &'a [&'a str],
}

static DEFAULT_COMMANDS: [LockCommand; 10] = [
    // xfce4-screensaver must be tried before xdg-screensaver to ensure success on XUbuntu
    LockCommand {
        exe: "/usr/bin/xfce4-screensaver-command",
        args: &["--lock"],
    },
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
        exe: "/usr/bin/light-locker-command",
        args: &["-lock"],
    },
    LockCommand {
        exe: "/usr/bin/xscreensaver-command",
        args: &["-lock"],
    },
    LockCommand {
        exe: "/usr/bin/mate-screensaver-command",
        args: &["--lock"],
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
        exe: "/usr/bin/physlock",
        args: &["-d"],
    },
];

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
fn run_first_found_exe(possible_cmds: &[LockCommand]) -> Result<usize> {
    fn io_error(cmd: &Command, e: &std::io::Error) -> Error {
        Error::ExeIoError {
            cmd: cmd.get_program().to_str().map(|s| s.to_string()),
            kind: e.kind(),
            msg: e.to_string(),
        }
    }

    // Find first command that exists on the system.
    let cmd_pos = possible_cmds
        .iter()
        .position(|cmd| Path::is_file(Path::new(cmd.exe)));

    // Execute the selected command, or return an error if no command could be
    // found on the user's system.
    match cmd_pos {
        Some(cmd_pos) => {
            let cmd = &possible_cmds[cmd_pos];

            let mut runnable = Command::new(cmd.exe);
            runnable.args(cmd.args.clone());

            match runnable.output() {
                Ok(output) => {
                    // Only return sucess if the command executed succesfully and
                    // returned an error code of zero.
                    if output.status.success() {
                        Ok(cmd_pos)
                    } else {
                        Err(Error::NonZeroExit {
                            cmd: Some(cmd.exe.to_string()),
                            exit_code: output.status.code(),
                        })
                    }
                }
                Err(e) => Err(io_error(&runnable, &e)),
            }
        }
        None => Err(Error::NoExeFound),
    }
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
    fn first_runnable_exe_is_used_even_if_error() {
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
        assert_eq!(
            Err(Error::NonZeroExit {
                cmd: Some(cmd_path1.to_str().unwrap().to_string()),
                exit_code: Some(27)
            }),
            result
        );
    }

    // TODO(smacdo): Write integrations tests for `lock_screen_linux`.
    //  - Write comprehensive integration tests including scenarios:
    //    - command that exists and returns non-zero with bad utf name.
    //    - command that exists and terminates via signal.
    //    - above but with bad utf name.
}
