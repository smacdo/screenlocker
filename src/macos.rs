use crate::Result;

#[link(name = "login", kind = "framework")]
extern "C" {
    fn SACLockScreenImmediate();
}

pub fn lock_screen_mac() -> Result<()> {
    unsafe {
        SACLockScreenImmediate();
        Ok(())
    }
}
