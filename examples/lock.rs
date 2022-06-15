use screenlocker::lock_screen;

fn main() {
    lock_screen().unwrap_or_else(|err| {
        eprintln!("{}", err);
    });
}
