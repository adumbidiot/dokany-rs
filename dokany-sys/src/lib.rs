#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

pub use std::os::raw::c_int;
pub use std::os::windows::raw::HANDLE;
pub use windows_sys::core::PCWSTR;
pub use windows_sys::Win32::Foundation::BOOL;
pub use windows_sys::Win32::Foundation::BOOLEAN;
pub use windows_sys::Win32::Foundation::CHAR;
pub use windows_sys::Win32::Foundation::FALSE;
pub use windows_sys::Win32::Foundation::MAX_PATH;
pub use windows_sys::Win32::Foundation::TRUE;

pub type USHORT = u16;
pub type ULONG = u32;
pub type ULONG64 = u64;
pub type LPCWSTR = PCWSTR;
pub type DWORD = u32;
pub type WCHAR = u16;
pub type PULONG = *mut ULONG;
pub type PVOID = *mut std::os::raw::c_void;
pub type UCHAR = u8;

pub type DokanMainResult = c_int;

/// Dokan mount succeed.
pub const DOKAN_SUCCESS: DokanMainResult = 0;
/// Dokan mount error.
pub const DOKAN_ERROR: DokanMainResult = -1;
/// Dokan mount failed - Bad drive letter.
pub const DOKAN_DRIVE_LETTER_ERROR: DokanMainResult = -2;
/// Dokan mount failed - Can't install driver.
pub const DOKAN_DRIVER_INSTALL_ERROR: DokanMainResult = -3;
/// Dokan mount failed - Driver answer that something is wrong.
pub const DOKAN_START_ERROR: DokanMainResult = -3;
/// Dokan mount failed.
///
/// Can't assign a drive letter or mount point.
/// Probably already used by another volume.
pub const DOKAN_MOUNT_ERROR: DokanMainResult = -5;
/// Dokan mount failed.
///
/// Mount point is invalid.
pub const DOKAN_MOUNT_POINT_ERROR: DokanMainResult = -6;
/// Dokan mount failed.
///
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

/// Dokan file information on the current operation.
#[repr(C)]
pub struct DOKAN_FILE_INFO {
    /// Context that can be used to carry information between operations.
    ///
    /// The context can carry whatever type like `HANDLE`, struct, int,
    /// internal reference that will help the implementation understand the request context of the event.
    pub Context: ULONG64,

    /// Reserved. Used internally by Dokan library. Never modify.
    pub DokanContext: ULONG64,

    /// A pointer to DOKAN_OPTIONS which was passed to [DokanMain] or [DokanCreateFileSystem].
    pub DokanOptions: PDOKAN_OPTIONS,

    /// Reserved. Used internally by Dokan library. Never modify.
    ///
    /// If the processing for the event requires extra data to be associated with it
    /// then a pointer to that data can be placed here
    pub ProcessingContext: PVOID,

    /// Process ID for the thread that originally requested a given I/O operation.
    pub ProcessId: ULONG,

    /// Requesting a directory file.
    ///
    /// Must be set in [DOKAN_OPERATIONS].ZwCreateFile if the file appears to be a folder.
    pub IsDirectory: UCHAR,

    /// Flag if the file has to be deleted during DOKAN_OPERATIONS. Cleanup event.
    pub DeleteOnClose: UCHAR,

    /// Read or write is paging IO.
    pub PagingIo: UCHAR,

    /// Read or write is synchronous IO.
    pub SynchronousIo: UCHAR,

    /// Read or write directly from data source without cache
    pub Nocache: UCHAR,

    /// If `TRUE`, write to the current end of file instead of using the Offset parameter.
    pub WriteToEndOfFile: UCHAR,
}

pub type PDOKAN_FILE_INFO = *mut DOKAN_FILE_INFO;

/// Dokan Mount point information
#[repr(C)]
pub struct DOKAN_MOUNT_POINT_INFO {
    /// File System Type
    pub Type: ULONG,
    /// Mount point. Can be "M:\" (drive letter) or "C:\mount\dokan" (path in NTFS)
    pub MountPoint: [WCHAR; MAX_PATH as usize],
    /// UNC name used for network volume
    pub UNCName: [WCHAR; 64],
    /// Disk Device Name
    pub DeviceName: [WCHAR; 64],
    /// Session ID of calling process
    pub SessionId: ULONG,
    /// Contains information about the flags on the mount
    pub MountOptions: ULONG,
}

