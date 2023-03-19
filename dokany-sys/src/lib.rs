#![allow(non_snake_case)]

pub use windows_sys::core::PWSTR;
pub use windows_sys::Win32::Foundation::BOOLEAN;
pub use windows_sys::Win32::Foundation::CHAR;

pub type USHORT = u16;
pub type ULONG = u32;
pub type ULONG64 = u64;
pub type LPCWSTR = PCWSTR;

/// This is arbitrary. There isn't really an absolute max, but we marshal it in
/// a fixed-size buffer.
const VOLUME_SECURITY_DESCRIPTOR_MAX_SIZE: usize = 1024 * 16;

/// Dokan mount options used to describe Dokan device behavior.
#[repr(C)]
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
