#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

pub use std::os::raw::c_int;
pub use std::os::raw::c_void;
pub use std::os::windows::raw::HANDLE;
pub use windows_sys::core::PCWSTR;
pub use windows_sys::core::PWSTR;
pub use windows_sys::Win32::Foundation::BOOL;
pub use windows_sys::Win32::Foundation::BOOLEAN;
pub use windows_sys::Win32::Foundation::CHAR;
pub use windows_sys::Win32::Foundation::FALSE;
pub use windows_sys::Win32::Foundation::FILETIME;
pub use windows_sys::Win32::Foundation::MAX_PATH;
pub use windows_sys::Win32::Foundation::NTSTATUS;
pub use windows_sys::Win32::Foundation::STATUS_INTERNAL_ERROR;
pub use windows_sys::Win32::Foundation::STATUS_NOT_IMPLEMENTED;
pub use windows_sys::Win32::Foundation::STATUS_NO_SUCH_FILE;
pub use windows_sys::Win32::Foundation::STATUS_SUCCESS;
pub use windows_sys::Win32::Foundation::TRUE;
pub use windows_sys::Win32::Security::SECURITY_DESCRIPTOR;
pub use windows_sys::Win32::Storage::FileSystem::BY_HANDLE_FILE_INFORMATION;
pub use windows_sys::Win32::Storage::FileSystem::DELETE;
pub use windows_sys::Win32::Storage::FileSystem::FILE_ACCESS_FLAGS;
pub use windows_sys::Win32::Storage::FileSystem::FILE_ADD_FILE;
pub use windows_sys::Win32::Storage::FileSystem::FILE_ADD_SUBDIRECTORY;
pub use windows_sys::Win32::Storage::FileSystem::FILE_ALL_ACCESS;
pub use windows_sys::Win32::Storage::FileSystem::FILE_APPEND_DATA;
pub use windows_sys::Win32::Storage::FileSystem::FILE_CREATE_PIPE_INSTANCE;
pub use windows_sys::Win32::Storage::FileSystem::FILE_DELETE_CHILD;
pub use windows_sys::Win32::Storage::FileSystem::FILE_EXECUTE;
pub use windows_sys::Win32::Storage::FileSystem::FILE_LIST_DIRECTORY;
pub use windows_sys::Win32::Storage::FileSystem::FILE_READ_ATTRIBUTES;
pub use windows_sys::Win32::Storage::FileSystem::FILE_READ_DATA;
pub use windows_sys::Win32::Storage::FileSystem::FILE_READ_EA;
pub use windows_sys::Win32::Storage::FileSystem::FILE_TRAVERSE;
pub use windows_sys::Win32::Storage::FileSystem::FILE_WRITE_ATTRIBUTES;
pub use windows_sys::Win32::Storage::FileSystem::FILE_WRITE_DATA;
pub use windows_sys::Win32::Storage::FileSystem::FILE_WRITE_EA;
pub use windows_sys::Win32::Storage::FileSystem::READ_CONTROL;
pub use windows_sys::Win32::Storage::FileSystem::SPECIFIC_RIGHTS_ALL;
pub use windows_sys::Win32::Storage::FileSystem::STANDARD_RIGHTS_ALL;
pub use windows_sys::Win32::Storage::FileSystem::STANDARD_RIGHTS_EXECUTE;
pub use windows_sys::Win32::Storage::FileSystem::STANDARD_RIGHTS_READ;
pub use windows_sys::Win32::Storage::FileSystem::STANDARD_RIGHTS_REQUIRED;
pub use windows_sys::Win32::Storage::FileSystem::STANDARD_RIGHTS_WRITE;
pub use windows_sys::Win32::Storage::FileSystem::SYNCHRONIZE;
pub use windows_sys::Win32::Storage::FileSystem::WIN32_FIND_DATAW;
pub use windows_sys::Win32::Storage::FileSystem::WIN32_FIND_STREAM_DATA;
pub use windows_sys::Win32::Storage::FileSystem::WRITE_DAC;
pub use windows_sys::Win32::Storage::FileSystem::WRITE_OWNER;
pub use windows_sys::Win32::System::SystemServices::ACCESS_SYSTEM_SECURITY;
pub use windows_sys::Win32::System::SystemServices::FILE_CASE_PRESERVED_NAMES;
pub use windows_sys::Win32::System::SystemServices::FILE_CASE_SENSITIVE_SEARCH;
pub use windows_sys::Win32::System::SystemServices::FILE_DAX_VOLUME;
pub use windows_sys::Win32::System::SystemServices::FILE_FILE_COMPRESSION;
pub use windows_sys::Win32::System::SystemServices::FILE_NAMED_STREAMS;
pub use windows_sys::Win32::System::SystemServices::FILE_PERSISTENT_ACLS;
pub use windows_sys::Win32::System::SystemServices::FILE_READ_ONLY_VOLUME;
pub use windows_sys::Win32::System::SystemServices::FILE_RETURNS_CLEANUP_RESULT_INFO;
pub use windows_sys::Win32::System::SystemServices::FILE_SEQUENTIAL_WRITE_ONCE;
pub use windows_sys::Win32::System::SystemServices::FILE_SUPPORTS_BLOCK_REFCOUNTING;
pub use windows_sys::Win32::System::SystemServices::FILE_SUPPORTS_ENCRYPTION;
pub use windows_sys::Win32::System::SystemServices::FILE_SUPPORTS_EXTENDED_ATTRIBUTES;
pub use windows_sys::Win32::System::SystemServices::FILE_SUPPORTS_GHOSTING;
pub use windows_sys::Win32::System::SystemServices::FILE_SUPPORTS_HARD_LINKS;
pub use windows_sys::Win32::System::SystemServices::FILE_SUPPORTS_INTEGRITY_STREAMS;
pub use windows_sys::Win32::System::SystemServices::FILE_SUPPORTS_OBJECT_IDS;
pub use windows_sys::Win32::System::SystemServices::FILE_SUPPORTS_OPEN_BY_FILE_ID;
pub use windows_sys::Win32::System::SystemServices::FILE_SUPPORTS_POSIX_UNLINK_RENAME;
pub use windows_sys::Win32::System::SystemServices::FILE_SUPPORTS_REMOTE_STORAGE;
pub use windows_sys::Win32::System::SystemServices::FILE_SUPPORTS_REPARSE_POINTS;
pub use windows_sys::Win32::System::SystemServices::FILE_SUPPORTS_SPARSE_FILES;
pub use windows_sys::Win32::System::SystemServices::FILE_SUPPORTS_SPARSE_VDL;
pub use windows_sys::Win32::System::SystemServices::FILE_SUPPORTS_TRANSACTIONS;
pub use windows_sys::Win32::System::SystemServices::FILE_SUPPORTS_USN_JOURNAL;
pub use windows_sys::Win32::System::SystemServices::FILE_UNICODE_ON_DISK;
pub use windows_sys::Win32::System::SystemServices::FILE_VOLUME_IS_COMPRESSED;
pub use windows_sys::Win32::System::SystemServices::FILE_VOLUME_QUOTAS;
pub use windows_sys::Win32::System::SystemServices::GENERIC_ALL;
pub use windows_sys::Win32::System::SystemServices::GENERIC_EXECUTE;
pub use windows_sys::Win32::System::SystemServices::GENERIC_READ;
pub use windows_sys::Win32::System::SystemServices::GENERIC_WRITE;
pub use windows_sys::Win32::System::SystemServices::MAXIMUM_ALLOWED;

