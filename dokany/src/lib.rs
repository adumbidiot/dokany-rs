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

/// Get the version.
pub fn version() -> u32 {
    init();

    unsafe { sys::DokanVersion() }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        init();

        let version = version();
        println!("Dokan Version: {version}");
    }
}
