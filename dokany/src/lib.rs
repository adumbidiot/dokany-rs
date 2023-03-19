pub use dokany_sys as sys;

use std::sync::Once;

/// Initialize the library, if needed.
///
/// If not called, this will be called automatically before any library function.
pub fn init() {
    static INIT: Once = Once::new();
    INIT.call_once(|| unsafe {
        sys::DokanInit();
    });
}

#[cfg(test)]
mod test {
    // use super::*;

    #[test]
    fn test() {
        // TODO
    }
}