// Primitives
pub type USHORT = u16;
pub type ULONG = u32;
pub type ULONG64 = u64;
pub type LPCWSTR = PCWSTR;
pub type DWORD = u32;
pub type WCHAR = u16;
pub type PULONG = *mut ULONG;
pub type PVOID = *mut c_void;
pub type UCHAR = u8;
pub type ACCESS_MASK = FILE_ACCESS_FLAGS;
pub type LPVOID = PVOID;
pub type LPDWORD = *mut DWORD;
pub type LONGLONG = i64;
pub type LPCVOID = *const c_void;
pub type LPBY_HANDLE_FILE_INFORMATION = *mut BY_HANDLE_FILE_INFORMATION;
pub type PWIN32_FIND_DATAW = *mut WIN32_FIND_DATAW;
pub type ULONGLONG = u64;
pub type PULONGLONG = *mut u64;
pub type LPWSTR = PWSTR;
pub type PSECURITY_DESCRIPTOR = *mut SECURITY_DESCRIPTOR;
pub type SECURITY_INFORMATION = DWORD;
pub type PSECURITY_INFORMATION = *mut SECURITY_INFORMATION;
pub type PWIN32_FIND_STREAM_DATA = *mut WIN32_FIND_STREAM_DATA;

pub type DokanOptionFlag = ULONG;

/// Enable ouput debug message
pub const DOKAN_OPTION_DEBUG: DokanOptionFlag = 1;
/// Enable ouput debug message to stderr
pub const DOKAN_OPTION_STDERR: DokanOptionFlag = 1 << 1;
/// Enable the use of alternate stream paths in the form
/// <file-name>:<stream-name>. If this is not specified then the driver will
// fail any attempt to access a path with a colon.
pub const DOKAN_OPTION_ALT_STREAM: DokanOptionFlag = 1 << 2;
/// Enable mount drive as write-protected
pub const DOKAN_OPTION_WRITE_PROTECT: DokanOptionFlag = 1 << 3;
/// Use network drive - Dokan network provider needs to be installed and a `DOKAN_OPTIONS.UNCName` provided
pub const DOKAN_OPTION_NETWORK: DokanOptionFlag = 1 << 4;
/// Use removable drive
///
/// Be aware that on some environments, the userland application will be denied
/// to communicate with the drive which will result in a unwanted unmount.
/// See <a href="https://github.com/dokan-dev/dokany/issues/843">Issue #843</a>
pub const DOKAN_OPTION_REMOVABLE: DokanOptionFlag = 1 << 5;
/// Use Windows Mount Manager.
///
/// This option is highly recommended to use for better system integration.
/// If a drive letter is used but is busy, Mount manager will assign one for us and
/// `DOKAN_OPERATIONS.Mounted` parameters will contain the new mount point.
pub const DOKAN_OPTION_MOUNT_MANAGER: DokanOptionFlag = 1 << 6;
/// Mount the drive on current session only
pub const DOKAN_OPTION_CURRENT_SESSION: DokanOptionFlag = 1 << 7;
/// Enable Lockfile/Unlockfile operations.
///
/// Otherwise Dokan will take care of it
pub const DOKAN_OPTION_FILELOCK_USER_MODE: DokanOptionFlag = 1 << 8;
/// Enable Case sensitive path.
///
/// By default all path are case insensitive.
/// For case sensitive: \dir\File & \diR\\file are different files
/// but for case insensitive they are the same.
pub const DOKAN_OPTION_CASE_SENSITIVE: DokanOptionFlag = 1 << 9;
/// Allows unmounting of network drive via explorer
pub const DOKAN_OPTION_ENABLE_UNMOUNT_NETWORK_DRIVE: DokanOptionFlag = 1 << 10;
/// Forward the kernel driver global and volume logs to the userland.
///
/// Can be very slow if single thread is enabled.
pub const DOKAN_OPTION_DISPATCH_DRIVER_LOGS: DokanOptionFlag = 1 << 11;
/// Pull batches of events from the driver instead of a single one and execute them parallelly.
///
/// This option should only be used on computers with low cpu count
/// and userland filesystem taking time to process requests (like remote storage).
pub const DOKAN_OPTION_ALLOW_IPC_BATCHING: DokanOptionFlag = 1 << 12;

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
pub const DOKAN_START_ERROR: DokanMainResult = -4;
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