pub type PDOKAN_MOUNT_POINT_INFO = *mut DOKAN_MOUNT_POINT_INFO;

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
    /// Unmount and wait until all resources of the `DokanInstance` are released.
    ///
    /// # Arguments
    /// `DokanInstance`: The dokan mount context created by [DokanCreateFileSystem].
    pub fn DokanCloseHandle(DokanInstance: DOKAN_HANDLE);

    /// Unmount a Dokan device from a driver letter.
    ///
    /// # Arguments
    /// `DriveLetter`: Dokan driver letter to unmount.
    ///
    /// # Return
    /// `TRUE` if device was unmounted or `FALSE` in case of failure or device not found.
    pub fn DokanUnmount(DriveLetter: WCHAR) -> BOOL;

    /// Unmount a Dokan device from a mount point
    ///
    /// # Parameters
    /// * MountPoint Mount point to unmount ("Z", "Z:", "Z:\", "Z:\MyMountPoint").
    ///
    /// # Return
    /// `TRUE` if device was unmounted or `FALSE` in case of failure or device not found.
    pub fn DokanRemoveMountPoint(MountPoint: LPCWSTR) -> BOOL;

    /// Checks whether Name matches Expression
    ///
    /// Behave like `FsRtlIsNameInExpression` routine from <a href="https://msdn.microsoft.com/en-us/library/ff546850(v=VS.85).aspx">Microsoft</a>\n
    /// `*` (asterisk) Matches zero or more characters.\n
    /// <tt>?</tt> (question mark) Matches a single character.\n
    /// `DOS_DOT` (`"` quotation mark) Matches either a period or zero characters beyond the name string.\n
    /// `DOS_QM` (`>` greater than) Matches any single character or, upon encountering a period or end
    ///        of name string, advances the expression to the end of the set of
    ///       contiguous DOS_QMs.\n
    /// `DOS_STAR` (`<` less than) Matches zero or more characters until encountering and matching
    ///          the final `.` in the name.
    ///
    /// # Arguments
    /// `Expression`: Expression can contain any of the above characters.
    /// `Name`: Name to check
    /// `IgnoreCase`: Case sensitive or not
    ///
    /// # Return
    /// `result` if name matches the expression
    pub fn DokanIsNameInExpression(Expression: LPCWSTR, Name: LPCWSTR, IgnoreCase: BOOL) -> BOOL;

    /// Get the version of Dokan.
    ///
    /// The returned ULONG is the version number without the dots.
    ///# Returns
    /// The version of Dokan
    pub fn DokanVersion() -> ULONG;

    /// Get the version of the Dokan driver.
    ///
    /// The returned ULONG is the version number without the dots.
    ///
    /// # Return
    /// The version of Dokan driver or 0 on failure.
    pub fn DokanDriverVersion() -> ULONG;

    /// Extends the timeout of the current IO operation in driver.
    ///
    /// # Parameters
    /// `Timeout`: Extended time in milliseconds requested.
    /// `DokanFileInfo`: [DOKAN_FILE_INFO] of the operation to extend.
    ///
    /// # Return
    /// If the operation was successful.
    pub fn DokanResetTimeout(Timeout: ULONG, DokanFileInfo: PDOKAN_FILE_INFO) -> BOOL;

    /// Get the handle to Access Token.
    ///
    /// This method needs be called in \ref DOKAN_OPERATIONS.ZwCreateFile callback.
    /// The caller must call <a href="https://msdn.microsoft.com/en-us/library/windows/desktop/ms724211(v=vs.85).aspx">CloseHandle</a>
    /// for the returned handle.
    ///
    /// # Arguments
    /// `DokanFileInfo`: [DOKAN_FILE_INFO] of the operation to extend.
    ///
    /// # Return
    /// A handle to the account token for the user on whose behalf the code is running.
    pub fn DokanOpenRequestorToken(DokanFileInfo: PDOKAN_FILE_INFO) -> HANDLE;

    /// Get active Dokan mount points.
    ///
    /// Returned array need to be released by calling [DokanReleaseMountPointList]
    ///
    /// # Arguments
    /// `uncOnly`: Get only instances that have UNC Name.
    /// `nbRead`: Number of instances successfully retrieved.
    ///
    /// # Return
    /// Allocate array of DOKAN_MOUNT_POINT_INFO.
    pub fn DokanGetMountPointList(uncOnly: BOOL, nbRead: PULONG) -> PDOKAN_MOUNT_POINT_INFO;

    /// Release Mount point list resources from [DokanGetMountPointList].
    ///
    /// After [DokanGetMountPointList] call you will receive a dynamically allocated array of DOKAN_MOUNT_POINT_INFO.
    /// This array needs to be released when no longer needed by calling this function.
    ///
    /// # Arguments
    /// `list`: Allocated array of DOKAN_MOUNT_POINT_INFO from [DokanGetMountPointList].
    ///
    /// # Return
    /// Nothing.
    pub fn DokanReleaseMountPointList(list: PDOKAN_MOUNT_POINT_INFO);
}
