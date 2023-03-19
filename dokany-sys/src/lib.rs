#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

pub use windows_sys::core::PCWSTR;
pub use windows_sys::Win32::Foundation::BOOL;
pub use windows_sys::Win32::Foundation::BOOLEAN;
pub use windows_sys::Win32::Foundation::CHAR;

pub type USHORT = u16;
pub type ULONG = u32;
pub type ULONG64 = u64;
pub type LPCWSTR = PCWSTR;
pub type DWORD = u32;

pub type DokanMainResult = std::os::raw::c_int;

/// Dokan mount succeed.
pub const DOKAN_SUCCESS: DokanMainResult = 0;
/// Dokan mount error.
pub const DOKAN_ERROR: DokanMainResult = -1;
/// Dokan mount failed - Bad drive letter.
pub const DOKAN_DRIVE_LETTER_ERROR: DokanMainResult = -2;
/// Dokan mount failed - Driver answer that something is wrong.
pub const DOKAN_DRIVER_INSTALL_ERROR: DokanMainResult = -3;
/// Dokan mount failed - Driver answer that something is wrong.
pub const DOKAN_START_ERROR: DokanMainResult = -3;
/// Dokan mount failed.
/// Can't assign a drive letter or mount point.
/// Probably already used by another volume.
pub const DOKAN_MOUNT_ERROR: DokanMainResult = -5;
/// Dokan mount failed.
/// Mount point is invalid.
pub const DOKAN_MOUNT_POINT_ERROR: DokanMainResult = -6;
/// Dokan mount failed.
/// Requested an incompatible version.
pub const DOKAN_VERSION_ERROR: DokanMainResult = -7;

/// This is arbitrary. There isn't really an absolute max, but we marshal it in
/// a fixed-size buffer.
pub const VOLUME_SECURITY_DESCRIPTOR_MAX_SIZE: usize = 1024 * 16;

pub type DOKAN_HANDLE = *const std::os::raw::c_void;

/// Dokan mount options used to describe Dokan device behavior.
#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub struct DOKAN_OPTIONS {
    /// Version of the Dokan features requested without dots (version "123" is equal to Dokan version 1.2.3).
    pub Version: USHORT,

    /// Only use a single thread to process events. This is highly not recommended as can easily create a bottleneck.
    pub SingleThread: BOOLEAN,

    /// Features enabled for the mount.
    pub Options: ULONG,

    /// FileSystem can store anything here.
    pub GlobalContext: ULONG64,

    /// Mount point. It can be a driver letter like "M:\" or a folder path "C:\mount\dokan" on a NTFS partition.
    pub MountPoint: LPCWSTR,

    /// UNC Name for the Network Redirector
    /// see <a href="https://msdn.microsoft.com/en-us/library/windows/hardware/ff556761(v=vs.85).aspx">Support for UNC Naming</a>
    pub UNCName: LPCWSTR,

    /// Max timeout in milliseconds of each request before Dokan gives up to wait events to complete.
    /// A timeout request is a sign that the userland implementation is no longer able to properly manage requests in time.
    /// The driver will therefore unmount the device when a timeout trigger in order to keep the system stable.
    /// The default timeout value is 15 seconds.
    pub Timeout: ULONG,

    /// Allocation Unit Size of the volume. This will affect the file size.
    pub AllocationUnitSize: ULONG,

    /// Sector Size of the volume. This will affect the file size.
    pub SectorSize: ULONG,

    /// Length of the optional VolumeSecurityDescriptor provided. Set 0 will disable the option.
    pub VolumeSecurityDescriptorLength: ULONG,

    /// Optional Volume Security descriptor.
    ///
    /// See <a href="https://docs.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-initializesecuritydescriptor">InitializeSecurityDescriptor</a>
    pub VolumeSecurityDescriptor: [CHAR; VOLUME_SECURITY_DESCRIPTOR_MAX_SIZE],
}

