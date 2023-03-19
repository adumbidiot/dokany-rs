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
    pub const DRIVER_INSTALL_ERROR: Self = Self(sys::DOKAN_DRIVER_INSTALL_ERROR);
    /// Dokan mount failed - Driver answer that something is wrong.
    pub const START_ERROR: Self = Self(sys::DOKAN_START_ERROR);
    /// Dokan mount failed.
    ///
    /// Can't assign a drive letter or mount point.
    /// Probably already used by another volume.
    pub const MOUNT_ERROR: Self = Self(sys::DOKAN_MOUNT_ERROR);
    /// Dokan mount failed.
    ///
    /// Mount point is invalid.
    pub const MOUNT_POINT_ERROR: Self = Self(sys::DOKAN_MOUNT_POINT_ERROR);
    /// Dokan mount failed.
    ///
    /// Requested an incompatible version.
    pub const VERSION_ERROR: Self = Self(sys::DOKAN_VERSION_ERROR);

    /// Returns true if this is [`SUCCESS`].
    pub fn is_success(self) -> bool {
        self == Self::SUCCESS
    }
}

impl From<MainResult> for Result<(), MainResult> {
    fn from(result: MainResult) -> Self {
        if result.is_success() {
            Ok(())
        } else {
            Err(result)
        }
    }
}

impl std::fmt::Display for MainResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::SUCCESS => write!(f, "mount successful"),
            Self::ERROR => write!(f, "mount failed, mount error"),
            Self::DRIVE_LETTER_ERROR => write!(f, "mount failed, invalid drive letter"),
            Self::DRIVER_INSTALL_ERROR => write!(f, "mount failed, failed to install driver"),
            Self::START_ERROR => write!(f, "mount failed, driver detected an issue"),
            Self::MOUNT_ERROR => write!(
                f,
                "mount failed, cannot assign drive letter or mount; it is likely already in use"
            ),
            Self::MOUNT_POINT_ERROR => write!(f, "mount failed, the mount point is invalid"),
            Self::VERSION_ERROR => write!(f, "mount failed, requested an incompatible version"),
            _ => write!(f, "Dokany Error Code {}", self.0),
        }
    }
}

impl Default for MainResult {
    fn default() -> Self {
        Self::SUCCESS
    }
}

impl std::error::Error for MainResult {}