pub type DOKAN_HANDLE = *const c_void;

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

pub type PDOKAN_IO_SECURITY_CONTEXT = *mut c_void;

/// FillFindData Used to add an entry in FindFiles operation
///
/// # Return
/// 1 if buffer is full, otherwise 0 (currently it never returns 1)
pub type PFillFindData =
    Option<unsafe extern "stdcall" fn(PWIN32_FIND_DATAW, PDOKAN_FILE_INFO) -> c_int>;

/// FillFindStreamData Used to add an entry in FindStreams
///
/// # Return
/// `FALSE` if the buffer is full, otherwise TRUE
pub type PFillFindStreamData =
    Option<unsafe extern "stdcall" fn(PWIN32_FIND_STREAM_DATA, PVOID) -> BOOL>;

pub type ZwCreateFileCallback = unsafe extern "stdcall" fn(
    FileName: LPCWSTR,
    SecurityContext: PDOKAN_IO_SECURITY_CONTEXT,
    DesiredAccess: ACCESS_MASK,
    FileAttributes: ULONG,
    ShareAccess: ULONG,
    CreateDisposition: ULONG,
    CreateOptions: ULONG,
    DokanFileInfo: PDOKAN_FILE_INFO,
) -> NTSTATUS;
pub type CleanupCallback =
    unsafe extern "stdcall" fn(FileName: LPCWSTR, DokanFileInfo: PDOKAN_FILE_INFO);
pub type CloseFileCallback =
    unsafe extern "stdcall" fn(FileName: LPCWSTR, DokanFileInfo: PDOKAN_FILE_INFO);
pub type ReadFileCallback = unsafe extern "stdcall" fn(
    FileName: LPCWSTR,
    Buffer: LPVOID,
    BufferLength: DWORD,
    ReadLength: LPDWORD,
    Offset: LONGLONG,
    DokanFileInfo: PDOKAN_FILE_INFO,
) -> NTSTATUS;
pub type WriteFileCallback = unsafe extern "stdcall" fn(
    FileName: LPCWSTR,
    Buffer: LPCVOID,
    NumberOfBytesToWrite: DWORD,
    NumberOfBytesWritten: LPDWORD,
    Offset: LONGLONG,
    DokanFileInfo: PDOKAN_FILE_INFO,
) -> NTSTATUS;
pub type FlushFileBuffersCallback =
    unsafe extern "stdcall" fn(FileName: LPCWSTR, DokanFileInfo: PDOKAN_FILE_INFO) -> NTSTATUS;
pub type GetFileInformationCallback = unsafe extern "stdcall" fn(
    FileName: LPCWSTR,
    Buffer: LPBY_HANDLE_FILE_INFORMATION,
    DokanFileInfo: PDOKAN_FILE_INFO,
);
pub type FindFilesCallback = unsafe extern "stdcall" fn(
    FileName: LPCWSTR,
    FillFindData: PFillFindData,
    DokanFileInfo: PDOKAN_FILE_INFO,
) -> NTSTATUS;
pub type FindFilesWithPatternCallback = unsafe extern "stdcall" fn(
    PathName: LPCWSTR,
    SearchPattern: LPCWSTR,
    FillFindData: PFillFindData,
    DokanFileInfo: PDOKAN_FILE_INFO,
) -> NTSTATUS;
pub type SetFileAttributesCallback = unsafe extern "stdcall" fn(
    FileName: LPCWSTR,
    FileAttributes: DWORD,
    DokanFileInfo: PDOKAN_FILE_INFO,
) -> NTSTATUS;
pub type SetFileTimeCallback = unsafe extern "stdcall" fn(
    FileName: LPCWSTR,
    CreationTime: *const FILETIME,
    LastAccessTime: *const FILETIME,
    LastWriteTime: *const FILETIME,
    DokanFileInfo: PDOKAN_FILE_INFO,
) -> NTSTATUS;
pub type DeleteFileCallback =
    unsafe extern "stdcall" fn(FileName: LPCWSTR, DokanFileInfo: PDOKAN_FILE_INFO) -> NTSTATUS;
pub type DeleteDirectoryCallback =
    unsafe extern "stdcall" fn(FileName: LPCWSTR, DokanFileInfo: PDOKAN_FILE_INFO) -> NTSTATUS;
