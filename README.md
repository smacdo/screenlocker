# screenlocker
![CI build state](https://img.shields.io/github/workflow/status/smacdo/screenlocker/Rust/main)
![code license](https://img.shields.io/github/license/smacdo/screenlocker)
![Crate version](https://img.shields.io/crates/v/screenlocker)

Screenlocker is a command line program (and reusable library) to request your
desktop session to be locked using the current platform's native API.

**Linux and Windows support coming soon**

## Installation
Use Rust's [cargo](https://www.rust-lang.org/tools/install) tool to install
screenlocker.

```shell
$ cargo install screenlocker
$ screenlocker     # if $PATH contains `~/.cargo/bin`
```

Please contact me or create a new issue if you would like to have downloadable
installers for your platform rather than use cargo.

## Usage
### Command line
Screenlocker is invoked on the command line with no parameters, and will
immediately lock your desktop session or print an error if it cannot.

For additional information on using the command line tool invoke the tool like
this: `screenlocker --help`.

### Library
Screenlocker is also available as a reusable Rust library. The latest API
documentation is available from a link on
[screenlockers's crates.io page](https://crates.io/crates/screenlocker).

Generally all you need is the `lock_screen` function, as seen here in this 
example:

```rust
use screenlocker::lock_screen;
lock_screen();
```

## Building
Make sure you install [Rust](https://www.rust-lang.org/tools/install) on your
computer before building.

```shell
$ git clone git@github.com:smacdo/screenlocker.git
$ cd screenlocker
$ cargo test
$ cargo run -- o cbiprt
```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to
discuss what you would like to change.

Please make sure to update tests as appropriate. 
