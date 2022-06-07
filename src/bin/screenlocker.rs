use screenlocker::lock_screen;

fn main() {
    lock_screen().unwrap_or_else(|err| {
        eprintln!("error when trying to lock screen: {}", err);
        std::process::exit(1);
    });
}