pub type MoveFileCallback = unsafe extern "stdcall" fn(
    FileName: LPCWSTR,
    NewFileName: LPCWSTR,
    ReplaceIfExisting: BOOL,
    DokanFileInfo: PDOKAN_FILE_INFO,
) -> NTSTATUS;
pub type SetEndOfFileCallback = unsafe extern "stdcall" fn(
    FileName: LPCWSTR,
    ByteOffset: LONGLONG,
    DokanFileInfo: PDOKAN_FILE_INFO,
) -> NTSTATUS;
pub type SetAllocationSizeCallback = unsafe extern "stdcall" fn(
    FileName: LPCWSTR,
    AllocSize: LONGLONG,
    DokanFileInfo: PDOKAN_FILE_INFO,
) -> NTSTATUS;
pub type LockFileCallback = unsafe extern "stdcall" fn(
    FileName: LPCWSTR,
    ByteOffset: LONGLONG,
    Length: LONGLONG,
    DokanFileInfo: PDOKAN_FILE_INFO,
) -> NTSTATUS;
pub type UnlockFileCallback = unsafe extern "stdcall" fn(
    FileName: LPCWSTR,
    ByteOffset: LONGLONG,
    Length: LONGLONG,
    DokanFileInfo: PDOKAN_FILE_INFO,
) -> NTSTATUS;
pub type GetDiskFreeSpaceCallback = unsafe extern "stdcall" fn(
    FreeBytesAvailable: PULONGLONG,
    TotalNumberOfBytes: PULONGLONG,
    TotalNumberOfFreeBytes: PULONGLONG,
    DokanFileInfo: PDOKAN_FILE_INFO,
) -> NTSTATUS;
pub type GetVolumeInformationCallback = unsafe extern "stdcall" fn(
    VolumeNameBuffer: LPWSTR,
    VolumeNameSize: DWORD,
    VolumeSerialNumber: LPDWORD,
    MaximumComponentLength: LPDWORD,
    FileSystemFlags: LPDWORD,
    FileSystemNameBuffer: LPWSTR,
    FileSystemNameSize: DWORD,
    DokanFileInfo: PDOKAN_FILE_INFO,
) -> NTSTATUS;
pub type MountedCallback =
    unsafe extern "stdcall" fn(MountPoint: LPCWSTR, DokanFileInfo: PDOKAN_FILE_INFO) -> NTSTATUS;
pub type Unmounted = unsafe extern "stdcall" fn(DokanFileInfo: PDOKAN_FILE_INFO) -> NTSTATUS;
pub type GetFileSecurityCallback = unsafe extern "stdcall" fn(
    FileName: LPCWSTR,
    SecurityInformation: PSECURITY_INFORMATION,
    SecurityDescriptor: PSECURITY_DESCRIPTOR,
    BufferLength: ULONG,
    LengthNeeded: PULONG,
    DokanFileInfo: PDOKAN_FILE_INFO,
) -> NTSTATUS;
pub type SetFileSecurityCallback = unsafe extern "stdcall" fn(
    FileName: LPCWSTR,
    SecurityInformation: PSECURITY_INFORMATION,
    SecurityDescriptor: PSECURITY_DESCRIPTOR,
    BufferLength: ULONG,
    DokanFileInfo: PDOKAN_FILE_INFO,
) -> NTSTATUS;
pub type FindStreamsCallback = unsafe extern "stdcall" fn(
    FileName: LPCWSTR,
    FillFindStreamData: PFillFindStreamData,
    FindStreamContext: PVOID,
    DokanFileInfo: PDOKAN_FILE_INFO,
) -> NTSTATUS;

/// Dokan API callbacks interface
///
/// DOKAN_OPERATIONS is a struct of callbacks that describe all Dokan API operations
/// that will be called when Windows access to the filesystem.
///
/// If an error occurs, return NTSTATUS (https://support.microsoft.com/en-us/kb/113996).
/// Win32 Error can be converted to `NTSTATUS` with [DokanNtStatusFromWin32]
///
/// All callbacks can be set to `NULL` or return `STATUS_NOT_IMPLEMENTED`
// if supporting one of them is not desired. Be aware that returning such values to important callbacks
/// such as DOKAN_OPERATIONS.ZwCreateFile / DOKAN_OPERATIONS.ReadFile / ... would make the filesystem not work or become unstable.
#[repr(C)]
pub struct DOKAN_OPERATIONS {
    /// CreateFile Dokan API callback
    ///
    /// CreateFile is called each time a request is made on a file system object.
    ///
    /// In case `OPEN_ALWAYS` & `CREATE_ALWAYS` are successfully opening an
    /// existing file, `STATUS_OBJECT_NAME_COLLISION` should be returned instead of `STATUS_SUCCESS`.
    /// This will inform Dokan that the file has been opened and not created during the request.
    ///
    /// If the file is a directory, CreateFile is also called.
    /// In this case, CreateFile should return `STATUS_SUCCESS` when that directory
    /// can be opened and DOKAN_FILE_INFO.IsDirectory has to be set to `TRUE`.
    /// On the other hand, if DOKAN_FILE_INFO.IsDirectory is set to `TRUE`
    /// but the path targets a file, `STATUS_NOT_A_DIRECTORY` must be returned.
    ///
    /// DOKAN_FILE_INFO.Context can be used to store Data (like `HANDLE`)
    /// that can be retrieved in all other requests related to the Context.
    /// To avoid memory leak, Context needs to be released in DOKAN_OPERATIONS.Cleanup.
    ///
    /// # Arguments
    /// `FileName`: File path requested by the Kernel on the FileSystem.
    /// `SecurityContext`: SecurityContext, see https://msdn.microsoft.com/en-us/library/windows/hardware/ff550613(v=vs.85).aspx
    /// `DesiredAccess`: Specifies an <a href="https://msdn.microsoft.com/en-us/library/windows/hardware/ff540466(v=vs.85).aspx">ACCESS_MASK</a> value that determines the requested access to the object.
    /// `FileAttributes`: Specifies one or more FILE_ATTRIBUTE_XXX flags, which represent the file attributes to set if a file is created or overwritten.
    /// `ShareAccess`: Type of share access, which is specified as zero or any combination of FILE_SHARE_* flags.
    /// `CreateDisposition`: Specifies the action to perform if the file does or does not exist.
    /// `CreateOptions`: Specifies the options to apply when the driver creates or opens the file.
    /// `DokanFileInfo`: Information about the file or directory.
    ///
    /// # Return
    /// `STATUS_SUCCESS` on success or NTSTATUS appropriate to the request result.
    ///
    /// # References
    /// See <a href="https://msdn.microsoft.com/en-us/library/windows/hardware/ff566424(v=vs.85).aspx">See ZwCreateFile for more information about the parameters of this callback (MSDN).</a>
    /// See DokanMapKernelToUserCreateFileFlags
    pub ZwCreateFile: Option<ZwCreateFileCallback>,

