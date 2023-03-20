mod main_result;
mod option_flags;
mod options;
// mod operations;
// mod filesystem;

pub use self::main_result::MainResult;
pub use self::option_flags::OptionFlags;
pub use self::options::Options;
pub use dokany_sys as sys;
use std::sync::Arc;
use std::sync::Once;

static OPERATIONS: sys::DOKAN_OPERATIONS = sys::DOKAN_OPERATIONS {
    ..sys::DOKAN_OPERATIONS::new()
};

/// Initialize the library, if needed.
///
/// If not called, this will be called automatically before any library functions.
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

/// Get the driver version.
pub fn driver_version() -> u32 {
    init();

    unsafe { sys::DokanDriverVersion() }
}

/// Mount and run a filesystem from the given options an mount object.
pub fn main<F>(mut options: Options, _filesystem: Arc<F>) -> Result<(), MainResult> {
    // Official docs also use a global static, so this is probably safe.
    let operations = &OPERATIONS as *const sys::DOKAN_OPERATIONS as *mut sys::DOKAN_OPERATIONS;

    MainResult(unsafe { sys::DokanMain(&mut options.options, operations) }).into()
}

/// Unmount the drive from the given drive letter.
///
/// # Panics
/// Panics if the given char cannot fit in a u8.
///
/// # Returns
/// Returns true if successful.
pub fn unmount(drive_letter: char) -> bool {
    init();

    let drive_letter: u8 = u32::from(drive_letter)
        .try_into()
        .expect("`drive_letter` cannot fit in a `u8`");

    unsafe { sys::DokanUnmount(drive_letter.into()) == sys::TRUE }
}

/// Shutdown the api and release all resources.
///
/// # Safety
/// * The api must have been initialized prior to this call.
/// * You must free all api objects before using this function.
pub unsafe fn shutdown() {
    unsafe { sys::DokanShutdown() }
}

#[cfg(test)]
mod test {
    use super::*;

    const MOUNT_POINT: &[u16] = &[b'Z' as u16];

    struct SimpleFilesystem;

    #[test]
    #[ignore]
    fn test() {
        init();

        let version = version();
        println!("Dokan Version: {version}");
        let driver_version = driver_version();
        println!("Dokan Driver Version: {driver_version}");

        let unmount_z = unmount('Z');
        println!("Unmount Z (no mount): {}", unmount_z);

        let handle = std::thread::spawn(|| {
            std::thread::sleep(std::time::Duration::from_secs(1));
            let unmount_z = unmount('Z');
            println!("Unmount Z (mount): {}", unmount_z);

            assert!(unmount_z);
        });

        let mut options = Options::new();
        options.set_version(209);
        options.set_mount_point(MOUNT_POINT);
        options.set_option_flags(OptionFlags::MOUNT_MANAGER);

        let simple_filesystem = Arc::new(SimpleFilesystem);

        match main(options, simple_filesystem) {
            Ok(()) => {}
            Err(e) => {
                panic!("{e}");
            }
        }

        handle.join().unwrap();

        unsafe { shutdown() }
    }
}
