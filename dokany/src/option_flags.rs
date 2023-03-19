use crate::sys;

bitflags::bitflags! {
    /// Options to configure the driver mount
    pub struct OptionFlags: sys::DokanOptionFlag {
        /// Enable ouput debug message
        const DEBUG = sys::DOKAN_OPTION_DEBUG;
        /// Enable ouput debug message to stderr
        const STDERR = sys::DOKAN_OPTION_STDERR;
        /// Enable the use of alternate stream paths in the form
        /// <file-name>:<stream-name>. If this is not specified then the driver will
        /// fail any attempt to access a path with a colon.
        const ALT_STREAM = sys::DOKAN_OPTION_ALT_STREAM;
        /// Enable mount drive as write-protected.
        const WRITE_PROTECT = sys::DOKAN_OPTION_WRITE_PROTECT;
        /// Use network drive - Dokan network provider needs to be installed and a `DOKAN_OPTIONS.UNCName` provided
        const NETWORK = sys::DOKAN_OPTION_NETWORK;
        /// Use removable drive
        ///
        /// Be aware that on some environments, the userland application will be denied
        /// to communicate with the drive which will result in a unwanted unmount.
        /// See <a href="https://github.com/dokan-dev/dokany/issues/843">Issue #843</a>
        const REMOVABLE = sys::DOKAN_OPTION_REMOVABLE;
        /// Use Windows Mount Manager.
        ///
        /// This option is highly recommended to use for better system integration
        /// If a drive letter is used but is busy, Mount manager will assign one for us and
        /// `DOKAN_OPERATIONS.Mounted` parameters will contain the new mount point.
        const MOUNT_MANAGER = sys::DOKAN_OPTION_MOUNT_MANAGER;
        /// Mount the drive on current session only.
        const CURRENT_SESSION = sys::DOKAN_OPTION_CURRENT_SESSION;
        /// Enable Lockfile/Unlockfile operations.
        ///
        /// Otherwise Dokan will take care of it.
        const FILELOCK_USER_MODE = sys::DOKAN_OPTION_FILELOCK_USER_MODE;
        /// Enable Case sensitive path.
        ///
        /// By default all path are case insensitive.
        /// For case sensitive: \dir\File & \diR\\file are different files
        /// but for case insensitive they are the same.
        const CASE_SENSITIVE = sys::DOKAN_OPTION_CASE_SENSITIVE;

        /// Allows unmounting of network drive via explorer
        const ENABLE_UNMOUNT_NETWORK_DRIVE = sys::DOKAN_OPTION_ENABLE_UNMOUNT_NETWORK_DRIVE;

        /// Forward the kernel driver global and volume logs to the userland.
        ///
        /// Can be very slow if single thread is enabled.
        const DISPATCH_DRIVER_LOGS = sys::DOKAN_OPTION_DISPATCH_DRIVER_LOGS;

        /// Pull batches of events from the driver instead of a single one and execute them parallelly.
        ///
        /// This option should only be used on computers with low cpu count
        /// and userland filesystem taking time to process requests (like remote storage).
        const ALLOW_IPC_BATCHING = sys::DOKAN_OPTION_ALLOW_IPC_BATCHING;
    }
}