    /// Cleanup Dokan API callback
    ///
    /// Cleanup request before `CloseFile` is called.
    ///
    /// When DOKAN_FILE_INFO.DeleteOnClose is `TRUE`, the file in Cleanup must be deleted.
    /// The function cannot fail therefore the filesystem need to ensure ahead
    /// that a the delete can safely happen during Cleanup.
    /// See DeleteFile documentation for explanation.
    ///
    /// # Arguments
    /// `FileName`: File path requested by the Kernel on the FileSystem.
    /// `DokanFileInfo`: Information about the file or directory.
    ///
    /// # References
    /// See DeleteFile
    /// See DeleteDirectory
    pub Cleanup: Option<CleanupCallback>,

    /// CloseFile Dokan API callback
    ///
    /// Clean remaining Context
    ///
    /// CloseFile is called at the end of the life of the context.
    /// Anything remaining in `DOKAN_FILE_INFO.Context` must be cleared before returning.
    ///
    /// # Arguments
    /// `FileName`: File path requested by the Kernel on the FileSystem.
    /// `DokanFileInfo`: Information about the file or directory.
    pub CloseFile: Option<CloseFileCallback>,

    /// ReadFile Dokan API callback
    ///
    /// ReadFile callback on the file previously opened in DOKAN_OPERATIONS.ZwCreateFile.
    /// It can be called by different threads at the same time, so the read/context has to be thread safe.
    ///
    /// When apps make use of memory mapped files, DOKAN_OPERATIONS.WriteFile or DOKAN_OPERATIONS.ReadFile
    /// functions may be invoked after DOKAN_OPERATIONS.Cleanup in order to complete the I/O operations.
    /// The file system application should also properly work in this case.
    ///
    /// # Arguments
    /// `FileName`: File path requested by the Kernel on the FileSystem.
    /// `Buffer`: Read buffer that has to be filled with the read result.
    /// `BufferLength`: Buffer length and read size to continue with.
    /// `ReadLength`: Total data size that has been read.
    /// `Offset`: Offset from where the read has to be continued.
    /// `DokanFileInfo`: Information about the file or directory.
    ///
    /// # Return
    /// `STATUS_SUCCESS` on success or NTSTATUS appropriate to the request result.
    ///
    /// # References
    /// See WriteFile
    pub ReadFile: Option<ReadFileCallback>,

    /// WriteFile Dokan API callback
    ///
    /// WriteFile callback on the file previously opened in DOKAN_OPERATIONS.ZwCreateFile
    /// It can be called by different threads at the same time, sp the write/context has to be thread safe.
    ///
    /// When apps make use of memory mapped files ( DOKAN_FILE_INFO.PagingIo ),
    /// DOKAN_OPERATIONS.WriteFile or DOKAN_OPERATIONS.ReadFile
    // functions may be invoked after DOKAN_OPERATIONS.Cleanup in order to complete the I/O operations.
    /// The file system application should also properly work in this case.
    /// This type of request should follow Windows rules like not extending the current file size.
    ///
    /// # Arguments  
    /// `FileName`: File path requested by the Kernel on the FileSystem.
    ///`Buffer`: Data that has to be written.
    /// `NumberOfBytesToWrite`: Buffer length and write size to continue with.
    /// `NumberOfBytesWritten`: Total number of bytes that have been written.
    /// `Offset`: Offset from where the write has to be continued.
    /// `DokanFileInfo`: Information about the file or directory.
    ///
    /// # Return
    /// `STATUS_SUCCESS` on success or NTSTATUS appropriate to the request result.
    ///
    /// References
    /// See ReadFile
    pub WriteFile: Option<WriteFileCallback>,

    /// FlushFileBuffers Dokan API callback
    ///
    /// Clears buffers for this context and causes any buffered data to be written to the file.
    ///
    /// # Arguments
    /// FileName File path requested by the Kernel on the FileSystem.
    /// DokanFileInfo Information about the file or directory.
    ///
    /// # Returns
    /// `STATUS_SUCCESS` on success or NTSTATUS appropriate to the request result.
    pub FlushFileBuffers: Option<FlushFileBuffersCallback>,

    /// GetFileInformation Dokan API callback
    ///
    /// Get specific information on a file.
    ///
    /// # Arguments
    /// `FileName`: File path requested by the Kernel on the FileSystem.
    /// `Buffer`: BY_HANDLE_FILE_INFORMATION struct to fill.
    /// `DokanFileInfo`: Information about the file or directory.
    ///
    /// # Returns
    /// `STATUS_SUCCESS` on success or NTSTATUS appropriate to the request result.
    pub GetFileInformation: Option<GetFileInformationCallback>,

    /// FindFiles Dokan API callback
    ///
    /// List all files in the requested path.
    /// `DOKAN_OPERATIONS.FindFilesWithPattern` is checked first. If it is not implemented or
    /// returns `STATUS_NOT_IMPLEMENTED`, then FindFiles is called, if assigned.
    /// It is recommended to have this implemented for performance reason.
    ///
    /// # Arguments
    /// `FileName`: File path requested by the Kernel on the FileSystem.
    /// `FillFindData`: Callback that has to be called with PWIN32_FIND_DATAW that contain file information.
    /// `DokanFileInfo`: Information about the file or directory.
    ///
    /// # Return
    /// `STATUS_SUCCESS` on success or NTSTATUS appropriate to the request result.
    ///
    /// # References
    /// See FindFilesWithPattern
    pub FindFiles: Option<FindFilesCallback>,