impl DOKAN_OPTIONS {
    /// Make an empty [`DOKAN_OPTIONS`].
    pub fn new() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl Default for DOKAN_OPTIONS {
    fn default() -> Self {
        Self::new()
    }
}

pub type PDOKAN_OPTIONS = *mut DOKAN_OPTIONS;
pub type PDOKAN_OPERATIONS = *mut std::os::raw::c_void;

extern "stdcall" {
    /// Initialize all required Dokan internal resources.
    ///
    /// This needs to be called only once before trying to use [DokanMain] or [DokanCreateFileSystem] for the first time.
    /// Otherwise both will fail and raise an exception.
    pub fn DokanInit();

    /// Release all allocated resources by [DokanInit] when they are no longer needed.
    ///
    /// This should be called when the application no longer expects to create a new FileSystem with
    /// [DokanMain] or [DokanCreateFileSystem] and after all devices are unmount.
    pub fn DokanShutdown();

    /// Mount a new Dokan Volume.
    ///
    /// This function block until the device is unmounted.
    /// If the mount fails, it will directly return a \ref DokanMainResult error.
    ///
    /// See [DokanCreateFileSystem] to create mount Dokan Volume asynchronously.
    ///
    /// # Arguments
    /// `DokanOptions`: a [DOKAN_OPTIONS] that describe the mount.
    /// `DokanOperations`: Instance of [DOKAN_OPERATIONS] that will be called for each request made by the kernel.
    ///
    /// # Returns
    /// [DokanMainResult] status.
    pub fn DokanMain(
        DokanOptions: PDOKAN_OPTIONS,
        DokanOperations: PDOKAN_OPERATIONS,
    ) -> DokanMainResult;

    /// Mount a new Dokan Volume.
    ///
    /// It is mandatory to have called \ref DokanInit previously to use this API.
    ///
    /// This function returns directly on device mount or on failure.
    /// See [`DokanMainResult`] for possible errors.
    ///
    /// [`DokanWaitForFileSystemClosed`] can be used to wait until the device is unmount.
    ///
    /// # Arguments
    /// `DokanOptions`: a [DOKAN_OPTIONS] that describe the mount.
    /// `DokanOperations`: Instance of [DOKAN_OPERATIONS] that will be called for each request made by the kernel.
    /// `DokanInstance`: Dokan mount instance context that can be used for related instance calls like [`DokanIsFileSystemRunning`].
    ///
    /// # Returns
    /// [`DokanMainResult`] status.
    pub fn DokanCreateFileSystem(
        DokanOptions: PDOKAN_OPTIONS,
        DokanOperations: PDOKAN_OPERATIONS,
        DokanInstance: *mut DOKAN_HANDLE,
    ) -> DokanMainResult;

    /// Check if the FileSystem is still running or not.
    ///
    /// # Arguments
    /// `DokanInstance`: The dokan mount context created by [`DokanCreateFileSystem`].
    ///
    /// # Return
    /// Whether the FileSystem is still running or not.
    pub fn DokanIsFileSystemRunning(DokanInstance: DOKAN_HANDLE) -> BOOL;

    /// Wait until the FileSystem is unmount.
    ///
    /// # Arguments
    /// `DokanInstance`: The dokan mount context created by \ref DokanCreateFileSystem .
    /// `dwMilliseconds`: The time-out interval, in milliseconds. See <a href="https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitforsingleobject">WaitForSingleObject</a>.
    ///
    /// # Return
    /// See <a href="https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitforsingleobject">WaitForSingleObject</a> for a description of return values.
    pub fn DokanWaitForFileSystemClosed(
        DokanInstance: DOKAN_HANDLE,
        dwMilliseconds: DWORD,
    ) -> DWORD;

    /// Unmount the Dokan instance.
    ///
    /// Unmount and wait until all resources of the DokanInstance are released.
    ///
    /// # Arguments
    /// `DokanInstance`: The dokan mount context created by [DokanCreateFileSystem].
    pub fn DokanCloseHandle(DokanInstance: DOKAN_HANDLE);

    /// Get the version of Dokan.
    ///
    /// The returned ULONG is the version number without the dots.
    ///# Returns
    /// The version of Dokan
    pub fn DokanVersion() -> ULONG;
}
