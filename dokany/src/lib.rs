mod file_system_flags;
mod main_result;
mod operations;
mod option_flags;
mod options;
mod wide;
// mod filesystem;
// mod wide_string;

pub use self::file_system_flags::FileSystemFlags;
pub use self::main_result::MainResult;
pub(crate) use self::operations::OPERATIONS;
pub use self::option_flags::OptionFlags;
pub use self::options::Options;
pub use self::wide::AsWide;
pub use dokany_sys as sys;
use std::mem::MaybeUninit;
use std::sync::Once;

/// A cell that wraps an unitialzed wide c string buffer, tracking its initialization.
pub struct WriteWideCStringCell<'a> {
    slice: &'a mut [MaybeUninit<u16>],
    initialized: bool,
}

impl<'a> WriteWideCStringCell<'a> {
    fn new(slice: &'a mut [MaybeUninit<u16>]) -> Self {
        Self {
            slice,
            initialized: false,
        }
    }

    /// Write the contents of an AsWide into this buffer, marking it as initialized.
    ///
    /// If the AsWide is too long, it is truncated.
    pub fn write(&mut self, value: impl AsWide) {
        // Reserve NUL
        let max_len = self.slice.len() - 1;
        for (buffer, value) in self
            .slice
            .iter_mut()
            .zip(value.as_wide().take(max_len).chain(std::iter::once(0)))
        {
            buffer.write(value);
        }
        self.initialized = true;
    }

    /// Get the total length of the buffer, in wide chars
    pub fn total_len(&self) -> usize {
        self.slice.len()
    }
}

/// The trait a type must implement to serve as a filesystem
pub trait Filesystem: Send + Sync + 'static {
    /// Called for calls to GetVolumeInformation
    fn get_volume_information(
        &self,
        _volume_name: WriteWideCStringCell<'_>,
        _volume_serial_number: &mut u32,
        _maximum_component_length: &mut u32,
        _file_system_flags: &mut FileSystemFlags,
        _file_system_name: WriteWideCStringCell<'_>,
    ) -> sys::NTSTATUS {
        sys::STATUS_NOT_IMPLEMENTED
    }

    /// Called when the filesystem is mounted
    fn mounted(&self, _mount_point: &[u16]) -> sys::NTSTATUS {
        sys::STATUS_SUCCESS
    }

    /// Called when the filesystem is unmounted
    fn unmounted(&self) -> sys::NTSTATUS {
        sys::STATUS_SUCCESS
    }
}

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

pub(crate) struct GlobalContext {
    pub filesystem: Box<dyn Filesystem>,
}

/// Mount and run a filesystem from the given options an mount object.
pub fn main(mut options: Options, filesystem: impl Filesystem) -> Result<(), MainResult> {
    // Inject the filesystem as context.
    let context = Box::new(GlobalContext {
        filesystem: Box::new(filesystem),
    });
    let context_ptr = Box::into_raw(context);
    options.options.GlobalContext = context_ptr as u64;

    // Official docs also use a global static, so this is probably safe.
    let operations = &OPERATIONS as *const sys::DOKAN_OPERATIONS as *mut sys::DOKAN_OPERATIONS;

    // Mount, run, unmount.
    let result = MainResult(unsafe { sys::DokanMain(&mut options.options, operations) });

    // Free context
    let context = unsafe { Box::from_raw(context_ptr) };
    drop(context);

    result.into()
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
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;

    struct SimpleFilesystem;

    impl Filesystem for SimpleFilesystem {
        fn get_volume_information(
            &self,
            mut volume_name: WriteWideCStringCell<'_>,
            volume_serial_number: &mut u32,
            maximum_component_length: &mut u32,
            file_system_flags: &mut FileSystemFlags,
            mut file_system_name: WriteWideCStringCell<'_>,
        ) -> sys::NTSTATUS {
            volume_name.write("Simple Filesystem");

            *volume_serial_number = 1337;
            *maximum_component_length = 255;

            *file_system_flags |= FileSystemFlags::UNICODE_ON_DISK;

            file_system_name.write("NTFS");

            println!("GetVolumeInformation");

            sys::STATUS_SUCCESS
        }

        fn mounted(&self, mounted: &[u16]) -> sys::NTSTATUS {
            println!("Mounted at {:?}", OsString::from_wide(mounted));
            sys::STATUS_SUCCESS
        }

        fn unmounted(&self) -> sys::NTSTATUS {
            println!("Unmounted");
            sys::STATUS_SUCCESS
        }
    }

    #[test]
    #[ignore]
    fn test() {
        init();

        let version = version();
        println!("Dokan Version: {version}");
        let driver_version = driver_version();
        println!("Dokan Driver Version: {driver_version}");

        let unmount_z = unmount('Z');
        println!("Unmount Z (no mount): {unmount_z}");

        let handle = std::thread::spawn(|| {
            std::thread::sleep(std::time::Duration::from_secs(1));
            let unmount_z = unmount('Z');
            println!("Unmount Z (mount): {unmount_z}");

            assert!(unmount_z);
        });

        let mut options = Options::new();
        options.set_version(209);
        options.set_mount_point("Z");
        options.set_option_flags(OptionFlags::MOUNT_MANAGER);

        let simple_filesystem = SimpleFilesystem;

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