    /// FindFilesWithPattern Dokan API callback
    ///
    /// Same as `DOKAN_OPERATIONS.FindFiles` but with a search pattern.\n
    /// The search pattern is a Windows MS-DOS-style expression.
    /// It can contain wild cards and extended characters or none of them. See [DokanIsNameInExpression].
    ///
    /// If the function is not implemented, `DOKAN_OPERATIONS.FindFiles`
    /// will be called instead and the result will be filtered internally by the library.
    /// It is recommended to have this implemented for performance reason.
    ///
    /// # Arguments
    /// `PathName`: Path requested by the Kernel on the FileSystem.
    /// `SearchPattern`: Search pattern.
    /// `FillFindData`: Callback that has to be called with PWIN32_FIND_DATAW that contains file information.
    /// `DokanFileInfo`: Information about the file or directory.
    ///
    /// # Returns
    /// `STATUS_SUCCESS` on success or NTSTATUS appropriate to the request result.
    ///
    /// # References
    /// See FindFiles
    /// See DokanIsNameInExpression
    pub FindFilesWithPattern: Option<FindFilesWithPatternCallback>,

    /// SetFileAttributes Dokan API callback
    ///
    /// Set file attributes on a specific file
    ///
    /// # Arguments
    /// FileName File path requested by the Kernel on the FileSystem.
    /// FileAttributes FileAttributes to set on file.
    /// DokanFileInfo Information about the file or directory.
    ///
    /// # Return
    /// `STATUS_SUCCESS` on success or NTSTATUS appropriate to the request result.
    pub SetFileAttributes: Option<SetFileAttributesCallback>,

    /// SetFileTime Dokan API callback
    ///
    /// Set file attributes on a specific file
    ///
    /// # Arguments
    /// `FileName`: File path requested by the Kernel on the FileSystem.
    /// `CreationTime`: Creation FILETIME.
    /// `LastAccessTime`: LastAccess FILETIME.
    /// `LastWriteTime`: LastWrite FILETIME.
    /// `DokanFileInfo`: Information about the file or directory.
    ///
    /// # Returns
    /// `STATUS_SUCCESS`: on success or NTSTATUS appropriate to the request result.
    pub SetFileTime: Option<SetFileTimeCallback>,

    /// DeleteFile Dokan API callback
    ///
    ///Check if it is possible to delete a file.
    ///
    /// DeleteFile will also be called with DOKAN_FILE_INFO.DeleteOnClose set to `FALSE`
    /// to notify the driver when the file is no longer requested to be deleted.
    ///
    /// The file in DeleteFile should not be deleted, but instead the file
    /// must be checked as to whether or not it can be deleted,
    /// and `STATUS_SUCCESS` should be returned (when it can be deleted) or
    /// appropriate error codes, such as `STATUS_ACCESS_DENIED` or
    /// `STATUS_OBJECT_NAME_NOT_FOUND`, should be returned.
    ///
    /// When `STATUS_SUCCESS` is returned, a Cleanup call is received afterwards with
    /// DOKAN_FILE_INFO.DeleteOnClose set to `TRUE`. Only then must the closing file
    /// be deleted.
    ///
    /// # Arguments
    /// `FileName`: File path requested by the Kernel on the FileSystem.
    /// `DokanFileInfo`: Information about the file or directory.
    ///
    /// # Return
    /// `STATUS_SUCCESS`: on success or NTSTATUS appropriate to the request result.
    ///
    /// # References
    /// See DeleteDirectory
    /// See Cleanup
    pub DeleteFile: Option<DeleteFileCallback>,

    //// DeleteDirectory Dokan API callback
    ///
    /// Check if it is possible to delete a directory.
    ///
    /// DeleteDirectory will also be called with DOKAN_FILE_INFO.DeleteOnClose set to `FALSE`
    /// to notify the driver when the file is no longer requested to be deleted.
    ///
    /// The Directory in DeleteDirectory should not be deleted, but instead
    /// must be checked as to whether or not it can be deleted,
    /// and `STATUS_SUCCESS` should be returned (when it can be deleted) or
    /// appropriate error codes, such as `STATUS_ACCESS_DENIED`,
    /// `STATUS_OBJECT_PATH_NOT_FOUND`, or `STATUS_DIRECTORY_NOT_EMPTY`, should
    /// be returned.
    ///
    /// When `STATUS_SUCCESS` is returned, a Cleanup call is received afterwards with
    /// DOKAN_FILE_INFO.DeleteOnClose set to `TRUE`. Only then must the closing file
    /// be deleted.
    ///
    /// # Arguments  
    /// `FileName`: File path requested by the Kernel on the FileSystem.
    /// `DokanFileInfo`: Information about the file or directory.
    ///
    /// # Return
    /// `STATUS_SUCCESS` on success or `NTSTATUS` appropriate to the request result.
    ///
    /// # References
    /// [DeleteFile]
    /// [Cleanup]
    pub DeleteDirectory: Option<DeleteDirectoryCallback>,

    /// MoveFile Dokan API callback
    ///
    ///Move a file or directory to a new destination
    ///
    /// # Arguments
    /// `FileName`: Path for the file to be moved.
    /// `NewFileName`: Path for the new location of the file.
    /// `ReplaceIfExisting`: If destination already exists, can it be replaced?
    /// `DokanFileInfo`: Information about the file or directory.
    ///
    /// # Return
    /// `STATUS_SUCCESS` on success or NTSTATUS appropriate to the request result.
    pub MoveFile: Option<MoveFileCallback>,

