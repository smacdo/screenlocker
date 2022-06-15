# screenlocker
![CI build state](https://img.shields.io/github/workflow/status/smacdo/screenlocker/Rust/main)
![Code license](https://img.shields.io/github/license/smacdo/screenlocker)
![Crate version](https://img.shields.io/crates/v/screenlocker)
![Maintenance state](https://img.shields.io/maintenance/yes/2032)


Screenlocker is a cross platform command line tool that locks your desktop session until a password is entered. It is also a reusable library for developers who want to perform this action programmatically.

## Installation
Use Rust's [cargo](https://www.rust-lang.org/tools/install) tool to install screenlocker.

```shell
$ cargo install screenlocker
$ screenlocker     # if $PATH contains `~/.cargo/bin`
```

Please contact me or create a new issue if you would like to have downloadable installers for your platform rather than use cargo.

## Usage
The latest API documentation is available from a link on [screenlockers's crates.io page](https://crates.io/crates/screenlocker). All you need is the `lock_screen` function, as seen here in this example:

```rust
use screenlocker::lock_screen;

fn main() {
    lock_screen().unwrap_or_else(|err| {
      eprintln!("{}", err);
    });
}
```

## Building
Make sure you install [Rust](https://www.rust-lang.org/tools/install) on your computer before building.

```shell
$ git clone https://github.com/smacdo/screenlocker.git
$ cd screenlocker
$ cargo test
$ cargo run
```

## Multi-platform support
On Mac and Windows this crate uses the platform's native SDK to implement screen locking. Linux does not have any such functionality, and instead relies on any number of programs to lock the screen. This crate hard codes the most common programs (`xdg-screensaver`, `gnome-screensaver-command`, etc) but it is possible that less mainstream window managers don't work. Please open an issue to flag this problem - or better - a pull request adding the missing program.

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.
