use crate::sys;

/// The error/result for mounting a filesystem.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct MainResult(pub sys::DokanMainResult);

impl MainResult {
    /// Dokan mount succeed.
    pub const SUCCESS: Self = Self(sys::DOKAN_SUCCESS);
    /// Dokan mount error.
    pub const ERROR: Self = Self(sys::DOKAN_ERROR);
    /// Dokan mount failed - Bad drive letter.
    pub const DRIVE_LETTER_ERROR: Self = Self(sys::DOKAN_DRIVE_LETTER_ERROR);
    /// Dokan mount failed - Can't install driver.
    pub const DOKAN_DRIVER_INSTALL_ERROR: Self = Self(sys::DOKAN_DRIVER_INSTALL_ERROR);
    /// Dokan mount failed - Driver answer that something is wrong.
    pub const DOKAN_START_ERROR: Self = Self(sys::DOKAN_START_ERROR);
    /// Dokan mount failed.
    ///
    /// Can't assign a drive letter or mount point.
    /// Probably already used by another volume.
    pub const DOKAN_MOUNT_ERROR: Self = Self(sys::DOKAN_MOUNT_ERROR);
    /// Dokan mount failed.
    ///
    /// Mount point is invalid.
    pub const DOKAN_MOUNT_POINT_ERROR: Self = Self(sys::DOKAN_MOUNT_POINT_ERROR);
    /// Dokan mount failed.
    ///
    /// Requested an incompatible version.
    pub const DOKAN_VERSION_ERROR: Self = Self(sys::DOKAN_VERSION_ERROR);

    /// Returns true if this is [`SUCCESS`].
    pub fn is_success(self) -> bool {
        self == Self::SUCCESS
    }
}

impl Default for MainResult {
    fn default() -> Self {
        Self::SUCCESS
    }
}