    /// SetEndOfFile Dokan API callback
    ///
    /// SetEndOfFile is used to truncate or extend a file (physical file size).
    ///
    /// # Arguments
    /// `FileName`: File path requested by the Kernel on the FileSystem.
    /// `ByteOffset`: File length to set.
    /// `DokanFileInfo`: Information about the file or directory.
    ///
    /// # Return
    /// `STATUS_SUCCESS` on success or NTSTATUS appropriate to the request result.
    pub SetEndOfFile: Option<SetEndOfFileCallback>,

    /// SetAllocationSize Dokan API callback
    ///
    /// SetAllocationSize is used to truncate or extend a file.
    ///
    /// # Arguments
    /// `FileName`: File path requested by the Kernel on the FileSystem.
    /// `AllocSize`: File length to set.
    /// `DokanFileInfo`: Information about the file or directory.
    ///
    /// # Returns
    /// `STATUS_SUCCESS` on success or NTSTATUS appropriate to the request result.
    pub SetAllocationSize: Option<SetAllocationSizeCallback>,

    /// LockFile Dokan API callback
    ///
    /// Lock file at a specific offset and data length.
    /// This is only used if [DOKAN_OPTION_FILELOCK_USER_MODE] is enabled.
    ///
    /// # Arguments
    /// `FileName`: File path requested by the Kernel on the FileSystem.
    /// `ByteOffset`: Offset from where the lock has to be continued.
    /// `Length`: Data length to lock.
    /// `DokanFileInfo`: Information about the file or directory.
    ///
    /// # Returns
    /// `STATUS_SUCCESS` on success or NTSTATUS appropriate to the request result.
    ///
    /// # References
    /// See UnlockFile
    pub LockFile: Option<LockFileCallback>,

    /// UnlockFile Dokan API callback
    ///
    /// Unlock file at a specific offset and data length.
    /// This is only used if [DOKAN_OPTION_FILELOCK_USER_MODE] is enabled.
    ///
    /// # Arguments
    /// `FileName`: File path requested by the Kernel on the FileSystem.
    /// `ByteOffset`: Offset from where the lock has to be continued.
    /// `Length`: Data length to lock.
    /// `DokanFileInfo`: Information about the file or directory.
    ///
    /// # Return
    /// `STATUS_SUCCESS` on success or NTSTATUS appropriate to the request result.
    ///
    /// # Return
    /// See LockFile
    pub UnlockFile: Option<UnlockFileCallback>,

    /// GetDiskFreeSpace Dokan API callback
    ///
    /// Retrieves information about the amount of space that is available on a disk volume.
    /// It consits of the total amount of space, the total amount of free space, and
    // the total amount of free space available to the user that is associated with the calling thread.
    ///
    /// Neither GetDiskFreeSpace nor `GetVolumeInformation`
    /// save the DOKAN_FILE_INFO.Context.
    /// Before these methods are called, \ref ZwCreateFile may not be called.
    /// (ditto [CloseFile] and [Cleanup])
    ///
    /// # Arguments
    /// FreeBytesAvailable Amount of available space.
    /// `TotalNumberOfBytes`: Total size of storage space
    /// `TotalNumberOfFreeBytes`: Amount of free space
    /// `DokanFileInfo`: Information about the file or directory.
    ///
    /// # Return
    /// `STATUS_SUCCESS` on success or \c NTSTATUS appropriate to the request result.
    ///
    /// # References
    /// See <a href="https://msdn.microsoft.com/en-us/library/windows/desktop/aa364937(v=vs.85).aspx"> GetDiskFreeSpaceEx function (MSDN)</a>
    /// See GetVolumeInformation
    pub GetDiskFreeSpace: Option<GetDiskFreeSpaceCallback>,

    /// GetVolumeInformation Dokan API callback
    ///
    /// Retrieves information about the file system and volume associated with the specified root directory.
    ///
    /// Neither GetVolumeInformation nor GetDiskFreeSpace
    /// save the `DOKAN_FILE_INFO#Context`.
    /// Before these methods are called, [ZwCreateFile] may not be called.
    /// (ditto [CloseFile] and [Cleanup])
    ///
    /// VolumeName length can be anything that fit in the provided buffer.
    /// But some Windows component expect it to be no longer than 32 characters
    /// that why it is recommended to set a value under this limit.
    ///
    /// FileSystemName could be anything up to 10 characters.
    /// But Windows check few feature availability based on file system name.
    /// For this, it is recommended to set NTFS or FAT here.
    ///
    /// `FILE_READ_ONLY_VOLUME` is automatically added to the
    /// FileSystemFlags if `DOKAN_OPTION_WRITE_PROTECT` was
    /// specified in DOKAN_OPTIONS when the volume was mounted.
    ///
    /// # Arguments
    /// `VolumeNameBuffer`: A pointer to a buffer that receives the name of a specified volume.
    /// `VolumeNameSize`: The length of a volume name buffer.
    /// `VolumeSerialNumber`: A pointer to a variable that receives the volume serial number.
    /// `MaximumComponentLength`: A pointer to a variable that receives the maximum length.
    /// `FileSystemFlags`: A pointer to a variable that receives flags associated with the specified file system.
    /// `FileSystemNameBuffer`: A pointer to a buffer that receives the name of the file system.
    /// `FileSystemNameSize`: The length of the file system name buffer.
    /// `DokanFileInfo`: Information about the file or directory.
    ///
    /// # Return
    /// `STATUS_SUCCESS` on success or NTSTATUS appropriate to the request result.
    ///
    /// # References
    /// See <a href="https://msdn.microsoft.com/en-us/library/windows/desktop/aa364993(v=vs.85).aspx"> GetVolumeInformation function (MSDN)</a>
    /// See GetDiskFreeSpace
    pub GetVolumeInformation: Option<GetVolumeInformationCallback>,

