use crate::Result;

#[link(name = "login", kind = "framework")]
extern "C" {
    fn SACLockScreenImmediate();
}

/// Trigger the Mac OS screen lock by calling into a private OS framework.
pub fn lock_screen_mac() -> Result<()> {
    unsafe {
        SACLockScreenImmediate();
        Ok(())
    }
}