    /// Mounted Dokan API callback
    ///
    /// Called when Dokan successfully mounts the volume.
    ///
    /// If [DOKAN_OPTION_MOUNT_MANAGER] is enabled and the drive letter requested is busy,
    /// the MountPoint can contain a different drive letter that the mount manager assigned us.
    ///
    /// # Arguments
    /// `MountPoint`: The mount point assign to the instance.
    /// `DokanFileInfo`: Information about the file or directory.
    ///
    /// # Return
    /// `STATUS_SUCCESS` on success or NTSTATUS appropriate to the request result. The value is currently not used by the library.
    ///
    /// # References
    /// See Unmounted
    pub Mounted: Option<MountedCallback>,

    /// Unmounted Dokan API callback
    ///
    ///Called when Dokan is unmounting the volume.
    ///
    /// # References
    /// `DokanFileInfo`: Information about the file or directory.
    ///
    /// # Return
    /// `STATUS_SUCCESS` on success or `NTSTATUS` appropriate to the request result. The value is currently not used by the library.
    ///
    /// # References
    /// See Mounted
    pub Unmounted: Option<Unmounted>,

    /// GetFileSecurity Dokan API callback
    ///
    /// Get specified information about the security of a file or directory.
    ///
    /// Return `STATUS_NOT_IMPLEMENTED` to let dokan library build a sddl of the current process user with authenticate user rights for context menu.
    /// Return `STATUS_BUFFER_OVERFLOW` if buffer size is too small.
    ///
    /// Supported since version 0.6.0. The version must be specified in `DOKAN_OPTIONS.Version`.
    ///
    /// # Parameters
    /// `FileName`: File path requested by the Kernel on the FileSystem.
    /// `SecurityInformation`: A SECURITY_INFORMATION value that identifies the security information being requested.
    /// `SecurityDescriptor`: A pointer to a buffer that receives a copy of the security descriptor of the requested file.
    /// `BufferLength`: Specifies the size, in bytes, of the buffer.
    /// `LengthNeeded`: A pointer to the variable that receives the number of bytes necessary to store the complete security descriptor.
    /// `DokanFileInfo`: Information about the file or directory.
    ///
    /// # Return
    /// `STATUS_SUCCESS` on success or NTSTATUS appropriate to the request result.
    ///
    /// # References
    /// See SetFileSecurity
    /// See <a href="https://msdn.microsoft.com/en-us/library/windows/desktop/aa446639(v=vs.85).aspx">GetFileSecurity function (MSDN)</a>
    pub GetFileSecurity: Option<GetFileSecurityCallback>,

    /// SetFileSecurity Dokan API callback
    ///
    /// Sets the security of a file or directory object.
    ///
    /// Supported since version 0.6.0. The version must be specified in `DOKAN_OPTIONS.Version`.
    ///
    /// # Arguments
    /// `FileName`: File path requested by the Kernel on the FileSystem.
    /// `SecurityInformation`: Structure that identifies the contents of the security descriptor pointed by \a SecurityDescriptor param.
    /// `SecurityDescriptor`: A pointer to a SECURITY_DESCRIPTOR structure.
    /// `BufferLength`: Specifies the size, in bytes, of the buffer.
    /// `DokanFileInfo`: Information about the file or directory.
    ///
    /// # Return
    /// `STATUS_SUCCESS` on success or NTSTATUS appropriate to the request result.
    /// References
    /// See GetFileSecurity
    /// See <a href="https://msdn.microsoft.com/en-us/library/windows/desktop/aa379577(v=vs.85).aspx">SetFileSecurity function (MSDN)</a>
    pub SetFileSecurity: Option<SetFileSecurityCallback>,

    /// FindStreams Dokan API callback
    ///
    /// Retrieve all NTFS Streams informations on the file.
    /// This is only called if `DOKAN_OPTION_ALT_STREAM` is enabled.
    ///
    /// Supported since version 0.8.0. The version must be specified in \ref DOKAN_OPTIONS.Version.
    ///
    /// # Arguments
    /// `FileName`: File path requested by the Kernel on the FileSystem.
    /// `FillFindStreamData`: Callback that has to be called with PWIN32_FIND_STREAM_DATA that contain stream information.
    /// `FindStreamContext`: Context for the event to pass to the callback FillFindStreamData.
    /// `DokanFileInfo`: Information about the file or directory.
    ///
    /// # Return
    /// `STATUS_SUCCESS` on success or NTSTATUS appropriate to the request result.
    pub FindStreams: Option<FindStreamsCallback>,
}

pub type PDOKAN_OPERATIONS = *mut DOKAN_OPERATIONS;

impl DOKAN_OPERATIONS {
    /// Make a new empty [`DOKAN_OPERATIONS`].
    pub const fn new() -> Self {
        Self {
            Cleanup: None,
            CloseFile: None,
            DeleteDirectory: None,
            DeleteFile: None,
            FindFiles: None,
            FindFilesWithPattern: None,
            FindStreams: None,
            FlushFileBuffers: None,
            GetDiskFreeSpace: None,
            GetFileInformation: None,
            GetFileSecurity: None,
            GetVolumeInformation: None,
            LockFile: None,
            Mounted: None,
            MoveFile: None,
            ReadFile: None,
            SetAllocationSize: None,
            SetEndOfFile: None,
            SetFileAttributes: None,
            SetFileSecurity: None,
            SetFileTime: None,
            UnlockFile: None,
            Unmounted: None,
            WriteFile: None,
            ZwCreateFile: None,
        }
    }
}

impl Default for DOKAN_OPERATIONS {
    fn default() -> Self {
        Self::new()
    }
}

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
